pub fn hello() {
    println!("hello, world");
}

pub trait MetadataVisitor {
    fn superblock_b(&self);
}

#[test]
fn test1() {
    pkg2::foo();
}
