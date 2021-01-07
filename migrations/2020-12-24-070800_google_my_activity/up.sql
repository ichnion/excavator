--
-- Tables for Google derrived data
--
CREATE TABLE google_my_activity (
    uuid      TEXT NOT NULL PRIMARY KEY,
    header    TEXT NOT NULL,
    title     TEXT NOT NULL,
    title_url TEXT,
    time      TEXT NOT NULL,
    UNIQUE(header,title,time)
);

CREATE TABLE activity_location_info (
    id     INTEGER PRIMARY KEY,
    a_uuid TEXT NOT NULL ,
    name   TEXT,
    url    TEXT,
    source TEXT,
    FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE   
);
CREATE INDEX gact1 on activity_location_info(a_uuid);

CREATE TABLE activity_sub_title (
    id     INTEGER PRIMARY KEY,
    a_uuid TEXT NOT NULL,
    name   TEXT,
    url    TEXT,
    FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE   
);
CREATE INDEX gact2 on activity_sub_title(a_uuid);

CREATE TABLE activity_details (
    id     INTEGER PRIMARY KEY,
    a_uuid TEXT NOT NULL,
    name   TEXT,
    FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE   
);
CREATE INDEX gact3 on activity_details(a_uuid);

CREATE TABLE activity_products (
    id     INTEGER PRIMARY KEY,
    a_uuid TEXT NOT NULL,
    name   TEXT,
    FOREIGN KEY (a_uuid) REFERENCES google_my_activity(uuid) ON DELETE CASCADE   
);
CREATE INDEX gact4 on activity_products(a_uuid);

CREATE TABLE location_history (
    id               INTEGER PRIMARY KEY,
    activity         TEXT,
    timestamp_msec   BIGINT NOT NULL,
    accuracy         INTEGER,
    verticalaccuracy INTEGER,
    altitude         INTEGER,
    lat              FLOAT NOT NULL,
    lng              FLOAT NOT NULL,
    UNIQUE(timestamp_msec,lat,lng)    
);