use rusqlite::{params, Connection};
use serde::Deserialize;

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LastLocationObjects {
  pub time: u64,
  pub coordinate: Coordinate
}
#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Coordinate {
  pub latitude: f64,
  pub longitude: f64
}
#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LastLocation {
  pub last_location_v2: Option<LastLocationObjects>,
  pub last_location: Option<LastLocationObjects>
}

#[allow(non_snake_case)]
impl LastLocation {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        match &self.last_location_v2 {
            None => None,
            Some(x) => Some(
                conn.execute(
                    "INSERT INTO facebook_last_location (
                        time,
                        latitude,
                        longitude
                    )
                    VALUES (?1, ?2, ?3)",
                    params![
                        &x.time.to_string(),
                        &x.coordinate.latitude.to_string(),
                        &x.coordinate.longitude.to_string()
                    ],
                )
                .map_err(|err| println!("{:?}", err))
                .ok(),
            ),
        };
        match &self.last_location {
            None => None,
            Some(x) => Some(
                conn.execute(
                    "INSERT INTO facebook_last_location (
                        time,
                        latitude,
                        longitude
                    )
                    VALUES (?1, ?2, ?3)",
                    params![
                        &x.time.to_string(),
                        &x.coordinate.latitude.to_string(),
                        &x.coordinate.longitude.to_string()
                    ],
                )
                .map_err(|err| println!("{:?}", err))
                .ok(),
            ),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_last_l_v2() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let test_coordinate = Coordinate {
            latitude: 10.0,
            longitude: 10.0,
        };
        let test_last_location1 = LastLocationObjects {
            time: 1,
            coordinate: test_coordinate,
        };
        let test_last_location = LastLocation {
            last_location_v2: Some(test_last_location1),
            last_location: None,
        };
        let result = LastLocation::saveToDb(&test_last_location, &conn);
        assert!(result == Ok(()));
        Ok(())
    }

    #[test]
    fn test_last_l() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let test_coordinate = Coordinate {
            latitude: 10.0,
            longitude: 10.0,
        };
        let test_last_location1 = LastLocationObjects {
            time: 1,
            coordinate: test_coordinate,
        };
        let test_last_location = LastLocation {
            last_location_v2: None,
            last_location: Some(test_last_location1),
        };
        let result = LastLocation::saveToDb(&test_last_location, &conn);
        assert!(result == Ok(()));
        Ok(())
    }
}
