use iota_client::options::FindTransactionsOptions;
use iota_conversion::trytes_converter;
use iota_lib_rs::prelude::*;

fn main() {
    let mut api = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let address =
        "ADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDR";

    let find_transactions_response = api
        .find_transactions(FindTransactionsOptions {
            addresses: vec![address.to_string()],
            ..FindTransactionsOptions::default()
        })
        .unwrap();

    if let Some(transactions) = find_transactions_response.hashes() {
        match api.get_trytes(&transactions) {
            Ok(trytes_array) => {
                let trytes = &trytes_array.trytes().as_ref().unwrap()[0];
                let message = trytes_converter::to_string(&trytes[..2186]);
                match message {
                    Ok(message) => println!("message: {}", message),
                    _ => println!("Couldn't convert trytes to string."),
                };
            }
            _ => println!("Couldn't get transaction data from hash."),
        };
    }
}
