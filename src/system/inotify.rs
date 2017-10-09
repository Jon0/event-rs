use std::io::Result;
use libc::*;
use inotify_sys::*;

use system::file::{cvtf, FileDesc};

pub struct INotify {
    fd: FileDesc,
}


impl INotify {
    pub fn create(fd: c_int) -> INotify {
        INotify {fd: FileDesc::create(fd)}
    }


    pub fn new() -> Result<INotify> {
        let r: c_int = unsafe {inotify_init1(IN_CLOEXEC | IN_NONBLOCK)} as c_int;
        return cvtf(r, |r| INotify { fd: FileDesc::create(r) });
    }
}
