//! What a crate that invokes `potext::catalog!` actually gets.
//
// Author: David M. Anderson
// Built with AI assistance (Claude, Anthropic)
//
// The unit tests inside `src/lib.rs` reach the parser directly. These reach it
// the way every real caller will: through a macro expanded in somebody else's
// crate, against storage that belongs to that crate. A macro that compiles
// inside its own crate and not outside it is a defect the unit tests cannot
// see, and this file is one crate away.

mod i18n {
    potext::catalog!();
}

/// A catalogue as `msginit` and a translator would leave it.
const GERMAN: &str = "\
msgid \"\"\n\
msgstr \"Language: de\\n\"\n\
\"Plural-Forms: nplurals=2; plural=(n != 1);\\n\"\n\
\n\
msgid \"Save\"\n\
msgstr \"Speichern\"\n\
\n\
#, fuzzy\n\
msgid \"Open a container…\"\n\
msgstr \"Behälter öffnen …\"\n\
\n\
msgid \"{n} byte\"\n\
msgid_plural \"{n} bytes\"\n\
msgstr[0] \"{n} Byte\"\n\
msgstr[1] \"{n} Bytes\"\n";

/// Would catch the macro handing a caller storage that is not its own, or a
/// lookup that never reaches the catalogue the caller put in force.
///
/// Everything a consumer depends on is in one test because the storage is a
/// `OnceLock` and this binary has one of them: split across tests, the order
/// they run in would decide which of them saw a catalogue at all.
#[test]
fn a_catalogue_declared_in_another_crate_translates_through_it() {
    assert_eq!(
        i18n::t("Save"),
        "Save",
        "nothing is translated before a language is in force"
    );

    let chosen = i18n::set_language("de-AT", &[("de", GERMAN)]);
    assert_eq!(
        chosen.as_deref(),
        Some("de"),
        "an Austrian machine reads the German catalogue"
    );

    assert_eq!(i18n::t("Save"), "Speichern");
    assert_eq!(
        i18n::t("Redo"),
        "Redo",
        "a message the catalogue does not carry falls back to its English"
    );
    assert_eq!(
        i18n::t("Open a container…"),
        "Open a container…",
        "and a fuzzy entry is a message the catalogue does not carry"
    );

    assert_eq!(i18n::tn("{n} byte", "{n} bytes", 1), "{n} Byte");
    assert_eq!(i18n::tn("{n} byte", "{n} bytes", 4), "{n} Bytes");

    assert_eq!(
        potext::fill(i18n::tn("{n} byte", "{n} bytes", 584), &[("n", "584")]),
        "584 Bytes"
    );

    assert_eq!(
        i18n::set_language("fr", &[("de", GERMAN)]),
        None,
        "a language nothing was shipped for chooses nothing"
    );
}
