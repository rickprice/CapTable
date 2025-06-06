use chrono::NaiveDate;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use std::collections::HashMap;

use crate::error::CapTableError;

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
    let s = date.format("%m/%d/%Y").to_string();
    serializer.serialize_str(&s)
}

fn f64_to_str_two_decimals<S>(number: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{:.2}", number);
    serializer.serialize_str(&s)
}

#[derive(Debug, Serialize)]
pub struct OwnershipRecord {
    pub investor: String,
    pub shares: u64,
    #[serde(serialize_with = "f64_to_str_two_decimals")]
    pub cash_paid: f64,
    #[serde(serialize_with = "f64_to_str_two_decimals")]
    pub ownership: f64,
}

impl OwnershipRecord {
    pub fn new(investor: String, shares: u64, cash_paid: f64) -> Self {
        Self {
            investor,
            shares,
            cash_paid,
            ownership: 0.0,
        }
    }

    pub fn fix_ownership_percentage(&mut self, total_shares: u64) {
        self.ownership = (self.shares as f64) / (total_shares as f64) * 100.0;
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
    pub fn new(date: NaiveDate) -> Self {
        Self {
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
    ) -> Result<(), CapTableError> {
        let filter_date = self.date;
        let records = transaction_records.filter(|r| r.investment_date <= filter_date);

        let mut ownership_accumulator = HashMap::new();

        for record in records {
            self.cash_raised += record.cash_paid;
            self.total_number_of_shares += record.shares_purchased;

            let entry = ownership_accumulator
                .entry(record.investor.clone())
                .or_insert_with(|| OwnershipRecord::new(record.investor.clone(), 0, 0.0));
            entry.shares += record.shares_purchased;
            entry.cash_paid += record.cash_paid;
        }

        if self.total_number_of_shares == 0 {
            return Err(CapTableError::TotalSharesIsZero);
        }

        self.ownership_list = ownership_accumulator
            .into_values()
            .map(|mut value| {
                value.fix_ownership_percentage(self.total_number_of_shares);
                value
            })
            .collect();

        Ok(())
    }
}
