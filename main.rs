use std::f64;
use std::time::Instant;

fn main() {
    let mut n = 0; // Starting at 100^0 (gets 1 digit)
    loop {
        let start_time = Instant::now();
        let m1: f64 = f64::MIN_POSITIVE; // Mass of first (small) block
        let m2 = (100.0f64).powi(n) * m1; // mass of second (large) block
        let mut count: i128 = 0; // The number of times we have hit something

        let mut v1: f64 = 0.0; // velocity of the first (small) block
        let mut v2: f64 = -1.0; // velocity of second (big) block

        // these expressions are used in the math below but are constant in each loop
        let mass_sum = m1 + m2;
        let mass_diff_1 = m1 - m2;
        let mass_diff_2 = m2 - m1;

        let mass_constant_11 = mass_diff_1 / mass_sum;
        let mass_constant_12 = 2.0 * m2 / mass_sum;
        let mass_constant_21 = 2.0 * m1 / mass_sum;
        let mass_constant_22 = mass_diff_2 / mass_sum;

        // collision happens
        while v1 > v2 {
            let oldv1 = v1;
            v1 = mass_constant_11 * v1 + mass_constant_12 * v2;
            v2 = mass_constant_21 * oldv1 + mass_constant_22 * v2;
            // Useful link for the above two lines:
            // https://www.khanacademy.org/science/physics/linear-momentum/elastic-and-inelastic-collisions/a/what-are-elastic-and-inelastic-collisions

            count += 1;
            // the small block changes direction when colliding with the wall
            v1 = v1.abs();
        }
        // between every two block collisions there is a wall collision.
        count = count * 2 - 1;

        let end_time = start_time.elapsed();
        println!("n = {:2} took {:2}.{:09} seconds, count = {}", n, end_time.as_secs(), end_time.subsec_nanos(), count);
        n += 1;
    }
}
