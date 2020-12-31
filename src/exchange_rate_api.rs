use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ExchangeRate {
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Clone)]
pub struct ExchangeRequestError {
    status_code: reqwest::StatusCode,
    error_message: String,
}

impl error::Error for ExchangeRequestError {}

impl fmt::Display for ExchangeRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Received an error in exchange rate request, got status {}: {}",
            self.status_code, self.error_message
        )
    }
}

pub struct ApiClient {
    reqwest_client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_currencies(&self) -> Result<HashSet<String>, Box<dyn error::Error>> {
        let response: ExchangeRate = self
            .reqwest_client
            .get("https://api.exchangeratesapi.io/latest")
            .send()?
            .json()?;
        let mut currencies: HashSet<String> =
            response.rates.into_iter().map(|entry| entry.0).collect();
        currencies.insert(response.base);
        Ok(currencies)
    }

    pub fn get_currency_exchange_rates(
        &self,
        currency: &str,
    ) -> Result<ExchangeRate, Box<dyn error::Error>> {
        let response = self
            .reqwest_client
            .get("https://api.exchangeratesapi.io/latest")
            .query(&[("base", currency)])
            .send()?;
        let status_code = response.status();
        if status_code.is_success() {
            let mut exchange_rate: ExchangeRate = response.json()?;
            exchange_rate.rates.remove(currency);
            Ok(exchange_rate)
        } else {
            let error_response: ErrorResponse = response.json()?;
            Err(ExchangeRequestError {
                status_code,
                error_message: error_response.error,
            }
            .into())
        }
    }

    pub fn get_highest_value_currency(&self) -> Result<String, Box<dyn error::Error>> {
        let response: ExchangeRate = self
            .reqwest_client
            .get("https://api.exchangeratesapi.io/latest")
            .send()?
            .json()?;
        let highest_rate = response
            .rates
            .into_iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        if highest_rate.1 < 1.0 {
            Ok(highest_rate.0)
        } else {
            Ok(response.base)
        }
    }

    pub fn get_lowest_value_currency(&self) -> Result<String, Box<dyn error::Error>> {
        let response: ExchangeRate = self
            .reqwest_client
            .get("https://api.exchangeratesapi.io/latest")
            .send()?
            .json()?;
        let lowest_rate = response
            .rates
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        if lowest_rate.1 > 1.0 {
            Ok(lowest_rate.0)
        } else {
            Ok(response.base)
        }
    }
}
