//! Hand-written product pages: what each app is, in the launcher's own words.
//!
//! The text is compiled in, so it works offline and on the web, and nothing from the network is
//! ever shown as page copy. What changes with every release (version, date, size, notes) comes
//! from the GitHub release instead. An app without an entry here simply has no details page.
//!
//! The pictures are not compiled in: they are fetched when a page is opened (see
//! [`AppDetails::screenshot_url`]), so the repository and the binary stay small.

/// One feature: a short name and a sentence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Feature {
    pub title: &'static str,
    pub text: &'static str,
}

/// A screenshot: `key` names the file to fetch (`<app>/<key>.jpg` in the pictures repository).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shot {
    pub key: &'static str,
    pub caption: &'static str,
}

// Unused until Oracle publishes pictures for its pages.
#[allow(dead_code)]
const fn shot(key: &'static str, caption: &'static str) -> Shot {
    Shot { key, caption }
}

/// The editorial half of an app's page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppDetails {
    /// Matches [`crate::CraftApp::id`].
    pub id: &'static str,
    /// One line under the name.
    pub headline: &'static str,
    /// One or two paragraphs, separated by a blank line.
    pub about: &'static str,
    pub features: &'static [Feature],
    /// The first is the lead picture. All are 8:5.
    pub shots: &'static [Shot],
    /// Where it runs, for display (`"macOS, Windows, Linux, web"`).
    pub platforms: &'static str,
}

const fn feature(title: &'static str, text: &'static str) -> Feature {
    Feature { title, text }
}

/// Where the pictures live: one 1200 px wide, 8:5 JPEG per [`Shot`], at `<app>/<key>.jpg`.
///
/// Pinned to a tag (`SCREENSHOTS_REV`) so that what a launcher fetches never changes under it, and
/// so that a saved copy never goes stale. To change a picture, publish a new tag and bump the
/// revision: older launchers keep working from the tag they know.
pub const SCREENSHOTS_BASE: &str = "https://raw.githubusercontent.com/Marvingxx/oracle-suite-assets";

/// The tag of the pictures repository the launcher reads. Also part of the saved copies' path.
pub const SCREENSHOTS_REV: &str = "v1";

/// The largest picture the launcher accepts. The real ones are 60 to 150 KB.
pub const MAX_SCREENSHOT_BYTES: usize = 2 << 20;

/// True when `bytes` start like a JPEG file. A captive portal's login page, a proxy's error page or
/// a truncated answer is refused before it is saved or decoded.
pub fn looks_like_jpeg(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xFF, 0xD8, 0xFF])
}

/// Where every Oracle app runs.
const PLATFORMS: &str = "macOS, Windows, Linux, FreeBSD, web";

pub const DETAILS: [AppDetails; 9] = [
    AppDetails {
        id: "oracle-edit",
        headline: "The image editor you already know how to use.",
        about: "Layers, masks, adjustment layers, layer styles, type, vectors and brushes in a native app \
            written in Rust. Open source, offline, and yours.\n\n\
            Oracle Edit keeps the menus, shortcuts and panels your hands already know, so there is \
            nothing to relearn.",
        features: &[
            feature("Native GPU canvas", "No Electron and no web view: a fast native compositor."),
            feature("Real layered files", "Opens and saves layered PSD and PSB documents."),
            feature("Non-destructive editing", "Adjustment layers, live-preview filters, smart objects and layer styles."),
            feature("On-device selection", "Subject and object selection and mask refinement run locally."),
            feature("Serious color", "8-, 16- and 32-bit RGB, CMYK, Lab and Grayscale, with ICC color management and soft proofing."),
        ],
        // No pictures until Oracle publishes its own; nothing is fetched for an empty list.
        shots: &[],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "oracle-vector",
        headline: "Vector illustration, reimagined in pure Rust.",
        about: "A native vector illustration app with the pen, panels and shortcuts illustrators expect. \
                Open source, and it also runs in a browser.",
        features: &[
            feature(
                "The workflow you know",
                "Pen, direct selection, shape booleans, snapping guides and an appearance panel, laid out the way you expect.",
            ),
            feature("Genuinely fast", "Drawings with thousands of shapes stay responsive on a retina display."),
            feature(
                "Booleans that don't fail",
                "Shape booleans work on the exact curves, so there is no \"cannot perform operation\", and undo is unlimited.",
            ),
            feature("Live, editable effects", "Blends, envelope distort, gradient mesh, repeats, glows and drop shadows all stay editable."),
            feature("Image tracing", "Turn raster images into clean vector paths, with presets for the common cases."),
            feature("Open formats", "Native JSON documents, SVG and PDF import and export, and PNG, JPEG and WebP output."),
        ],
        // No pictures until Oracle publishes its own; nothing is fetched for an empty list.
        shots: &[],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "oracle-brand-kit",
        headline: "Your brand's source of truth, and the files your client receives.",
        about: "Logos sorted by role, palette, typefaces and rules, then every delivery file made from your masters in one \
                click: SVG, PDF, PNG, WebP and JPEG, with a README and a manifest. Local, no account, no AI.",
        features: &[
            feature("Masters never touched", "Exports are made from your files and recorded with their hash; delivered folders are never rewritten."),
            feature("Export presets", "Web, avatar, favicon, print at 300 dpi: sizes, margins, backgrounds and names you control."),
            feature("Change detection", "A modified logo is spotted, and only what you choose is made again."),
        ],
        shots: &[],
        platforms: "Windows",
    },
    AppDetails {
        id: "oracle-mockup-studio",
        headline: "Your logos on real photos, in two dimensions.",
        about: "Four corners, masks, a foreground and overlays prepared once per template; then your Brand Kit logos land on \
                them, sharp, in one render. No AI, no 3D.",
        features: &[
            feature("Linked to the Brand Kit", "Change a logo and the scenes that use it say so; render again with nothing to place again."),
            feature("Real perspective", "A true four-corner perspective, with masks and objects that stay in front."),
            feature("PNG, JPEG, WebP", "At the photo's size or any width, one scene or a batch."),
        ],
        shots: &[],
        platforms: "Windows",
    },
    AppDetails {
        id: "oracle-guide",
        headline: "Brand guidelines that follow your brand.",
        about: "A page-layout app forked from DesignCraft, with master pages, styles and PDF export, whose colours, type, \
                logos and mockups stay linked to your Brand Kit.",
        features: &[
            feature("Linked blocks", "Colour values, typefaces, rules, logos and mockups update when you ask, with a preview of every change."),
            feature("Your own templates", "Reuse a guide for another brand: logos by role, colours by position."),
            feature("PDF export", "Bleed, crop marks and embedded fonts."),
        ],
        shots: &[],
        platforms: "Windows",
    },
    AppDetails {
        id: "oracle-motion",
        headline: "Motion graphics and visual effects, in the Oracle suite.",
        about: "Compositions, layers, keyframes, effects, expressions, 3D cameras and lights, and a render queue, forked \
                from EffectCraft. Your Oracle Vector logos and Oracle Edit pictures come in from the library.",
        features: &[
            feature("Animate anything", "Every property keyframes, with the graph editor, easing and expressions."),
            feature("Effects and compositing", "Hundreds of effects, masks, track mattes and blending modes."),
            feature("Render queue", "Video, image sequences and Lottie, from the same project."),
        ],
        shots: &[],
        platforms: "Windows",
    },
    AppDetails {
        id: "oracle-video",
        headline: "Video editing, in the Oracle suite.",
        about: "Projects, bins, sequences and a multitrack timeline, trimming tools, effects, colour grading, scopes, \
                audio mixing, captions and exports, forked from FilmCraft. Your Oracle Vector logos and Oracle Edit \
                pictures come in from the library, and follow when you save them.",
        features: &[
            feature("Edit on a timeline", "Ripple, roll, slip and slide, multicam, nesting and markers."),
            feature("Colour and sound", "Grading with scopes, and an audio mixer with loudness meters."),
            feature("Export", "H.264, ProRes and image stills, from the same project."),
        ],
        shots: &[],
        platforms: "Windows",
    },
    AppDetails {
        id: "oracle-assets",
        headline: "Your assets, outside the apps.",
        about: "The Oracle Library as an app of its own: textures, brushes, fonts, vectors, mockups and the brands of \
                Brand Kit, searched in French and English, with favourites and tags. Drag any of them to any app.",
        features: &[
            feature("Find anything", "Search by word, format, section or tag, in French or English."),
            feature("Your brands", "Each Brand Kit brand as a collection: its logos, palette and fonts."),
            feature("Drag anywhere", "Drop an asset into an Oracle app or any other software."),
        ],
        shots: &[],
        platforms: "Windows",
    },
    AppDetails {
        id: "oracle-font",
        headline: "Every font at hand, tried before use.",
        about: "The system's fonts, the library's and the brands' from Brand Kit: a sample line in each, then a waterfall, \
                a paragraph, the glyphs, side-by-side comparisons, heading and body pairings and the text on a brand's colours.",
        features: &[
            feature("Try them out", "Your own text in every font, variable axes included."),
            feature("Pair and compare", "Headings with body text, several fonts side by side, contrast on brand colours."),
            feature("Activate and share", "Activate a font without administrator rights, add it to the library or to a brand."),
        ],
        shots: &[],
        platforms: "Windows",
    },
];

impl AppDetails {
    /// Where the picture `key` of this app is published, or `None` if the page has no such picture.
    /// The only way to turn a request into an address: callers never build one from their own text.
    pub fn screenshot_url(&self, key: &str) -> Option<String> {
        self.shots.iter().find(|s| s.key == key).map(|s| format!("{SCREENSHOTS_BASE}/{SCREENSHOTS_REV}/{}/{}.jpg", self.id, s.key))
    }

    pub fn by_id(id: &str) -> Option<&'static AppDetails> {
        DETAILS.iter().find(|d| d.id == id)
    }

    pub fn paragraphs(&self) -> impl Iterator<Item = &'static str> {
        self.about.split("\n\n").map(str::trim).filter(|p| !p.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CraftApp, SUITE};

    #[test]
    fn every_page_belongs_to_a_suite_app_once() {
        let mut ids: Vec<_> = DETAILS.iter().map(|d| d.id).collect();
        for id in &ids {
            assert!(CraftApp::by_id(id).is_some(), "{id}");
        }
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), DETAILS.len());
    }

    #[test]
    fn every_app_has_a_page() {
        for a in &SUITE {
            assert!(AppDetails::by_id(a.id).is_some(), "{}", a.id);
        }
    }

    #[test]
    fn picture_addresses_come_only_from_the_pages_own_pictures() {
        // Oracle publishes no pictures yet, so no key names one.
        let page = AppDetails::by_id("oracle-edit").unwrap();
        for key in ["demo", "", "nope", "../oracle-vector/neon", "demo/../masks", "demo.jpg", "/etc/passwd", "demo?x=1", "demo#"] {
            assert_eq!(page.screenshot_url(key), None, "{key:?}");
        }
        // Every picture of every page has an address of the one shape, with nothing to escape.
        for d in &DETAILS {
            for s in d.shots {
                let url = d.screenshot_url(s.key).unwrap();
                let path = url.strip_prefix(&format!("{SCREENSHOTS_BASE}/{SCREENSHOTS_REV}/")).unwrap();
                assert_eq!(path, format!("{}/{}.jpg", d.id, s.key));
                assert!(path.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '/' | '-' | '.')), "{path}");
            }
        }
    }

    #[test]
    fn only_jpeg_files_are_accepted() {
        assert!(looks_like_jpeg(&[0xFF, 0xD8, 0xFF, 0xE0, 0, 0x10]));
        assert!(!looks_like_jpeg(b""));
        assert!(!looks_like_jpeg(&[0xFF, 0xD8]));
        assert!(!looks_like_jpeg(b"<!DOCTYPE html>"));
        // A PNG's signature.
        assert!(!looks_like_jpeg(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]));
    }

    #[test]
    fn pages_are_filled_in() {
        for d in &DETAILS {
            assert!(!d.headline.is_empty() && !d.features.is_empty() && d.paragraphs().next().is_some(), "{}", d.id);
        }
    }
}
