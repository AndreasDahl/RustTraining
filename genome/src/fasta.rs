use std::fs;
use std::io;
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
                        Ok(ref line) if line.as_bytes()[0] == b'>' => {
                            let ret = Some(FastaString { description: String::from(self.next_line.trim_left_matches('>')),
                                                         sequence: seq });
                            self.next_line = line.clone();
                            return ret;
                        }
                        Ok(line) => {
                            seq.push_str(&line);
                        }
                        Err(_) => return None
                    }
                }
                None => { return None; }
            }
        }
    }
}
