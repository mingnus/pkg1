pub fn hello() {
    println!("hello, world");
}

#[test]
fn test1() {
    pkg2::foo();
}
