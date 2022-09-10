pub struct Uni {
    uni_u: [f32; 98], /* Was U(97) in Fortran version -- too lazy to fix */
    uni_c: f32,
    uni_cd: f32,
    uni_cm: f32,
    uni_ui: usize,
    uni_uj: usize,
}

impl Uni {
    pub fn new(seed: u32) -> Self {
        let mut uni = Uni {
            uni_u: [0.0; 98],
            uni_c: 0.0,
            uni_cd: 0.0,
            uni_cm: 0.0,
            uni_ui: 0,
            uni_uj: 0,
        };
        uni.rinit(seed);
        uni
    }

    pub fn uni(&mut self) -> f32 {
        let mut luni: f32 = self.uni_u[self.uni_ui] - self.uni_u[self.uni_uj];
        if luni < 0.0 {
            luni += 1.0;
        }
        self.uni_u[self.uni_ui] = luni;
        self.uni_ui -= 1;
        if self.uni_ui == 0 {
            self.uni_ui = 97;
        }
        self.uni_uj -= 1;
        if self.uni_uj == 0 {
            self.uni_uj = 97;
        }
        self.uni_c -= self.uni_cd;
        if self.uni_c < 0.0 {
            self.uni_c += self.uni_cm;
        }
        luni -= self.uni_c;
        if luni < 0.0 {
            luni += 1.0;
        }
        luni
    }

    pub fn rinit(&mut self, ijkl: u32) {
        /* check ijkl is within range */
        if ijkl > 900000000 {
            println!("rinit: ijkl = {} -- out of range\n", ijkl);
            panic!();
        }

        /* decompose the long integer into the the equivalent four
         * integers for rstart. This should be a 1-1 mapping
         *	ijkl <--> (i, j, k, l)
         * though not quite all of the possible sets of (i, j, k, l)
         * can be produced.
         */

        let ij = ijkl / 30082;
        let kl = ijkl - (30082 * ij);

        let i = ((ij / 177) % 177) + 2;
        let j = (ij % 177) + 2;
        let k = ((kl / 169) % 178) + 1;
        let l = kl % 169;

        if i <= 0 || i > 178 {
            println!("rinit: i = {} -- out of range\n", i);
            panic!();
        }

        if j <= 0 || j > 178 {
            println!("rinit: j = {} -- out of range\n", j);
            panic!();
        }

        if k <= 0 || k > 178 {
            println!("rinit: k = {} -- out of range\n", k);
            panic!();
        }

        if l > 168 {
            println!("rinit: l = {} -- out of range\n", l);
            panic!();
        }

        if i == 1 && j == 1 && k == 1 {
            println!("rinit: 1 1 1 not allowed for 1st 3 seeds\n");
            panic!();
        }

        self.rstart(i, j, k, l);
    }

    pub fn rstart(&mut self, i1: u32, j1: u32, k1: u32, l1: u32) {
        let (mut i, mut j, mut k, mut l) = (i1, j1, k1, l1);

        for ii in 1..97 {
            let mut s = 0.0;
            let mut t = 0.5;
            for _jj in 1..24 {
                let m = ((i * j % 179) * k) % 179;
                i = j;
                j = k;
                k = m;
                l = (53 * l + 1) % 169;
                if l * m % 64 >= 32 {
                    s += t;
                }
                t *= 0.5;
            }
            self.uni_u[ii] = s;
        }
        self.uni_c = 362436.0 / 16777216.0;
        self.uni_cd = 7654321.0 / 16777216.0;
        self.uni_cm = 16777213.0 / 16777216.0;
        self.uni_ui = 97; /*  There is a bug in the original Fortran version */
        self.uni_uj = 33; /*  of UNI -- i and j should be SAVEd in UNI()     */
    }
}
