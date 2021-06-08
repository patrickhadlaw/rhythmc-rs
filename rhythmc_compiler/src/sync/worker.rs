use super::threadpool::Task;

use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

enum WorkerSignal {
  Run(Task),
  Close,
}

pub struct Worker {
  sender: Mutex<Sender<WorkerSignal>>,
  queued: Arc<AtomicIsize>,
  handle: Option<JoinHandle<()>>,
}

impl Default for Worker {
  fn default() -> Self {
    let (tx, rx) = std::sync::mpsc::channel();
    let queued = Arc::new(AtomicIsize::new(0));
    Worker {
      sender: Mutex::new(tx),
      queued: queued.clone(),
      handle: Some(Self::run(rx, queued)),
    }
  }
}

impl Worker {
  pub fn new() -> Self {
    Self::default()
  }

  fn run(
    receiver: Receiver<WorkerSignal>,
    queued: Arc<AtomicIsize>,
  ) -> JoinHandle<()> {
    static mut ID: AtomicUsize = AtomicUsize::new(0);
    let id = unsafe { ID.fetch_add(1, Ordering::Relaxed) };
    std::thread::Builder::new()
      .name(format!("worker{}", id))
      .spawn(move || {
        crate::utils::sync::kill_process_on_panic();
        while let WorkerSignal::Run(task) = receiver.recv().unwrap() {
          task.invoke();
          queued.fetch_add(-1, Ordering::Relaxed);
        }
      })
      .unwrap()
  }

  /// Submits a given task to be run asynchronously
  ///
  /// `submit` sends the task to the worker thread using a channel
  ///
  /// - Tasks are guaranteed to be run, when the `Worker` instance is
  /// dropped the dropping thread signals the worker to close. Once the worker
  /// receives this signal it quits its thread loop.
  ///
  /// # Example
  /// ```
  /// use rhythm::sync::worker::Worker;
  /// use std::sync::{Arc, atomic::{AtomicU16, Ordering}};
  ///
  /// let atomic = Arc::new(AtomicU16::new(0));
  /// {
  ///   let worker = Worker::new();
  ///   let (copy1, copy2) = (atomic.clone(), atomic.clone());
  ///   worker.submit(move || { copy1.fetch_add(10, Ordering::Relaxed); });
  ///   worker.submit(move || { copy2.fetch_add(5, Ordering::Relaxed); });
  /// }
  /// assert_eq!(atomic.load(Ordering::Relaxed), 15);
  /// ```
  pub fn submit<F>(&self, job: F)
  where
    F: Fn() + Send + Sync + 'static,
  {
    self.submit_raw(Task::new(job));
  }

  pub fn submit_raw(&self, task: Task) {
    self.queued.fetch_add(1, Ordering::Relaxed);
    self
      .sender
      .lock()
      .unwrap()
      .send(WorkerSignal::Run(task))
      .unwrap();
  }

  pub fn done(&self) -> bool {
    self.queued.load(Ordering::Relaxed) == 0
  }
}

impl Drop for Worker {
  fn drop(&mut self) {
    self
      .sender
      .lock()
      .unwrap()
      .send(WorkerSignal::Close)
      .unwrap();
    if let Some(handle) = self.handle.take() {
      handle
        .join()
        .expect("expected worker thread to join gracefully");
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils::testing::async_context;

  use std::sync::atomic::AtomicUsize;

  #[test]
  fn submit_worker_jobs_test() {
    async_context(|| {
      let worker = Worker::new();
      let atomic = Arc::new(AtomicUsize::new(0));
      for _ in 0..100 {
        let reference = atomic.clone();
        worker.submit(move || {
          reference.fetch_add(1, Ordering::Relaxed);
        });
      }
      while !worker.done() {}
      assert_eq!(atomic.load(Ordering::Relaxed), 100);
    });
  }
}
