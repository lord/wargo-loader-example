fn main() {
    println!("Hello, world!")
}

// Functions that you wish to access from Javascript
// must be marked as no_mangle
#[no_mangle]
pub fn add_plus_five(a: i32, b: i32) -> i32 {
    return a + b + 5
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
