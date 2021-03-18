use rusqlite::{params, Connection};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LocationHistory {
    pub locations: Vec<Locations>,
}

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Locations {
    pub timestampMs      : String,
    pub latitudeE7       : i32,
    pub longitudeE7      : i32,
    pub accuracy         : i32,
    pub altitude         : Option<i32>,
    pub verticalAccuracy : Option<i32>,
    pub activity         : Option<Vec<Activity>>
}

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Activity {
    pub timestampMs : String,
    pub activity    : Vec<Activities>
}

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Activities {
    pub r#type     : String,
    pub confidence : i32
}

#[allow(non_snake_case)]
impl LocationHistory {
    pub fn saveToDb(&self, conn: &Connection) {
        for elem in self.locations.iter() {
            let activity: String;
            let altitude: i32;
            let verticalAccuracy: i32;

            if let Some(val) = elem.altitude {
                altitude = val;
            } else {
                altitude = 0;
            }

            if let Some(val) = elem.verticalAccuracy {
                verticalAccuracy = val;
            } else {
                verticalAccuracy = 0;
            }

            if let Some(act) = &elem.activity {
                activity = act[0].activity[0].r#type.to_string();
            } else {
                activity = "na".to_string();
            }
            conn.execute(
                "insert into google_location_history
                (activity,timestamp_msec,accuracy,verticalaccuracy,altitude,lat,lng,source)
                values(?1, $2, $3, $4, $5, $6/10000000.0, $7/10000000.0,'location_history')",
                params![
                    &activity,
                    elem.timestampMs.parse::<i64>().unwrap(),
                    elem.accuracy,
                    verticalAccuracy,
                    altitude,
                    elem.latitudeE7,
                    elem.longitudeE7
                ],
            ) /*.map_err(|err| println!("{:?}", err))*/
            .ok();
        }
    }
}
