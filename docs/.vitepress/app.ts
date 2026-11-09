/** The app's identity, shared by the site config and the theme. */
export const app = {
  name: 'Process Control',
  slug: 'process-control',
  description: 'A fast, light task manager',
  accent: '#2fbf8f',
  /** Whether releases carry an AppImage: `appimage` in Cargo.toml's [package.metadata.pkgforge]. */
  appimage: true,
  get repo() {
    return `https://github.com/pkgforge-apps/${this.slug}`
  },
}
