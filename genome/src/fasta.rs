use std::fs;
use std::io;
use std::str::Pattern;
use std::io::BufRead;

use fasta_string::FastaString;

pub struct FastaIO {
    file: io::Lines<io::BufReader<fs::File>>,
    next_line: String
}

impl FastaIO {
    pub fn new(read: fs::File) -> FastaIO {
        let mut lines = io::BufReader::new(read).lines();
        let first_line = lines.next().unwrap().unwrap();
        FastaIO { file: lines, next_line: first_line }
    }
}

impl Iterator for FastaIO {
    type Item = FastaString;

    fn next(&mut self) -> Option<FastaString> {
        let mut seq = String::new();

        loop {
            match self.file.next() {
                Some(line_res) => {
                    match line_res {
                        Ok(ref line) if '>'.is_prefix_of(line.as_slice()) => {
                            let ret = Some(FastaString { description: String::from_str(self.next_line.as_slice().trim_left_matches('>')), 
                                                         sequence: seq });
                            self.next_line = line.clone();
                            return ret;
                        }
                        Ok(line) => {
                            seq.push_str(line.as_slice());
                        }
                        Err(_) => return None
                    }
                }
                None => { return None; }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::FastaIO;
	
    #[test]
    fn test_iteration() {
        let mut file = fs::File::open("res/p3_clean_C-148-2-Caecum_S128_sorted.fa").unwrap();
        let mut fasta = FastaIO::new( file );
        for fastaString in fasta {
            // println!("{:?}", fastaString)
        }
    }
}





