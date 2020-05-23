struct Builder {}

trait BuilderError {
  fn builder_error(&self) -> bool;
}

trait Build {}

impl Build for Builder {}

impl BuilderError for Builder {
  fn builder_error(&self) -> bool {
    return true;
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_has_static_members_that_are_part_of_the_public_API() {
    // https://github.com/broccolijs/broccoli/blob/master/test/builder_test.js#L77

    let builder = Builder {};

    assert!(builder.builder_error());
  }
}
