

use super::At;


#[test]
fn new() {
    let _at: At = At::new("test");
}

#[test]
fn new_filename() {
    let at: At = At::new("test");
    assert_eq!(at.filename(), "test");
}
