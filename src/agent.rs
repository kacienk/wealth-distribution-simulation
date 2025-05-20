use rand::Rng;

#[derive(Debug)]
pub struct Agent {
    pub id: usize,
    pub wealth: f64,
    pub x: f64,
    pub y: f64,
    pub income: f64,
    pub education: f64, // 0.0 to 1.0
    pub age: u8,
    pub tax_paid: f64,
    pub benefits_received: f64,
}

impl Agent {
    pub fn new(id: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            wealth: rng.gen_range(50.0..200.0),
            x: rng.gen_range(0.0..100.0),
            y: rng.gen_range(0.0..100.0),
            income: rng.gen_range(10.0..30.0),
            education: rng.gen_range(0.0..0.5),
            age: 1,
            tax_paid: 0.0,
            benefits_received: 0.0,
        }
    }

    pub fn move_randomly(&mut self, max_distance: f64) {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f64::consts::TAU);
        let distance = rng.gen_range(0.0..max_distance);
        self.x += distance * angle.cos();
        self.y += distance * angle.sin();
    }

    /// Update income based on education and age
    pub fn update_income(&mut self) {
        let base_income = 20.0;
        self.income = base_income * (0.5 + 0.5 * self.education) * self.age as f64;
    }

    /// Apply tax to current income
    pub fn pay_tax(&mut self, tax_rate: f64) -> f64 {
        let tax = self.income * tax_rate;
        self.wealth -= tax;
        self.tax_paid += tax;
        tax
    }

    /// Receive benefits (e.g., healthcare or education support)
    pub fn receive_benefit(&mut self, amount: f64) {
        self.wealth += amount;
        self.benefits_received += amount;
    }

    /// Update education level
    pub fn improve_education(&mut self, effort: f64) {
        self.education += effort;
        if self.education > 1.0 {
            self.education = 1.0;
        }
    }

    /// Update age
    pub fn update_age(&mut self) {
        self.age += 1;
    }
}
