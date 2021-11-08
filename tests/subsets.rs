#[test]
fn gen_subset() {
    let n = 5;
    let expected_total = 1 << n;

    let mut total = 0;
    let mut all = HashSet::new();
    let mut g = Gen::new();
    while !g.done() {
        let s: Vec<_> = (0..n).map(|_| g.gen(1) == 1).collect();

        all.insert(s);
        total += 1;
    }

    assert_eq!(expected_total, total);
    assert_eq!(expected_total, all.len() as u32);
}
