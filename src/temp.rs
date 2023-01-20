struct Test {
  data: String,
  count: i32,
}

impl Test {
  fn send (self: &self, data: String) -> Option<i32> {
    thread::send(self.data);
    self.count = self.count + 1;
    self.count
  }
}
