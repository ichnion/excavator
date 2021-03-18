use rusqlite::{params, Connection};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TimeLineObjects {
    pub timelineObjects: Vec<TimeLineObject>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TimeLineObject {
    pub placeVisit: Option<PlaceVisit>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PlaceVisit {
    pub location: PlaceVisitLocation,
    pub duration: PlaceVisitDuration,
    pub centerLatE7: i32,
    pub centerLngE7: i32,
    pub visitConfidence: i32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PlaceVisitLocation {
    pub latitudeE7: Option<i32>,
    pub longitudeE7: Option<i32>,
    pub placeId: String,
    pub address: Option<String>,
    pub name: Option<String>,
    pub locationConfidence: f64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PlaceVisitDuration {
    pub startTimestampMs: String,
    pub endTimestampMs: String,
}

#[allow(non_snake_case)]
impl TimeLineObjects {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let _ = self.timelineObjects.iter().map(|timeline| {
            if let Some(place_visit) = &timeline.placeVisit {
                let lat = place_visit.location.latitudeE7.unwrap_or(-99);
                let lng = place_visit.location.longitudeE7.unwrap_or(-99);

                if lat != -99 && lng != 99 {
                    conn.execute(
                        "insert into google_location_history
                        (place_name,timestamp_msec,accuracy,address,lat,lng,source)
                        values(?1, $2, $3, $4, $5/10000000.0, $6/10000000.0,'place_visit')",
                        params![
                            place_visit
                                .location
                                .name
                                .as_ref()
                                .unwrap_or(&"".to_string()),
                            place_visit
                                .duration
                                .startTimestampMs
                                .parse::<i64>()
                                .unwrap_or(0),
                            place_visit.location.locationConfidence,
                            place_visit
                                .location
                                .address
                                .as_ref()
                                .unwrap_or(&"".to_string()),
                            lat,
                            lng
                        ],
                    )
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                }
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_semantic_location_history() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;

        let place_visit_location = PlaceVisitLocation {
            latitudeE7: Some(0),
            longitudeE7: Some(0),
            placeId: "1".to_string(),
            address: Some("Tokyo".to_string()),
            name: Some("test name".to_string()),
            locationConfidence: 0.00,
        };
        let place_visit_duration = PlaceVisitDuration {
            startTimestampMs: "2020/10/10/10:10".to_string(),
            endTimestampMs: "2020/11/11/11:11".to_string(),
        };
        let place_visit = PlaceVisit {
            location: place_visit_location,
            duration: place_visit_duration,
            centerLatE7: 0,
            centerLngE7: 0,
            visitConfidence: 0,
        };
        let timeline_object = TimeLineObject {
            placeVisit: Some(place_visit),
        };
        let timeline_objects = TimeLineObjects {
            timelineObjects: vec![timeline_object],
        };

        let result = TimeLineObjects::saveToDb(&timeline_objects, &conn);

        assert_eq!(result, Ok(()));

        Ok(())
    }
}
