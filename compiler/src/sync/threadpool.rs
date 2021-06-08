extern crate num_cpus;

use std::collections::VecDeque;
use std::sync::{
  atomic::{AtomicUsize, Ordering},
  Arc, Condvar, Mutex,
};
use std::thread;
use std::thread::JoinHandle;

pub struct Task {
  func: Box<dyn Fn() + Send + Sync + 'static>,
}

impl Task {
  pub fn new<F>(func: F) -> Self
  where
    F: Fn() + Send + Sync + 'static,
  {
    Task {
      func: Box::new(func),
    }
  }

  pub fn invoke(self) {
    (self.func)()
  }
}

pub struct ThreadPoolBuilder {
  name: Option<String>,
  workers: usize,
}

impl Default for ThreadPoolBuilder {
  fn default() -> Self {
    ThreadPoolBuilder {
      name: None,
      workers: num_cpus::get(),
    }
  }
}

impl ThreadPoolBuilder {
  pub fn new() -> Self {
    ThreadPoolBuilder::default()
  }

  pub fn named(name: String) -> Self {
    ThreadPoolBuilder::new().name(name)
  }

  pub fn name(mut self, name: String) -> Self {
    self.name = Some(name);
    self
  }

  pub fn workers(mut self, amount: usize) -> Self {
    self.workers = amount;
    self
  }

  pub fn build(&self) -> ThreadPool {
    static mut ID: AtomicUsize = AtomicUsize::new(0);
    let id = unsafe { ID.fetch_add(1, Ordering::Relaxed) };
    let shared = Arc::new(SharedData {
      signal: (Mutex::new(ThreadPoolStatus::Pending(0)), Condvar::new()),
      jobs: Mutex::new(VecDeque::new()),
      running_count: AtomicUsize::new(0),
    });
    let mut workers = Vec::new();
    for i in 0..self.workers {
      let name = match &self.name {
        Some(name) => format!("{}_worker{}", name, i),
        None => format!("threadpool{}_worker{}", id, i),
      };
      workers.push(ThreadPool::worker(name, shared.clone()));
    }
    ThreadPool { workers, shared }
  }
}

enum ThreadPoolStatus {
  Pending(usize),
  Closing,
}

struct SharedData {
  signal: (Mutex<ThreadPoolStatus>, Condvar),
  jobs: Mutex<VecDeque<Task>>,
  running_count: AtomicUsize,
}

pub struct ThreadPool {
  workers: Vec<JoinHandle<()>>,
  shared: Arc<SharedData>,
}

impl ThreadPool {
  /// Submits a given task to be run asynchronously
  ///
  /// `submit` queues the given task in the thread pool shared data and wakes
  /// one worker to take on the task
  ///
  /// - Tasks are guaranteed to be run, when the `ThreadPool` instance is
  /// dropped the dropping thread signals the workers to close. Once each worker
  /// receives this signal it depletes the rest of the jobs in a tight loop -
  /// after which the worker thread joins with the dropping thread
  ///
  /// # Example
  /// ```
  /// use rhythm::sync::threadpool::ThreadPoolBuilder;
  /// use std::sync::{Arc, atomic::{AtomicU16, Ordering}};
  ///
  /// let atomic = Arc::new(AtomicU16::new(0));
  /// {
  ///   let pool = ThreadPoolBuilder::new().build();
  ///   let (copy1, copy2) = (atomic.clone(), atomic.clone());
  ///   pool.submit(move || { copy1.fetch_add(10, Ordering::Relaxed); });
  ///   pool.submit(move || { copy2.fetch_add(5, Ordering::Relaxed); });
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
    self.shared.jobs.lock().unwrap().push_back(task);
    {
      let mut guard = self.shared.signal.0.lock().unwrap();
      let count = match *guard {
        ThreadPoolStatus::Pending(count) => count,
        ThreadPoolStatus::Closing => {
          panic!("attempted to submit task to expired thread pool")
        }
      };
      *guard = ThreadPoolStatus::Pending(count + 1);
    }
    self.shared.signal.1.notify_one();
  }

  /// Returns true if pool has completed all jobs and is standing idle
  pub fn done(&self) -> bool {
    let _guard = self.shared.signal.0.lock().unwrap();
    self.shared.running_count.load(Ordering::Relaxed) == 0
      && self.shared.jobs.lock().unwrap().is_empty()
  }

  fn worker(name: String, shared: Arc<SharedData>) -> JoinHandle<()> {
    thread::Builder::new()
      .name(name)
      .spawn(move || {
        crate::utils::sync::kill_process_on_panic();
        loop {
          {
            let mut status_guard = shared.signal.0.lock().unwrap();
            let count = match *status_guard {
              ThreadPoolStatus::Pending(count) => count,
              ThreadPoolStatus::Closing => break,
            };
            if count == 0 {
              status_guard = shared.signal.1.wait(status_guard).unwrap();
              match *status_guard {
                ThreadPoolStatus::Pending(count) => {
                  if count == 0 {
                    continue;
                  }
                  *status_guard = ThreadPoolStatus::Pending(count - 1)
                }
                ThreadPoolStatus::Closing => break,
              }
            } else {
              *status_guard = ThreadPoolStatus::Pending(count - 1);
            }
          }
          shared.running_count.fetch_add(1, Ordering::Relaxed);
          let job = shared.jobs.lock().unwrap().pop_front().unwrap();
          job.invoke();
          shared.running_count.fetch_sub(1, Ordering::Relaxed);
        }
        ThreadPool::deplete(shared);
      })
      .unwrap()
  }

  fn deplete(shared: Arc<SharedData>) {
    while let Some(job) = shared.jobs.lock().unwrap().pop_front() {
      job.invoke();
    }
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    *self.shared.signal.0.lock().unwrap() = ThreadPoolStatus::Closing;
    self.shared.signal.1.notify_all();
    while let Some(handle) = self.workers.pop() {
      handle.join().expect("pool worker panicked");
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils::testing::async_context;

  #[test]
  fn thread_builder_test() {
    let pool = ThreadPoolBuilder::new().workers(4).build();
    assert_eq!(pool.workers.len(), 4);
  }

  #[test]
  fn submit_threadpool_jobs_test() {
    async_context(|| {
      let pool = ThreadPoolBuilder::new().build();
      let atomic = Arc::new(AtomicUsize::new(0));
      for _ in 0..100 {
        let reference = atomic.clone();
        pool.submit(move || {
          reference.fetch_add(1, Ordering::Relaxed);
        });
      }
      while !pool.done() {}
      assert_eq!(atomic.load(Ordering::Relaxed), 100);
    });
  }
}
