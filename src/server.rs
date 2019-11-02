extern crate futures;
extern crate grpcio;
extern crate protos;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use protos::$param.main_service_name_snake_case$::{Result, Item, Order};
use protos::$param.main_service_name_snake_case$_grpc::{self, $param.main_service_name_pascal_case$};

#[derive(Clone)]
struct $param.main_service_name_pascal_case$Service;

impl $param.main_service_name_pascal_case$ for $param.main_service_name_pascal_case$Service {
    fn request(&mut self, ctx: RpcContext, order: Order, sink: UnarySink<Result>) {
        println!("Received Order {{ {:?} }}", order);
        let mut result = Result::new();
        result.set_total(order.get_items().iter().fold(0.0, |total, &item| {
            total + match item {
                Item::ITEM1 => 0.05,
                Item::ITEM2 => 0.25,
                Item::ITEM3 => 1.0,
            }
        }));
        let f = sink
            .success(result.clone())
            .map(move |_| println!("Responded with Result {{ {:?} }}", result))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = $param.main_service_name_snake_case$_grpc::create_$param.main_service_name_snake_case$($param.main_service_name_pascal_case$Service);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
