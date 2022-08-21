#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}
#[no_mangle]
pub extern "C" fn foo() {
    println!("it is from rust");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        foo();
    }
}
