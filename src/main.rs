use std::os;
use client::client;
use server::server;

fn main() {
    let args = os::args();
    if args == "server" {
        server();
    }
    else if args == "client" {
        client();
    }
}
