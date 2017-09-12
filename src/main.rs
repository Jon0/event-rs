extern crate libc;

mod system;

use system::socket;

fn main() {
    let addr = socket::SockAddr4::create(1234);
    match socket::SockAcceptor::open(&addr) {
        Ok(acceptor) => loop {
            let mut socket = acceptor.accept();
            socket.listen(&mut handler);
            println!("socket connected");
        },
        Err(err) => println!("Error: {}", err),
    }
}
