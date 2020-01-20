# CapTable
Cap Table Calculator

+++ Commentary +++

Hi, I chose to use Rust for the exercise because you wanted an example that was production quality and well structured. In my experience handling errors is a critical part of production software and Rust handles errors better and moreesaily than most languages because it is part of the culture. Most of the errors that are handled, were handled because errors are not ignored in Rust, I added a few more to check for things like (0 share total which would have caused a division by zero error), the errors that the program *flags* are in (error.rs->CapTableError).

In the exercise I tried to minimize copying of data, which is also part of the Rust culture.

For the overall structure, I used a CSV reader library to read the CSV data into a structure (model.rs->struct Record), I then iterated over the rows of the CSV putting the data into another structure (model.rs->OutputAccumlator) to accumulate the totals and ownership records (models.rs->OwnershipRecord). I used another library to output the JSON.

The CSV reader library doesn't return an array, but instead returns an iterator, this is much, much more memory efficient, because even if the CSV file was extremely large, only the absolute minimum data is loaded at a time. Using an iterator can also reduce data copying.

I tried to make as few passes over the data as possible, but in the end, I ended up doing 3 because of needing to update the Ownership->ownership percentage value. So we have the first pass to read the CSV file, another (presumably smaller pass over the Ownership records to update the ownership percentage, and then a final pass to output the totals and the Ownership records to JSON.

I have tried to think about this example as I would for my production software at my current work, where I would assume that there are hundreds of thousands to millions of rows to be processed.

Using libraries for the command line parameters, CSV reading, JSON outputting saved time and energy and improved user experience over what might have been *hand rolled*.

Because the CSV reader and JSON outputter libraries use another library called Serde, all conversions (date, number) and error handling were done for me. Not having to worry about edge cases is a good thing.


+++ Installing the Rust compiler +++

Good instructions are here on the Internet at: https:\\rustup.rs

Basically you download the installer that you need, and then run the installer.


+++ Running the program +++

Once Rust is installed, go into the project directory and run "cargo build", cargo (the Rust equivalent of Make, Maven or Gradle), will then download all the required packages (listed in Cargo.toml) and compile everything.

To run the program, type "cargo run -- --CSVFile test_input_1.csv", the output will be the requested JSON.

Another way to run the program is like this "target/debug/cap_table.exe --CSVFile test_input_1.csv"

To do a release build do this "cargo build --release", you will find that release builds are _very_ fast.

To run the release program, run it like this "target/release/cap_table.exe --CSVFile test_input_1.csv"

There are 7 csv files in the directory as follows:

- test_input_1.csv -> The test data given with the quiz
- test_input_2.csv -> The test data given with the quiz, but line 3 purposely has a negative number of shares, this is flagged by the program
- test_input_3.csv -> The test data given with the quiz, but line 1 purposely has an invalid date, this is flagged by the program
- test_input_4.csv -> The test data given with the quiz, but line 7 has a record that is in the future, unless you use a report date in the future, that record will be ignored in the calculations
- test_input_5.csv -> The test data given with the quiz, with one addition, and all the share numbers are zero, which lets us test for the division by zero avoidance.
- test_input_6.csv -> No actual test data, shows the division by zero avoidance
- test_input_7.csv -> Nothing in the file, shows the division by zero avoidance

To run the program with a report date, do it like this "target/release/cap_table.exe --CSVFile test_input_4.csv --report_date 2028-01-20"

That date is in the future, and the extra shares will be recorded in the output.

When no report date is specified, the date today is used as the report date.

To test that a division by zero error is avoided when there are no shares to process, do it like this "target/release/cap_table.exe --CSVFile test_input_5.csv --report_date 2028-01-20"

You can also do "target/release/cap_table.exe --CSVFile test_input_6.csv" and "target/release/cap_table.exe --CSVFile test_input_7.csv"


I can write software in other languages as well like Java, C# and Go, I just used Rust because it would be quicker for me to deliver a quality product with greater efficiency of my time.

