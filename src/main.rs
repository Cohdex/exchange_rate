mod exchange_rate_api;
mod ip_api;

use text_io::read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_ip = ip_api::get_my_ip()?;
    println!(
        "Client with IP={} is requesting currency exchange rates",
        my_ip
    );

    let exchange_rate_client = exchange_rate_api::ApiClient::new();

    let currencies = exchange_rate_client.get_currencies()?;
    let highest_value_currency = exchange_rate_client.get_highest_value_currency()?;
    let lowest_value_currency = exchange_rate_client.get_lowest_value_currency()?;
    println!("Currencies: {:#?}", currencies);
    println!(
        "Highest value: {}, lowest value: {}",
        highest_value_currency, lowest_value_currency
    );

    let currency_to_get = loop {
        println!("Enter currency:");
        let currency_to_get: String = read!();
        let currency_to_get = currency_to_get.to_uppercase();
        if currencies.get(&currency_to_get).is_some() {
            break currency_to_get;
        } else {
            println!("Unknown currency, try again!");
        }
    };

    match exchange_rate_client.get_currency_exchange_rates(&currency_to_get) {
        Ok(exchange_rates) => println!(
            "The current exchange rates for {} as of {}: {:#?}",
            exchange_rates.base, exchange_rates.date, exchange_rates.rates
        ),
        Err(error) => eprintln!("{}", error),
    }
    Ok(())
}
