#[test]
fn gen_permutations() {
  let n = 5;
  let expected_total = (1..=n).product::<u32>();

  let mut total = 0;
  let mut all = HashSet::new();
  let mut g = Gen::new();
  while !g.done() {
    let mut candidates: Vec<i32> = (1..=n).collect();
    let mut permutation = Vec::new();
    for _ in 0..n {
      let idx = g.gen(candidates.len() as u32 - 1);
      permutation.push(candidates.remove(idx as usize));
    }

    all.insert(permutation);
    total += 1;
  }

  assert_eq!(expected_total, total);
  assert_eq!(expected_total, all.len() as u32);
}
