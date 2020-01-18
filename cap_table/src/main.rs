extern crate cap_table_error;
extern crate chrono;
extern crate clap;
extern crate csv;
extern crate serde;

use cap_table_error::error::CapTableError;
use chrono::NaiveDate;
use clap::{crate_version, App, Arg};

use std::fs::File;
use std::path::Path;

use serde::Deserialize;

fn main() -> Result<(), CapTableError> {
    let matches = App::new("Cap Table Program")
        .version(crate_version!())
        .author("The Author <the@author.ca>")
        .about("Creates a JSON Cap Table from a CSV input file")
        // CSVFile is a required argument to the program
        .arg(
            Arg::with_name("CSVFile")
                .short("f")
                .long("CSVFile")
                .takes_value(true)
                .help("A CSV input file that contains data used to create a Cap Table"),
        )
        // JSONOutputFile is an optional argument to the program
        .arg(
            Arg::with_name("JSONOutputFile")
                .short("o")
                .long("JSONOutputFile")
                .takes_value(true)
                .help("A file to write the JSON output to"),
        )
        // reportDate is an optional argument to the program
        .arg(
            Arg::with_name("reportDate")
                .short("d")
                .long("report_date")
                .takes_value(true)
                .help("Date to calculate report to"),
        )
        .get_matches();

    // Get the input file path or return an error, notice the ? at the end to short circuit on
    // errors
    let input_file_path = matches
        .value_of("CSVFile")
        .ok_or(CapTableError::NoCSVFileSupplied)?;
    // Get the output file path, but we don't care if it was not passed in
    let output_file_path = matches.value_of("JSONOutputFile");

    // if no report_date was passed in, then fine, otherwise parse it into a NaiveDate, if that
    // fails, then return an error...
    let report_date = match matches.value_of("reportDate") {
        None => None,
        // we ignore the exact error returned by parse_from_str here, we just flag it as an invalid
        // date, but we could have maybe returned it to the user in the InvalidReportDateSupplied
        // enum value.
        Some(s) => Some(
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_e| CapTableError::InvalidReportDateSupplied)?,
        ),
    };

    // To make testing easier, we put the important parts of main into another function so we can
    // call it in test code
    testable_main(input_file_path, output_file_path, report_date)?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename(deserialize = "#INVESTMENT DATE"))]
    investment_date: String,
    #[serde(rename(deserialize = " SHARES PURCHASED"))]
    shares_purchased: u64,
    #[serde(rename(deserialize = " CASH PAID"))]
    cash_paid: f64,
    #[serde(rename(deserialize = " INVESTOR"))]
    investor: String,
}

fn testable_main(
    input_file_path: &str,
    output_file_path: Option<&str>,
    report_date: Option<NaiveDate>,
) -> Result<(), CapTableError> {
    // We know we will never have a null input_file_path
    println!(
        "We will be reading the CSV data from the file located at: {}",
        input_file_path
    );
    // In this case we may or may not get an output_path, but if not, we will write to stdout, this
    // just says so, it does not set the output file to stdout
    println!(
        "We will be writing our output to: {}",
        match output_file_path {
            None => "Stdout",
            Some(s) => s,
        }
    );
    // Decide what to print based on whether we got a report_date or not...
    match report_date {
        None => println!("We will be using today's date for the report date"),
        Some(d) => println!(
            "We will be using the following date for the report date: {}",
            d
        ),
    };

    let input_file_path_path = Path::new(input_file_path);
    // This time, if we have an error, pass the error value into the CapTableError...
    let input_file = File::open(&input_file_path_path)
        .map_err(|e| CapTableError::UnableToOpenCSVFileForRead(e))?;

    let mut rdr = csv::Reader::from_reader(input_file);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result.map_err(|e| CapTableError::UnableToReadCSVData(e))?;
        println!("{:?}", record);
    }

    return Ok(());
}
