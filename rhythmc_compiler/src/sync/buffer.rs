use super::spinlock::SpinLock;
use std::cell::UnsafeCell;

pub struct RingBuffer<T> {
  lock: SpinLock,
  front: UnsafeCell<usize>,
  filled: UnsafeCell<bool>,
  raw: UnsafeCell<Vec<T>>,
  size: usize,
}

impl<T> RingBuffer<T>
where
  T: Clone,
{
  pub fn new(size: usize) -> RingBuffer<T> {
    RingBuffer {
      lock: SpinLock::new(),
      front: UnsafeCell::new(0),
      filled: UnsafeCell::new(false),
      raw: UnsafeCell::new(Vec::new()),
      size,
    }
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn push(&self, value: T) {
    unsafe {
      self.lock.lock();
      #[allow(unused_mut)]
      let mut raw = &mut *self.raw.get();
      if raw.len() < self.size {
        raw.push(value);
      } else {
        raw[*self.front.get()] = value;
      }
      *self.front.get() = (*self.front.get() + 1) % self.size;
      if *self.front.get() == 0 {
        *self.filled.get() = true;
      }
      self.lock.unlock();
    }
  }

  unsafe fn get_critical(&self, num: usize) -> Vec<T> {
    let mut result = Vec::new();
    result.reserve(num);
    let fetch = (if num < self.size { num } else { self.size }) as isize;
    let front = *self.front.get() as isize;
    let raw = &mut *self.raw.get();
    if *self.filled.get() {
      let back = if front - fetch < 0 {
        ((self.size as isize) + front - fetch) as usize
      } else {
        (front - fetch) as usize
      };
      if back < *self.front.get() {
        for item in raw.iter().take(*self.front.get()).skip(back) {
          result.push(item.clone());
        }
      } else {
        for i in (back..self.size).chain(0..*self.front.get()) {
          result.push(raw[i].clone());
        }
      }
    } else {
      let back = front - fetch;
      let start = if back < 0 { 0 } else { back as usize };
      for item in raw.iter().take(*self.front.get()).skip(start) {
        result.push(item.clone());
      }
    }
    result
  }

  pub fn get(&self, num: usize) -> Vec<T> {
    if num == 0 {
      return Vec::new();
    }
    unsafe {
      self.lock.lock();
      let result = self.get_critical(num);
      self.lock.unlock();
      result
    }
  }

  pub fn get_and_do<F>(&self, num: usize, task: F) -> Vec<T>
  where
    F: Fn(),
  {
    if num == 0 {
      return Vec::new();
    }
    unsafe {
      self.lock.lock();
      let result = self.get_critical(num);
      task();
      self.lock.unlock();
      result
    }
  }

  pub fn get_all(&self) -> Vec<T> {
    self.get(self.size)
  }
}

unsafe impl<T> Sync for RingBuffer<T> {}
unsafe impl<T> Send for RingBuffer<T> {}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils::testing::async_context;

  #[test]
  fn new_ring_buffer_test() {
    let ring = RingBuffer::<()>::new(5);
    assert_eq!(ring.size, 5);
    unsafe {
      assert_eq!(*ring.filled.get(), false);
      assert_eq!(*ring.front.get(), 0);
    }
  }

  #[test]
  fn push_test() {
    async_context(|| {
      let ring = RingBuffer::new(5);
      ring.push(1);
      ring.push(2);
      ring.push(3);
      unsafe {
        assert_eq!(ring.raw.get().as_ref().unwrap().len(), 3);
        assert_eq!(*ring.raw.get(), [1, 2, 3]);
      }
    });
  }

  #[test]
  fn overflow_test() {
    async_context(|| {
      let ring = RingBuffer::new(3);
      ring.push(1);
      ring.push(2);
      ring.push(3);
      ring.push(4);
      ring.push(5);
      ring.push(6);
      unsafe {
        assert_eq!(ring.raw.get().as_ref().unwrap().len(), 3);
        assert_eq!(*ring.raw.get(), [4, 5, 6]);
      }
    });
  }

  #[test]
  fn get_unfilled_test() {
    async_context(|| {
      let ring = RingBuffer::new(5);
      ring.push(1);
      ring.push(2);
      ring.push(3);
      assert_eq!(ring.get(2), [2, 3]);
    });
  }

  #[test]
  fn get_filled_test() {
    async_context(|| {
      let ring = RingBuffer::new(3);
      ring.push(1);
      ring.push(2);
      ring.push(3);
      ring.push(4);
      ring.push(5);
      unsafe {
        assert_eq!(*ring.filled.get(), true);
      };
      assert_eq!(ring.get(2), [4, 5]);
    });
  }

  #[test]
  fn get_wraparound_test() {
    async_context(|| {
      let ring = RingBuffer::new(3);
      ring.push(1);
      ring.push(2);
      ring.push(3);
      ring.push(4);
      ring.push(5);
      unsafe {
        assert_eq!(*ring.filled.get(), true);
      };
      assert_eq!(ring.get(3), [3, 4, 5]);
    });
  }
}
