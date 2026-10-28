//! The Oracle Suite: the apps the launcher knows about.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`photocraft`).
    pub id: &'static str,
    /// Display name (`PhotoCraft`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// Brand colour (sRGB), for surfaces: the app's own colour.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (PdfCraft shipped as PrintCraft until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (PdfCraft's MSI said PrintCraft until 0.2.1).
    pub former_names: &'static [&'static str],
}

impl CraftApp {
    /// `app.oracle.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        format!("app.oracle.{}", self.id)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's page: its repository until Oracle has a website.
    pub fn website(&self) -> String {
        self.repo_url()
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp { id, name, tagline, color, ink_light, ink_dark, repo, former_ids: &[], former_names: &[] }
}

/// The Oracle Suite, in display order.
pub const SUITE: [CraftApp; 9] = [
    app("oracle-edit", "Oracle Edit", "Image editing", [[0xd3, 0x1c, 0x3b], [0xa3, 0x12, 0x2b], [0xf0, 0x56, 0x6f]], "Marvingxx/oracle-edit"),
    app(
        "oracle-vector",
        "Oracle Vector",
        "Vector illustration",
        [[0xff, 0x6a, 0x3d], [0xb8, 0x45, 0x1f], [0xff, 0x9a, 0x78]],
        "Marvingxx/oracle-vector",
    ),
    app("oracle-brand-kit", "Oracle Brand Kit", "Brand identity and client deliveries", [[0x1d, 0x1d, 0x1b], [0x1d, 0x1d, 0x1b], [0xd8, 0xd6, 0xd0]], "Marvingxx/oracle-brand"),
    app("oracle-mockup-studio", "Oracle Mockup Studio", "2D mockups", [[0x1d, 0x1d, 0x1b], [0x1d, 0x1d, 0x1b], [0xd8, 0xd6, 0xd0]], "Marvingxx/oracle-brand"),
    app("oracle-layout", "Oracle Layout", "Page layout, brand guidelines and presentations", [[0x1d, 0x1d, 0x1b], [0x1d, 0x1d, 0x1b], [0xd8, 0xd6, 0xd0]], "Marvingxx/oracle-guide"),
    app("oracle-motion", "Oracle Motion", "Motion graphics and compositing", [[0x8a, 0x3f, 0xfc], [0x5f, 0x24, 0xc4], [0xb0, 0x85, 0xff]], "Marvingxx/oracle-motion"),
    app("oracle-video", "Oracle Video", "Video editing", [[0x1f, 0x6f, 0xeb], [0x14, 0x4f, 0xb0], [0x6a, 0xa4, 0xff]], "Marvingxx/oracle-video"),
    app("oracle-assets", "Oracle Assets", "Your assets, outside the apps", [[0x0e, 0x8a, 0x6f], [0x0a, 0x63, 0x50], [0x4c, 0xc2, 0xa5]], "Marvingxx/oracle-brand"),
    app("oracle-font", "Oracle Font", "Fonts and type tests", [[0x6d, 0x4a, 0xff], [0x4c, 0x2e, 0xc4], [0xa8, 0x94, 0xff]], "Marvingxx/oracle-brand"),
];

/// Oracle's website. Until there is one, the suite's GitHub account.
pub const WEBSITE_URL: &str = GITHUB_URL;
/// The GitHub account every Oracle app's repository lives under: the only one the launcher
/// installs from, whatever the release feed says.
///
/// Releases are published from these repositories (`Marvingxx/oracle-edit`, …); a repository
/// that doesn't exist yet simply has no release.
pub const GITHUB_OWNER: &str = "Marvingxx";
/// Oracle on GitHub: every Oracle app's source.
pub const GITHUB_URL: &str = "https://github.com/Marvingxx";
/// Oracle's social accounts: (service, URL). None yet.
pub const SOCIAL: [(&str, &str); 0] = [];

/// Oracle has no Discord community yet; kept so the shell's link plumbing still compiles.
pub const DISCORD_URL: &str = GITHUB_URL;
/// Deep link into the Discord application; unused while [`DISCORD_URL`] is not a Discord invite.
pub const DISCORD_APP_URL: &str = GITHUB_URL;

/// Oracle Suite itself, described like an Oracle app so it can update itself with the same
/// machinery (asset names `oracle-suite-<ver>-…`, bundle id `app.oracle.oracle-suite`,
/// `Oracle Suite.app`). Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = app(
    "oracle-suite",
    "Oracle Suite",
    "Installs and updates the Oracle apps",
    [[0xd3, 0x1c, 0x3b], [0xa3, 0x12, 0x2b], [0xf0, 0x56, 0x6f]],
    "Marvingxx/oracle-suite",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "Marvingxx/oracle-suite";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "oracle-suite";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_apps_have_unique_ids_under_the_owner() {
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            // Each app's repo is its id, except Brand Kit and Mockup Studio, built from one repo
            // (`oracle-brand`) whose releases carry both apps' packages.
            let repo = match a.id {
                "oracle-brand-kit" | "oracle-mockup-studio" | "oracle-assets" | "oracle-font" => "oracle-brand",
                // Oracle Layout was Oracle Brand Guide: its repository keeps that name for now.
                "oracle-layout" => "oracle-guide",
                id => id,
            };
            assert_eq!(a.repo, format!("{GITHUB_OWNER}/{repo}"));
            assert_eq!(a.name.to_lowercase().replace(' ', "-"), a.id);
        }
    }

    #[test]
    fn website_falls_back_to_the_repository() {
        let edit = CraftApp::by_id("oracle-edit").unwrap();
        assert_eq!(edit.website(), "https://github.com/Marvingxx/oracle-edit");
        assert_eq!(edit.bundle_id(), "app.oracle.oracle-edit");
    }
}
