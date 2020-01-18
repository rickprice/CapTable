extern crate cap_table_error;
extern crate chrono;
extern crate serde;

use chrono::NaiveDate;

use serde::{de, Deserialize, Deserializer};

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
