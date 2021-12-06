use reqwest;
use std::collections::HashMap;

fn main() {
  match get_last_price() {
    Ok(lastprice) => println!("Last price of Binance is  {} .", lastprice),
    Err(e) => eprintln!("Last price not found ( \n  {}", e),
  }
}

fn get_last_price() -> Result<String, reqwest::Error> {
  let url = "https://api2.binance.com/api/v3/ticker/24hr";
  let result = reqwest::blocking::get(url);

  let response = match result {
    Ok(res) => res,
    Err(err) => return Err(err),
  };

  let body = response.json::<HashMap<String, i32>>();

  let json = match body {
    Ok(json) => json,
    Err(err) => return Err(err),
  };

  let lastprice = json["lastPrice"].to_string();

  Ok(lastprice)
}