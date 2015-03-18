use std::old_io::{TcpListener, TcpStream};
use std::old_io::{Acceptor, Listener};

fn server() {

    let listener = TcpListener::bind("127.0.0.1:10240");

    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => {}
            Ok(stream) => {
                thread::spawn(move|| {
                    handler_client(stream)
                })
            }
        }    
    }
}

fn handler_client(mut stream: TcpStream) {
    //
}
