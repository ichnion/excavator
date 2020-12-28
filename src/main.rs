#[macro_use]
extern crate diesel;

//use crate::trackpoints::places::SavedPlace;
use std::path::PathBuf;
use structopt::StructOpt;

mod trackpoints;
mod google;
mod db;

#[derive(Debug, StructOpt)]
#[structopt(name = "excavator")]
struct Opt {
    command: String,
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    let rawdata = std::fs::read_to_string(&args.file)?;
    
    //let result: SavedPlace = serde_json::from_str(&rawdata)?;
    //println!("{:?}", result);    
    
    let result: Vec<google::my_activity::MyActivity> = 
        serde_json::from_str(&rawdata)?;
    
    for elem in result.iter()  {
        elem.saveToDb();
    }
    
    Ok(())
}
