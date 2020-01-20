extern crate chrono;
extern crate clap;
extern crate csv;
extern crate serde;

mod error;
mod model;

use chrono::{Local, NaiveDate};
use clap::{crate_version, App, Arg};
use error::CapTableError;

use std::fs::File;
use std::path::Path;

use model::OutputAccumulator;
use model::Record;

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

    // if no report_date was passed in, then fine, otherwise parse it into a NaiveDate, if that
    // fails, then return an error...
    let report_date = match matches.value_of("reportDate") {
        None => None,
        Some(s) => Some(
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                // We map the error so that we can return it in our own structure
                .map_err(|e| CapTableError::InvalidReportDateSupplied(e))?,
        ),
    };

    // To make testing easier, we put the important parts of main into another function so we can
    // call it in test code
    testable_main(input_file_path, report_date)?;

    Ok(())
}

fn testable_main(
    input_file_path: &str,
    report_date: Option<NaiveDate>,
) -> Result<(), CapTableError> {
    // We know we will never have a null input_file_path, because we weren't passed an Option<>
/*    
    println!(
        "We will be reading the CSV data from the file located at: {}",
        input_file_path
    );
*/

/*    
    // Decide what to print based on whether we got a report_date or not...
    match report_date {
        None => println!("We will be using today's date for the report date"),
        Some(d) => println!(
            "We will be using the following date for the report date: {}",
            d
        ),
    };
*/

    let input_file_path_path = Path::new(input_file_path);
    // Pass the error value into the CapTableError...
    let input_file = File::open(&input_file_path_path)
        // We map the error so that we can return it in our own structure
        .map_err(|e| CapTableError::UnableToOpenCSVFileForRead(e))?;

    let mut rdr = csv::Reader::from_reader(input_file);

    // Get an iterator of some sort to our Records, if we hit an error, then quit the program with
    // an error
    let all_records = rdr
        .deserialize()
        .map(|r: std::result::Result<Record, csv::Error>| {
            r.map_err(|e| CapTableError::UnableToReadCSVData(e))
                .unwrap()
        });

    // If no report_date has been specified, then use the current date
    let filter_date = match report_date {
        None => Local::today().naive_local(),
        Some(filter_date) => filter_date,
    };

    let mut output_accumulator = OutputAccumulator::new(filter_date);

    output_accumulator.accumulate_ownership_transactions(all_records)?;

    //    println!("Output accumulator is: {:?}", output_accumulator);

    //    let serialized = serde_json::to_string(&output_accumulator).unwrap();
    let serialized = serde_json::to_string_pretty(&output_accumulator).unwrap();

    println!("{}", serialized);

    return Ok(());
}
