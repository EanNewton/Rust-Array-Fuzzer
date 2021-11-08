/// Test for a single nothing array
#[test]
fn gen_nothing() {
  let expected_total = 1;

  let mut total = 0;
  let mut g = Gen::new();
  while !g.done() {
    total += 1;
  }
  assert_eq!(expected_total, total)
}
