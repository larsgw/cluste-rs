use kmeans::{KMeans, Algorithm, Point};
use std::convert::TryInto;
use std::io::BufRead;
use std::time::Instant;

const K: usize = 4;
const M: usize = 2;
const R: usize = 50000;

const ALGO: Algorithm = Algorithm::Simple;

fn read_data() -> [Point<M>; R] {
    let mut data = Vec::with_capacity(R);
    let stdin = std::io::stdin();
    for line in stdin.lock().lines().skip(1) {
        match line {
            Err(_) => break,
            Ok(text) => {
                let mut point = [0.0; M];
                for (i, value) in text.split(',').skip(1).enumerate() {
                    point[i] = value.parse::<f64>().unwrap();
                }
                data.push(Point(point));
            }
        }
    }

    data.try_into().unwrap()
}

fn main() {
    let data = read_data();

    let now = Instant::now();
    let model = KMeans::<K, M, R>::fit_with_random_state(&data, ALGO, 0);
    eprintln!("total: {:?}", now.elapsed());

    for point in model.centers {
        for x in point.0 {
            print!("{},", x);
        }
        println!("");
    }
}
