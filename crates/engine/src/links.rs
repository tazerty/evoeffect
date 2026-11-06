//! Project links (shown in Help ▸ and by the start screen).
//!
//! Evora (EXC-K1, patch E1): this fork carries no Upstream mark and no community link. A modified
//! build must drop the Upstream marks (docs/brand/LICENSE-brand.txt): the Discord invite and the
//! upstream-site.com pages are removed, and the app page points at the source repository.

pub const APP_NAME: &str = "EvoEffect";
pub const APP_SLUG: &str = "evoeffect";
pub const GITHUB: &str = "https://github.com/tazerty/evoeffect";
pub const APP_PAGE: &str = GITHUB;
pub const ISSUES: &str = "https://github.com/tazerty/evoeffect/issues";

/// Sibling apps of the same upstream family: (name, GitHub repo).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("EvoPhoto", "evophoto"),
    ("EvoVector", "evovector"),
    ("EvoFilm", "evofilm"),
    ("EvoLight", "evolight"),
    ("EvoPdf", "evopdf"),
    ("EvoDesign", "evodesign"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
/// Evora (EXC-K1): a sibling's page is its source repository (no upstream-site.com page).
pub fn page_of(slug: &str) -> String {
    github_of(slug)
}
