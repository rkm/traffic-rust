use std::time::Instant;

mod trafficlib;
mod uni;

fn main() {
    const NCELL: usize = 10240000;
    const SEED: u32 = 5743;

    let mut oldroad = Box::new([0; NCELL + 2]);
    let mut newroad = Box::new([0; NCELL + 2]);

    let maxiter: u32 = (1.024e9 / (NCELL as f64)).round() as u32;
    let printfreq: u32 = maxiter / 10;

    // Set target density of cars
    let density = 0.52;

    println!();
    println!("Length of road is {}", NCELL);
    println!("Number of iterations is {}", maxiter);
    println!("Target density of cars is {}", density);

    // Initialise road accordingly using random number generator

    for i in 1..NCELL {
        oldroad[i] = 0;
        newroad[i] = 0;
    }

    println!("Initialising road ...");

    let ncars = trafficlib::initroad(oldroad.as_mut_slice(), NCELL, density, SEED);

    println!("...done");

    println!(
        "Actual density of cars is {}\n",
        (ncars as f32) / (NCELL as f32)
    );

    let mut nmove: u32;

    let start = Instant::now();

    for iter in 1..maxiter {
        trafficlib::updatebcs(oldroad.as_mut_slice(), NCELL);

        nmove = trafficlib::updateroad(newroad.as_mut_slice(), oldroad.as_mut_slice(), NCELL);

        for i in 1..NCELL {
            oldroad[i] = newroad[i];
        }

        if iter % printfreq == 0 {
            println!(
                "At iteration {} average velocity is {}",
                iter,
                nmove as f32 / ncars as f32
            );
        }
    }

    let elapsed = start.elapsed();

    println!("\nFinished");
    println!("\nTime taken was  {} seconds", elapsed.as_secs_f64());
    println!(
        "Update rate was {} MCOPs\n",
        1e-6 * (NCELL as f64) * (maxiter as f64) / elapsed.as_secs_f64()
    );
}
