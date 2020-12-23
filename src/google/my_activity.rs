use serde::Deserialize;
#[allow(non_snake_case)]

#[derive(Deserialize, Debug)]
pub struct MyActivity {
    pub header        : String,
    pub title         : String,
    pub subtitles     : Option<Vec<SubTitle>>,    
    pub titleUrl      : Option<String>,
    pub time          : String,
    pub products      : Vec<String>,
    pub details       : Option<Vec<Details>>,
    pub locationInfos : Option<Vec<LocationInfo>>
}


#[derive(Deserialize, Debug)]
pub struct LocationInfo {
    pub name   : String,
    pub url    : String,
    pub source : String
}

#[derive(Deserialize, Debug)]
pub struct Details {
    pub name   : String
}

#[derive(Deserialize, Debug)]
pub struct SubTitle {
    pub name   : String,
    pub url    : Option<String>
}
