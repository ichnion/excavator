use rusqlite::Connection;
use rusqlite::NO_PARAMS;

pub fn create_tables(conn: &Connection) {
    /*
     * google_my_activity
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS  google_my_activity (
            uuid      TEXT NOT NULL PRIMARY KEY,
            header    TEXT NOT NULL,
            title     TEXT NOT NULL,
            title_url TEXT,
            time      TEXT NOT NULL,
            UNIQUE(header,title,time)
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * activity_location_info
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activity_location_info (
            id     INTEGER PRIMARY KEY,
            a_uuid TEXT NOT NULL ,
            name   TEXT,
            url    TEXT,
            source TEXT,
            FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE
        ) ",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    conn.execute(
        "CREATE INDEX IF NOT EXISTS gact1 ON activity_location_info(a_uuid)",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * activity_sub_title
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activity_sub_title (
           id     INTEGER PRIMARY KEY,
           a_uuid TEXT NOT NULL,
           name   TEXT,
           url    TEXT,
           FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    conn.execute(
        "CREATE INDEX IF NOT EXISTS gact2 ON activity_sub_title(a_uuid)",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * activity_details
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activity_details (
            id     INTEGER PRIMARY KEY,
            a_uuid TEXT NOT NULL,
            name   TEXT,
            FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    conn.execute(
        "CREATE INDEX IF NOT EXISTS gact3 ON activity_details(a_uuid) ",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * activity_products
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activity_products (
            id     INTEGER PRIMARY KEY,
            a_uuid TEXT NOT NULL,
            name   TEXT,
            FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    conn.execute(
        "CREATE INDEX IF NOT EXISTS gact4 ON activity_products(a_uuid)",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * google_location_history
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS google_location_history (
            id               INTEGER PRIMARY KEY,
            source           TEXT,
            activity         TEXT,
            address          TEXT,
            place_name       TEXT,
            timestamp_msec   BIGINT NOT NULL,
            accuracy         INTEGER,
            verticalaccuracy INTEGER,
            altitude         INTEGER,
            lat              FLOAT NOT NULL,
            lng              FLOAT NOT NULL,
            UNIQUE(timestamp_msec,lat,lng)
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * google_saved_places
     */
    conn.execute(
        "create table if not exists google_saved_places (
            id               INTEGER PRIMARY KEY,
            activity         TEXT,
            timestamp_msec   BIGINT NOT NULL,
            accuracy         INTEGER,
            verticalaccuracy INTEGER,
            altitude         INTEGER,
            lat              FLOAT NOT NULL,
            lng              FLOAT NOT NULL,
            UNIQUE(timestamp_msec,lat,lng)
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * facebook_device_location
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS facebook_device_location (
            id               INTEGER PRIMARY KEY,
            spn              TEXT,
            country_code     TEXT,
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * facebook_device_location
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS facebook_device_location (
            id               INTEGER PRIMARY KEY,
            spn              TEXT,
            country_code     TEXT,
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * facebook_primary_location
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS facebook_primary_location (
            id                INTEGER PRIMARY KEY,
            city_region_pairs TEXT,
            zipcode           TEXT,
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();

    /*
     * facebook_primary_public_location
     */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS facebook_primary_public_location (
            id                INTEGER PRIMARY KEY,
            city              TEXT,
            region            TEXT,
            country           TEXT,
        )",
        NO_PARAMS,
    )
    .map_err(|err| println!("{:?}", err))
    .ok();
}
