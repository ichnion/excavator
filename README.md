# excavator
[![Cargo Test](https://github.com/ichnion/excavator/actions/workflows/test.yml/badge.svg)](https://github.com/ichnion/excavator/actions/workflows/test.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![excavator at crates.io](https://img.shields.io/crates/v/excavator.svg)](https://crates.io/crates/excavator)

Visualize your digital footprint.

## Installing Excavator
### Compiling from source
You need to have a [package manager](https://doc.rust-lang.org/cargo/appendix/glossary.html#package-manager) called [cargo](https://doc.rust-lang.org/cargo/) in your local.

If you already have got `rustc` installed in your local, you also have `cargo` installed locally.

And then you can install with this command.

```sh
$ cargo install --git https://github.com/ichnion/excavator --branch develop
```

### Install from crates.io
You can also install from package registry.

```sh
$ cargo install excavator
```

### Install from Homebrew
```sh
$ brew tap ichnion/tap
$ brew install excavator
```

## How to use Excavator
Excavator read and store your exported digital data.
Currently, we support **part of** Google Takeout and Facebook data.

Once you specify the directory, for instance `Google Takeout`, excavator recursively find an applicable file and read the data.

```sh
$ excavator read ~/Downloads/Takeout // Downloaded Google Takeout directory
```

Or you can specify the single file
cargo install
```sh
$ excavator read Location History.json // A single file from downloaded Google Takeout
```

**Note:** To enforce Delete Cascade on tables with Foreign Keys in Sqlite, the following has to be set before deletions:

```
sqlite> PRAGMA foreign_keys = ON;
```
