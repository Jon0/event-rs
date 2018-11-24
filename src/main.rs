extern crate num;
extern crate libc;
extern crate nix;
extern crate inotify_sys;

mod event;
mod system;

use std::env;
use std::str;
use inotify_sys::*;

use event::*;
use system::epoll::*;
use system::file::*;
use system::inotify::*;
use system::socket;
use system::timer::*;

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

    let mut l = EventLoop::new();
    l.listen();
    l.join();


    // let mut buf: [u8; 64] = [0; 64];
    // let args: Vec<String> = env::args().collect();
    // let mut epoll = EPoll::new().unwrap();
    // let mut timerfd = Timer::new().unwrap();
    // timerfd.set();
    // epoll.add_source(timerfd.as_fd());

    // for arg in &args[1..] {
    //     println!("Listen to: {}", arg);

    //     let file = FileDesc::open(arg).unwrap();
    //     epoll.add_source(&file);

    //     loop {
    //         epoll.wait();
    //         timerfd.read();
    //         let result = file.read_at(&mut buf, 0).unwrap();
    //         let content = str::from_utf8(&buf[0..result - 1]).unwrap();
    //         println!("Read: {}", content);
    //     }
    // }


    // for arg in &args[1..] {
    //     let mut inotify = INotify::new().unwrap();
    //     let wd = inotify.add_watch(arg, IN_MODIFY).unwrap();
    //     let event = inotify.read();
    //     println!("Event: {:?}", wd);
    // }

}
