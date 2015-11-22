use std::sync::Arc;
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc;

pub trait Producer<T : Send + Sync + 'static, G: Send + 'static> {
    fn produce(T) -> G;

    #[must_use]
    fn start_producing<I : Iterator<Item = T> + Send + Sync + 'static>(todo : I)
            -> std::io::Result<mpsc::Receiver<G>> {
        Self::start_producers(1, todo)
    }

    #[must_use]
    fn start_producers<I : Iterator<Item = T> + Send + Sync + 'static>(n : u32, iter : I)
            -> std::io::Result<mpsc::Receiver<G>> {
        let todo = Arc::new(Mutex::new(iter));
        let (sender, receiver) = mpsc::channel();

        for x in 0..n {
            let name = format!("Producer {}", x);

            let todo = todo.clone();
            let sender = sender.clone();
            let thread_result = thread::Builder::new().name(name).spawn(move || {
                while let Some(next) = Self::get_next(&todo) {
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

    fn get_next<I : Iterator<Item = T>>(mutex : &Arc<Mutex<I>>) -> Option<T> {
        mutex.lock().unwrap().next()
    }

}

#[cfg(test)]
mod tests {
    use super::Producer;

    use std::fmt::Display;
    use std::iter::{IntoIterator, FromIterator};

    struct EchoProducer;

    impl <T : Display + Send + Sync + 'static>Producer<T, T> for EchoProducer {
        fn produce(from : T) -> T {
            println!("{}: {}", thread::current().name().unwrap(), from);
            from
        }
    }

    #[test]
    fn it_works() {
        let work = Vec::from_iter(1..1000);

        let expected = work.clone();

        let rx = EchoProducer::start_producers(2, work.into_iter()).unwrap();

        let mut i = 0;
        while let Ok(n) = rx.recv() {
            assert_eq!(expected[i], n);
            i += 1;
        };
    }
}
