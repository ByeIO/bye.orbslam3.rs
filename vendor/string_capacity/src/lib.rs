use std::borrow::Cow;
use std::collections::TryReserveError;

macro_rules! count_digits {
  ($value:expr) => {{
    let mut value = $value;
    if value == 0 {
      1
    } else {
      let mut count = 0;
      while value > 0 {
        value /= 10;
        count += 1;
      }
      count
    }
  }};
}

macro_rules! impl_appendable_for_int {
  ($($t:ty),*) => {
    $(
      impl Appendable for $t {
        fn byte_len(&self) -> usize {
          count_digits!(*self)
        }

        fn push_to(&self, text: &mut String) {
          let value = *self;
          // no need to reuse buffers as per the documentation
          // and as found in my benchmarks
          let mut buffer = itoa::Buffer::new();
          let s = buffer.format(value);
          text.push_str(s);
        }
      }
    )*
  };
}

pub trait Appendable {
  fn byte_len(&self) -> usize;
  fn push_to(&self, text: &mut String);
}

impl Appendable for &str {
  #[inline(always)]
  fn byte_len(&self) -> usize {
    self.len()
  }

  #[inline(always)]
  fn push_to(&self, text: &mut String) {
    text.push_str(self);
  }
}

impl Appendable for &String {
  #[inline(always)]
  fn byte_len(&self) -> usize {
    self.len()
  }

  #[inline(always)]
  fn push_to(&self, text: &mut String) {
    text.push_str(self);
  }
}

impl<'a> Appendable for &'a Cow<'a, str> {
  #[inline(always)]
  fn byte_len(&self) -> usize {
    self.len()
  }

  #[inline(always)]
  fn push_to(&self, text: &mut String) {
    text.push_str(self);
  }
}

impl_appendable_for_int!(
  i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

impl Appendable for char {
  #[inline(always)]
  fn byte_len(&self) -> usize {
    self.len_utf8()
  }

  #[inline(always)]
  fn push_to(&self, text: &mut String) {
    text.push(*self);
  }
}

impl<T: Appendable> Appendable for Option<T> {
  #[inline(always)]
  fn byte_len(&self) -> usize {
    match self {
      Some(value) => value.byte_len(),
      None => 0,
    }
  }

  #[inline(always)]
  fn push_to(&self, text: &mut String) {
    if let Some(value) = self {
      value.push_to(text);
    }
  }
}

impl<T: Appendable + ?Sized> Appendable for &T {
  #[inline(always)]
  fn byte_len(&self) -> usize {
    (**self).byte_len()
  }

  #[inline(always)]
  fn push_to(&self, text: &mut String) {
    (**self).push_to(text)
  }
}

pub struct StringBuilder<'a> {
  capacity: usize,
  text: Option<String>,
  phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> StringBuilder<'a> {
  #[inline(always)]
  pub fn build(
    build: impl Fn(&mut StringBuilder<'a>),
  ) -> Result<String, TryReserveError> {
    let mut builder = StringBuilder {
      capacity: 0,
      text: None,
      phantom: std::marker::PhantomData,
    };
    build(&mut builder);
    let mut text = String::new();
    text.try_reserve_exact(builder.capacity)?;
    builder.text = Some(text);
    build(&mut builder);
    debug_assert_eq!(builder.capacity, builder.text.as_ref().unwrap().len());
    Ok(builder.text.unwrap())
  }

  #[inline(always)]
  pub fn append(&mut self, value: impl Appendable + 'a) {
    match &mut self.text {
      Some(t) => value.push_to(t),
      None => self.capacity += value.byte_len(),
    }
  }
}
