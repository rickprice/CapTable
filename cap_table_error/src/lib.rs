pub mod error {

    /// This Enum lists the errors we expect to deal with in CapTable
    #[derive(Debug)]
    pub enum CapTableError {
        NoCSVFileSupplied,
        InvalidReportDateSupplied,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
