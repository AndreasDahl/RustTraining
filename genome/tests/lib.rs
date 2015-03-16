#![feature(io)]

extern crate genome;

use genome::clustering::seq_clust;
use std::io;

#[test]
fn it_works() {
    let data = vec!["abc", "abd", "aabc", "cbd"];
    let mut out = io::Cursor::new(Vec::new());
    
    seq_clust(&mut data.into_iter(), 0.5, &mut out);
    
    String::from_utf8(out.into_inner()).ok().expect("ERROR");
}
