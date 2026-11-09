# Introduction

Process Control is a task manager: it shows what your machine is running and
lets you act on it.

It is an alternative to Mission Center, GNOME's System Monitor and the other
GTK task managers. Those take a moment to open and can freeze while the machine
is loaded, which is precisely when a task manager is opened. This one is a
single native Rust binary drawn with egui, and its process table is read on a
thread of its own, so the window stays responsive whatever the machine is
doing.

What makes it different is that it shows applications rather than only
processes. A program's window tells the display server which process owns it
and what its window class is; Process Control matches that against the desktop
entries on your system, so a browser is one row with its own name and icon and
its renderer processes folded away beneath it. That works whatever the program
was packaged as, since nothing here goes through Flatpak, Snap or systemd.

The numbers are per process as well as for the machine: CPU per core and in
total, memory and swap, GPU usage, and network and storage traffic, each with a
history across the top of the window. Terminate, force stop or send any signal
from the row or from the details panel.
