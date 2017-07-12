use std::cmp::Ordering;

// Copied from std::cmp with feature gate removed.

/// A helper struct for reverse ordering.
///
/// This struct is a helper to be used with functions like `Vec::sort_by_key` and
/// can be used to reverse order a part of a key.
///
/// Example usage:
///
/// ```
/// use discrepancy::reverse::Reverse;
///
/// let mut v = vec![1, 2, 3, 4, 5, 6];
/// v.sort_by_key(|&num| (num > 3, Reverse(num)));
/// assert_eq!(v, vec![3, 2, 1, 6, 5, 4]);
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct Reverse<T>(pub T);

impl<T: PartialOrd> PartialOrd for Reverse<T> {
    #[inline]
    fn partial_cmp(&self, other: &Reverse<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        other.0 < self.0
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        other.0 <= self.0
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        other.0 >= self.0
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        other.0 > self.0
    }
}

impl<T: Ord> Ord for Reverse<T> {
    #[inline]
    fn cmp(&self, other: &Reverse<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}
