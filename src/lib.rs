use reqwest::Client;
use csv::{ReaderBuilder,StringRecord};
use std::error::Error;
use std::fmt;
use url::Url;

/**
 * Maybe I should have made a Coding system Struct instead of an
 * enum... Now I am limited to these types.
 * 
 * maybe... struct with 
 *     HashMap of code -> <title,level,parent>...
 *     levels..
 *     codename
 * */
pub struct CodingSystem{
    pub name: String,
    pub codes: Vec<Code>
}

impl CodingSystem {
    pub async fn from_csv_url(url: &str,coding_system:&str) -> Result<Self, Box<dyn Error>> {
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
            match Code::from_record(record, 0, 1, coding_system) {
                Ok(code) => codes.push(code),
                Err(error) => eprintln!("Warning: error on line {line} {error:?}")
            };
        }


        Ok(CodingSystem { name: coding_system.to_string(), codes: codes })
    }

    pub fn add_code(&mut self,code:&str,title:&str){
        self.codes.push(Code::new(code, title, &self.name));
    }

}


#[derive(Debug)]
pub enum DataSource{
    File(String),
    URL(String)
}
impl DataSource {
    pub fn new(path:&str) -> Self{
        if Url::parse(path).is_ok() {
            return DataSource::URL(path.to_string())
        }
        return DataSource::File(path.to_string())
    }    
}


#[derive(Default,Debug,PartialEq)]
pub struct Code{
    code: String,
    title: String,
    coding_system: String
}

impl Code{
    pub fn new(code:&str,title:&str,coding_system:&str) ->Self {
        Code{
            code: code.to_string(),
            title: title.to_string(),
            coding_system: coding_system.to_string()
        }
    }

    pub fn from_record(rec:StringRecord,code_index:usize,title_index:usize,coding_system:&str) -> Result<Self,Box<dyn Error>> {
        let code = rec.get(code_index).ok_or("Problem getting code for line")?;
        let title = rec.get(title_index).ok_or("Problem getting title for line")?;
        let coding_system = coding_system;
        Ok(Code::new(code,title,coding_system))
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "| {} | {} | {}", self.coding_system, self.code, self.title)
    }
}


#[derive(Default,Debug)]
pub struct JobDescription{
    job_title:String,
    job_task:String,
    occupation_code:Code,
}

impl JobDescription{
    pub fn new(job_title:&str,job_task:&str,occupation_code:Code)->Self{
        JobDescription{
            job_title:job_title.to_string(),
            job_task:job_task.to_string(),
            occupation_code:occupation_code
        }
    }

    pub fn clear(&mut self){
        self.job_task="".to_string();
        self.job_title="".to_string();
        self.occupation_code=Code::new("", "", "");
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_job_description() {
        
        let code:Code = Code::new("11-1011","","soc 2018");
        println!("The code is: {code:?}");
        assert_eq!(code.code,"11-1011");
        assert_eq!(code.coding_system,"soc 2018");

        let job:JobDescription = JobDescription{
            job_title:String::from("Dentist"),
            job_task:String::from(""),
            occupation_code:Code::new("29-1021","","soc 2018")
        };
        assert_eq!(job.job_title,"Dentist");
        assert_eq!(job.occupation_code.code,"29-1021");
        assert_eq!(job.occupation_code.coding_system,"soc 2018");
 
        let job=JobDescription::new("Dentist","clean teeth",Code::new("29-1021","Dentists, General","soc 2018"));
        assert_eq!(job.job_title,"Dentist");
        assert_eq!(job.job_task,"clean teeth");
        assert_eq!(job.occupation_code.code,"29-1021");
        assert_eq!(job.occupation_code.title,"Dentists, General");
        assert_eq!(job.occupation_code.coding_system,"soc 2018");
    }


    #[tokio::test]
    async fn test_load_noc2011(){
        let noc2011 = CodingSystem::from_csv_url("https://danielruss.github.io/codingsystems/noc_2011_all.csv", "noc 2011").await.unwrap();
        assert_eq!(noc2011.codes.len(),690,"Should have 690 code - returned num_codes: {}",noc2011.codes.len());

        let first_code = noc2011.codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"0");
    }

    #[tokio::test]
    async fn test_load_soc1980(){
        let soc1980=CodingSystem::from_csv_url("https://danielruss.github.io/codingsystems/soc1980_all.csv", "soc 1980").await.unwrap();
        assert_eq!(soc1980.codes.len(),850,"Should have 850 code - returned num_codes: {}",soc1980.codes.len());
        assert_eq!(soc1980.name,"soc 1980");

        let first_code = soc1980.codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"11-18");
    }

    #[tokio::test]
    async fn test_load_soc2000(){
        let soc2000 = CodingSystem::from_csv_url("https://danielruss.github.io/codingsystems/soc2000.csv", "soc 2000").await.unwrap();
        assert_eq!(soc2000.codes.len(),1389,"Should have 1389 code - returned num_codes: {}",soc2000.codes.len());

        let first_code = soc2000.codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"11-0000");
    }

    #[tokio::test]
    async fn test_load_soc2010(){
        let soc2010 = CodingSystem::from_csv_url("https://danielruss.github.io/codingsystems/soc2010_all.csv", "soc 2010").await.unwrap();
        assert_eq!(soc2010.codes.len(),1425,"Should have 1425 code - returned num_codes: {}",soc2010.codes.len());

        let first_code = soc2010.codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"11-0000");
    }

    #[tokio::test]
    async fn test_load_soc2018(){
        let soc2018 = CodingSystem::from_csv_url("https://danielruss.github.io/codingsystems/soc2018_all.csv","soc 2018").await.unwrap();
        assert_eq!(soc2018.codes.len(),1447,"Should have 1447 code - returned num_codes: {}",soc2018.codes.len());

        let first_code = soc2018.codes.get(0).unwrap() ;
        assert_eq!(first_code.code,"11-0000");
    }

    #[tokio::test]
    async fn test_add_code(){
        let mut soc2018 = CodingSystem::from_csv_url("https://danielruss.github.io/codingsystems/soc2018_all.csv","soc 2018").await.unwrap();
        soc2018.add_code("99-9997", "No Clue");
        let code =soc2018.codes.get( soc2018.codes.len() - 1 as usize ).unwrap();
        println!("===> {:?}", code);
        assert_eq!(code.code,"99-9997");
        assert_eq!(code.title,"No Clue");
        assert_eq!(code.coding_system,"soc 2018");
    }

}
