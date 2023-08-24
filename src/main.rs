mod stream;

// use std::net::{TcpListener, TcpStream};
// use std::io::{Read,Write};
use stream::Stream;





fn main() {
    let server = Stream::new(9090).unwrap();    
    server.listen();

}
