use std::f64;
fn main() {
    let mut n: f64 = 0.0; // Starting at 100^0 (gets 1 digit)
    loop {
        let m1: f64 = f64::MIN_POSITIVE; // Mass of first (small) block
        let m2 = (100.0f64).powf(n) * m1; // mass of second (large) block
        let mut count: i128 = 0; // The number of times we have hit something
        
        let mut v1: f64 = 0.0; // velocity of the first (small) block
        let mut v2: f64 = -1.0; // velocity of second (big) block


        let mut oldv1; // For duplication reasons
        
        // collision happens
        while (v1.abs() > v2) || (v1 < 0.0) {
            oldv1 = v1.clone();
            v1 = ((m1 - m2) / (m1 + m2)) * v1 + (2.0 * m2)/(m1 + m2)*v2;
            v2 = (2.0 * m1)/(m1 + m2)* oldv1 + ((m2 - m1)/ (m1 + m2)) * v2;
            // Useful link for the above two lines:
            // https://www.khanacademy.org/science/physics/linear-momentum/elastic-and-inelastic-collisions/a/what-are-elastic-and-inelastic-collisions
            if v1 < 0.0 {
                v1 = v1 * -1.0; // bounces off wall
                count = count + 1;
            }
            //println!("v1 is {}", v1);
            //println!("v2 is {}", v2);
            count = count + 1;
            
        }
        print!("for n = {} ", n);
        println!("count = {}", count);
        n = n + 1.0;
    }
}
