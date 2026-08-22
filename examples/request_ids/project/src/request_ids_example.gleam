import example_request_ids as request_ids

pub fn main() {
  assert request_ids.issued() == 0
  assert request_ids.next() == "request-1"
  assert request_ids.issued() == 1
  assert request_ids.next() == "request-2"
  assert request_ids.issued() == 2
}
