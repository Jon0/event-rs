
use std::thread;
use std::sync::Mutex;
use std::os::unix::io::RawFd;
use nix::sys::epoll::*;



pub struct EventLoop {
	epoll: RawFd
}


impl EventLoop {
	fn new() {
		let fd = epoll_create().unwrap();

		let handle = thread::spawn(move || {
			let mut event = [EpollEvent::empty()];

			let count = epoll_wait(fd, &mut event, 0).unwrap();

			println!("{}", count);
		});


	}
}
