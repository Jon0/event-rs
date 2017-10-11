use libc::*;

#[repr(C)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

extern "C" {
    pub fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
    pub fn timerfd_settime(fd: c_int, flags: c_int, new_value: *const itimerspec, old_value: *mut itimerspec) -> c_int;
    pub fn timerfd_gettime(fd: c_int, curr_value: *mut itimerspec) -> c_int;
}
