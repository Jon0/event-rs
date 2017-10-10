use std::ffi::CString;
use std::io::Result;
use std::path::Path;
use std::slice::from_raw_parts_mut;
use std::mem::size_of;
use libc::*;
use inotify_sys::*;

use system::file::{cvtf, FileDesc};


fn serial_read<T: Sized>(buf: &mut T) -> &mut [u8] {
    unsafe {
        from_raw_parts_mut((buf as *mut T) as *mut u8, size_of::<T>())
    }
}


type WatchMask = u32;


#[derive(Debug)]
pub struct WatchDescriptor {
    wd: c_int,
}


#[derive(Debug)]
pub struct INotify {
    fd: FileDesc,
}


impl INotify {
    pub fn create(fd: c_int) -> INotify {
        INotify {fd: FileDesc::create(fd)}
    }


    pub fn new() -> Result<INotify> {
        let flags = IN_CLOEXEC; // | IN_NONBLOCK;
        let r: c_int = unsafe {inotify_init1(flags)} as c_int;
        return cvtf(r, |r| INotify {fd: FileDesc::create(r)});
    }


    pub fn add_watch(&mut self, filepath: &str, mask: WatchMask) -> Result<WatchDescriptor> {
        let path = CString::new(filepath).unwrap();
        let wd = unsafe {inotify_add_watch(self.fd.raw_fd(), path.as_ptr(), mask)};
        return cvtf(wd, |wd| WatchDescriptor {wd: wd});
    }


    pub fn read(&mut self) -> inotify_event {
        let mut event = inotify_event {wd: 0, mask: 0, cookie: 0, len: 0};
        let result = self.fd.read(serial_read(&mut event));
        return event;
    }
}
