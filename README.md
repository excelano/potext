# potext

Read a GNU `.po` message catalogue that was compiled into your binary, and ask
the platform which language the person in front of it reads.

It exists for desktop applications built on egui, which have no toolkit of their
own to ask and no place to install a catalogue that means the same thing in a
`.deb`, an MSIX package and an App Store sandbox. So the catalogue is
`include_str!`ed and parsed once at startup, and there is nothing to find on
disk at run time.

## Using it

```rust
mod i18n {
    potext::catalog!();
}
use i18n::t;

fn main() {
    i18n::activate(&[("de", include_str!("../po/de.po"))]);
    println!("{}", t("Save"));
}
```

`t` takes the English text as the key and hands back the translation or, where
there is none, the English it was given. A call site therefore reads as the
sentence a person sees, and a catalogue that has fallen behind the source
degrades one message at a time rather than showing a missing-key placeholder.
`tc` adds a context where one English word means two different things, `tn`
chooses between a singular and a plural, and `potext::fill` puts values into a
translated sentence's `{placeholders}` in a single pass, so a filename carrying
the text of another placeholder cannot rewrite the sentence around it.

`activate` reads the platform and picks the closest catalogue: an exact tag
first, then the language alone, so a machine set to `de-AT` reads the `de`
catalogue. `set_language` puts a named language in force instead, which is what
a library takes from its host.

## One catalogue per crate

`catalog!` declares its storage inside the crate that invokes it. A single
global here would be one catalogue for everything linked against this crate, and
a published widget has to carry its own messages rather than borrow the
application's. The only thing that should cross a crate boundary is a language
tag, never a catalogue, which also means a version skew between the two costs
nothing.

## Why `.po`

For one property: it can say that a translation has gone *stale*. When an
English message is reworded, `msgmerge` pairs the new text with the entry it
descended from, carries the old translation over and marks it `#, fuzzy`. This
crate refuses to load a fuzzy entry, so the interface falls back to English
until somebody has read the new sentence and written a translation for it. A
key-value catalogue cannot express that: the reworded string is a new key, and
the old translation sits in the file still looking finished. Over years and
several applications, that is the difference that compounds.

It is not gettext. There is no `.mo`, no text domain, no `bindtextdomain`, and
no runtime beyond this crate. It compiles no C, runs no build script, and its
own source is `forbid(unsafe_code)`.

## What it does not do

It does not write or merge a catalogue. `xgettext` extracts, `msgmerge` merges
and marks what went stale, and `msgfmt` validates; all three are gettext's and
none is needed at run time or on a user's machine. It understands one plural
rule, two forms chosen by `n != 1`, which is English's and German's — a
catalogue declaring anything else keeps its entries and gets the English forms,
which is the better of the two ways to be wrong.

## Where the language comes from

`POTEXT_LANG` first on every platform, then `LANGUAGE`, `LC_ALL`, `LC_MESSAGES`
and `LANG` in POSIX order, so a machine already set up for translated software
behaves as its owner expects and a pseudolocale can be forced anywhere. Failing
those it asks the platform: nothing further on Linux, where `LANG` is always
set; `NSLocale.preferredLanguages` on macOS, where a bundle opened from Finder
has no environment at all; and `Control Panel\International\LocaleName` on
Windows, read through the registry rather than through an FFI call this crate
could not make.

## Licence

MIT. Author: David M. Anderson. Built with AI assistance (Claude, Anthropic).
