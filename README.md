<p align="center">
  <img src="assets/icon.svg" width="128" alt="">
</p>

<h1 align="center">App Template</h1>

<p align="center">A lightweight native desktop app built with egui</p>

<p align="center">
  <a href="https://pkgforge-apps.github.io/app-template/">Documentation</a> |
  <a href="https://github.com/pkgforge-apps/app-template/releases/latest">Latest release</a>
</p>

## Install

Download the file for your platform from the
[latest release](https://github.com/pkgforge-apps/app-template/releases/latest).

| Platform | File |
|----------|------|
| Linux, portable | `app-template-<arch>-linux.onelf` |
| Linux, AppImage | `app-template-anylinux-<arch>.AppImage` |
| Linux | `app-template-<arch>-linux.tar.xz` |
| macOS | `app-template-<arch>-macos.dmg` |
| Windows | `app-template-x86_64-windows.exe` |

The `.onelf` file is a single executable that carries its own libraries.
Make it executable and run it:

```sh
chmod +x app-template-x86_64-linux.onelf
./app-template-x86_64-linux.onelf
```

Run it once with `--onelf-integrate` to add it to your application menu,
and update it in place with `--onelf-update`, which downloads only what
changed.

<!-- AppImage: keep this section, and the table row above, only when
Cargo.toml sets appimage = true under [package.metadata.pkgforge]. -->
The `.AppImage` file is the same app as an
[Anylinux AppImage](https://github.com/pkgforge-dev/Anylinux-AppImages),
for AppImage managers such as [AM](https://github.com/ivan-hc/AM) or
[soar](https://github.com/pkgforge/soar). It runs the same way and
updates itself:

```sh
chmod +x app-template-anylinux-x86_64.AppImage
./app-template-anylinux-x86_64.AppImage
```
<!-- /AppImage -->

## Build from source

```sh
cargo build --release --locked
./target/release/app-template
```

## License

[MIT](LICENSE)
