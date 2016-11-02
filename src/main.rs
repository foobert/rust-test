extern crate libc;
use std::ffi::CString;

pub enum Mosquitto {}

#[repr(C)]
pub struct MosquittoMessage {
    mid: u32,
    topic: *mut libc::c_char,
    payload: *mut libc::c_void,
    payloadlen: u32,
    qos: u32,
    retain: bool,
}

#[link(name = "mosquitto")]
extern {
    fn mosquitto_lib_version(major: *mut u32, minor: *mut u32, revision: *mut u32) -> u32;
    fn mosquitto_lib_init() -> u32;
    fn mosquitto_lib_cleanup() -> u32;
    fn mosquitto_new(id: *const libc::c_char, clean_session: bool, arg: *mut libc::c_void) -> *const Mosquitto;
    fn mosquitto_destroy(mosq: *const Mosquitto);
    fn mosquitto_connect(mosq: *const Mosquitto, host: *const libc::c_char, port: u32, keepalive: u32) -> u32;
    //fn mosquitto_loop(mosq: *const Mosquitto, timeout: u32, max_packets: u32) -> u32;
    fn mosquitto_loop_forever(mosq: *const Mosquitto, timeout: u32, max_packets: u32) -> u32;
    fn mosquitto_subscribe(mosq: *const Mosquitto, mid: *const u32, sub: *const libc::c_char, qos: u32) -> u32;
    fn mosquitto_message_callback_set(mosq: *const Mosquitto, on_message: extern fn(*const Mosquitto, *const libc::c_void, *mut MosquittoMessage));
}

extern fn on_message(mosq: *const Mosquitto, obj: *const libc::c_void, message: *mut MosquittoMessage) {
    println!("on_message");
    unsafe {
        let ref m = *message;
        let mut t = m.topic;
        let topic = CString::from_raw(t).into_string().expect("unable to load topic");
        println!("topic: {}", topic);
    }
}

fn main() {
    println!("Hello, world!");
    let mut major = 0;
    let mut minor = 0;
    let mut revision = 0;
    let id = "foo";
    let host = "test.mosquitto.org";
    let port = 1883;
    let topic = "footest";
    let mid = 0;

    unsafe {
        mosquitto_lib_init();
        mosquitto_lib_version(&mut major, &mut minor, &mut revision);

        let id: *const libc::c_char = CString::new(id).unwrap().as_ptr();
        let host: *const libc::c_char = CString::new(host).unwrap().as_ptr();
        let mosq = mosquitto_new(id, true, std::ptr::null_mut());

        mosquitto_message_callback_set(mosq, on_message);

        let res = mosquitto_connect(mosq, host, port, 30);
        println!("connect: {}", res);

        let topic: *const libc::c_char = CString::new(topic).unwrap().as_ptr();
        mosquitto_subscribe(mosq, &mid, topic, 1);

        mosquitto_loop_forever(mosq, 1000, 1);

        mosquitto_destroy(mosq);
        mosquitto_lib_cleanup();
    }

    println!("{}.{}.{}", major, minor, revision);
}
