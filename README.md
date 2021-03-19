# excavator
[![Cargo Test](https://github.com/ichnion/excavator/actions/workflows/test.yml/badge.svg)](https://github.com/ichnion/excavator/actions/workflows/test.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## Installing Excavator
### Compiling from source
You need to have a [package manager](https://doc.rust-lang.org/cargo/appendix/glossary.html#package-manager) called [cargo](https://doc.rust-lang.org/cargo/) in your local.

If you already have got `rustc` installed in your local, you also have `cargo` installed locally.

And then you can install with this command.

```sh
$ cargo install --git https://github.com/ichnion/excavator --branch rusqlite
```

### Install from crates.io
TBD

### Install from Homebrew
TBD

**Note:** To enforce Delete Cascade on tables with Foreign Keys in Sqlite, the following has to be set before deletions:

```
sqlite> PRAGMA foreign_keys = ON;
```
