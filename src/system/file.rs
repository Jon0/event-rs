use std::cmp::*;
use std::ffi::CString;
use std::io::*;
use std::mem;
use num::Zero;

use libc::*;


pub fn cvt<T: PartialOrd + Zero>(t: T) -> Result<T> {
    if t < T::zero() {
        Err(Error::last_os_error())
    } else {
        Ok(t)
    }
}


pub fn cvtf<T: PartialOrd + Zero, R>(t: T, c: fn (T) -> R) -> Result<R> {
    if t < T::zero() {
        Err(Error::last_os_error())
    } else {
        Ok(c(t))
    }
}


#[derive(Debug)]
pub struct FileDesc {
    fd: c_int,
}


impl FileDesc {
    pub fn create(fd: c_int) -> FileDesc {
        FileDesc {fd: fd}
    }

    pub fn raw_fd(&self) -> c_int {
        self.fd
    }

    pub fn open(filepath: &str) -> Result<FileDesc> {
        let path = CString::new(filepath).unwrap();
        return cvtf(unsafe {open(path.as_ptr(), O_RDONLY | O_NONBLOCK)}, FileDesc::create);
    }

    pub fn seek(&self, offset: usize) {
        unsafe {lseek (self.fd, offset as i64, SEEK_SET)};
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let ret = cvt(unsafe {read(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len())}).unwrap();
        Ok(ret as usize)
    }

    pub fn read_at(&self, buf: &mut [u8], offset: usize) -> Result<usize> {
        let ret = cvt(unsafe {pread(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len(), offset as i64)}).unwrap();
        Ok(ret as usize)
    }
}
