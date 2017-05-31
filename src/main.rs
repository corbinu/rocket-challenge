#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate csv;
extern crate slug;
extern crate rustc_serialize;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{JSON, Value};
use std::collections::HashMap;

#[derive(RustcDecodable)]
struct CSVData {
    headers: Vec<String>,
    rows: Vec<Vec<String>>
}

#[derive(Debug)]
enum ParseError {
    CSV(csv::Error)
}

impl From<csv::Error> for ParseError {
    fn from(err: csv::Error) -> ParseError {
        ParseError::CSV(err)
    }
}

fn parse_csv() -> Result<CSVData, ParseError> {
    let mut rdr = csv::Reader::from_file("./data/wholesale.csv")?;

    let  headers: Vec<String> = rdr.headers()?.into_iter().map(move |header| {
        slug::slugify(header)
    }).collect();

    let mut rows: Vec<Vec<String>> = Vec::new();

    for record in rdr.decode() {
        let record: Vec<String> = record?;

        // convert to read the headers and slugify them and then return header and string vector

        rows.push(record);
    }

    Ok(CSVData {
        headers,
        rows
    })
}

fn to_json_arr(csv_data: CSVData) -> Vec<Value> {
     // convert the new vector to a JSON object
     let mut json_arr: Vec<Value> = Vec::new();

     for row in csv_data.rows.iter() {
        let mut csv_row: HashMap<String, String> = HashMap::new();

        for (i, header) in csv_data.headers.iter().enumerate() {
            csv_row.insert(header.clone(), row[i].clone());
        }

        json_arr.push(json!(csv_row));
     }

     json_arr
}

#[get("/")]
fn index() -> Result<JSON<Value>, ParseError> {
    let rocket_data = parse_csv()?;

    let rocket_json = to_json_arr(rocket_data);

    Ok(JSON(json!(rocket_json)))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
