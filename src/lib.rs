use reqwest::Client;
use csv::{ReaderBuilder,StringRecord};
use std::error::Error;
use tokio;



#[derive(Default,Debug,Clone,PartialEq)]
pub enum CodingSystem{
    NOC2011,
    SOC1980,
    SOC2000,
    #[default]
    SOC2010,
    SOC2018,
}


#[derive(Default,Debug,PartialEq)]
pub struct Code{
    code: String,
    title: String,
    coding_system: CodingSystem
}

impl Code{
    pub fn from_record(rec:StringRecord,code_index:usize,title_index:usize,coding_system:CodingSystem) -> Result<Self,Box<dyn Error>> {
        let code = rec.get(code_index).ok_or("Problem getting code for line")?.to_string();
        let title = rec.get(title_index).ok_or("Problem getting title for line")?.to_string();
        Ok(Code{
            code: code,
            title: title,
            coding_system: coding_system
        })
    }
}

#[derive(Default,Debug)]
pub struct JobDescription{
    job_title:String,
    code:Code,
}

async fn load_codes(url: &str,system:CodingSystem) -> Result<Vec<Code>, Box<dyn Error>> {
    let mut codes = vec![];

    println!("The url is {}", url);

    let client = Client::new();
    let res = client.get(url).send().await?;
    let data = res.text().await?;

    let mut rdr = ReaderBuilder::new().from_reader(data.as_bytes());
    let mut line = 0;
    for result in rdr.records() {
        line = line+1;
        let record = result?;
        match Code::from_record(record, 0, 1, system.clone()) {
            Ok(code) => codes.push(code),
            Err(error) => eprintln!("Warning: error on line {line} {error:?}")
        };
    }

    Ok(codes)
}

pub async fn load_coding_system(coding_system:CodingSystem)  -> Result<Vec<Code>, Box<dyn Error>>{
    let url = match coding_system {
        CodingSystem::NOC2011 => "https://danielruss.github.io/codingsystems/noc_2011_all.csv",
        CodingSystem::SOC1980 => "https://danielruss.github.io/codingsystems/soc1980_all.csv",
        CodingSystem::SOC2000 => "https://danielruss.github.io/codingsystems/soc2000.csv",
        CodingSystem::SOC2010 => "https://danielruss.github.io/codingsystems/soc2010_all.csv",
        CodingSystem::SOC2018 => "https://danielruss.github.io/codingsystems/soc2018_all.csv",   
    };
    load_codes(url,coding_system).await
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_job_description() {
        let code:Code = Code {
            code:String::from("11-1011"),
            title: String::from(""),
            coding_system:CodingSystem::SOC2018
        };
        println!("The code is: {code:?}");
        assert_eq!(code.code,"11-1011");
        assert_eq!(code.coding_system,CodingSystem::SOC2018);

        let job:JobDescription = JobDescription{
            job_title:String::from("Dentist"),
            code:Code {
                code:String::from("29-1021"),
                title:String::from(""),
                coding_system:CodingSystem::SOC2018
            }
        };
        assert_eq!(job.job_title,"Dentist");
        assert_eq!(job.code.code,"29-1021");
        assert_eq!(job.code.coding_system,CodingSystem::SOC2018);
    }

    /*
    #[tokio::test]
    async fn test_load(){
        let url = "https://danielruss.github.io/codingsystems/soc2010_all.csv";
        let codes = load_codes(url,CodingSystem::SOC2010.clone()).await.unwrap();

        assert!(codes.len()>0,"no codes returned num_codes: {}",codes.len());
        assert_eq!(codes.len(),1425,"Should have 1425 code - returned num_codes: {}",codes.len());

        let first_code = codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"11-0000");
    }
    */
    
    #[tokio::test]
    async fn test_load_noc2011(){
        let codes = load_coding_system(CodingSystem::NOC2011).await.unwrap();
        assert_eq!(codes.len(),690,"Should have 690 code - returned num_codes: {}",codes.len());

        let first_code = codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"0");
    }

    #[tokio::test]
    async fn test_load_soc1980(){
        let codes = load_coding_system(CodingSystem::SOC1980).await.unwrap();
        assert_eq!(codes.len(),850,"Should have 850 code - returned num_codes: {}",codes.len());

        let first_code = codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"11-18");
    }

    #[tokio::test]
    async fn test_load_soc2000(){
        let codes = load_coding_system(CodingSystem::SOC2000).await.unwrap();
        assert_eq!(codes.len(),1389,"Should have 1389 code - returned num_codes: {}",codes.len());

        let first_code = codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"11-0000");
    }

    #[tokio::test]
    async fn test_load_soc2010(){
        let codes = load_coding_system(CodingSystem::SOC2010).await.unwrap();
        assert_eq!(codes.len(),1425,"Should have 1425 code - returned num_codes: {}",codes.len());

        let first_code = codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"11-0000");
    }

    #[tokio::test]
    async fn test_load_soc2018(){
        let codes = load_coding_system(CodingSystem::SOC2018).await.unwrap();
        assert_eq!(codes.len(),1447,"Should have 1447 code - returned num_codes: {}",codes.len());

        let first_code = codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"11-0000");
    }
}
