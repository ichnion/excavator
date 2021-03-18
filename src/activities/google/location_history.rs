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
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let _ = self.locations.iter().map(|x| {
            let altitude = x.altitude.unwrap_or(0);
            let timestamp = x.timestampMs.parse::<i64>().unwrap_or(0);
            let verticalAccuracy = x.verticalAccuracy.unwrap_or(0);
            let activity = match &x.activity {
                Some(t) => t[0].activity[0].r#type.to_string(),
                None => "na".to_string(),
            };

            conn.execute(
                "INSERT into google_location_history
                (activity, timestamp_msec,accuracy, verticalaccuracy, altitude, lat, lng, source)
                values(?1, $2, $3, $4, $5, $6/10000000.0, $7/10000000.0,'location_history')",
                params![
                    &activity,
                    timestamp,
                    x.accuracy,
                    verticalAccuracy,
                    altitude,
                    x.latitudeE7,
                    x.longitudeE7
                ],
            )
            .map_err(|err| println!("{:?}", err))
            .ok()
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_location_history() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let activities = Activities {
            r#type: "".to_string(),
            confidence: 0,
        };

        let activity = Activity {
            timestampMs: "".to_string(),
            activity: vec![activities],
        };

        let location = Locations {
            timestampMs: "".to_string(),
            latitudeE7: 0,
            longitudeE7: 0,
            accuracy: 0,
            altitude: Some(0),
            verticalAccuracy: Some(0),
            activity: Some(vec![activity]),
        };

        let location_history = LocationHistory {
            locations: vec![location],
        };

        let result = LocationHistory::saveToDb(&location_history, &conn);

        assert_eq!(result, Ok(()));

        Ok(())
    }
}
