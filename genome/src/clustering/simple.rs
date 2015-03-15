use std::io::Write;
use std::io::stdout;

pub fn seq_clust(input: &mut Iterator<Item = &str>, similarity: f32, output: &mut Write) {
    for e in input {
        writeln!(output, "Output {}", e).ok().expect("Could not write to output");
    }
    output.flush();
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use std::io;

    use super::seq_clust;
	
    #[test]
    fn test_seq_clust() {
        let data = vec!["abc", "abd", "aabc", "cbd"];
        let mut out = io::Cursor::new(Vec::new());
        
        seq_clust(&mut data.into_iter(), 0.5, &mut out);
        
        let s = String::from_utf8(out.into_inner()).ok().expect("ERROR");
    }
}
