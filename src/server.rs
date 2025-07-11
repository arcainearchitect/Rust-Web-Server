use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use crate::router::Router;

pub struct Server{
    address:String,
    router: Router,
}

impl Server{
    pub fn new(address:&str)->Self{
        Server{
            address: address.to_string(),
            router: Router::new(),
        }
    }

    pub fn run(&self){
        let listener=TcpListener::bind(&self.address)
            .expect("Failed to bind to address.");

        println!("Server running on {}", self.address);
        
        for stream in listener.incoming(){
            match stream{
                Ok(stream)=>{
                    let router=self.router.clone();
                    thread::spawn(move || {
                        handle_connection(stream, router);
                    });
                }
                Err(error)=>{
                    eprintln!("Connection failed: {}", error);
                }
            }
        }
    }
}

fn handle_connection(mut stream:TcpStream, router:Router){
    let mut buffer=[0; 1024];
    
    match stream.read(&mut buffer){
        Ok(_)=>{
            let request=String::from_utf8_lossy(&buffer[..]);
            let response=router.handle_request(&request);
            
            if let Err(error)=stream.write(response.as_bytes()){
                eprintln!("Failed to write response: {}", error);
            }
            
            if let Err(error)=stream.flush(){
                eprintln!("Failed to flush stream: {}", error);
            }
        }
        Err(error) => {
            eprintln!("Failed to read from stream: {}", error);
        }
    }
}

