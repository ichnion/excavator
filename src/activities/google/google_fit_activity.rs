use rusqlite::{params, Connection};
use serde::Deserialize;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Fit {
    pub duration         : String,
    pub startTime        : String,
    pub endTime          : String,
    pub fitnessActivity  : String
}

#[allow(non_snake_case)]
impl Fit {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        //for elem in self.fit.iter() {
            let my_uuid = Uuid::new_v4();
            let duration = &self.duration;
            let startTime = &self.startTime;
            let endTime = &self.endTime;
            let fitnessActivity = &self.fitnessActivity;

            conn.execute(
                "INSERT into google_fit_activity
                (uuid, activity, start, end, timestamp)
                values($1, $2, $3, $4, $5)",
                params![
                    &my_uuid.to_string(),
                    fitnessActivity,
                    startTime,
                    endTime,
                    duration
                ],
            )
            .map_err(|err| println!("{:?}", err))
            .ok();
       // }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_google_fit_activity() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let fit = Fit {
            duration         : "".to_string(),
            startTime        : "".to_string(),
            endTime          : "".to_string(),
            fitnessActivity  : "".to_string()
        };

        let result = Fit::saveToDb(&fit, &conn);

        assert_eq!(result, Ok(()));

        Ok(())
    }
}