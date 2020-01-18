extern crate clap;
extern crate cap_table_error;
extern crate chrono;

use clap::{crate_version, App, Arg};
use cap_table_error::error::CapTableError;
use chrono::{NaiveDate};


fn main() ->Result<(),CapTableError> {
    let matches = App::new("Cap Table Program")
        .version(crate_version!())
        .author("Frederick Price <rprice@pricemail.ca>")
        .about("Creates a JSON Cap Table from a CSV input file")
        .arg(
            Arg::with_name("CSVFile")
                .short("f")
                .long("CSVFile")
                .takes_value(true)
                .help("A CSV input file that contains data used to create a Cap Table"),
        )
        .arg(
            Arg::with_name("JSONOutputFile")
                .short("o")
                .long("JSONOutputFile")
                .takes_value(true)
                .help("A file to write the JSON output to"),
        )
        .arg(
            Arg::with_name("report_date")
                .short("d")
                .long("report_date")
                .takes_value(true)
                .help("Date to calculate report to"),
        )
        .get_matches();

    let input_file_path = matches.value_of("CSVFile").ok_or(CapTableError::NoCSVFileSupplied)?;
    let output_file_path = matches.value_of("JSONOutputFile");


    println!("The file passed is: {}", input_file_path);

    let report_date = match matches.value_of("report_date") {
        None=> None,
        Some(s) => Some(NaiveDate::parse_from_str(s,"%Y-%m-%d").map_err(|e| CapTableError::InvalidReportDateSupplied)?),
    };

    let num_str = matches.value_of("report_date");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Your favorite number must be {}.", n + 5),
            Err(_) => println!("That's not a number! {}", s),
        },
    }
    testable_main(input_file_path, output_file_path, report_date)?;

    Ok(())
}

fn testable_main(input_file_path: &str, output_file_path: Option<&str>, report_date: Option<NaiveDate>) -> Result<(), CapTableError> {

    return Ok(());
}
