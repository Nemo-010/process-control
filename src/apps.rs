//! Turning a process table into the rows the window draws.
//!
//! Two views are built from one sample. The tree view holds every process
//! under its parent, and a program that owns a window is a root of its own
//! with its helpers folded away beneath it. The application view groups
//! processes into the applications they belong to, so a browser's renderers
//! count towards the browser.
//!
//! Both start from the same rules: which processes belong to an application,
//! and which belong to none.

use std::collections::{HashMap, HashSet};

use crate::desktop::Entry;
use crate::sample::{ProcessInfo, Sample, Window};

/// One row of either view.
#[derive(Clone, Debug)]
pub struct Row {
    /// The processes the row stands for, the row's own process first.
    pub pids: Vec<i32>,
    /// The name the row shows.
    pub name: String,
    /// The icon to draw beside the name, from a desktop entry.
    pub icon: Option<String>,
    /// How deep the row sits, 0 for a root.
    pub depth: usize,
    /// The parent row's index, when it is folded under one.
    pub parent: Option<usize>,
    /// Whether the row is an application, and so draws its icon and can have
    /// its processes folded away.
    pub is_application: bool,
    /// Whether the row is folded.
    pub folded: bool,
    /// Whether the row has processes beneath it, whether they are drawn or
    /// folded away.
    ///
    /// A folded row draws no children, so this cannot be worked out from the
    /// rows alone, and without it the arrow that opens the row back up would
    /// be missing exactly when it is needed.
    pub has_children: bool,
    /// Whether the row stands for a program rather than a kernel thread.
    pub is_user: bool,
    /// The summed statistics of every process the row stands for.
    pub totals: Totals,
    /// The busiest process among those, whose details a click on the row
    /// shows when the row is an application.
    pub main: usize,
}

/// The statistics a row stands for, summed over its processes.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Totals {
    /// CPU usage, as a percentage of one core.
    pub cpu: f32,
    /// Memory held, in bytes.
    pub memory: u64,
    /// Bytes read from storage per second.
    pub disk_read: f64,
    /// Bytes written to storage per second.
    pub disk_write: f64,
    /// Bytes received over the network per second.
    pub net_read: f64,
    /// Bytes sent over the network per second.
    pub net_write: f64,
    /// The busiest GPU engine's share, as a percentage.
    pub gpu: f32,
    /// GPU memory held, in bytes.
    pub gpu_memory: u64,
    /// How many processes the row stands for.
    pub count: usize,
}

impl Totals {
    /// Adds one process to the totals.
    fn add(&mut self, process: &ProcessInfo) {
        self.cpu += process.cpu;
        self.memory += process.memory;
        self.disk_read += process.disk_read;
        self.disk_write += process.disk_write;
        self.net_read += process.net_read;
        self.net_write += process.net_write;
        // Two processes using the same GPU at once are using it more than one
        // alone, so the shares add up; memory is counted once, since the
        // driver reports it per client and a client's memory is its own.
        self.gpu += process.gpu;
        self.gpu_memory += process.gpu_memory;
        self.count += 1;
    }
}

/// The value a row is sorted by.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SortKey {
    /// The name, alphabetically.
    #[default]
    Name,
    /// CPU usage, busiest first when descending.
    Cpu,
    /// Memory, largest first when descending.
    Memory,
    /// The process id.
    Pid,
    /// Storage traffic.
    Disk,
    /// Network traffic.
    Network,
    /// GPU usage.
    Gpu,
    /// How many processes the row stands for.
    Processes,
}

impl SortKey {
    /// The name the column title carries, which is also what the settings
    /// file stores.
    pub fn name(self) -> &'static str {
        match self {
            SortKey::Name => "Name",
            SortKey::Cpu => "CPU",
            SortKey::Memory => "Memory",
            SortKey::Pid => "PID",
            SortKey::Disk => "Disk",
            SortKey::Network => "Network",
            SortKey::Gpu => "GPU",
            SortKey::Processes => "Processes",
        }
    }

    /// The key with this name, as the settings file stores it.
    pub fn from_name(name: &str) -> Option<Self> {
        [
            SortKey::Name,
            SortKey::Cpu,
            SortKey::Memory,
            SortKey::Pid,
            SortKey::Disk,
            SortKey::Network,
            SortKey::Gpu,
            SortKey::Processes,
        ]
        .into_iter()
        .find(|key| key.name().eq_ignore_ascii_case(name))
    }

    /// Whether a larger value is a better one, which is what the newest
    /// renderer sorts descending by default.
    fn prefers_larger(self) -> bool {
        !matches!(self, SortKey::Name | SortKey::Pid)
    }
}

/// What the window is showing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ViewMode {
    /// Every process under its parent.
    #[default]
    Tree,
    /// Processes grouped into the applications they belong to.
    Applications,
}

impl ViewMode {
    /// The name the View menu shows.
    pub fn name(self) -> &'static str {
        match self {
            ViewMode::Tree => "Process Tree",
            ViewMode::Applications => "Applications",
        }
    }
}

/// The applications a sample's processes belong to.
pub struct Applications {
    /// The name and icon of each application, by the pid that owns its
    /// window.
    owners: HashMap<i32, (String, Option<String>)>,
    /// The name and icon of the application each process belongs to.
    membership: HashMap<i32, (String, Option<String>)>,
    /// The windows, indexed by the process that owns them, which the details
    /// panel reads a window class from.
    windows: HashMap<i32, Window>,
}

impl Applications {
    /// Works out which processes belong to which application.
    ///
    /// A window's `_NET_WM_PID` names the process that created it, which is
    /// the application's own, and every process beneath that one in the tree
    /// belongs to the same application: a browser's renderer is a child of
    /// the browser. A process with no window and no ancestor with one belongs
    /// to no application and is listed by its own name.
    pub fn new(sample: &Sample, windows: &[Window], entries: &dyn EntryLookup) -> Self {
        let mut applications = Applications {
            owners: HashMap::new(),
            membership: HashMap::new(),
            windows: HashMap::new(),
        };
        let by_pid: HashMap<i32, usize> = sample.index();
        for window in windows {
            if window.pid <= 0 || !by_pid.contains_key(&window.pid) {
                continue;
            }
            let executable = sample
                .process(window.pid)
                .and_then(|process| process.exe.as_deref())
                .and_then(|path| path.rsplit('/').next());
            let entry = entries.matching(&window.keys(), executable);
            let name = entry
                .filter(|entry| !entry.name.is_empty())
                .map(|entry| entry.name.clone())
                .unwrap_or_else(|| {
                    window
                        .keys()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| format!("process {}", window.pid))
                });
            let icon = entry
                .map(|entry| entry.icon.clone())
                .filter(|icon| !icon.is_empty());
            applications
                .owners
                .entry(window.pid)
                .or_insert((name, icon));
            applications
                .windows
                .entry(window.pid)
                .or_insert_with(|| window.clone());
        }
        applications
    }

    /// The application a process belongs to, if any.
    ///
    /// The walk up the tree stops at a process that owns a window, so a
    /// helper started by the desktop's shell belongs to the shell only if the
    /// shell owns a window it was started from.
    pub fn resolve(&mut self, sample: &Sample) {
        let by_pid: HashMap<i32, usize> = sample.index();
        let mut resolved: HashMap<i32, Option<(String, Option<String>)>> = HashMap::new();
        for process in &sample.processes {
            let mut seen = HashSet::new();
            let mut current = process;
            let found = loop {
                if let Some(owner) = self.owners.get(&current.pid) {
                    break Some(owner.clone());
                }
                if !seen.insert(current.pid) {
                    break None;
                }
                let Some(parent) = by_pid
                    .get(&current.ppid)
                    .map(|index| &sample.processes[*index])
                else {
                    break None;
                };
                if parent.pid == current.pid {
                    break None;
                }
                current = parent;
            };
            resolved.insert(process.pid, found);
        }
        for (pid, found) in resolved {
            if let Some(found) = found {
                self.membership.insert(pid, found);
            }
        }
    }

    /// The application a process belongs to, if any.
    pub fn of(&self, pid: i32) -> Option<&(String, Option<String>)> {
        self.membership.get(&pid)
    }

    /// Whether this process is one an application's window belongs to, which
    /// is what makes it the head of an application's subtree.
    pub fn owns_window(&self, pid: i32) -> bool {
        self.owners.contains_key(&pid)
    }
}

/// The lookup an application name comes from, which the desktop entry index
/// provides and a test can stand in for.
pub trait EntryLookup {
    /// The entry whose names include one of `classes`, or whose executable is
    /// `executable`.
    fn matching(&self, classes: &[String], executable: Option<&str>) -> Option<&Entry>;
}

impl EntryLookup for crate::desktop::Index {
    fn matching(&self, classes: &[String], executable: Option<&str>) -> Option<&Entry> {
        self.matching(classes, executable)
    }
}

/// Builds the tree rows: every process under its parent.
///
/// The tree is rooted at the processes whose parent is missing from the
/// sample, which is init and anything whose parent exited. Kernel threads are
/// left out unless `kernel_threads` is set, since they are dozens of rows that
/// do nothing a user can act on.
pub fn tree(
    sample: &Sample,
    applications: &Applications,
    kernel_threads: bool,
    folded: &HashSet<i32>,
) -> Vec<Row> {
    let index = sample.index();
    let shown = |process: &ProcessInfo| kernel_threads || !process.is_kernel_thread();
    let mut children: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut roots: Vec<usize> = Vec::new();
    for (position, process) in sample.processes.iter().enumerate() {
        if !shown(process) {
            continue;
        }
        // An application is a root of its own, even when the process that
        // started it has a window too: a program launched by another one is
        // its own program, and folding the launcher away must not hide it.
        // That is what makes each application one tree.
        let own_head = applications.owns_window(process.pid);
        // Otherwise a process is a root when its parent is not in the table,
        // when it names itself, or when its parent is a kernel thread the
        // window is leaving out: a user process started by a kernel thread
        // would otherwise have no row above it and be dropped.
        match index.get(&process.ppid) {
            Some(parent)
                if !own_head
                    && *parent != position
                    && sample.processes[*parent].pid != process.pid
                    && shown(&sample.processes[*parent]) =>
            {
                children.entry(process.ppid).or_default().push(position);
            }
            _ => roots.push(position),
        }
    }

    let mut rows = Vec::new();
    let mut visited = HashSet::new();
    let mut context = Build {
        sample,
        applications,
        children: &children,
        folded,
        rows: &mut rows,
        visited: &mut visited,
    };
    for root in roots {
        context.add(root, 0, None);
    }
    // A cycle, which two processes that adopted each other make, has no root.
    // Those are listed flat rather than dropped.
    for position in 0..sample.processes.len() {
        if !context.visited.contains(&position) && shown(&sample.processes[position]) {
            context.add(position, 0, None);
        }
    }
    rows
}

/// The state one walk down the tree carries.
struct Build<'a> {
    /// The sample being walked.
    sample: &'a Sample,
    /// Which processes belong to which application.
    applications: &'a Applications,
    /// Every process's children, by the parent's pid.
    children: &'a HashMap<i32, Vec<usize>>,
    /// The processes whose children are folded away.
    folded: &'a HashSet<i32>,
    /// The rows built so far.
    rows: &'a mut Vec<Row>,
    /// The processes already given a row, so a cycle cannot be walked twice.
    visited: &'a mut HashSet<usize>,
}

impl Build<'_> {
    /// Marks everything beneath `pid` as walked, so a folded subtree is not
    /// listed by the orphan pass either.
    fn mark_hidden(&mut self, pid: i32) {
        let Some(pids) = self.children.get(&pid).cloned() else {
            return;
        };
        for child in pids {
            if self.visited.insert(child) {
                self.mark_hidden(self.sample.processes[child].pid);
            }
        }
    }

    /// Adds `position` and everything beneath it to the rows.
    fn add(&mut self, position: usize, depth: usize, parent: Option<usize>) {
        if !self.visited.insert(position) {
            return;
        }
        let process = &self.sample.processes[position];
        // A row is an application only where its window is: the head of the
        // subtree names the application and carries its icon, and the helpers
        // beneath it keep their own names, which is what makes the tree say
        // which part of a program is doing the work.
        let head = self.applications.owns_window(process.pid);
        let application = self.applications.of(process.pid).filter(|_| head);
        let mut totals = Totals::default();
        totals.add(process);
        // A row with nothing beneath it is not folded, whatever the fold set
        // says: a program that started no helpers must not draw an arrow that
        // opens onto nothing.
        let has_children = self.children.contains_key(&process.pid);
        let row = Row {
            pids: vec![process.pid],
            name: application.map_or_else(
                || process.display_name().to_string(),
                |(name, _)| name.clone(),
            ),
            icon: application.and_then(|(_, icon)| icon.clone()),
            depth,
            parent,
            is_application: head,
            folded: has_children && self.folded.contains(&process.pid),
            has_children,
            is_user: !process.is_kernel_thread(),
            totals,
            main: position,
        };
        let row_index = self.rows.len();
        self.rows.push(row);
        if has_children && self.folded.contains(&process.pid) {
            // A folded row hides everything beneath it, so those processes are
            // marked as walked rather than listed.
            self.mark_hidden(process.pid);
            return;
        }
        let Some(pids) = self.children.get(&process.pid) else {
            return;
        };
        let mut sorted = pids.clone();
        sorted.sort_unstable_by_key(|index| self.sample.processes[*index].pid);
        // A copy, since adding a row borrows the rows the list came from.
        let sorted: Vec<usize> = sorted
            .into_iter()
            .filter(|child| !self.visited.contains(child))
            .collect();
        for child in sorted {
            self.add(child, depth + 1, Some(row_index));
        }
    }
}

/// Builds the application rows: one row per application, with its processes
/// folded away beneath it.
///
/// A process that belongs to an application is not a row of its own; it is
/// counted in the application's totals. A process that belongs to none is
/// listed after the applications, in tree order, so nothing is hidden.
pub fn applications(sample: &Sample, membership: &Applications, folded: &HashSet<i32>) -> Vec<Row> {
    let mut order: Vec<(String, Option<String>, Vec<i32>)> = Vec::new();
    let mut positions: HashMap<String, usize> = HashMap::new();
    for process in &sample.processes {
        let Some((name, icon)) = membership.of(process.pid) else {
            continue;
        };
        match positions.get(name) {
            Some(position) => order[*position].2.push(process.pid),
            None => {
                positions.insert(name.clone(), order.len());
                order.push((name.clone(), icon.clone(), vec![process.pid]));
            }
        }
    }

    let index = sample.index();
    let mut rows: Vec<Row> = Vec::new();
    for (name, icon, pids) in order {
        let mut totals = Totals::default();
        let mut busiest = 0;
        let mut busiest_cpu = f32::MIN;
        for pid in &pids {
            let Some(position) = index.get(pid) else {
                continue;
            };
            let process = &sample.processes[*position];
            totals.add(process);
            if process.cpu > busiest_cpu {
                busiest_cpu = process.cpu;
                busiest = *position;
            }
        }
        let key = pids.first().copied().unwrap_or_default();
        let has_children = pids.len() > 1;
        let open = !folded.contains(&key);
        rows.push(Row {
            pids: pids.clone(),
            name,
            icon,
            depth: 0,
            parent: None,
            is_application: true,
            folded: has_children && !open,
            has_children,
            is_user: true,
            totals,
            main: busiest,
        });
        // An application that is not folded lists the processes it stands
        // for beneath it, so a stop can land on one helper rather than on the
        // whole program.
        if open {
            let parent = rows.len() - 1;
            let mut sorted = pids.clone();
            sorted.sort_unstable();
            for pid in sorted {
                let Some(position) = index.get(&pid) else {
                    continue;
                };
                let process = &sample.processes[*position];
                let mut totals = Totals::default();
                totals.add(process);
                rows.push(Row {
                    pids: vec![pid],
                    name: process.display_name().to_string(),
                    icon: None,
                    depth: 1,
                    parent: Some(parent),
                    is_application: false,
                    folded: false,
                    has_children: false,
                    is_user: !process.is_kernel_thread(),
                    totals,
                    main: *position,
                });
            }
        }
    }
    // The processes that belong to no application keep their own names and
    // are listed under the applications, in the order the tree gives them.
    let tree_rows = tree(sample, membership, false, folded);
    for row in tree_rows {
        let pid = row.pids.first().copied().unwrap_or_default();
        if membership.of(pid).is_some() {
            continue;
        }
        rows.push(Row {
            parent: None,
            depth: 0,
            is_application: false,
            ..row
        });
    }
    rows
}

/// Sorts rows by `key`, keeping each parent directly above its children.
///
/// A row's children are the rows whose `parent` names it, so sorting a sibling
/// group reorders that group and leaves every subtree attached to its head: a
/// browser's helpers follow the browser wherever it lands. The parent links
/// are rewritten to the new positions, which is what keeps the next frame's
/// drawing and the next sort from reading a stale index.
pub fn sort(rows: &mut [Row], key: SortKey, descending: bool) {
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); rows.len()];
    let mut roots: Vec<usize> = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        match row.parent {
            Some(parent) if parent < rows.len() && parent != index => children[parent].push(index),
            _ => roots.push(index),
        }
    }
    for list in &mut children {
        list.sort_by(|left, right| compare(&rows[*left], &rows[*right], key, descending));
    }
    roots.sort_by(|left, right| compare(&rows[*left], &rows[*right], key, descending));

    let mut order: Vec<usize> = Vec::with_capacity(rows.len());
    let mut seen = vec![false; rows.len()];
    // An explicit stack rather than recursion, so a deep tree is one a thread
    // with a small stack can still sort. A row is emitted before its children,
    // which is what keeps a head above its subtree.
    let mut stack: Vec<usize> = roots.iter().rev().copied().collect();
    while let Some(index) = stack.pop() {
        if std::mem::replace(&mut seen[index], true) {
            continue;
        }
        order.push(index);
        for child in children[index].iter().rev() {
            if !seen[*child] {
                stack.push(*child);
            }
        }
    }
    // A parent link that forms a cycle has no root, so those rows are walked
    // from themselves rather than dropped.
    for index in 0..rows.len() {
        if seen[index] {
            continue;
        }
        let mut stack = vec![index];
        while let Some(index) = stack.pop() {
            if std::mem::replace(&mut seen[index], true) {
                continue;
            }
            order.push(index);
            for child in children[index].iter().rev() {
                if !seen[*child] {
                    stack.push(*child);
                }
            }
        }
    }

    let mut positions = vec![0usize; rows.len()];
    for (position, index) in order.iter().enumerate() {
        positions[*index] = position;
    }
    let mut sorted: Vec<Row> = order.iter().map(|index| rows[*index].clone()).collect();
    for row in &mut sorted {
        row.parent = row.parent.map(|parent| positions[parent]);
    }
    rows.clone_from_slice(&sorted);
}

/// Compares two rows by one key.
fn compare(left: &Row, right: &Row, key: SortKey, descending: bool) -> std::cmp::Ordering {
    let ordering = match key {
        SortKey::Name => left.name.to_lowercase().cmp(&right.name.to_lowercase()),
        SortKey::Cpu => left.totals.cpu.total_cmp(&right.totals.cpu),
        SortKey::Memory => left.totals.memory.cmp(&right.totals.memory),
        SortKey::Pid => left.pids.first().cmp(&right.pids.first()),
        SortKey::Disk => (left.totals.disk_read + left.totals.disk_write)
            .total_cmp(&(right.totals.disk_read + right.totals.disk_write)),
        SortKey::Network => (left.totals.net_read + left.totals.net_write)
            .total_cmp(&(right.totals.net_read + right.totals.net_write)),
        SortKey::Gpu => left.totals.gpu.total_cmp(&right.totals.gpu),
        SortKey::Processes => left.totals.count.cmp(&right.totals.count),
    };
    // Ties keep a stable order, so the list does not jitter between two
    // samples that changed nothing.
    let ordering = ordering.then_with(|| {
        left.pids
            .first()
            .copied()
            .unwrap_or_default()
            .cmp(&right.pids.first().copied().unwrap_or_default())
    });
    if descending {
        ordering.reverse()
    } else {
        ordering
    }
}

/// The direction a key sorts in the first time it is chosen.
pub fn default_direction(key: SortKey) -> bool {
    key.prefers_larger()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process(pid: i32, ppid: i32, name: &str) -> ProcessInfo {
        ProcessInfo {
            pid,
            ppid,
            name: name.to_string(),
            virtual_memory: 4096,
            ..ProcessInfo::default()
        }
    }

    fn sample(processes: Vec<ProcessInfo>) -> Sample {
        Sample {
            processes,
            ..Sample::default()
        }
    }

    /// A stand-in for the desktop entry index.
    struct NoApplications;

    impl EntryLookup for NoApplications {
        fn matching(&self, _classes: &[String], _executable: Option<&str>) -> Option<&Entry> {
            None
        }
    }

    fn window(pid: i32, class: &str) -> Window {
        Window {
            id: pid as u32,
            pid,
            class: class.to_string(),
            instance: class.to_lowercase(),
            title: class.to_string(),
        }
    }

    fn tree_of(processes: Vec<ProcessInfo>, folded: &[i32]) -> Vec<Row> {
        let sample = sample(processes);
        let applications = Applications::new(&sample, &[], &NoApplications);
        tree(
            &sample,
            &applications,
            true,
            &folded.iter().copied().collect(),
        )
    }

    #[test]
    fn the_tree_holds_every_process_under_its_parent() {
        let rows = tree_of(
            vec![
                process(1, 0, "init"),
                process(2, 1, "child"),
                process(3, 2, "grandchild"),
            ],
            &[],
        );
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].pids, [1]);
        assert_eq!(rows[0].depth, 0);
        assert_eq!(rows[1].pids, [2]);
        assert_eq!(rows[1].depth, 1);
        assert_eq!(rows[1].parent, Some(0));
        assert_eq!(rows[2].pids, [3]);
        assert_eq!(rows[2].depth, 2);
        assert_eq!(rows[2].parent, Some(1));
    }

    #[test]
    fn a_folded_row_hides_the_ones_beneath_it() {
        let rows = tree_of(
            vec![
                process(1, 0, "init"),
                process(2, 1, "child"),
                process(3, 2, "grandchild"),
            ],
            &[2],
        );
        assert_eq!(rows.len(), 2, "the grandchild is folded away");
        assert!(rows[1].folded);
    }

    #[test]
    fn a_process_whose_parent_is_gone_is_a_root() {
        let rows = tree_of(vec![process(1, 0, "init"), process(50, 49, "orphan")], &[]);
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.depth == 0));
    }

    #[test]
    fn an_application_launched_by_another_is_its_own_tree() {
        // A terminal starts an editor, and both have windows. Folding the
        // terminal away must not hide the editor.
        let mut sample = sample(vec![
            process(1, 0, "init"),
            process(100, 1, "kitty"),
            process(200, 100, "code"),
        ]);
        for process in &mut sample.processes {
            process.exe = Some(format!("/usr/bin/{}", process.name));
        }
        let windows = [window(100, "kitty"), window(200, "Code")];
        let mut applications = Applications::new(&sample, &windows, &NoApplications);
        applications.resolve(&sample);
        let rows = tree(&sample, &applications, true, &[100].into_iter().collect());
        let names: Vec<&str> = rows.iter().map(|row| row.name.as_str()).collect();
        assert!(
            names.contains(&"Code"),
            "the editor is not hidden: {names:?}"
        );
        assert_eq!(
            rows.iter()
                .find(|row| row.name == "Code")
                .map(|row| row.depth),
            Some(0),
            "it is a root of its own"
        );
    }

    #[test]
    fn a_cycle_is_listed_rather_than_dropped() {
        // Two processes that name each other as their parent have no root,
        // which a naive walk would follow forever.
        let rows = tree_of(vec![process(10, 11, "a"), process(11, 10, "b")], &[]);
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn kernel_threads_are_left_out_unless_asked_for() {
        let sample = sample(vec![
            process(1, 0, "init"),
            ProcessInfo {
                pid: 2,
                ppid: 1,
                name: "kthreadd".to_string(),
                ..ProcessInfo::default()
            },
        ]);
        let applications = Applications::new(&sample, &[], &NoApplications);
        let without = tree(&sample, &applications, false, &HashSet::new());
        assert_eq!(without.len(), 1);
        let with = tree(&sample, &applications, true, &HashSet::new());
        assert_eq!(with.len(), 2);
    }

    #[test]
    fn a_window_is_matched_to_its_process_and_the_processes_beneath_it() {
        let mut sample = sample(vec![
            process(1, 0, "init"),
            process(100, 1, "firefox"),
            process(101, 100, "Web Content"),
            process(102, 1, "unrelated"),
        ]);
        for process in &mut sample.processes {
            process.exe = Some(format!("/usr/bin/{}", process.name));
        }
        let windows = [window(100, "Navigator")];
        let mut applications = Applications::new(&sample, &windows, &NoApplications);
        applications.resolve(&sample);
        assert_eq!(
            applications.of(100).map(|(name, _)| name.as_str()),
            Some("Navigator")
        );
        assert_eq!(
            applications.of(101).map(|(name, _)| name.as_str()),
            Some("Navigator"),
            "a helper belongs to the application its parent is"
        );
        assert!(applications.of(102).is_none());
        assert!(applications.of(1).is_none());

        let rows = applications_view(&sample, &applications, &[]);
        let named: Vec<&str> = rows.iter().map(|row| row.name.as_str()).collect();
        assert!(named.contains(&"Navigator"));
        assert!(
            named.contains(&"unrelated"),
            "a loose process is still listed"
        );
        let navigator = rows.iter().find(|row| row.name == "Navigator").unwrap();
        assert_eq!(navigator.totals.count, 2, "the helper is counted with it");
        assert!(navigator.is_application);
    }

    fn applications_view(sample: &Sample, known: &Applications, folded: &[i32]) -> Vec<Row> {
        applications(sample, known, &folded.iter().copied().collect())
    }

    #[test]
    fn an_application_row_sums_its_processes() {
        let mut sample = sample(vec![
            process(1, 0, "init"),
            process(100, 1, "thing"),
            process(101, 100, "thing helper"),
        ]);
        for process in &mut sample.processes {
            process.cpu = 2.0;
            process.memory = 1024;
        }
        let windows = [window(100, "Thing")];
        let mut applications = Applications::new(&sample, &windows, &NoApplications);
        applications.resolve(&sample);
        let rows = applications_view(&sample, &applications, &[]);
        let thing = rows.iter().find(|row| row.name == "Thing").unwrap();
        assert_eq!(thing.totals.cpu, 4.0);
        assert_eq!(thing.totals.memory, 2048);
        assert_eq!(thing.totals.count, 2);
        assert_eq!(thing.parent, None, "an application row is a root");
    }

    /// A row at `depth` under `parent`, which is the index of its head.
    fn row(name: &str, depth: usize, parent: Option<usize>, cpu: f32) -> Row {
        Row {
            pids: vec![name.len() as i32],
            name: name.to_string(),
            icon: None,
            depth,
            parent,
            is_application: false,
            folded: false,
            has_children: false,
            is_user: true,
            totals: Totals {
                cpu,
                count: 1,
                ..Totals::default()
            },
            main: 0,
        }
    }

    #[test]
    fn siblings_sort_without_moving_children_away_from_their_parent() {
        let mut rows = vec![
            row("b", 0, None, 1.0),
            row("b child", 1, Some(0), 5.0),
            row("a", 0, None, 9.0),
            row("a child", 1, Some(2), 2.0),
        ];
        sort(&mut rows, SortKey::Name, false);
        let names: Vec<&str> = rows.iter().map(|row| row.name.as_str()).collect();
        assert_eq!(names, ["a", "a child", "b", "b child"]);

        sort(&mut rows, SortKey::Cpu, true);
        let names: Vec<&str> = rows.iter().map(|row| row.name.as_str()).collect();
        assert_eq!(
            names,
            ["a", "a child", "b", "b child"],
            "children stay under their parent even when sorted by CPU"
        );
    }

    #[test]
    fn a_sort_key_round_trips_through_its_name() {
        for key in [
            SortKey::Name,
            SortKey::Cpu,
            SortKey::Memory,
            SortKey::Pid,
            SortKey::Disk,
            SortKey::Network,
            SortKey::Gpu,
            SortKey::Processes,
        ] {
            assert_eq!(SortKey::from_name(key.name()), Some(key));
        }
        assert_eq!(SortKey::from_name("Disk"), Some(SortKey::Disk));
        assert_eq!(SortKey::from_name("nonsense"), None);
        assert!(!default_direction(SortKey::Name));
        assert!(default_direction(SortKey::Cpu));
    }
}
