use std::sync::Arc;
use std::thread;
use std::sync::RwLock;
use std::sync::mpsc;
use std::fmt::Display;

pub trait Producer<T : Send + Sync + 'static, G: Send + 'static> {
    fn produce(T) -> G;

    fn start_producing<I : Iterator<Item = T> + Send + Sync + 'static>(todo : I)
            -> std::io::Result<mpsc::Receiver<G>> {
        Self::start_producers(1, todo)
    }

    fn start_producers<I : Iterator<Item = T> + Send + Sync + 'static>(n : u32, iter : I)
            -> std::io::Result<mpsc::Receiver<G>> {
        let todo = Arc::new(RwLock::new(iter));
        let (sender, receiver) = mpsc::channel();

        for x in 0..n {
            let name = format!("Producer {}", x);

            let todo = todo.clone();
            let sender = sender.clone();
            let thread_result = thread::Builder::new().name(name).spawn(move || {
                while let Some(next) = todo.write().unwrap().next() {
                    let answer = Self::produce(next);
                    sender.send(answer).unwrap();
                }
                println!("{} done", thread::current().name().unwrap());
            });
            if let Err(e) = thread_result {
                return Err(e);
            }
        }

        Ok(receiver)
    }
}

pub struct EchoProducer;

impl <T : Display + Send + Sync + 'static>Producer<T, T> for EchoProducer {
    fn produce(from : T) -> T {
        println!("{}: {}", thread::current().name().unwrap(), from);
        from
    }
}

#[cfg(test)]
mod tests {
    use super::{EchoProducer, Producer};

    use std::iter::IntoIterator;

    #[test]
    fn it_works() {
        let work = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
        let expected = work.clone();

        let rx = EchoProducer::start_producers(2, work.into_iter()).unwrap();

        let mut i = 0;
        while let Ok(n) = rx.recv() {
            assert_eq!(expected[i], n);
            i += 1;
        };
    }
}
