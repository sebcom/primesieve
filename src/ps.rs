use bitvec::prelude::*;
use num_integer::Roots;

const MAX_SIZE_SIEVE: usize = 1000_000_000;

pub struct PrimeSieveSingle {
    sieve: Box<[bool]>,
}

impl PrimeSieveSingle {
    pub fn new() -> PrimeSieveSingle {
        let ps = Self {
            // work around to create boxed static array (slice) 
            sieve: vec![false; MAX_SIZE_SIEVE].into_boxed_slice(),
        };
        return ps;
    }

    pub fn calc(&mut self) {
        for n in 2..MAX_SIZE_SIEVE.nth_root(2) {
            if self.sieve[n]==false {
                let mut c = n;
                
                    println!("{}", c);

                while c < MAX_SIZE_SIEVE {
                    self.sieve[c]=true;
                    c += n;
                }
            }
        }
    }
}
