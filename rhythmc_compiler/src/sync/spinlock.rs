use std::sync::atomic::{fence, AtomicBool, Ordering};

pub struct SpinLock {
  flag: AtomicBool,
}

impl SpinLock {
  pub fn new() -> Self {
    SpinLock {
      flag: AtomicBool::new(false),
    }
  }

  pub unsafe fn lock(&self) {
    while self.flag.compare_and_swap(false, true, Ordering::Relaxed) {
      // NOTE: add spin_loop(); here once it becomes stable
    }
    fence(Ordering::Acquire);
  }

  pub unsafe fn unlock(&self) {
    self.flag.store(false, Ordering::Release);
  }
}
