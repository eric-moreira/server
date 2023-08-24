use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


pub struct Stream {
    pub listener: TcpListener,
}

impl Stream {
    pub fn new(port: u16) -> Result<Self, std::io::Error> {
        let address = format!("127.0.0.1:{port}");
        let listener = TcpListener::bind(address)?;
        Ok(Stream { listener })
    }

    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut _stream) => {
                    println!("Connection Accepted");
                    let _ = self.handle_connection(&mut _stream);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }

    pub fn handle_connection(&self, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = vec![0;1024];
        let bytes_read = stream.read(&mut buf).unwrap();
        let command = String::from_utf8(buf[..bytes_read].to_vec())?;
        println!("{:#?}", stream);
        self.execute(stream, command);
        Ok(())
    
    }

    fn execute(&self, stream: &mut TcpStream, command: String) {
        match command.trim() {
            "PING" => {
                //println!("+PONG\r\n");
                
                let _ = stream.write_all(b"+PONG\r\n");
            },
            &_ => {
                println!("No command");
            }
        }
    }
}