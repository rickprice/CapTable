use thiserror::Error;

/// This Enum lists the errors we expect to deal with in CapTable
#[derive(Error,Debug)]
pub enum CapTableError {
    // Problems with the CSV input file
    #[error("Unable to open CSV input file")]
    UnableToOpenCSVFileForRead(#[from] std::io::Error),

    #[error("Unable to read CSV data")]
    UnableToReadCSVData(#[from] csv::Error),

    // Logic problems with the data
    #[error("Total shares is zero")]
    TotalSharesIsZero,

    // Problems with the report date
    #[error("Invalid report date supplied")]
    InvalidReportDateSupplied(#[from] chrono::ParseError),
}
