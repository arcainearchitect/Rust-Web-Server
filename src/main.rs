use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs;
use std::collections::HashMap;

mod server;
mod router;
mod handler;

use server::Server;

fn main() {
    println!("Starting server on 127.0.0.1:7878...");
    let server= Server::new("127.0.0.1:7878");
    server.run();
}