pub use self::mrkd::*;
pub use self::quickselect::*;

mod mrkd;
mod quickselect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<const N: usize> ([f64; N]);

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn it_works() {
        Point([0.0, 0.2, 0.5]);
    }
}
