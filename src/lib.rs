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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestWriter {
    }
    impl MetadataVisitor for TestWriter {
        fn superblock_b(&self) {
        }
    }

    fn pass_metadata_visitor(v: &dyn MetadataVisitor) {
        v.superblock_b();
    }

    fn test2(g: &dyn pkg2::MetadataGenerator) {
        let w = TestWriter {};
        w.superblock_b(); // ok
        pass_metadata_visitor(&w); // ok

        g.generate_metadata(&w); // failed
        pkg2::accept_metadata_visitor(&w); // failed
    }
}
