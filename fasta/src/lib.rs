use std::thread;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::RecvError;
use std::fmt;

pub struct Fasta {
    name: String,
    seq: String,
}

impl fmt::Display for Fasta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.seq)
    }
}

pub fn start(filename : &'static str) -> std::io::Result<Receiver<String>> {
    let (sender, receiver) = mpsc::channel();

    if let Err(e) = thread::Builder::new().spawn(move || {
        let f = File::open(filename).unwrap();
        let mut reader = BufReader::new(f);

        let mut line = String::new();
        while reader.read_line(&mut line).unwrap() > 0 {
            sender.send(line).unwrap();
            line = String::new();
        }
        println!("Bufferer is done");
    }) {
        return Err(e);
    }

    Ok(receiver)
}

pub fn get_next_fasta(receiver : &Receiver<String>) -> Result<Fasta, RecvError> {
    let na = receiver.recv();
    let se = receiver.recv();
    match na {
        Ok(n) => match se {
            Ok(s)  => Ok(Fasta {name : n, seq : s}),
            Err(e) => Err(e)
        },
        Err(e) => Err(e)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let recv = start("test1.fasta").unwrap();
        while let Ok(fasta) = get_next_fasta(&recv) {
            println!("{}", fasta);
        }
    }
}
