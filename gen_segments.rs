#[test]
fn gen_segments() {
  let n = 5;
  let m = 6;

  let mut total = 0;
  let mut all = HashSet::new();
  let mut g = Gen::new();
  while !g.done() {
    let l = g.gen(n);

    let mut xs = Vec::new();
    for _ in 0..l {
      if m > 0 {
        let l = g.gen(m - 1);
        let r = l + 1 + g.gen(m - l - 1);
        if !xs.contains(&(l, r)) {
          xs.push((l, r))
        }
      }
    }

    all.insert(xs);
    total += 1;
  }
  assert_eq!(all.len(), 2_593_942);
  assert_eq!(total, 4_288_306);
}
