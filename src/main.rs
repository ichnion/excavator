use rusqlite::{Connection, Result};
use structopt::StructOpt;
use activities::facebook::{device_location, primary_location, primary_public_location};
use activities::google::{location_history, my_activity, saved_places};
use walkdir::WalkDir;

mod db;
mod activities;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "excavator",
    about = "Visualize your digital footprint",
    setting(clap::AppSettings::ArgRequiredElseHelp),
    setting(clap::AppSettings::ColoredHelp)
)]
struct Opt {
    command: String,
    directory_name: String,
    //#[structopt(parse(from_os_str))]
    //file: PathBuf,
    #[structopt(short = "d", long = "dbfile", default_value = "ichnion.db")]
    dbfile: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("ichnion.db")?;
    db::schema::create_tables(&conn);

    let args = Opt::from_args();

    let conn = Connection::open(&args.dbfile)?;
    db::schema::create_tables(&conn);


    let directory_name = &args.directory_name;

    for entry in WalkDir::new(directory_name)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let d_name = entry.path().to_string_lossy();
        let file_name = f_name.as_ref();
        println!("file_name: {:?}", file_name);
        match file_name {
            "MyActivity.json" | "search-history.json" | "watch-history.json" => {
                print!("processing {}", d_name);

                let rawdata = std::fs::read_to_string(&entry.path())?;

                let result: Vec<my_activity::MyActivity> = serde_json::from_str(&rawdata)?;

                for elem in result.iter() {
                    elem.saveToDb(&conn);
                }
                println!("( {} records )", result.len());
            }
            "Location\tHistory.json" => {
                print!("processing {}", d_name);

                let rawdata = std::fs::read_to_string(&entry.path())?;

                let result: location_history::LocationHistory = serde_json::from_str(&rawdata)?;

                result.saveToDb(&conn);

                println!("( {} records )", result.locations.len());
            }
            "Saved\tPlace.json" => {
                println!("processing {}", d_name);

                let rawdata = std::fs::read_to_string(&entry.path())?;

                let result: saved_places::SavedPlace = serde_json::from_str(&rawdata)?;

                result.saveToDb(&conn);
            }
            "device_location.json" => {
                println!("processing {}", d_name);

                let rawdata = std::fs::read_to_string(&entry.path())?;

                let result: device_location::DeviceLocation = serde_json::from_str(&rawdata)?;

                let response = result.saveToDb(&conn)?;

                println!("( {} records )", result.phone_number_location.len());
                println!("{:?}", response);
            }
            "primary_location.json" => {
                println!("processing {}", d_name);

                let rawdata = std::fs::read_to_string(&entry.path())?;

                let result: primary_location::PrimaryLocation = serde_json::from_str(&rawdata)?;

                let response = result.saveToDb(&conn)?;

                println!(
                    "( {} records )",
                    result.primary_location.city_region_pairs.len()
                );
                println!("{:?}", response);
            }
            "primary_public_location.json" => {
                println!("processing {}", d_name);

                let rawdata = std::fs::read_to_string(&entry.path())?;

                let result: primary_public_location::PrimaryPublicLocation =
                    serde_json::from_str(&rawdata)?;

                let response = result.saveToDb(&conn)?;
                println!("{:?}", response);
            }
            _ => println!("No files are matched"),
        }
    }

    println!("\n*** process completed ***");
    Ok(())
}
