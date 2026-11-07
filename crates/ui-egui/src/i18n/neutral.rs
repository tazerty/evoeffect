//! Evora (patch E9): the display-name override catalogue ("catalogue de surcharge d'affichage").
//!
//! Third-party product and company names that the upstream interface shows (Adobe, Premiere Pro,
//! Lumetri) are replaced on screen by neutral terms, after translation, in every language,
//! English included. The rules apply where the interface draws text: [`super::tr`] and
//! [`super::tr_args`] (panels, dialogs, forms, tooltips), the menu labels
//! (`menus::entry_label`), the panel tabs, the effect names of the Effects & Presets panel, the
//! Effect Controls headers, the Timeline's group rows and the status line. Nothing below
//! `ui-egui` changes: effect ids, match names, project files and the engine's own labels keep the
//! upstream text (apps/kinetika/UPSTREAM.md §6.1; the wording may change with decision D-16).
//!
//! Lightroom, Acrobat, Cycore and "Premiere Pro Compatible" do not occur in this repository (the
//! CI audit, `.github/evora/gates.py terms`, measures them); the rules cover what does. That audit
//! reads [`RULES`] from this file, applies them to every displayed string literal and fails if a
//! term is left.
//!
//! Patch EVORA-KIN-58 (decision Q-EVO-1 of 2026-11-06): on screen the app is called by its own name,
//! never by its engine's. [`neutral`] also writes [`SHOWN_NAME`] where a string names an engine
//! ([`ENGINE_NAMES`]), with the article, elision or ending the new name asks for, and leaves out the
//! file extensions that carry that name ([`ENGINE_EXTENSIONS`]). Nothing below this module changes:
//! identifiers, the `window.evoeffect` API, the files' extensions and formats, storage keys and the
//! licence files (never drawn through this module) keep the engine's name.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// The terms the rules remove; a string without any of them is shown unchanged (fast path).
const TERMS: [&str; 3] = ["Adobe", "Premiere", "Lumetri"];

/// (as written, shown instead), applied in this order: longer phrases first, then the generic
/// fallbacks that catch the translated catalogs.
pub(crate) const RULES: &[(&str, &str)] = &[
    // File ▸ Import / Export ▸ Adobe Premiere Pro Project… (menus, form title, every language).
    ("Export Adobe Premiere Pro Project", "Export FCP XML Project"),
    ("Adobe Premiere Pro", "FCP XML"),
    // The export form and the engine's messages about it.
    ("Final Cut Pro XML (.xml) – Premiere Pro", "Final Cut Pro XML (.xml)"),
    ("Final Cut Pro XML (Premiere Pro)", "Final Cut Pro XML"),
    ("which Premiere Pro opens", "which video editors open"),
    ("Layers Premiere can't show", "Layers a video editor can't show"),
    ("import it in Premiere Pro with", "import it in a video editor with"),
    ("native Premiere Pro projects", "native video editor projects"),
    ("in Premiere Pro use", "in the video editor use"),
    ("Premiere Pro", "video editor"),
    ("Premiere", "video editor"),
    // Project Settings ▸ Color Engine, and its translations.
    ("Adobe-style built-in", "Built-in (classic)"),
    ("Adobe 风格内置", "经典内置"),
    ("Вбудований у стилі Adobe", "Вбудований (класичний)"),
    ("Vestavěné ve stylu Adobe", "Vestavěné (klasické)"),
    ("Adobe After Effects", "After Effects"),
    ("Adobe", "classic"),
    // The colour-grading effect and its scopes panel.
    ("Lumetri Color", "Color Grading"),
    ("Lumetri Scopes", "Color Scopes"),
    ("Lumetri", "Color"),
];

/// The name the interface shows for the app.
const SHOWN_NAME: &str = "Kinetika";

/// The engines' names, as the catalogs and the messages write them.
const ENGINE_NAMES: [&str; 7] = ["EvoEffect", "EvoFilm", "EvoVector", "EvoPhoto", "EvoLight", "EvoPdf", "EvoDesign"];

/// The project extension, which starts all the engine's file extensions.
const PROJECT_EXTENSION: &str = ".evoeffect";

/// The engine's file extensions, longest first: the files keep them, the interface doesn't show them.
const ENGINE_EXTENSIONS: [&str; 4] = [".evoeffecttemplate", ".evoeffectpreset", ".evoeffectx", PROJECT_EXTENSION];

/// Wordings the new name can't just be dropped into: a sentence about the upstream project rather
/// than this app, and what a name ending in `-a` changes in the languages that decline it.
const NAME_PHRASES: &[(&str, &str)] = &[
    ("based on EvoEffect", "based on an open-source engine"),
    // Czech: the name stays as it is behind the word for "application", as "O aplikaci" already has it.
    ("Nápověda k EvoEffect", "Nápověda k aplikaci Kinetika"),
    ("Skrýt EvoEffect", "Skrýt aplikaci Kinetika"),
    ("Ukončit EvoEffect", "Ukončit aplikaci Kinetika"),
    ("Domovská stránka EvoEffect", "Domovská stránka aplikace Kinetika"),
    ("Vítejte v EvoEffect", "Vítejte v aplikaci Kinetika"),
    ("EvoEffect nebyl při posledním použití ukončen", "Kinetika nebyla při posledním použití ukončena"),
    // Ukrainian: the verb agrees with a feminine name.
    ("EvoEffect завершив", "Kinetika завершила"),
];

/// The word in front of the name that depends on its first letter, as the catalogs and the messages
/// write it before a vowel, and before [`SHOWN_NAME`]: English `an`, French `d’`.
const BEFORE_NAME: &[(&str, &str)] = &[("an ", "a "), ("An ", "A "), ("d’", "de "), ("d'", "de ")];

/// The ending after the name that depends on its last letter, as the catalogs write it after a
/// consonant, and after [`SHOWN_NAME`]: the Georgian genitive and ablative.
const AFTER_NAME: &[(&str, &str)] = &[("-ის", "-ს"), ("-იდან", "-დან")];

/// Whether `text` names an engine, by name or by file extension; `Evo` first, so that almost every
/// string is answered at once.
fn names_an_engine(text: &str) -> bool {
    (text.contains("Evo") && ENGINE_NAMES.iter().any(|n| text.contains(n))) || text.contains(PROJECT_EXTENSION)
}

/// `text` with `from` written `to` where it is a word of its own right in front of `name`.
fn word_before(text: &str, from: &str, name: &str, to: &str) -> String {
    let pattern = format!("{from}{name}");
    let mut out = String::with_capacity(text.len());
    let mut copied = 0;
    for (at, _) in text.match_indices(pattern.as_str()) {
        let starts_word = !text[..at].chars().next_back().is_some_and(char::is_alphabetic);
        out.push_str(&text[copied..at]);
        out.push_str(if starts_word { to } else { from });
        copied = at + from.len();
    }
    out.push_str(&text[copied..]);
    out
}

/// `text` with [`SHOWN_NAME`] in place of every engine's name, without the engine's extensions.
fn with_shown_name(text: &str) -> String {
    let mut out = text.to_string();
    for extension in ENGINE_EXTENSIONS {
        out = out.replace(extension, "");
    }
    for (from, to) in NAME_PHRASES {
        out = out.replace(from, to);
    }
    for name in ENGINE_NAMES {
        if !out.contains(name) {
            continue;
        }
        for (from, to) in BEFORE_NAME {
            out = word_before(&out, from, name, to);
        }
        for (from, to) in AFTER_NAME {
            out = out.replace(&format!("{name}{from}"), &format!("{name}{to}"));
        }
        out = out.replace(name, SHOWN_NAME);
    }
    out
}

/// `text` as the interface shows it: borrowed, without allocating, when no rule applies.
pub(crate) fn neutral(text: &str) -> Cow<'_, str> {
    let has_term = TERMS.iter().any(|t| text.contains(t));
    let has_name = names_an_engine(text);
    if !has_term && !has_name {
        return Cow::Borrowed(text);
    }
    let mut out = text.to_string();
    if has_term {
        for (from, to) in RULES {
            if out.contains(from) {
                out = out.replace(from, to);
            }
        }
    }
    if has_name {
        out = with_shown_name(&out);
    }
    Cow::Owned(out)
}

/// [`neutral`] for an owned label (menu entries, tab titles): the same `String` when no rule applies.
pub(crate) fn neutral_owned(text: String) -> String {
    let replaced = match neutral(&text) {
        Cow::Owned(s) => Some(s),
        Cow::Borrowed(_) => None,
    };
    replaced.unwrap_or(text)
}

/// [`neutral`] for the `&str` that [`super::tr`] returns: a rewritten string is built once and kept
/// for the session (the catalogs and engine labels it sees are a finite set).
pub(crate) fn neutral_str(text: &str) -> &str {
    let Cow::Owned(rewritten) = neutral(text) else { return text };
    static CACHE: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
    let Ok(mut cache) = CACHE.get_or_init(Default::default).lock() else { return text };
    if let Some(shown) = cache.get(text) {
        return *shown;
    }
    let shown: &'static str = Box::leak(rewritten.into_boxed_str());
    cache.insert(text.to_string(), shown);
    shown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_neutralise_the_upstream_terms() {
        assert_eq!(neutral("Adobe Premiere Pro Project..."), "FCP XML Project...");
        assert_eq!(neutral("Export Adobe Premiere Pro Project"), "Export FCP XML Project");
        assert_eq!(neutral("Lumetri Color"), "Color Grading");
        assert_eq!(neutral("Lumetri Scopes"), "Color Scopes");
        assert_eq!(neutral("Adobe-style built-in"), "Built-in (classic)");
        assert_eq!(neutral("Projet Adobe Premiere Pro..."), "Projet FCP XML...");
        assert!(matches!(neutral("Gaussian Blur"), Cow::Borrowed(_)));
        for (from, _) in RULES {
            assert!(TERMS.iter().any(|t| from.contains(t)), "{from} has no term: the fast path would skip it");
            assert!(!TERMS.iter().any(|t| neutral(from).contains(t)), "{from} keeps a term");
        }
    }

    /// EVORA-KIN-58: the name, and the words around it that depend on its first or last letter.
    #[test]
    fn the_app_is_called_kinetika_on_screen() {
        assert_eq!(neutral("About EvoEffect..."), "About Kinetika...");
        assert_eq!(neutral("{} video in MXF (EvoFilm has no {} decoder)"), "{} video in MXF (Kinetika has no {} decoder)");
        assert_eq!(neutral("Built-in (EvoEffect minimal)"), "Built-in (Kinetika minimal)");
        assert_eq!(neutral("not an EvoEffect template"), "not a Kinetika template");
        assert_eq!(neutral("An EvoFilm effect id"), "A Kinetika effect id");
        assert_eq!(neutral("À propos d’EvoEffect..."), "À propos de Kinetika...");
        assert_eq!(neutral("Nápověda k EvoEffect..."), "Nápověda k aplikaci Kinetika...");
        assert_eq!(neutral("Vítejte v EvoEffect"), "Vítejte v aplikaci Kinetika");
        assert_eq!(neutral("Projekt EvoEffect..."), "Projekt Kinetika...");
        assert_eq!(neutral("EvoEffect nebyl při posledním použití ukončen běžným způsobem."), "Kinetika nebyla při posledním použití ukončena běžným způsobem.");
        assert_eq!(neutral("Минулого разу EvoEffect завершив роботу некоректно."), "Минулого разу Kinetika завершила роботу некоректно.");
        assert_eq!(neutral("EvoEffect-ის პროექტი..."), "Kinetika-ს პროექტი...");
        assert_eq!(neutral("EvoEffect-იდან გასვლა"), "Kinetika-დან გასვლა");
        assert_eq!(neutral("This modified build is based on EvoEffect (MIT)"), "This modified build is based on an open-source engine (MIT)");
        // `an` inside a word is not the article, and the other languages keep their words.
        assert_eq!(neutral("Titan EvoEffect"), "Titan Kinetika");
        assert_eq!(neutral("Sair do EvoEffect"), "Sair do Kinetika");
        // The engine's extensions don't show, and text that only looks like a name is returned as it is.
        assert_eq!(neutral("Saved /Untitled Project.evoeffect"), "Saved /Untitled Project");
        assert_eq!(neutral("Auto-saved to /EvoEffect Auto-Save/P auto-save 1.evoeffect"), "Auto-saved to /Kinetika Auto-Save/P auto-save 1");
        assert_eq!(neutral("Saved an XML copy to P.evoeffectx"), "Saved an XML copy to P");
        assert_eq!(neutral("Exported template “T” to /T.evoeffecttemplate"), "Exported template “T” to /T");
        assert_eq!(neutral("Preset.evoeffectpreset"), "Preset");
        assert!(matches!(neutral("Saved frame to /Comp 1.png"), Cow::Borrowed(_)));
        assert!(matches!(neutral("Exported to Evora's Drive."), Cow::Borrowed(_)));
        // Both layers apply to one string, and the rewritten text is made once.
        assert_eq!(neutral("EvoEffect and Lumetri Color"), "Kinetika and Color Grading");
        assert!(std::ptr::eq(neutral_str("Quit EvoEffect"), neutral_str("Quit EvoEffect")));
    }

    /// EVORA-KIN-58: the menu catalogs, every language (the panel catalogs: `i18n::ui`'s tests).
    #[test]
    fn no_menu_row_shows_an_engine_name() {
        for code in super::super::CATALOG_CODES {
            for (_, source, translated) in super::super::menu_catalog(code).unwrap_or_default() {
                for text in [source, translated] {
                    let shown = neutral(text);
                    assert!(!names_an_engine(&shown), "{code}: {text:?} shows {shown:?}");
                }
            }
        }
    }
}
