# Installation

Pick the file for your system. Every link points at the latest release.

<Downloads />

## Linux

The `.onelf` file is a single executable that carries its own libraries,
graphics drivers included, so it runs on any distribution, musl ones
like Alpine too:

```sh
chmod +x app-template-x86_64-linux.onelf
./app-template-x86_64-linux.onelf
```

Add it to your application menu with:

```sh
./app-template-x86_64-linux.onelf --onelf-integrate
```

Update it in place, downloading only what changed since your copy:

```sh
./app-template-x86_64-linux.onelf --onelf-update
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
chmod +x app-template-anylinux-x86_64.AppImage
./app-template-anylinux-x86_64.AppImage
```
<!-- /AppImage -->

The `.tar.xz` archive holds the plain binary with its desktop entry and
icons under `share/`. It uses the display libraries your system already
has.

## macOS

Open the disk image and drag **App Template** to **Applications**. The app
is not notarized, so the first launch needs a right click and **Open**.

## Windows

Run `app-template-x86_64-windows.exe`. It needs no installer and nothing
else installed. Windows may warn that it is from an unknown publisher,
since it is not signed: choose **More info**, then **Run anyway**.

## From source

```sh
cargo install --locked --git https://github.com/pkgforge-apps/app-template
```
