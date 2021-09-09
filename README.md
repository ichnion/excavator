# excavator
[![Cargo Test](https://github.com/ichnion/excavator/actions/workflows/test.yml/badge.svg)](https://github.com/ichnion/excavator/actions/workflows/test.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![excavator at crates.io](https://img.shields.io/crates/v/excavator.svg)](https://crates.io/crates/excavator)

Visualize your digital footprint.

## Installing Excavator
There are different ways to install Excavator. The first method is particularly meant to be used by non-developer users.

### Non-developer users
First click on "Releases" :

![](/docs/img-1-install.png)

You should see the last release on your screen (v0.2.0 when we are writing this).
Depending on your OS (Windows, Linux or Mac), click on one the three files highlighted.

![](/docs/img-2-install.png)

It should download the file on your device. Then you need to uncompress it.

#### Windows 
We are using 7zip to unzip the file. 

![](/docs/img-3-install.png)

Put this file in `C:\Windows\System32`, and the installation is completed. You should now be able to use Excavator in a terminal. 

#### Linux
Click on the file you just downloaded.

Then extract it wherever you want. In our case, we extract it in `Desktop`.

![](/docs/img-5-install.png)

Finally, open a terminal and type the following command `sudo cp PATH_TO_EXCAVATOR /usr/local/bin` (replace PATH_TO_EXCAVATOR by the path to where you extracted excavator in the previous step). 

![](/docs/img-6-install.png)

#### MacOS
Extract `excavator-linux.tar.gz` in usr/local/bin.
The installation is completed, you should now be able to use Excavator in a terminal.

----------------------------
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

### Update to the latest version
```sh
$ brew upgrade ichnion/tap/excavator
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
