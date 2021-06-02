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

pub unsafe extern "C" fn exit(status: i32) {
    ::std::process::exit(status);
}


pub unsafe extern "C" fn pii_stat(si: &pii_stat_info) -> libc::c_int {
    println!("pii_start");

    1
}

pub unsafe extern "C" fn pii_open(mut path: *const libc::c_char,
                                  mut mode: libc::c_int) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    return -(1 as libc::c_int);
}

pub unsafe extern "C" fn pii_close(handle: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);   
}

pub unsafe extern "C" fn pii_unlink(mut path: *const libc::c_char)
 -> libc::c_int {
    return -(1 as libc::c_int);   
}

pub unsafe extern "C" fn pii_read(mut fd: libc::c_int,
                                  mut addr: *mut libc::c_void,
                                  mut len: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);   
}

pub unsafe extern "C" fn pii_write(mut fd: libc::c_int,
                                  mut addr: *mut libc::c_void,
                                  mut len: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);   
}

#[no_mangle]
pub unsafe extern "C" fn pii_mmap(mut len: libc::c_int, mut fd: libc::c_int,
                                  mut mode: libc::c_int)
 -> *mut libc::c_void {    
    return -(1 as libc::c_int) as *mut libc::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn pii_munmap(mut addr: *mut libc::c_void,
                                    mut len: libc::c_int,
                                    mut mode: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn pii_mprotect(mut addr: *mut libc::c_void,
                                      mut len: libc::c_int,
                                      mut mode: libc::c_int) -> libc::c_int {
     1
}

pub unsafe extern "C" fn pii_gettime() -> libc::c_int{
    0
}

#[no_mangle]
pub unsafe extern "C" fn pii_open_shared(mut path: *const libc::c_char,
                                         mut len: libc::c_int)
 -> libc::c_int {
    1
}

#[no_mangle]
pub unsafe extern "C" fn pii_close_shared(mut path: *const libc::c_char,
                                          mut hndl: libc::c_int)
 -> libc::c_int {
    1
}

#[no_mangle]
pub unsafe extern "C" fn pii_flush_icache(mut addr: *mut libc::c_void,
                                          mut len: libc::c_int)
 -> *mut libc::c_void {
    return addr;
}

#[no_mangle]
pub unsafe extern "C" fn pii_dirlist(mut path: *const libc::c_char,
                                     mut buf: *mut libc::c_char,
                                     mut buf_len: libc::c_int)
 -> libc::c_int {
    return 2 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn pii_remove(mut fqname: *const libc::c_char)
 -> libc::c_int {
    let mut res: libc::c_int = -(1 as libc::c_int);
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn pii_seek(mut fd: libc::c_int, mut pos: libc::c_int,
                                  mut offset: libc::c_uchar) -> libc::c_int {
    1
}

pub unsafe extern "C" fn pii_random() -> libc::c_int{
    println!("pii_random");
    1
}

pub unsafe extern "C" fn pii_sleep(delay: u64) {
    println!("pii_sleep");
}

#[no_mangle]
pub static mut lk_data_size: libc::c_int = 984 as libc::c_int;
#[no_mangle]
pub static mut lk_page_size: libc::c_int = 4096 as libc::c_int;
#[no_mangle]
pub static mut usb_monitor: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn pii_usb_start(mut buffer: *mut lk_msg)
 -> libc::c_int {
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn pii_usb_stop(mut buffer: *mut lk_msg)
 -> libc::c_int {
    return 0 as libc::c_int;
}

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
    unsafe {



    }


}