/** The app's identity, shared by the site config and the theme. */
export const app = {
  name: 'App Template',
  slug: 'app-template',
  description: 'A lightweight native desktop app built with egui',
  accent: '#5b4cf0',
  /** Whether releases carry an AppImage: `appimage` in Cargo.toml's [package.metadata.pkgforge]. */
  appimage: false,
  get repo() {
    return `https://github.com/pkgforge-apps/${this.slug}`
  },
}
