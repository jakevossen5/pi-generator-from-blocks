fn main() {
    let m1: f64 = 1.0; // Mass of first (small) block
    let m: f64 = 100.0; // Mass of the second (big) block
    let n: f64 = 12.0;
    let m2 = m.powf(n);
    let mut count: i32 = 0;
    
    let mut v1: f64 = 0.0; // velocity of the first (small) block
    let mut v2: f64 = -5.0; // velocity of second (big) block


    let mut oldv1;
    let mut oldv2;
    
    // collision happens
    while (v1 < 0.0) || (v1.abs() > v2) {
        // do stuff
        oldv1 = v1.clone();
        oldv2 = v2.clone();
        v1 = ((m1 - m2) / (m1 + m2)) * oldv1 + (2.0 * m2)/(m1 + m2)*oldv2;
        v2 = (2.0 * m1)/(m1 + m2)* oldv1 + ((m2 - m1)/ (m1 + m2)) * oldv2;
        if v1 < 0.0 {
            v1 = v1 * -1.0; // bounces off wall
            count = count + 1;
        }
        //println!("v1 is {}", v1);
        //println!("v2 is {}", v2);
        count = count + 1;
        
    }

    println!("{}", count);

    // calculate new velocity
//    print!("The kinetic energy of block 1 is: ");
//    println!("{}", k1);
//    println!("The kinetic energy of block 2 is: {} ", k2);
    // m1*v1 = m2*v2

    
    println!("Hello world");
}
