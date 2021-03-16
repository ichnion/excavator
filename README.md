# excavator   
[![Cargo Test](https://github.com/ichnion/excavator/actions/workflows/test.yml/badge.svg)](https://github.com/ichnion/excavator/actions/workflows/test.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)


#### Diesel with SQLite setup

* Diesel-cli install

```
cargo install diesel_cli 
```

* Create SQLite Database file

```
diesel setup --database-url=ichneos.db
```

* Create Migration scripts

```
diesel migration generate google_my_activity
```

* Populate the migration scripts


* Run the migration

```
diesel --database-url=ichneos.db migration run
```

**Note:** To enforce Delete Cascade on tables with Foreign Keys in Sqlite, the following has to be set before deletions:

```
sqlite> PRAGMA foreign_keys = ON;
```
