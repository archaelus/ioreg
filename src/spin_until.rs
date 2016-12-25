#[macro_export]
macro_rules! spin_until(
  ($cond:expr) => (
    loop {
      if $cond {
        break;
      }
    }
  )
);
