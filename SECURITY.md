# Security Policy

## Reporting a vulnerability

Please report suspected vulnerabilities privately through GitHub Security Advisories at https://github.com/excelano/potext/security/advisories/new. If you would rather not use GitHub, email david.anderson@excelano.com instead. I aim to respond within seven days.

Please do not open public issues for security problems.

## Supported versions

The latest release receives security fixes. Older versions are not supported.

## What potext can access

potext is a library, not a service. It parses a `.po` catalogue the calling program compiled into itself, so the text it reads is the program's own and never a file a user supplied. It opens no files, makes no network calls, runs no subprocesses, and executes nothing a catalogue contains — a `.po` entry is text in and text out.

It reads the environment to learn which language to use: `POTEXT_LANG`, then `LANGUAGE`, `LC_ALL`, `LC_MESSAGES` and `LANG`. On macOS it asks `NSLocale.preferredLanguages` and on Windows it reads `Control Panel\International\LocaleName` from the current user's registry hive. All three are reads. Nothing is written anywhere, and a value from any of them is used only to pick between the catalogues the calling program shipped — a name that matches none of them selects nothing and the program stays in English.

The crate's own source is `forbid(unsafe_code)`. It compiles no C and runs no build script. Its two dependencies are platform-gated and exist because the locale query on those platforms is an operating-system call: `objc2-foundation` on macOS and `windows-registry` on Windows, each used through safe functions only.

## What potext stores

Nothing. No telemetry, no analytics, no caching, no files. The parsed catalogue lives in memory in the calling program for as long as that program runs, and `potext::fill` returns a `String` to its caller rather than keeping one.
