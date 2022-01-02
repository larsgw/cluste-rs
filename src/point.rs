#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<const N: usize> (pub [f64; N]);

impl<const N: usize> Point<N> {
    pub fn new(coords: [f64; N]) -> Self {
        Point(coords)
    }

    /// d(x, y) as defined in Section 2 (p. 278)
    ///
    /// Time complexity: O(M)
    pub fn distance(&self, point: &Self) -> f64 {
        (0..N).map(|d| (self.0[d] - point.0[d]).powi(2)).sum::<f64>().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn it_works() {
        Point([0.0, 0.2, 0.5]);
    }

    #[test]
    fn distance() {
        assert_eq!(Point::distance(
            &Point([1.0, 2.0, 3.0]),
            &Point([4.0, 5.0, 6.0])
        ), 5.196152422706632);
    }
}
