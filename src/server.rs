use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
   pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        
        let listener = TcpListener::bind(&self.addr).unwrap();

        // infinite loop
        loop {

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("{:?}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }
                        },
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("Error: {}", e)
                }
            }

        }
    }
}
