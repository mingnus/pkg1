pub fn hello() {
    println!("hello, world");
}

pub trait MetadataVisitor {
    fn superblock_b(&self);
}

struct XmlWriter;

impl MetadataVisitor for XmlWriter {
    fn superblock_b(&self) {
    }
}

#[test]
fn test1() {
    pkg2::foo();
}
