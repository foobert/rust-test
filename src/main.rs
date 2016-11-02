extern crate libc;
use libc::size_t;
use std::ffi::CString;

pub enum Mosquitto {}

#[link(name = "mosquitto")]
extern {
    fn mosquitto_lib_version(major: *mut u32, minor: *mut u32, revision: *mut u32) -> u32;
    fn mosquitto_lib_init() -> u32;
    fn mosquitto_lib_cleanup() -> u32;
    fn mosquitto_new(id: *const libc::c_char, clean_session: bool, arg: *mut libc::c_void) -> *const Mosquitto;
    fn mosquitto_destroy(mosq: *const Mosquitto);
    fn mosquitto_connect(mosq: *const Mosquitto, host: *const libc::c_char, port: u32, keepalive: u32) -> u32;
}

fn main() {
    println!("Hello, world!");
    let mut major = 0;
    let mut minor = 0;
    let mut revision = 0;
    let id = "foo";
    let host = "test.mosquitto.org";
    let port = 1883;

    unsafe {
        mosquitto_lib_init();
        mosquitto_lib_version(&mut major, &mut minor, &mut revision);

        let id: *const libc::c_char = CString::new(id).unwrap().as_ptr();
        let host: *const libc::c_char = CString::new(host).unwrap().as_ptr();
        let mosq = mosquitto_new(id, true, std::ptr::null_mut());

        let res = mosquitto_connect(mosq, host, port, 30);
        println!("connect: {}", res);


        mosquitto_destroy(mosq);
        mosquitto_lib_cleanup();
    }

    println!("{}.{}.{}", major, minor, revision);
}
