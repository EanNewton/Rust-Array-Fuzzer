/// Check how many distinct arrays we created

#[test]
fn gen_arrays() {
  let n = 5;
  let m = 4;
  let expected_total =
    (0..=n).map(|l| (m + 1).pow(l)).sum::<u32>();

  let mut total = 0;
  let mut all = HashSet::new();

  let mut g = Gen::new();
  while !g.done() {
    let l = g.gen(n) as usize;
    let xs: Vec<_> =
      std::iter::repeat_with(|| g.gen(m)).take(l).collect::<_>();

    all.insert(xs);
    total += 1
  }

  assert_eq!(all.len(), total);
  assert_eq!(expected_total, total as u32)
}
