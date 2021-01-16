use rusqlite::{Connection};
use rusqlite::NO_PARAMS;

pub fn create_tables( conn: &Connection ) {
    
    /*
     * google_my_activity
     */
    conn.execute(
        "CREATE TABLE if not exists  google_my_activity (
            uuid      TEXT NOT NULL PRIMARY KEY,
            header    TEXT NOT NULL,
            title     TEXT NOT NULL,
            title_url TEXT,
            time      TEXT NOT NULL,
            UNIQUE(header,title,time)
        )",
        NO_PARAMS,
    ).map_err(|err| println!("{:?}", err)).ok();
    
    /* 
     * activity_location_info
     */
    conn.execute(
        "create table if not exists activity_location_info (
            id     INTEGER PRIMARY KEY,
            a_uuid TEXT NOT NULL ,
            name   TEXT,
            url    TEXT,
            source TEXT,
            FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE   
        ) ",
        NO_PARAMS,
    ).map_err(|err| println!("{:?}", err)).ok();
    

    conn.execute(
        "CREATE INDEX if not exists gact1 on activity_location_info(a_uuid)",
        NO_PARAMS,
    ).map_err(|err| println!("{:?}", err)).ok();

    /*
     * activity_sub_title
     */
    conn.execute(
        "create table if not exists activity_sub_title (
           id     INTEGER PRIMARY KEY,
           a_uuid TEXT NOT NULL,
           name   TEXT,    
           url    TEXT,    
           FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE   
        )",
        NO_PARAMS,
    ).map_err(|err| println!("{:?}", err)).ok();

    conn.execute(
        "create index if not exists gact2 on activity_sub_title(a_uuid)",
        NO_PARAMS,
    ).map_err(|err| println!("{:?}", err)).ok();

    /*
     * activity_details
     */
    conn.execute(
        "create table if not exists activity_details (
            id     INTEGER PRIMARY KEY,
            a_uuid TEXT NOT NULL,
            name   TEXT,
            FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE   
        )",
        NO_PARAMS,
    ).map_err(|err| println!("{:?}", err)).ok();

    conn.execute(
        "create index if not exists gact3 on activity_details(a_uuid) ",
        NO_PARAMS,
    ).map_err(|err| println!("{:?}", err)).ok();

    /*
     * activity_products
     */
    conn.execute(
        "create table if not exists activity_products (
            id     INTEGER PRIMARY KEY,
            a_uuid TEXT NOT NULL,
            name   TEXT,
            FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE   
        )",
        NO_PARAMS,
    ).map_err(|err| println!("{:?}", err)).ok();

    conn.execute(
        "create index if not exists gact4 on activity_products(a_uuid)",
        NO_PARAMS,
    ).map_err(|err| println!("{:?}", err)).ok();

    /*
     * google_location_history
     */
    conn.execute(
        "create table if not exists google_location_history (
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
    ).map_err(|err| println!("{:?}", err)).ok();
    
}
