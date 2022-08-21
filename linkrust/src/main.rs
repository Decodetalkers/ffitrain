//use core::ffi::{c_int, c_uint, c_ulong};
use ffitrain::get_crc32;
fn main() {
    println!("it is the fuction from zilb");

    let s = "beta";
    assert_eq!(get_crc32(0, s.as_ptr(), s.len() as u32), 2408645731);
}
