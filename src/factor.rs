use std::ops::Index;

pub struct Factor {
    factors: Vec<Vec<u32>>,
}

impl Factor {
    pub fn new() -> Factor {
        Factor { factors: Vec::new() }
    }

    pub fn compute(&mut self, n: u32) {
        while self.factors.len() <= n as usize {
            let m = self.factors.len() as u32;
            self.factors.push(
                (2..)
                    .take_while(|&k| k * k <= m)
                    .filter(|&k| m % k == 0)
                    .collect(),
            );
        }
    }
}

impl Index<u32> for Factor {
    type Output = Vec<u32>;
    fn index(&self, n: u32) -> &Vec<u32> {
        &self.factors[n as usize]
    }
}
