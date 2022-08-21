use core::ffi::{c_int, c_uint, c_ulong};

#[repr(C)]
pub struct MyStruct {
    pub a: c_int,
}
extern "C" {
    fn getmessage(input: MyStruct) -> c_int;
    fn crc32(crc: c_ulong, buf: *const u8, len: c_uint) -> c_ulong;
    fn Print();
}
pub fn get_the_message(input: MyStruct) -> i32 {
    unsafe { getmessage(input) }
}
pub fn get_crc32(crc: u64, buf: *const u8, len: u32) -> c_ulong {
    unsafe { crc32(crc, buf, len) }
}
pub fn get_print() {
    unsafe {
        Print();
    }
}
extern "C" {}
#[test]
fn test_crc32() {
    let s = "hello";
    unsafe {
        assert_eq!(crc32(0, s.as_ptr(), s.len() as c_uint), 0x3610a686);
    }
}
