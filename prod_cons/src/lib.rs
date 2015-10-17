#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::collections::linked_list::LinkedList;
    use std::thread;
    use std::sync::RwLock;
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

        let mu = data.clone();
        let p = thread::Builder::new().name("Producer".to_string()).spawn(move || {
            while !mu.read().unwrap().is_empty() {
                let mut list = mu.write().unwrap();
                let answer = list.pop_front().unwrap();
                println!("Produced: {}", answer);
                tx.send(answer).unwrap();
            }
            println!("Producer done");
        }).unwrap();

        let c = thread::Builder::new().name("Consumer".to_string()).spawn(move || {
            while let Ok(n) = rx.recv() {
                println!("Receviced: {}", n)
            };

        }).unwrap();

        p.join().unwrap();
        c.join().unwrap();
    }
}
