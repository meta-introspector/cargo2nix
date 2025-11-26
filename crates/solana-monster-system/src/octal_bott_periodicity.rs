pub struct OctalBottPeriodicity {
    pub ko_sigma: i64, // KO(Σ^n)
    pub period: i64,   // 8-fold periodicity
}

impl OctalBottPeriodicity {
    pub fn new(n: i64) -> Self {
        Self {
            ko_sigma: n,
            period: 8,
        }
    }

    pub fn bott_periodicity(&self) -> i64 {
        self.ko_sigma % self.period
    }

    pub fn is_periodic(&self) -> bool {
        self.bott_periodicity() == 0
    }
}
