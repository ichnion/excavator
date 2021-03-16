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
  pub phone_number_location: Vec<CellPhoneCarrier>
}

#[allow(non_snake_case)]
impl DeviceLocation {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let my_uuid = Uuid::new_v4();

        for elem in self.phone_number_location.iter() {
            conn.execute(
                "INSERT INTO facebook_device_location (
              uuid,
              spn,
              country_code
          )
          VALUES (?1, ?2, ?3)",
                params![my_uuid.to_string(), &elem.spn, &elem.country_code,],
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
    fn test_device_location() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let cell_phone_carrier = CellPhoneCarrier {
            spn: "NTT DoCoMo".to_string(),
            country_code: "440".to_string(),
        };
        let device_location = DeviceLocation {
            phone_number_location: vec![cell_phone_carrier],
        };
        let result = DeviceLocation::saveToDb(&device_location, &conn);
        assert_eq!(result, Ok(()));
        Ok(())
    }
}
