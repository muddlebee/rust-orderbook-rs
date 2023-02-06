
extern crate orderbook;
extern crate rand;
use std::time::{Instant, SystemTime};
use orderbook::{Orderbook, OrderSide, orders};
use rand::Rng;


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BrokerAsset {
    USD,
    EUR,
    BTC,
    ETH,
}


fn parse_asset(asset: &str) -> Option<BrokerAsset> {
    match asset {
        "USD" => Some(BrokerAsset::USD),
        "EUR" => Some(BrokerAsset::EUR),
        "BTC" => Some(BrokerAsset::BTC),
        "ETH" => Some(BrokerAsset::ETH),
        _ => None,
    }
}


fn main() {
    let now = Instant::now();
    let mut orderbook = Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD);
    let order_asset = parse_asset("BTC").unwrap();
    let price_asset = parse_asset("USD").unwrap();

    // create order requests
    let mut order_list =
        vec![
            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Buy,
                0.98,
                5.0,
                SystemTime::now()
            ),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Sell,
                1.02,
                1.0,
                SystemTime::now()
            ),

            orders::amend_order_request(1, OrderSide::Buy, 0.99, 4.0, SystemTime::now()),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Buy,
                1.01,
                0.4,
                SystemTime::now()
            ),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Sell,
                1.03,
                0.5,
                SystemTime::now()
            ),

            orders::new_market_order_request(order_asset, price_asset, OrderSide::Buy, 1.0, SystemTime::now()),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Sell,
                1.05,
                0.5,
                SystemTime::now()
            ),

            orders::limit_order_cancel_request(4, OrderSide::Sell),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Buy,
                1.06,
                0.6,
                SystemTime::now()
            ),
        ];

    // for loop for random order requests
/*    for _ in 0..50 {
        let side = if rand::random::<bool>() { OrderSide::Buy } else { OrderSide::Sell };
        let order = orders::new_limit_order_request(
            order_asset,
            price_asset,
            side,
            rand::thread_rng().gen_range(1199..1200) as f64,
            rand::thread_rng().gen_range(1000..2000) as f64,
            SystemTime::now()
        );
        order_list.push(order);
    }*/

    // for loop for random order requests for amend order requests
/*    for idx in 0..10000 {
        let side = if rand::random::<bool>() { OrderSide::Buy } else { OrderSide::Sell };
        let order = orders::amend_order_request(
            rand::thread_rng().gen_range(1..10000) as u64,
            side,
            rand::thread_rng().gen_range(1000..1200) as f64,
            rand::thread_rng().gen_range(1000..2000) as f64,
            SystemTime::now()
        );
        order_list.push(order);
    }
*/

    // processing
    for order in order_list {
        println!("Order => {:?}", &order);
        let res = orderbook.process_order(order);
        println!("Processing => {:?}", res);
        if let Some((bid, ask)) = orderbook.current_spread() {
            println!("Spread => bid: {}, ask: {}\n", bid, ask);
        } else {
            println!("Spread => not available\n");
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:?} ", elapsed);
}
