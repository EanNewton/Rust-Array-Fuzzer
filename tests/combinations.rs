#[test]
fn gen_combinations() {
    let n = 5;
    let m = 3;
    let fact = |n: u32| -> u32 { (1..=n).product() };
    let expected_total = fact(n) / (fact(m) * fact(n - m));

    let mut total = 0;
    let mut all = HashSet::new();
    let mut g = Gen::new();
    while !g.done() {
        let mut candidates: Vec<u32> = (1..=n).collect();
        let mut combination = BTreeSet::new();
        for _ in 0..m {
            let idx = g.gen(candidates.len() as u32 - 1);
            combination.insert(candidates.remove(idx as usize));
        }

        all.insert(combination);
        total += 1;
    }

    assert_eq!(expected_total, total);
    assert_eq!(expected_total, all.len() as u32);
}
