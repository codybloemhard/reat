fn main() {
    let mut xattrs = xattr::list("./test").unwrap().peekable();
    for attr in xattrs {
        println!("{:?}", attr);
    }
}
