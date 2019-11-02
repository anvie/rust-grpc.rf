extern crate grpcio;
extern crate protos;

use std::env;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use protos::$param.main_service_name_snake_case$::{Item, Order};
use protos::$param.main_service_name_snake_case$_grpc::$param.main_service_name_pascal_case$Client;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Expected exactly one argument, the port to connect to.")
    }
    let port = args[1]
        .parse::<u16>()
        .unwrap_or_else(|_| panic!("{} is not a valid port number", args[1]));

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());
    let client = $param.main_service_name_pascal_case$Client::new(ch);

    let mut order = Order::new();
    order.set_items(vec![Item::ITEM1, Item::ITEM2]);
    let result = client.request(&order).expect("RPC Failed!");
    println!("Order {:?} and got total ${:.2}", order, result.get_total());
}
