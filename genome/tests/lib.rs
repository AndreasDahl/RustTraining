#![feature(io)]

extern crate genome;

use genome::clustering::simple::seq_clust;
use std::io::stdout;

#[test]
fn it_works() {
    let data = vec!["abc", "abd", "aabc", "cbd"];
    
    seq_clust(&mut data.into_iter(), 0.5, &mut stdout());
}
