use crate::point::Point;

#[derive(PartialEq, Debug)]
pub struct HyperRectangle<const N: usize> (pub Point<N>, pub Point<N>);

impl<const N: usize> HyperRectangle<N> {
    pub fn new(a: Point<N>, b: Point<N>) -> Self {
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
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    fn hyper_rectangle_split() {
        let h = HyperRectangle(Point([0.0, 0.0]), Point([2.0, 2.0]));
        let (h1, h2) = h.split(1, 1.0);
        assert_eq!(h1, HyperRectangle(Point([0.0, 0.0]), Point([2.0, 1.0])));
        assert_eq!(h2, HyperRectangle(Point([0.0, 1.0]), Point([2.0, 2.0])));
    }
}
