struct Builder {}

struct Node {
  directory: String,
}

trait Build {
  fn build(&self, dir: String) -> Node;
}

impl Build for Builder {
  fn build(&self, dir: String) -> Node {
    let node = Node { directory: dir };
    return node;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_passes_through_string_tree() {
    let builder = Builder {};
    let build_tree = builder.build(String::from("test-app/"));
    assert_eq!("test-app/", build_tree.directory);
  }

  #[test]
  #[ignore]
  fn it_calls_read_on_the_given_tree_object() {}

  #[test]
  #[ignore]
  fn it_read_tree_deduplicates() {}
}
