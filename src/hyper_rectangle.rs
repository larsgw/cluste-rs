use crate::point::Point;

#[derive(PartialEq, Debug)]
pub struct HyperRectangle<const M: usize> (pub Point<M>, pub Point<M>);

impl<const M: usize> HyperRectangle<M> {
    pub fn new(a: Point<M>, b: Point<M>) -> Self {
        HyperRectangle(a, b)
    }

    pub fn split(&self, d: usize, v: f64) -> (Self, Self) {
        let mut a = self.1.clone();
        a.0[d] = v;
        let mut b = self.0.clone();
        b.0[d] = v;

        (
            HyperRectangle(self.0, a),
            HyperRectangle(b, self.1),
        )
    }

    /// closest(x, h) as defined in Section 2 (p. 278)
    ///
    /// Time complexity: O(M)
    pub fn closest(&self, point: &Point<M>) -> Point<M> {
        let mut coords = [0.0; M];
        for d in 0..M {
            coords[d] = point.0[d].clamp(self.0.0[d], self.1.0[d]);
        }
        Point(coords)
    }

    /// d(x, h) as defined in Section 2 (p. 278)
    ///
    /// Time complexity: O(M)
    pub fn distance(&self, point: &Point<M>) -> f64 {
        self.closest(point).distance(point)
    }

    /// width(h) as defined in Section 2 (p. 278)
    ///
    /// Time complexity: O(M)
    pub fn width(&self) -> Point<M> {
        let mut coords = [0.0; M];
        for d in 0..M {
            coords[d] = self.1.0[d] - self.0.0[d];
        }
        Point(coords)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    fn split() {
        let h = HyperRectangle(Point([0.0, 0.0]), Point([2.0, 2.0]));
        let (h1, h2) = h.split(1, 1.0);
        assert_eq!(h1, HyperRectangle(Point([0.0, 0.0]), Point([2.0, 1.0])));
        assert_eq!(h2, HyperRectangle(Point([0.0, 1.0]), Point([2.0, 2.0])));
    }

    #[test]
    fn closest() {
        let h = HyperRectangle(Point([0.0, 0.0]), Point([2.0, 2.0]));
        let point = Point([-2.0, 3.0]);
        assert_eq!(h.closest(&point), Point([0.0, 2.0]));
    }

    #[test]
    fn distance() {
        let h = HyperRectangle(Point([0.0, 0.0]), Point([2.0, 2.0]));
        let point = Point([-2.0, 3.0]);
        assert_eq!(h.distance(&point), 2.23606797749979);
    }

    #[test]
    fn width() {
        let h = HyperRectangle(Point([1.0, 0.0]), Point([2.0, 2.0]));
        assert_eq!(h.width(), Point([1.0, 2.0]));
    }
}
