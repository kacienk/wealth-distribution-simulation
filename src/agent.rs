use rand::Rng;

#[derive(Debug)]
pub struct Agent {
    pub id: usize,
    pub wealth: f64,
    pub income: f64,
    pub education: f64, // 0.0 to 1.0
    pub health: f64,    // 0.0 to 1.0
    pub tax_paid: f64,
    pub benefits_received: f64,
}

impl Agent {
    pub fn new(id: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            wealth: rng.gen_range(50.0..200.0),
            income: rng.gen_range(10.0..30.0),
            education: rng.gen_range(0.0..0.5),
            health: rng.gen_range(0.5..1.0),
            tax_paid: 0.0,
            benefits_received: 0.0,
        }
    }

    /// Update income based on education and health
    pub fn update_income(&mut self) {
        let base_income = 20.0;
        self.income = base_income * (0.5 + 0.5 * self.education) * self.health;
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

    /// Update health level
    pub fn update_health(&mut self, public_fund_factor: f64) {
        let mut rng = rand::thread_rng();
        let health_risk = rng.gen::<f64>();

        if health_risk > self.health {
            self.health -= 0.1;
        }

        self.health += 0.05 * public_fund_factor;
        if self.health > 1.0 {
            self.health = 1.0;
        }
        if self.health < 0.0 {
            self.health = 0.0;
        }
    }
}
