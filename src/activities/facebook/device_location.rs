use rusqlite::{params, Connection};
use serde::Deserialize;
use uuid::Uuid;

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CellPhoneCarrier {
  pub spn:  String,
  pub country_code:  String
}
#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DeviceLocation {
  pub phone_number_location_v2: Option<Vec<CellPhoneCarrier>>,
  pub phone_number_location: Option<Vec<CellPhoneCarrier>>
}

#[allow(non_snake_case)]
impl DeviceLocation {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        match &self.location_history_v2 {
            Some(x) => x.iter().for_each(|elem| {
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
            }),
            None => println!("Old version of Facebook's data"),
        };
        match &self.location_history {
            Some(x) => x.iter().for_each(|elem| {
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
            }),
            None => println!("Current version of Facebook's data"),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_device_location_v2() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let cell_phone_carrier = CellPhoneCarrier {
            spn: "NTT DoCoMo".to_string(),
            country_code: "440".to_string(),
        };
        let device_location = DeviceLocation {
            phone_number_location_v2: Some(vec![cell_phone_carrier]),
            phone_number_location: None,
        };
        let result = DeviceLocation::saveToDb(&device_location, &conn);
        assert_eq!(result, Ok(()));
        Ok(())
    }
    #[test]
    fn test_device_location() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let cell_phone_carrier = CellPhoneCarrier {
            spn: "NTT DoCoMo".to_string(),
            country_code: "440".to_string(),
        };
        let device_location = DeviceLocation {
            phone_number_location_v2: None,
            phone_number_location: Some(vec![cell_phone_carrier]),
        };
        let result = DeviceLocation::saveToDb(&device_location, &conn);
        assert_eq!(result, Ok(()));
        Ok(())
    }
}
