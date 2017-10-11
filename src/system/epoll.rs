use std::io::Result;
use libc::*;

use system::file::{cvtf, FileDesc};


pub struct EPoll {
    fd: FileDesc,
}


impl EPoll {
    pub fn new() -> Result<EPoll> {
        return cvtf(unsafe {epoll_create1(EPOLL_CLOEXEC)}, |r| EPoll {fd: FileDesc::create(r)});
    }

    pub fn add_source(&self, fd: &FileDesc) {
        let events = (EPOLLIN | EPOLLET) as u32;
        let mut ev = epoll_event { events: events, u64: 0 };
        unsafe {
            let result = epoll_ctl(self.fd.raw_fd(), EPOLL_CTL_ADD, fd.raw_fd(), &mut ev);
        }
    }

    pub fn wait(&self) {
        let mut event = [epoll_event { events: 0, u64: 0 }; 32];
        unsafe {
            let result = epoll_wait(self.fd.raw_fd(), event.as_mut_ptr(), event.len() as i32, -1);
        }

    }
}


pub trait EventSource {
    fn listen(&self, handler: &mut EPoll);
}
