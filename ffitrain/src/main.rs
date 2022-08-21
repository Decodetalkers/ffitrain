use ffitrain::*;
fn main() {
    println!("Hello, world!");
    let c = MyStruct { a: 10 };
    println!("{}", get_the_message(c));
    let s = "hello";
    assert_eq!(get_crc32(0, s.as_ptr(), s.len() as u32), 0x3610a686);
    println!("test: {}", get_crc32(0, s.as_ptr(), s.len() as u32));
    get_print();
}

