use std::io::Result;
use std::ptr;
use libc::*;
use system::file::{cvtf, FileDesc};

pub struct Timer {
    fd: FileDesc,
}


impl Timer {
    pub fn new() -> Result<Timer> {
        return cvtf(unsafe {timerfd_create(CLOCK_MONOTONIC, 0)}, |r| Timer {fd: FileDesc::create(r)});
    }

    pub fn as_fd(&self) -> &FileDesc {
        return &self.fd;
    }

    pub fn set(&self) {
        let time = itimerspec {it_interval: timespec {tv_sec: 1, tv_nsec: 0}, it_value: timespec {tv_sec: 1, tv_nsec: 0}};
        unsafe {
            let result = timerfd_settime(self.fd.raw_fd(), 0, &time, ptr::null_mut());
        }
    }

    pub fn read(&mut self) {
        let mut buf: [u8; 64] = [0; 64];
        let result = self.fd.read(&mut buf);
    }
}
