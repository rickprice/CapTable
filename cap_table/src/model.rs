extern crate chrono;
extern crate serde;
extern crate serde_json;

use chrono::NaiveDate;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
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

fn naive_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}

fn naive_date_to_str<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // I don't like serializing with a different format than we deserialize with, maybe there is a
    // way to pass in a custom format parameter...
    let s = format!("{}", date.format("%m/%d/%Y"));
    serializer.serialize_str(&s)
}

#[derive(Debug, Clone, Serialize)]
pub struct OwnershipRecord {
    pub investor: String,
    pub shares: u64,
    pub cash_paid: f64,
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
    ) {
        // We only want to process records that are less than or equal to our report date, we
        // ignore any others
        let filter_date = self.date;
        let records = transaction_records.filter(|r| r.investment_date <= filter_date);

        let mut ownership_accumulator = HashMap::new();

        records.for_each(|re| {
            // Update totals
            self.cash_raised += re.cash_paid;
            self.total_number_of_shares += re.shares_purchased;

            // Create or Update ownership entry without having to worry about whether its in the hashmap or
            // not
            let record_entry = ownership_accumulator
                .entry(re.investor.clone())
                .or_insert_with(|| OwnershipRecord::new(re.investor.clone(), 0, 0.0));
            record_entry.shares += re.shares_purchased;
            record_entry.cash_paid += re.cash_paid;
        });

        ownership_accumulator.values_mut().for_each(|r| r.fix_ownership_percentage(self.total_number_of_shares));

        // I hate having to a clone here, maybe there is a way to pull the value out instead to
        // avoid the memory turnover
        self.ownership_list = ownership_accumulator.values().cloned().collect();
    }
}
