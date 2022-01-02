#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<const N: usize> (pub [f64; N]);

impl<const N: usize> Point<N> {
    pub fn new(coords: [f64; N]) -> Self {
        Point(coords)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn it_works() {
        Point([0.0, 0.2, 0.5]);
    }
}
