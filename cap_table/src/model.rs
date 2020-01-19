extern crate cap_table_error;
extern crate chrono;
extern crate serde;

use chrono::NaiveDate;

use serde::{de, Deserialize, Deserializer};

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(
        rename(deserialize = "#INVESTMENT DATE"),
        deserialize_with = "naive_date_from_str"
    )]
    pub investment_date: NaiveDate,
    #[serde(rename(deserialize = " SHARES PURCHASED"))]
    pub shares_purchased: u64,
    #[serde(rename(deserialize = " CASH PAID"))]
    pub cash_paid: f64,
    #[serde(rename(deserialize = " INVESTOR"))]
    pub investor: String,
}

fn naive_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}


#[derive(Debug)]
pub struct OwnershipRecord<'a> {
    pub investor: &'a str,
    pub shares: u64,
    pub cash_paid: f64,
}

impl<'a> OwnershipRecord<'a> {
    pub fn new(investor: &'a str,shares: u64,cash_paid: f64) -> OwnershipRecord<'a> {
        OwnershipRecord {
            investor,
            shares,
            cash_paid,
        }
    }
}


#[derive(Debug)]
pub struct OutputAccumulator<'a> {
    pub date: NaiveDate,
    pub cash_raised: f64,
    pub total_number_of_shares: u64,
    pub ownership_accumulator: HashMap<&'a str,OwnershipRecord<'a>>,
}

impl<'a> OutputAccumulator<'a> {
    pub fn new(date: NaiveDate) -> OutputAccumulator<'a> {
        OutputAccumulator {
            date,
            cash_raised:0.0,
            total_number_of_shares:0,
       ownership_accumulator:HashMap::new(),     
        }
    }
}


