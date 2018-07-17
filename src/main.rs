extern crate tokio;
extern crate futures;

use tokio::prelude::*;
use futures::prelude::*;
use futures::sync::mpsc;
use futures::stream;

fn main() {
    let (tx, rx) = mpsc::channel(10);
    let s = stream::iter_ok(0..1000)
        .for_each(move |i| {
            println!("sending: {}", i);
            // The capacity is supposed to be 10, but each use of `clone`
            // adds a slot for the sending data inside the sender. This effectively
            // makes the channel unbounded.
            tx
                .clone()
                .send(i)
                .map(|_| println!("sent!"))
                .map_err(|e| println!("erred: {:?}", e))
        });
    tokio::run(s);

    let r = rx.for_each(move |i| {
        println!("recv: {}", i);
        Ok(())
    });
    tokio::run(r);
}
