use serde::Deserialize;

use rusqlite::{params, Connection};

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LocationHistory {
    pub location_history_v2: Vec<FbLocationHistory>
}

#[rustfmt::skip]
#[derive(Deserialize, Debug)]
pub struct FbLocationHistory {
    pub name: String,
    pub coordinate: Coordinate,
    pub creation_timestamp: u64
}

#[rustfmt::skip]
#[derive(Deserialize, Debug)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64
}

#[rustfmt::skip]
#[allow(non_snake_case)]
impl LocationHistory {
    pub fn saveToDb(&self,conn: &Connection) -> Result<(), rusqlite::Error> {
        for elem in self.location_history_v2.iter() {
            conn.execute(
                "INSERT into facebook_location_history
                (time, name, latitude, longitude)
                values(?1, $2, $3, $4)",
                params![
                    elem.creation_timestamp.to_string(),
                    elem.name,
                    elem.coordinate.latitude.to_string(),
                    elem.coordinate.longitude.to_string()
                ],
            )
            .map_err(|err| println!("{:?}", err))
            .ok();
        }

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
        let test_coordinate = Coordinate {
            latitude: 10.0,
            longitude: 10.0,
        };
        let fbtest = FbLocationHistory {
            name: "test".to_string(),
            coordinate: test_coordinate,
            creation_timestamp: 10,
        };
        let test = LocationHistory {
            location_history_v2: vec![fbtest],
        };

        let result = LocationHistory::saveToDb(&test, &conn);

        assert_eq!(result, Ok(()));

        Ok(())
    }
}
