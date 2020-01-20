extern crate csv;

/// This Enum lists the errors we expect to deal with in CapTable
#[derive(Debug)]
pub enum CapTableError {
    // Problems with the CSV input file
    NoCSVFileSupplied,
    UnableToOpenCSVFileForRead(std::io::Error),
    UnableToReadCSVData(csv::Error),

    // Logic problems with the data
    TotalSharesIsZero,

    // Problems with the report date
    InvalidReportDateSupplied(chrono::ParseError),
}
