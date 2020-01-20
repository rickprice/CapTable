extern crate chrono;
extern crate serde;
extern crate serde_json;

use chrono::NaiveDate;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use std::collections::HashMap;

use crate::error::CapTableError;

// This is our Record where we model the contents of the CSV file, we don't actually use an array
// of these values, we use an iterator which is more efficient
// Debug lets us output the contents of the structure in print statements
// Serialize lets us serialize the structure with Serde, which is a high powered
// serialization/deseriazation library
// Deserialize lets us deserialize the structure using Serde
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    // (rename) We need to tell Serde that the name in the CSV file won't match the name of the
    // field in this structure
    // (deserialize_with) Tell Serde to deserialize this field with a special function
    // (serialize_with) Tell Serde to serialize this field with a special function
    #[serde(
        rename(deserialize = "#INVESTMENT DATE"),
        deserialize_with = "naive_date_from_str",
        serialize_with = "naive_date_to_str"
    )]
    pub investment_date: NaiveDate,
    #[serde(rename(deserialize = " SHARES PURCHASED"))]
    pub shares_purchased: u64,
    #[serde(rename(deserialize = " CASH PAID"))]
    pub cash_paid: f64,
    #[serde(rename(deserialize = " INVESTOR"))]
    pub investor: String,
}

// Special function to be used by the Serde library to deserialize a Naieve date, normally things
// including dates deserialize automagically, but I guess the library authers didn't expect us to
// serialize or deserialize Naive Dates, Rust prefers dates with time zones since you can get into
// so many problems when you ignore time zones in the real world.
fn naive_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}

// Special function to serilize Naive dates, see comments above...
fn naive_date_to_str<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // I don't like serializing with a different format than we deserialize with, maybe there is a
    // way to pass in a custom format parameter...
    let s = format!("{}", date.format("%m/%d/%Y"));
    serializer.serialize_str(&s)
}

// Serde normally outputs float values with a lot more precision, this is how we reduce that
// precision for the report
fn f64_to_str_two_decimals<S>(number: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // I don't like serializing with a different format than we deserialize with, maybe there is a
    // way to pass in a custom format parameter...
    let s = format!("{:.2}", number);
    serializer.serialize_str(&s)
}


// This is the Ownership Record, which is used inside the OutputAccumulator to track the summary
// data for people who own shares
#[derive(Debug, Clone, Serialize)]
pub struct OwnershipRecord {
    pub investor: String,
    pub shares: u64,
    #[serde(serialize_with = "f64_to_str_two_decimals")]
    pub cash_paid: f64,
    #[serde(serialize_with = "f64_to_str_two_decimals")]
    pub ownership: f64,
}

impl OwnershipRecord {
    pub fn new(investor: String, shares: u64, cash_paid: f64) -> OwnershipRecord {
        OwnershipRecord {
            investor,
            shares,
            cash_paid,
            ownership:0.0,
        }
    }

    // Update the ownership percentage after all the share records have been processed
    pub fn fix_ownership_percentage(&mut self,total_shares: u64) {
        self.ownership = (self.shares as f64)/(total_shares as f64)*100.0;
    }
}

#[derive(Debug, Serialize)]
pub struct OutputAccumulator {
    #[serde(
        deserialize_with = "naive_date_from_str",
        serialize_with = "naive_date_to_str"
    )]
    pub date: NaiveDate,
    // When we output this as JSON, we want the output value to have two decimal places
    #[serde(serialize_with = "f64_to_str_two_decimals")]
    pub cash_raised: f64,
    pub total_number_of_shares: u64,
    pub ownership_list: Vec<OwnershipRecord>,
}

impl OutputAccumulator {
    pub fn new(date: NaiveDate) -> OutputAccumulator {
        OutputAccumulator {
            date,
            cash_raised: 0.0,
            total_number_of_shares: 0,
            ownership_list: Vec::new(),
        }
    }
}

impl OutputAccumulator {
    pub fn accumulate_ownership_transactions(
        &mut self,
        transaction_records: impl Iterator<Item = Record>,
    ) -> Result<(),CapTableError> {
        // We only want to process records that are less than or equal to our report date, we
        // ignore any others, so filter the incoming records based on our report date
        // We create a local variable for filter_date to avoid creating a Lambda, which causes
        // ownership issues with self in this case (Rust is very, very careful to always know 
        // who is carrying the ball, and one one thing can have the ball at one time, meaning 
        // we don't need a garbage collector).
        let filter_date = self.date;
        let records = transaction_records.filter(|r| r.investment_date <= filter_date);

        // Create a hashmap to accumulate incoming records
        let mut ownership_accumulator = HashMap::new();

        // For each record, call the lambda with the record value, called re here
        records.for_each(|re| {
            // Update our internal totals, with the incoming data
            self.cash_raised += re.cash_paid;
            self.total_number_of_shares += re.shares_purchased;

            // Create or Update ownership entry without having to worry about whether its in the hashmap or
            // not, this pattern (or_insert_with) handles when the value does not exist in the
            // hashmap already, and lets you supply a default value
            let record_entry = ownership_accumulator
                // We have to clone the string because the lifetime of the incoming record is less than this
                // structure, so its copy of the string will be deleted before this one should be,
                // hence we create a new one that will live long enough
                .entry(re.investor.clone())
                .or_insert_with(|| OwnershipRecord::new(re.investor.clone(), 0, 0.0));
            // Accumulate values into our new record where we are recording the investor
            record_entry.shares += re.shares_purchased;
            record_entry.cash_paid += re.cash_paid;
        });

        // Have to test for the total_number_of_shares value being zero because otherwise we will get a division 
        // by zero error when we try to fixup the ownership percentage, in the function call after
        // this
        if self.total_number_of_shares == 0 {
            return Err(CapTableError::TotalSharesIsZero);
        }

        // Correct the ownership percentage of all our records since they start out at zero, when
        // we create the record
        ownership_accumulator.values_mut().for_each(|r| r.fix_ownership_percentage(self.total_number_of_shares));

        // I hate having to a clone here, maybe there is a way to pull the value out instead to
        // avoid the memory turnover, I couldn't find a way to crack the hashmap and reuse the
        // values inside, in that case the hashmap shell would be automatically discarded
        self.ownership_list = ownership_accumulator.values().cloned().collect();

        // Return nothing, but signal that there was no error (this function doesn't have a value
        // return, just nothing or an error of some sort.
        Ok(())
    }
}
