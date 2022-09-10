use crate::uni::Uni;

pub fn initroad(road: &mut [u32], n: usize, density: f32, seed: u32) -> u32 {
    // seed random number generator
    let mut uni = Uni::new(seed);

    let mut rng: f32;
    let mut ncar = 0;

    for i in 0..=n {
        rng = uni.uni();

        if rng < density {
            road[i] = 1;
        } else {
            road[i] = 0;
        }

        ncar += road[i];
    }

    return ncar;
}

pub fn updateroad(newroad: &mut [u32], oldroad: &mut [u32], n: usize) -> u32 {
    let mut nmove = 0;

    for i in 1..n {
        if oldroad[i] == 1 {
            if oldroad[i + 1] == 1 {
                newroad[i] = 1;
            } else {
                newroad[i] = 0;
                nmove += 1;
            }
        } else {
            if oldroad[i - 1] == 1 {
                newroad[i] = 1;
            } else {
                newroad[i] = 0;
            }
        }
    }

    nmove
}

pub fn updatebcs(road: &mut [u32], n: usize) {
    road[0] = road[n];
    road[n + 1] = road[1];
}
