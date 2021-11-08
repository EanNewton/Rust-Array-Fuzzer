//! A small Rust program to:
//! generate non-repeating random number arrays for testing purposes
//!

struct Gen {
    /**
        v stores (value, bound) ipairs
        p tracks the current position in the middle of the titlebar_floating_button_normal_active
    **/
  started: bool,
  v: Vec<(u32, u32)>,
  p: usize,
}

impl Gen {
    /**
    v is conceptually an infinite vector with finite number of non-zero elements. So,
    when p gets past the end of v, we just materialize the implicit zero by pushing it onto v.

    As we store zeros implicitly anyways, we can just truncate the vector in done instead of
    zeroing-out the elements after the incremented one.

    The bounds are treated inclusively. This removes the panic when bound is zero, and allows
    to generate a full set of numbers via gen(u32::MAX).
    **/
  fn new() -> Gen {
    Gen { started: false, v: Vec::new(), p: 0 }
  }
  fn done(&mut self) -> bool {
    if !self.started {
      self.started = true;
      return false;
    }

    for i in (0..self.v.len()).rev() {
      if self.v[i].0 < self.v[i].1 {
        self.v[i].0 += 1;
        self.v.truncate(i + 1);
        self.p = 0;
        return false;
      }
    }

    true
  }
  fn gen(&mut self, bound: u32) -> u32 {
    if self.p == self.v.len() {
      self.v.push((0, 0));
    }
    self.p += 1;
    self.v[self.p - 1].1 = bound;
    self.v[self.p - 1].0
  }
}
