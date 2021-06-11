use rusqlite::{params, Connection};
use serde::Deserialize;
use uuid::Uuid;

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct GeneralStructure {
  #[serde(rename = "Areas you may have visited in the last two years")]
  pub areas: Vec<SmallerStructure>
}
#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SmallerStructure {
  pub Time: String,
  pub City: String,
  pub Region: String,
  #[serde(rename = "Postal Code")]
  pub Postal: String

}

#[allow(non_snake_case)]
impl GeneralStructure {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        for elem in self.areas.iter() {
            let my_uuid = Uuid::new_v4();
            conn.execute(
                "INSERT INTO snapchat_areas_visited_history (
                        uuid,
                        Time,
                        City,
                        Region,
                        Postal_Code
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    &my_uuid.to_string(),
                    &elem.Time,
                    &elem.City,
                    &elem.Region,
                    &elem.Postal
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
    fn test_snapchat_areas_visited_history() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let SmallerTest = SmallerStructure {
            Time: "2021/02/12 11:51:31 UTC".to_string(),
            City: "Paris".to_string(),
            Region: "Ile - De - France".to_string(),
            Postal: "75000".to_string(),
        };
        let GeneralTest = GeneralStructure {
            areas: vec![SmallerTest],
        };
        let result = GeneralStructure::saveToDb(&GeneralTest, &conn);

        assert_eq!(result, Ok(()));

        Ok(())
    }
}
