extern crate num;
extern crate libc;
extern crate inotify_sys;

mod system;

use std::env;
use std::str;
use inotify_sys::*;

use system::file::*;
use system::inotify::*;
use system::socket;


fn connect() {
    let addr = socket::SockAddr4::create(1234);
    match socket::SockAcceptor::open(&addr) {
        Ok(acceptor) => loop {
            let mut socket = acceptor.accept();

        },
        Err(err) => println!("Error: {}", err),
    }
}


fn main() {
    let mut buf: [u8; 64] = [0; 64];
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        let mut inotify = INotify::new().unwrap();
        let wd = inotify.add_watch(arg, IN_MODIFY).unwrap();
        let event = inotify.read();
        println!("Event: {:?}", wd);
        let file = FileDesc::open(arg).unwrap();
        let result = file.read(&mut buf).unwrap();
        let temp_str = str::from_utf8(&buf[0..result - 1]).unwrap();
        println!("Read: {}", temp_str);
    }


}
