// 싱글스레드 기반 웹 서버 만들기
// TCP와 HTTP: 요청-응답(request-response) 프로토콜

// TCP 연결에 대한 처리

use std::fmt;
use std::sync::mpsc;
pub use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug, Clone)]
pub struct PoolCreationError;
impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pool must be usize but got wrong number")
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        // mutex를 통해 ThreadPool의 sender가 보낸 메시지가
        // 오직 하나의 Worker(receiver)에 의해 처리된다.
        let thread = thread::spawn(move || loop {
            // MutexGuard의 수명이 여기서 바로 소멸되므로, lock이 자동으로 해제됨
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job.call_box();
        });
        return Self {
            id,
            thread: Some(thread),
        };
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<Self, PoolCreationError> {
        if size < 0 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        return Ok(Self { workers, sender });
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        return self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {} ", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
