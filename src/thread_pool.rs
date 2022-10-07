use std::{
    num::NonZeroUsize,
    sync::{mpsc, Arc, Mutex},
};

pub struct Static {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Task>>,
}

impl Static {
    pub fn build(size: NonZeroUsize) -> Result<Static, String> {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size.get());
        for _ in 0..size.get() {
            workers.push(Worker::new(receiver.clone()));
        }

        Ok(Static {
            workers,
            sender: Some(sender),
        })
    }

    pub fn submit<T>(&self, task: T)
    where
        T: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(task)).unwrap();
    }
}

impl Drop for Static {
    fn drop(&mut self) {
        drop(self.sender.take());
        self.workers.clear();
    }
}

struct Worker {
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        Worker {
            handle: Some(std::thread::spawn(move || loop {
                let task = receiver.lock().unwrap().recv();
                if task.is_err() {
                    break;
                }
                task.unwrap()();
            })),
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.handle.take().unwrap().join().unwrap();
    }
}

type Task = Box<dyn FnOnce() + Send>;
