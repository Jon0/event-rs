
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use std::sync::Mutex;
use std::os::unix::io::RawFd;

use nix::libc::off_t;
use nix::fcntl::{open, OFlag};
use nix::sys::epoll::*;
use nix::sys::stat::Mode;
use nix::sys::socket::{accept, bind, listen, recv, send, socket, shutdown};
use nix::sys::socket::SockFlag;
use nix::sys::socket::SockProtocol;
use nix::sys::socket::{AddressFamily, MsgFlags, SockAddr, SockType, Shutdown, InetAddr, IpAddr};
use nix::unistd::{lseek, read, Whence};


pub struct EventHandler {

}


impl EventHandler {
	pub fn new() -> EventHandler {
		return EventHandler {};
	}

	pub fn exec(&self) {
		println!("EventHandler");
	}
}



pub struct EventLoop {
	epoll: RawFd,
	next_id: u64,
	event_map: HashMap<u64, EventHandler>,
	thread_pool: Vec<JoinHandle<()>>
}


impl EventLoop {
	pub fn new() -> EventLoop {
		let epoll_fd = epoll_create().unwrap();

		let event = EpollEvent::new(EpollFlags::EPOLLIN, 0);


		let handle = thread::spawn(move || {
			let mut event = [EpollEvent::empty()];

			let count = epoll_wait(epoll_fd, &mut event, -1).unwrap();

			println!("Recv: {}", count);
			for c in 0..count {
				let id = event[c].data();

				println!("Data: {}", id);

				// match self.event_map.get(id) {
				// 	Some(handler) => handler.exec(),
				// 	None => println!("No handler");
				// }
			}
		});


		return EventLoop {
			epoll: epoll_fd,
			next_id: 1,
			event_map: HashMap::new(),
			thread_pool: Vec::new()
		}

	}


	pub fn create_threads(this: Arc<Mutex<Self>>) {

		let a = this.clone();
		let mut b = a.lock().unwrap();

		let handle = thread::spawn(move || {
			
			let mut event = [EpollEvent::empty()];

			let a = this.clone();
			let emap = a.lock().unwrap();
			let epoll_fd = emap.epoll;

			let count = epoll_wait(epoll_fd, &mut event, -1).unwrap();

			println!("Recv: {}", count);
			for c in 0..count {
				let id = event[c].data();

				println!("Data: {}", id);

				match emap.event_map.get(&id) {
					Some(handler) => handler.exec(),
					None => println!("No handler")
				}
			}
		});

		b.thread_pool.push(handle);
	}


	pub fn register(&mut self, fd: RawFd, handler: EventHandler) {
		let id = self.next_id;
		self.next_id += 1;
		self.event_map.insert(id, handler);

		// add to the epoll object
		let mut event = EpollEvent::new(EpollFlags::EPOLLIN, id);
		let c = epoll_ctl(self.epoll, EpollOp::EpollCtlAdd, fd, Some(&mut event));
	}


	pub fn listen(&mut self) {
		let listen_fd = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), SockProtocol::Tcp).unwrap();
		let b = bind(listen_fd, &SockAddr::new_inet(InetAddr::new(IpAddr::new_v4(0, 0, 0, 0), 8080))).unwrap();
		let l = listen(listen_fd, 5);

		self.register(listen_fd, EventHandler::new());
	}


	pub fn join(self) {
		for t in self.thread_pool {
			t.join();
		}
	}

}
