use super::threadpool::{Task, ThreadPool, ThreadPoolBuilder};

use std::sync::Arc;

pub struct Runtime;

impl Runtime {
  fn get() -> Arc<ThreadPool> {
    lazy_static! {
      static ref POOL: Arc<ThreadPool> =
        Arc::new(ThreadPoolBuilder::named("runtime".to_owned()).build());
    }
    POOL.clone()
  }

  /// Submits a given task to be run asynchronously on the rhythm runtime which
  /// is currently implemented as a thread pool, see
  /// [this method](ThreadPool::submit) for details
  pub fn submit<F>(job: F)
  where
    F: Fn() + Send + Sync + 'static,
  {
    Runtime::submit_raw(Task::new(job));
  }

  pub fn submit_raw(task: Task) {
    Runtime::get().submit_raw(task);
  }

  /// Returns true if runtime has completed all jobs and is standing idle
  pub fn done() -> bool {
    Runtime::get().done()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils::testing::async_context;

  use std::sync::atomic::{AtomicUsize, Ordering};

  #[test]
  fn submit_runtime_jobs_test() {
    async_context(|| {
      let atomic = Arc::new(AtomicUsize::new(0));
      for _ in 0..100 {
        let reference = atomic.clone();
        Runtime::submit(move || {
          reference.fetch_add(1, Ordering::Relaxed);
        });
      }
      while !Runtime::done() {}
      assert_eq!(atomic.load(Ordering::Relaxed), 100);
    });
  }
}
