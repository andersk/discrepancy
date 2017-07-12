extern crate discrepancy;

use discrepancy::monoid::Monoid;
use discrepancy::range_vec::RangeVec;

#[derive(Clone, Debug, PartialEq, Eq)]
enum TestMonoid {
    Empty,
    Range(usize, usize),
}

impl Monoid for TestMonoid {
    fn empty() -> TestMonoid {
        TestMonoid::Empty
    }

    fn append(&self, other: &TestMonoid) -> TestMonoid {
        match (self, other) {
            (&TestMonoid::Empty, _) => other.clone(),
            (_, &TestMonoid::Empty) => self.clone(),
            (&TestMonoid::Range(a, b), &TestMonoid::Range(c, d)) => {
                assert_eq!(b, c);
                TestMonoid::Range(a, d)
            }
        }
    }
}

#[test]
fn test_concat_all() {
    let mut v = RangeVec::new();
    for n in 0..150 {
        assert_eq!(v.len(), n);
        assert_eq!(
            v.concat_all(),
            if n == 0 {
                TestMonoid::Empty
            } else {
                TestMonoid::Range(0, n)
            }
        );
        v.push(TestMonoid::Range(n, n + 1));
    }
}

#[test]
fn test_concat_range() {
    let mut v = RangeVec::new();
    for n in 0..150 {
        assert_eq!(v.len(), n);
        for j in 0..n {
            assert_eq!(v[j], TestMonoid::Range(j, j + 1));
        }
        for a in 0..n + 1 {
            assert_eq!(v.concat_range(a..a), TestMonoid::Empty);
            for b in a + 1..n + 1 {
                assert_eq!(v.concat_range(a..b), TestMonoid::Range(a, b));
            }
        }
        v.push(TestMonoid::Range(n, n + 1));
    }
}

#[test]
fn test_set() {
    let mut v = RangeVec::new();
    for n in 0..50 {
        assert_eq!(v.len(), n);
        for i in 0..n {
            v.set(i, TestMonoid::Range(i, i + 1));
            for j in 0..n {
                assert_eq!(v[j], TestMonoid::Range(j, j + 1));
            }
            for a in 0..n + 1 {
                assert_eq!(v.concat_range(a..a), TestMonoid::Empty);
                for b in a + 1..n + 1 {
                    assert_eq!(v.concat_range(a..b), TestMonoid::Range(a, b));
                }
            }
        }
        v.push(TestMonoid::Range(n, n + 1));
    }
}
