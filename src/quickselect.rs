use crate::point::Point;
use rand::Rng;

fn partition<const M: usize>(
    list: &mut Vec<Point<M>>,
    left: usize,
    right: usize,
    pivot_index: usize,
    d: usize
) -> usize {
    let pivot_value = list[pivot_index].0[d];
    list.swap(pivot_index, right);
    let mut store_index = left;
    for i in left..right {
        if list[i].0[d] < pivot_value {
            list.swap(store_index, i);
            store_index += 1;
        }
    }
    list.swap(right, store_index);
    store_index
}

/// Implementation of the quickselect algorithm for determining the median.
/// Adapted from the pseudo-code on Wikipedia (<https://en.wikipedia.org/wiki/Quickselect>)
pub fn median<const M: usize>(points: &[Point<M>], d: usize, rng: &mut impl Rng) -> f64 {
    let mut list = points.to_vec();

    let length = list.len();
    let mut left = 0;
    let mut right = length - 1;
    let k = (length - 1) / 2;

    loop {
        if left == right {
            return list[left].0[d];
        }
        let pivot_index = left + rand::seq::index::sample(rng, right - left, 1).index(0);
        let sorted_pivot_index = partition(&mut list, left, right, pivot_index, d);
        if k == sorted_pivot_index {
            return list[k].0[d];
        } else if k < sorted_pivot_index {
            right = sorted_pivot_index - 1;
        } else {
            left = sorted_pivot_index + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use super::median;

    #[test]
    fn even() {
        let mut rng = StdRng::seed_from_u64(0);
        let points = vec![
            Point([1.0]),
            Point([2.0]),
            Point([5.0]),
            Point([8.0]),
            Point([9.0]),
            Point([6.0]),
            Point([4.0]),
            Point([10.0]),
            Point([7.0]),
            Point([3.0]),
        ];
        assert_eq!(median(&points, 0, &mut rng), 5.0);
    }

    #[test]
    fn odd() {
        let mut rng = StdRng::seed_from_u64(0);
        let points = vec![
            Point([1.0]),
            Point([2.0]),
            Point([5.0]),
            Point([8.0]),
            Point([9.0]),
            Point([6.0]),
            Point([4.0]),
            Point([7.0]),
            Point([3.0]),
        ];
        assert_eq!(median(&points, 0, &mut rng), 5.0);
    }
}
