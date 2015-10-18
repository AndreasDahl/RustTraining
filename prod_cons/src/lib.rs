use std::sync::Arc;
use std::thread;
use std::sync::RwLock;
use std::sync::mpsc;
use std::fmt::Display;

pub trait Producer<T : Send + Sync + 'static, G: Send + 'static> {
    fn produce(T) -> G;

    fn start_producing<I : Iterator<Item = T> + Send + Sync + 'static>(todo : Arc<RwLock<I>>, sender : mpsc::Sender<G>)
            -> () {
        Self::start_producers(1, todo, sender);
    }

    fn start_producers<I : Iterator<Item = T> + Send + Sync + 'static>(n : u32, todo : Arc<RwLock<I>>, sender : mpsc::Sender<G>)
            -> () {
        for x in 0..n {
            let name = format!("Producer {}", x);

            let todo = todo.clone();
            let sender = sender.clone();
            thread::Builder::new().name(name).spawn(move || {
                while let Some(next) = todo.write().unwrap().next() {
                    let answer = Self::produce(next);
                    sender.send(answer).unwrap();
                }
                println!("Producer done");
            }).unwrap();  // TODO: Return handle
        }

    }
}

pub struct EchoProducer;

impl <T : Display + Send + Sync + 'static>Producer<T, T> for EchoProducer {
    fn produce(from : T) -> T {
        println!("Producing {}", from);
        from
    }
}

#[cfg(test)]
mod tests {
    use super::{EchoProducer, Producer};

    use std::sync::RwLock;
    use std::thread;
    use std::collections::linked_list::LinkedList;
    use std::sync::Arc;
    use std::sync::mpsc;
    use std::iter::IntoIterator;

    #[test]
    fn it_works() {
        let mut work = LinkedList::new();

        work.push_back(0);
        work.push_back(1);
        work.push_back(2);
        work.push_back(3);
        work.push_back(4);

        let work_iter = work.into_iter();

        let data = Arc::new(RwLock::new(work_iter));

        let (tx, rx) = mpsc::channel();

        EchoProducer::start_producing(data, tx);

        let c = thread::Builder::new().name("Consumer".to_string()).spawn(move || {
            while let Ok(n) = rx.recv() {
                println!("Receviced: {}", n)
            };

        }).unwrap();

        c.join().unwrap();
    }
}
