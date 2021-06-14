use indicatif::{ProgressBar, ProgressStyle};
use rusqlite::{Connection, Result};
use std::thread;
use std::time::{Duration, Instant};
use structopt::StructOpt;
use walkdir::WalkDir;

use excavator::activities::facebook::facebook_last_location;
use excavator::activities::facebook::facebook_location_history;
use excavator::activities::google::google_fit_activity;
use excavator::activities::google::{location_history, saved_places, semantic_location_history};
use excavator::activities::{
    facebook::{device_location, primary_public_location},
    MyActivity, PrimaryLocation,
};
use excavator::db::schema;
#[derive(Debug, StructOpt)]
#[structopt(
    name = "excavator",
    about = "Visualize your digital footprint",
    setting(structopt::clap::AppSettings::ArgRequiredElseHelp),
    setting(structopt::clap::AppSettings::ColoredHelp)
)]
struct Opt {
    command: String,
    directory_name: String,
    //#[structopt(parse(from_os_str))]
    // file: PathBuf,
    #[structopt(short = "d", long = "dbfile", default_value = "ichnion.db")]
    dbfile: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut total_records : usize =0;
    let args = Opt::from_args();
    let start = Instant::now();
    let conn = Connection::open(&args.dbfile)?;
    schema::create_tables(&conn);
    let directory_name = &args.directory_name;
    let total_count = WalkDir::new(directory_name).into_iter().count();
    let pb = ProgressBar::new(total_count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.purple} [{elapsed_precise}] [{bar:70.cyan/blue}] ({pos}/{len}, ETA \
                 {eta})",
            )
            .progress_chars("#>-"),
    );

    for i in 0..=total_count {
        pb.set_position(i as u64);
        thread::sleep(Duration::from_millis(124));
    }

    for entry in WalkDir::new(directory_name)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let d_name = entry.path().to_string_lossy();
        if f_name.starts_with("MyActivity.json")
            || f_name.starts_with("search-history.json")
            || f_name.starts_with("watch-history.json")
        {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;

            let result: Vec<MyActivity> = serde_json::from_str(&rawdata)?;

            for elem in result.iter() {
                elem.saveToDb(&conn)?;
            }
            total_records += result.len();
            println!("( {} records )", result.len());
        } else if f_name.starts_with("Location History.json") {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;

            let result: location_history::LocationHistory = serde_json::from_str(&rawdata)?;

            result.saveToDb(&conn)?;
            total_records += result.locations.len();
            println!("( {} records )", result.locations.len());
        } else if f_name.starts_with(
            "
            Saved Place.json",
        ) {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;

            let result: saved_places::SavedPlace = serde_json::from_str(&rawdata)?;
            total_records += result.features.len();
            result.saveToDb(&conn)?;
            println!("( {} records )", result.features.len());
        } else if d_name.contains("Semantic Location History") && f_name.ends_with(".json") {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;

            let result: semantic_location_history::TimeLineObjects =
                serde_json::from_str(&rawdata)?;
            total_records += result.timelineObjects.len();
            println!("( {} records )", result.timelineObjects.len());
            result.saveToDb(&conn)?;
        } else if d_name.contains("All Sessions") && f_name.ends_with(".json") {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;
            total_records += 1;
            let result: google_fit_activity::Fit = serde_json::from_str(&rawdata)?;
            println!("( 1 record )");
            result.saveToDb(&conn)?;

        // Facebook activities
        } else if f_name.starts_with("device_location.json") {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;
            total_records += 1;
            let result: device_location::DeviceLocation = serde_json::from_str(&rawdata)?;
            println!("( 1 record )");
            let response = result.saveToDb(&conn)?;
            println!("{:?}", response);
        } else if f_name.starts_with("primary_location.json") {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;
            total_records += 1;
            let result: PrimaryLocation = serde_json::from_str(&rawdata)?;
            println!("( 1 record )");
            let response = result.saveToDb(&conn)?;
            println!("{:?}", response);
        } else if f_name.starts_with("primary_public_location.json") {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;
            total_records += 1;
            let result: primary_public_location::PrimaryPublicLocation =
                serde_json::from_str(&rawdata)?;
            println!("( 1 record )");
            let response = result.saveToDb(&conn)?;
            println!("{:?}", response);
        } else if f_name.starts_with("last_location.json") {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;
            total_records += 1;
            let result: facebook_last_location::LastLocation = serde_json::from_str(&rawdata)?;
            println!("( 1 record )");
            let response = result.saveToDb(&conn)?;
            println!("{:?}", response);
        } else if f_name.starts_with("location_history.json") {
            println!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;

            let result: facebook_location_history::LocationHistory =
                serde_json::from_str(&rawdata)?;
            let response = result.saveToDb(&conn)?;
            let total_records2 = total_records;
            total_records= total_records + result.location_history.unwrap_or_default().len() + result.location_history_v2.unwrap_or_default().len();
            println!(
                "({} records)",
                total_records-total_records2
            );
            println!("{:?}", response);
        }
        else {
            if f_name.ends_with(".json") {
                println!("{} (skipped)", d_name);
            }
        }
    }
    println!("total duration : {:?}", start.elapsed());
    println!("Total records : {}", total_records);
    //println!(""); //I don't know why it is needed. If we don't write something on this line, the "Total records" is not printed. 
    Ok(())
}

