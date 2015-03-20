use std::old_io::TcpStream;
use std::str;

fn client() {
    let mut socket = TcpStream::connect("127.0.0.1:10240").unwrap();
    let response = socket.read_to_end();
    // do something with response
    match response {
	    Err(e) => { println!("error: {}", e) },
	    Ok(buf) => {
			match str::from_utf8(buf.as_slice()) {
			    None => { println!("response error!") },
			    Some(s) => { println!("response get! {}", s) }
			}
		}
	}
}

fn main() {
    // send some request
    client();
}
