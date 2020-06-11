use std::error::Error;
use serde;
use serde::Deserialize;
use itertools::Itertools;
use std::time::{Instant};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct FakeData {
    #[serde(rename = "Column1")]
    column1: String,
    #[serde(rename = "Column2")]
    column2: u32,
    #[serde(rename = "Column3")]
    column3: u32,
    #[serde(rename = "Column4")]
    column4: String,
    #[serde(rename = "Column5")]
    column5: String
}

fn read_csv() -> Result<Vec<FakeData>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(r"C:\projects\csv-processing\examples\simple.csv")?;

    let headers = reader.headers()?;
    println!("Headers: {:?}", headers.iter().format(" "));

    let mut read_data = Vec::new();

    for result in reader.deserialize() {
        let record: FakeData = result?;
        read_data.push(record);
    }

    println!("Processed {} records", read_data.len());

    return Ok(read_data);
}

fn write_csv(data_points: &Vec<FakeData>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(r"C:\projects\csv-processing\examples\output.csv")?;

    for line in data_points {
        file.write_all(format!("{}|{}|{}|{}|{}\n", line.column1, line.column2, line.column3, line.column4, line.column5).as_bytes())?;
    }

    file.flush()?;

    return Ok(());
}

fn main() {

    let now = Instant::now();

    let fake_data = read_csv().unwrap();
    write_csv(&fake_data).unwrap();

    println!("Total time taken: {}", now.elapsed().as_millis());
}
