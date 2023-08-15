use rand::distributions::{Uniform};
use rand::{self, Rng};

const N_ITERS: usize = 100_000_000;

fn main() {
    let mut rng = rand::thread_rng();
    let mut circle_points = 0.;
    let mut square_points = 0.;
    let mut pi = 0.;
    for _ in 0..N_ITERS {
    let rand_x: f64 = rng.gen_range(-1.0..1.);
    let rand_y: f64 = rng.gen_range(-1.0..1.);
    let origin_distance = rand_x.powi(2) + rand_y.powi(2);
    if origin_distance <= 1. {
        circle_points += 1.;
    }
    square_points += 1.;
    pi = 4. * circle_points / square_points;
    } 
    println!("Final Estimation of Pi = {pi}")
    
}
