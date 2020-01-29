extern crate clap;
use clap::{Arg, App};
use rust_decimal::{Decimal, RoundingStrategy};
use std::str::FromStr;

fn perform_conversion(amt: Decimal, rate: Decimal) -> Decimal {
    amt * rate
}

fn main() {
    let conversion_rate = Decimal::new(907820, 6);
    let matches = App::new("rusty_currency")
        .version("0.1")
        .about("Convert USD to EUR")
        .arg(
            Arg::with_name("amount")
                .short("a")
                .long("amount")
                .required(true)
                .takes_value(true)
        )
        .get_matches();
    
    let amount_to_convert = matches.value_of("amount").unwrap();
    let amount_to_convert = Decimal::from_str(amount_to_convert).unwrap();
    
    println!(
        "{} USD equals {}", 
        amount_to_convert, 
        perform_conversion(amount_to_convert, conversion_rate).round_dp_with_strategy(2, RoundingStrategy::RoundHalfDown)
    );
}
