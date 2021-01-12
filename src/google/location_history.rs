use crate::db::save::*;
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
    pub fn saveToDb(&self) {
        let connection = establish_connection();

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

            save_location_history(
                &connection,
                &activity,
                //(&elem.timestampMs.parse::<i64>().unwrap()/1000) as i32,
                elem.timestampMs.parse::<i64>().unwrap(),
                elem.accuracy,
                verticalAccuracy,
                altitude,
                elem.latitudeE7 as f32 / 10000000.0,
                elem.longitudeE7 as f32 / 10000000.0,
            );
            //println!("activity: {}, timestamp: {}, lat: {}, lng: {}",
            //println!("insert into lochistory (activity,timestamp_sec,geom) values ('{}',{},st_geomfromtext('POINT({} {})',4326));",
            //    activity,
            //    elem.timestampMs.parse::<i64>().unwrap() /1000,
            //    elem.longitudeE7 as f64/10000000.0,
            //    elem.latitudeE7  as f64/10000000.0,);
        }
    }
}
