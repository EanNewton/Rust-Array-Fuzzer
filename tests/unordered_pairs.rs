//! We expect to see n*(n-1)/2 unordered pairs of numbers
//! This one is interesting — here, our second decision is based on the first one,
//! but we still enumerate all the cases efficiently (without duplicates).
//! (Aside: did you ever realise that the number of ways to pick two objects out of n is equal to the sum of first n natural numbers?)

#[test]
fn gen_number_combination() {
  let n = 5;
  let expected_total = n * (n + 1) / 2;

  let mut total = 0;
  let mut all = HashSet::new();
  let mut g = Gen::new();
  while !g.done() {
    let a = g.gen(n - 1);
    let b = a + 1 + g.gen(n - a - 1);
    all.insert((a, b));
    total += 1;
  }

  assert_eq!(expected_total, total);
  assert_eq!(expected_total, all.len() as u32);
}
