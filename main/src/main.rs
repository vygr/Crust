extern crate memmap2;

use std::env;
use std::fs::File;

use memmap2::Mmap;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pii_stat_info {
    pub mtime: libc::c_int,
    pub fsize: libc::c_int,
    pub mode: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lk_msg {
    pub m_status: libc::c_int,
    pub m_task_count: libc::c_int,
    pub m_peer_node_id: libc::c_int,
    pub m_stamp: libc::c_int,
    pub m_data: [libc::c_char; 984],
}

/// # Safety
/// Chyrsalisp exit... this will stop the boat
pub unsafe extern "C" fn exit(status: i32) {
    ::std::process::exit(status);
}


/// # Safety
/// Chyrsalisp pii_stat
pub unsafe extern "C" fn pii_stat(si: &pii_stat_info) -> libc::c_int {
    println!("pii_start");

    1
}

/// # Safety
/// Chyrsalisp pii_open
pub unsafe extern "C" fn pii_open(mut path: *const libc::c_char,
                                  mut mode: libc::c_int) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    -(1 as libc::c_int)
}

/// # Safety
/// Chyrsalisp pii_close
pub unsafe extern "C" fn pii_close(handle: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);
}

/// # Safety
/// Chyrsalisp pii_unlink (delete)
pub unsafe extern "C" fn pii_unlink(mut path: *const libc::c_char)
 -> libc::c_int {
    return -(1 as libc::c_int);
}

/// # Safety
/// Chyrsalisp pii_read
pub unsafe extern "C" fn pii_read(mut fd: libc::c_int,
                                  mut addr: *mut libc::c_void,
                                  mut len: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);
}

/// # Safety
/// Chyrsalisp pii_write
pub unsafe extern "C" fn pii_write(mut fd: libc::c_int,
                                  mut addr: *mut libc::c_void,
                                  mut len: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);
}

/// # Safety
/// Chyrsalisp pii_mmap for memory map allication
#[no_mangle]
pub unsafe extern "C" fn pii_mmap(mut len: libc::c_int, mut fd: libc::c_int,
                                  mut mode: libc::c_int)
 -> *mut libc::c_void {
    return -(1 as libc::c_int) as *mut libc::c_void;
}

/// # Safety
/// Chyrsalisp pii_munmap for memory map deallocation
#[no_mangle]
pub unsafe extern "C" fn pii_munmap(mut addr: *mut libc::c_void,
                                    mut len: libc::c_int,
                                    mut mode: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);
}

/// # Safety
/// Chyrsalisp pii_mprotect for memory map
#[no_mangle]
pub unsafe extern "C" fn pii_mprotect(mut addr: *mut libc::c_void,
                                      mut len: libc::c_int,
                                      mut mode: libc::c_int) -> libc::c_int {
     1
}

/// # Safety
/// Chyrsalisp pii_gettime
pub unsafe extern "C" fn pii_gettime() -> libc::c_int{
    0
}

/// # Safety
/// Chyrsalisp pii_open_shared
#[no_mangle]
pub unsafe extern "C" fn pii_open_shared(mut path: *const libc::c_char,
                                         mut len: libc::c_int)
 -> libc::c_int {
    1
}

/// # Safety
/// Chyrsalisp pii_close shared
#[no_mangle]
pub unsafe extern "C" fn pii_close_shared(mut path: *const libc::c_char,
                                          mut hndl: libc::c_int)
 -> libc::c_int {
    1
}

/// # Safety
/// Chyrsalisp pii_flush_icache
#[no_mangle]
pub unsafe extern "C" fn pii_flush_icache(mut addr: *mut libc::c_void,
                                          mut len: libc::c_int)
 -> *mut libc::c_void {
    return addr;
}

/// # Safety
/// Chyrsalisp pii_dirlist
#[no_mangle]
pub unsafe extern "C" fn pii_dirlist(mut path: *const libc::c_char,
                                     mut buf: *mut libc::c_char,
                                     mut buf_len: libc::c_int)
 -> libc::c_int {
    return 2 as libc::c_int;
}

/// # Safety
/// Chyrsalisp pii_remove
#[no_mangle]
pub unsafe extern "C" fn pii_remove(mut fqname: *const libc::c_char)
 -> libc::c_int {
    let mut res: libc::c_int = -(1 as libc::c_int);
    return res;
}

/// # Safety
/// Chyrsalisp pii_seek
#[no_mangle]
pub unsafe extern "C" fn pii_seek(mut fd: libc::c_int, mut pos: libc::c_int,
                                  mut offset: libc::c_uchar) -> libc::c_int {
    1
}

/// # Safety
/// Chyrsalisp pii_random
pub unsafe extern "C" fn pii_random() -> libc::c_int{
    println!("pii_random");
    1
}

/// # Safety
/// Chyrsalisp pii_sleep
pub unsafe extern "C" fn pii_sleep(delay: u64) {
    println!("pii_sleep");
}

#[no_mangle]
pub static mut lk_data_size: libc::c_int = 984 as libc::c_int;
#[no_mangle]
pub static mut lk_page_size: libc::c_int = 4096 as libc::c_int;
#[no_mangle]
pub static mut usb_monitor: libc::c_int = 0;
/// # Safety
/// Chyrsalisp pii_usb_start
#[no_mangle]
pub unsafe extern "C" fn pii_usb_start(mut buffer: *mut lk_msg)
 -> libc::c_int {
    return 0 as libc::c_int;
}

/// # Safety
/// Chyrsalisp pii_usb_stop
#[no_mangle]
pub unsafe extern "C" fn pii_usb_stop(mut buffer: *mut lk_msg)
 -> libc::c_int {
    return 0 as libc::c_int;
}

/// # Safety
/// Chyrsalisp pii_usb_running
#[no_mangle]
pub unsafe extern "C" fn pii_usb_running(mut buffer: *mut lk_msg)
 -> libc::c_int {
    return 0 as libc::c_int;
}


fn main() {
    let path = env::args()
        .nth(1)
        .expect("supply a single path as the program argument");

    let file = File::open(path).expect("failed to open the file");

    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };


    // image file is now mapped
    // unsafe {
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gettime() {
        unsafe {
            println!("{}", pii_gettime());
        }
    }
}