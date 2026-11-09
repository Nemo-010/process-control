# Installation

Pick the file for your system. Every link points at the latest release.

<Downloads />

## Linux

The `.onelf` file is a single executable that carries its own libraries,
graphics drivers included, so it runs on any distribution, musl ones
like Alpine too:

```sh
chmod +x process-control-x86_64-linux.onelf
./process-control-x86_64-linux.onelf
```

Add it to your application menu with:

```sh
./process-control-x86_64-linux.onelf --onelf-integrate
```

Update it in place, downloading only what changed since your copy:

```sh
./process-control-x86_64-linux.onelf --onelf-update
```

<!-- AppImage: keep this part only when Cargo.toml sets appimage = true
under [package.metadata.pkgforge], and set appimage in .vitepress/app.ts
to match. -->
The `.AppImage` file is the same app as an
[Anylinux AppImage](https://github.com/pkgforge-dev/Anylinux-AppImages),
for AppImage managers such as [AM](https://github.com/ivan-hc/AM) or
[soar](https://github.com/pkgforge/soar). It needs no FUSE and
updates itself:

```sh
chmod +x process-control-anylinux-x86_64.AppImage
./process-control-anylinux-x86_64.AppImage
```
<!-- /AppImage -->

The `.tar.xz` archive holds the plain binary with its desktop entry and
icons under `share/`. It uses the display libraries your system already
has.

Process Control reads the process table, which only Linux has, and it builds
for no other system. It asks the X display which window belongs to which
process, so on a Wayland session it sees the Xwayland windows, which is every
X11 program and most of what a desktop runs.

## From source

```sh
cargo install --locked --git https://github.com/pkgforge-apps/process-control
```
