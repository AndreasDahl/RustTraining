use std::sync::Arc;
use std::collections::linked_list::LinkedList;
use std::thread;
use std::sync::RwLock;
use std::sync::mpsc;

pub trait Producer<T : Send + Sync + 'static, G: Send + 'static> {
    fn produce(T) -> G;

    fn start_producing(todo : Arc<RwLock<LinkedList<T>>>, sender : mpsc::Sender<G>)
            -> () {
        Self::start_producers(1, todo, sender);
    }

    fn start_producers(n : u32, todo : Arc<RwLock<LinkedList<T>>>, sender : mpsc::Sender<G>)
            -> () {
        for x in 0..n {
            let name = format!("Producer {}", x);

            let todo = todo.clone();
            let sender = sender.clone();
            thread::Builder::new().name(name).spawn(move || {
                while !todo.read().unwrap().is_empty() {
                    let mut list = todo.write().unwrap();
                    let answer = Self::produce(list.pop_front().unwrap());
                    sender.send(answer).unwrap();
                }
                println!("Producer done");
            }).unwrap();  // TODO: Return result
        }

    }
}

pub struct EchoProducer;

impl Producer<u32, u32> for EchoProducer {
    fn produce(from : u32) -> u32 {
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

    #[test]
    fn it_works() {
        let mut work = LinkedList::new();

        work.push_back(0);
        work.push_back(1);
        work.push_back(2);
        work.push_back(3);
        work.push_back(4);

        let data = Arc::new(RwLock::new(work));

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
