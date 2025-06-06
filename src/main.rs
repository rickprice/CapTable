mod error;
mod model;

use chrono::{Local, NaiveDate};
use clap::Parser;
use error::CapTableError;

use std::fs::File;
use std::path::Path;

use model::OutputAccumulator;
use model::Record;

#[derive(Parser)]
#[command(name = "cap-table")]
#[command(about = "Creates a JSON Cap Table from a CSV input file")]
#[command(version)]
struct Args {
    #[arg(
        short = 'f',
        long,
        help = "A CSV input file that contains data used to create a Cap Table"
    )]
    csv_file: String,

    #[arg(
        short = 'd',
        long,
        help = "Date to calculate report to (YYYY-MM-DD format)"
    )]
    report_date: Option<String>,
}

fn main() -> Result<(), CapTableError> {
    let args = Args::parse();

    let report_date = args
        .report_date
        .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
        .transpose()
        .map_err(CapTableError::InvalidReportDateSupplied)?;

    testable_main(&args.csv_file, report_date)?;

    Ok(())
}

fn testable_main(
    input_file_path: &str,
    report_date: Option<NaiveDate>,
) -> Result<(), CapTableError> {
    let input_file_path_path = Path::new(input_file_path);

    let input_file =
        File::open(&input_file_path_path).map_err(CapTableError::UnableToOpenCSVFileForRead)?;

    let mut rdr = csv::Reader::from_reader(input_file);

    let all_records = rdr
        .deserialize::<Record>()
        .collect::<Result<Vec<_>, _>>()
        .map_err(CapTableError::UnableToReadCSVData)?;

    let filter_date = report_date.unwrap_or_else(|| Local::now().date_naive());

    let mut output_accumulator = OutputAccumulator::new(filter_date);

    output_accumulator.accumulate_ownership_transactions(all_records.into_iter())?;

    let serialized = serde_json::to_string_pretty(&output_accumulator)
        .map_err(|_| CapTableError::SerializationError)?;

    println!("{}", serialized);

    Ok(())
}
