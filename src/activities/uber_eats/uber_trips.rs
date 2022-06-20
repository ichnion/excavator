use rusqlite::{params, Connection};
use serde::Deserialize;
use uuid::Uuid;
extern crate csv;

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct GeneralStructure {
  #[serde(rename = "Begin Trip Time")]
  pub begin_trip_time: String,
  #[serde(rename = "Begin Trip Lat")]
  pub begin_trip_lat: String,
  #[serde(rename = "Begin Trip Lng")]
  pub begin_trip_lng: String,
  #[serde(rename = "Begin Trip Address")]
  pub begin_trip_address: String,
  #[serde(rename = "Dropoff Time")]
  pub dropoff_time: String,
  #[serde(rename = "Dropoff Lat")]
  pub dropoff_lat: String, 
  #[serde(rename = "Dropoff Lng")]
  pub dropoff_lng: String,
  #[serde(rename = "Dropoff Address")]
  pub dropoff_address: String
}
#[allow(non_snake_case)]
impl GeneralStructure {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let my_uuid = Uuid::new_v4();
        conn.execute(
            "INSERT INTO uber_eats_trips (
                        uuid,
                        Begin_Trip_Time,
                        Begin_Trip_Lat,
                        Begin_Trip_Long,
                        Begin_Trip_Address,
                        Dropoff_Time,
                        Dropoff_Lat,
                        Dropoff_Lng,
                        Dropoff_Address
                                )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &my_uuid.to_string(),
                &self.begin_trip_time,
                &self.begin_trip_lat,
                &self.begin_trip_lng,
                &self.begin_trip_address,
                &self.dropoff_time,
                &self.dropoff_lat,
                &self.dropoff_lng,
                &self.dropoff_address
            ],
        )
        .map_err(|err| println!("{:?}", err))
        .ok();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    #[test]
    fn test_uber_eats_trips() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let general_structure_test = GeneralStructure {
            begin_trip_time: "2021-05-07 18:59:15 +0000 UTC".to_string(),
            begin_trip_lat: "10".to_string(),
            begin_trip_lng: "10".to_string(),
            begin_trip_address: "1 rue Test".to_string(),
            dropoff_time: "2021-05-07 18:59:15 +0000 UTC".to_string(),
            dropoff_lat: "10".to_string(), 
            dropoff_lng: "10".to_string(),
            dropoff_address: "2 rue Test".to_string(),
        };
        let result = GeneralStructure::saveToDb(&general_structure_test, &conn);

        assert_eq!(result, Ok(()));

        Ok(())
    }
}

