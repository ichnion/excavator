# excavator   
[![Cargo Test](https://github.com/ichnion/excavator/actions/workflows/test.yml/badge.svg)](https://github.com/ichnion/excavator/actions/workflows/test.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)



**Note:** To enforce Delete Cascade on tables with Foreign Keys in Sqlite, the following has to be set before deletions:

```
sqlite> PRAGMA foreign_keys = ON;
```
