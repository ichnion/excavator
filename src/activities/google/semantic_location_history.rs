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
    pub fn saveToDb(&self, conn: &Connection) {
        for elem in self.timelineObjects.iter() {
            if let Some(pVisit) = &elem.placeVisit {
                let place_name: String;
                let address: String;
                let lat: i32;
                let lng: i32;

                if let Some(val) = &pVisit.location.name {
                    place_name = val.to_string();
                } else {
                    place_name = "".to_string();
                }

                if let Some(val) = &pVisit.location.address {
                    address = val.to_string();
                } else {
                    address = "".to_string();
                }

                if let Some(val) = pVisit.location.latitudeE7 {
                    lat = val;
                } else {
                    lat = -99;
                }

                if let Some(val) = pVisit.location.longitudeE7 {
                    lng = val;
                } else {
                    lng = -99;
                }

                if lat != -99 && lng != 99 {
                    conn.execute(
                        "insert into google_location_history
                        (place_name,timestamp_msec,accuracy,address,lat,lng,source)
                        values(?1, $2, $3, $4, $5/10000000.0, $6/10000000.0,'place_visit')",
                        params![
                            place_name,
                            pVisit.duration.startTimestampMs.parse::<i64>().unwrap(),
                            pVisit.location.locationConfidence,
                            address,
                            lat,
                            lng
                        ],
                    ) /*.map_err(|err| println!("{:?}", err))*/
                    .ok();
                }
            }
        }
    }
}
