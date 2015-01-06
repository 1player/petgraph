
extern crate petgraph;

use std::rand::{Rng, thread_rng, ChaChaRng};
use std::collections::HashSet;
use petgraph::unionfind::UnionFind;

#[test]
fn uf_test() {
    let n = 8;
    let mut u = UnionFind::new(n);
    for i in range(0, n) {
        assert_eq!(u.find(i), i);
        assert_eq!(u.find_mut(i), i);
        assert!(!u.union(i, i));
    }

    u.union(0, 1);
    assert_eq!(u.find(0), u.find(1));
    u.union(1, 3);
    u.union(1, 4);
    u.union(4, 7);
    assert_eq!(u.find(0), u.find(3));
    assert_eq!(u.find(1), u.find(3));
    assert!(u.find(0) != u.find(2));
    assert_eq!(u.find(7), u.find(0));
    u.union(5, 6);
    assert_eq!(u.find(6), u.find(5));
    assert!(u.find(6) != u.find(7));

    // check that there are now 3 disjoint sets
    let set = range(0, n).map(|i| u.find(i)).collect::<HashSet<_>>();
    assert_eq!(set.len(), 3);
}

#[test]
fn uf_rand() {
    let n = 1 << 14;
    let mut rng: ChaChaRng = thread_rng().gen();
    let mut u = UnionFind::new(n);
    for i in range(0, n * 8) {
        let a = rng.gen_range(0, n);
        let b = rng.gen_range(0, n);
        let ar = u.find(a);
        let br = u.find(b);
        assert_eq!(ar != br, u.union(a, b));
        if (i + 1) % n == 0 {
            let set = range(0, n).map(|i| u.find(i)).collect::<HashSet<_>>();
            println!("Disjoint parts={}", set.len());
            //println!("Disjoint parts={} maxrank={}", set.len(), u.rank.iter().max_by(|t| *t));
        }
    }
}
