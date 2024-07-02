use coding::{Code, CodingSystem, DataSource};
use serde::Deserialize;
use serde_json;
use std::fs;
use reqwest;

pub async fn load(filename:&str) -> Result<String,Box<dyn std::error::Error>> {
    println!(".... in load {}: ",filename);
    let ds=DataSource::new(filename);
    println!("{:?}",ds);
    let txt = match ds {
        DataSource::URL(x) => {
            let response = reqwest::get(x).await?;
            response.text().await?
        },
        DataSource::File(x) => fs::read_to_string(&x)?
    };
    Ok(txt)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct  JsonLine {
    soc_code:String,
    title:String
}


// need to move this to lib.rs
// CodingSystem::from_json, from_csv, from_tdt
pub async fn load_coding_system(filename:&str,coding_system_name:&str) -> 
    Result<CodingSystem,Box<dyn std::error::Error>> {   

    let mut codes:Vec<Code> = vec![];
    let txt = load(filename).await?;
    let parts: Vec<&str> = filename.split('.').collect();
    let suffix = parts.last();
    match suffix {
        Some(v) => match *v {
            "json" => {
                println!("==== JSON ===");
                let jobs:Vec<JsonLine> = serde_json::from_str(&txt)?;
                for job in jobs{
                    codes.push(Code::new(&job.soc_code,&job.title,coding_system_name))
                }
            },
            "csv" => {
                println!("==== CSV ===");
                let mut reader = csv::Reader::from_reader(txt.as_bytes());
                for record in reader.records() {
                    let record = record?;
                    codes.push(Code::new(&record[0],&record[1],coding_system_name ))
                }
            },
            "tdt" => {
                println!("==== TDT ===");
                let mut reader = csv::ReaderBuilder::new()
                    .delimiter(b'\t')
                    .from_reader(txt.as_bytes());
                for record in reader.records() {
                    let record = record?;
                    codes.push(Code::new(&record[0],&record[1],coding_system_name ))
                }
            },
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, 
              format!("Cannot handle file type: .{}.  Please use .csv, .tdt, or .json files.",v)) ))
        },
        None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,
             format!("Cannot handle file {}",filename))))
    };

    Ok(CodingSystem{
        name:coding_system_name.to_string(),
        codes
    })
}

#[tokio::main]
async fn main() {
    let result =load_coding_system("https://danielruss.github.io/codingsystems/soc1980_all.csv","soc 1980").await;
    match result {
        Ok(coding_system) => println!("{:?}", coding_system.name),
        Err(error) => eprintln!("Error loading coding system: {}", error)
    };
    let result =load_coding_system("https://danielruss.github.io/codingsystems/soc2010_6digit.json","soc 2010").await;
    match result {
        Ok(coding_system) => println!("{:?}", coding_system.name),
        Err(error) => eprintln!("Error loading coding system: {}", error)
    };
    let result =load_coding_system("/Users/druss/Downloads/soc2010_all.csv","soc 2010").await;
    match result {
        Ok(coding_system) => println!("{:?}", coding_system.name),
        Err(error) => eprintln!("Error loading coding system: {}", error)
    };
    let result =load_coding_system("/Users/druss/Downloads/soc2010_all.td","soc 2010").await;
    match result {
        Ok(coding_system) => println!("{:?}", coding_system.name),
        Err(error) => eprintln!("Error loading coding system: {}", error)
    };
    let result =load_coding_system("/Users/druss/Downloads/demo.txt","soc 2010").await;
    match result {
        Ok(coding_system) => println!("{:?}", coding_system.name),
        Err(error) => eprintln!("Error loading coding system: {}", error)
    };
    let result =load_coding_system("/Users/druss/Downloads/soc2010_all.tdt","soc 2010").await;
    match result {
        Ok(coding_system) => println!("{:?}", coding_system.name),
        Err(error) => eprintln!("Error loading coding system: {}", error)
    };
}