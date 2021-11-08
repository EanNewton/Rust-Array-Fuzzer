//! We expect to see n numbers and n*2 ordered pairs

#[test]
fn gen_number() {
  let n = 5;
  let expected_total = n + 1;

  let mut total = 0;
  let mut all = HashSet::new();
  let mut g = Gen::new();
  while !g.done() {
    let a = g.gen(n);

    all.insert(a);
    total += 1;
  }

  assert_eq!(expected_total, total);
  assert_eq!(expected_total, all.len() as u32);
}

#[test]
fn gen_number_pair() {
  let n = 5;
  let expected_total = (n + 1) * (n + 1);

  let mut total = 0;
  let mut all = HashSet::new();
  let mut g = Gen::new();
  while !g.done() {
    let a = g.gen(n);
    let b = g.gen(n);

    all.insert((a, b));
    total += 1;
  }

  assert_eq!(expected_total, total);
  assert_eq!(expected_total, all.len() as u32);
}
