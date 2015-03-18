use std::old_io::TcpStream;
use std::str;

fn client() {
    let mut socket = TcpStream::connect("127.0.0.1:10240").unwrap();
    let response = socket.read_to_end();
    // do something with response
}

fn main() {
    //send some request
}
