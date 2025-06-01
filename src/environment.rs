use crate::agent::Agent;
use crate::environment_config::EnvironmentConfig;
use rand::Rng;

pub struct Environment {
    pub agents: Vec<Agent>,
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
    pub interaction_radius: f64,
    pub max_movement: f64,
    pub tax_rate: f64,
}

impl Environment {
    pub fn new(size: usize, config: &EnvironmentConfig) -> Self {
        let agents = (0..size).map(Agent::new).collect();
        Self {
            agents,
            min_x: 0,
            max_x: config.length,
            min_y: 0,
            max_y: config.width,
            interaction_radius: config.interaction_radius,
            max_movement: config.max_movement,
            tax_rate: config.tax_rate,
        }
    }

    pub fn config(&self) -> EnvironmentConfig {
        EnvironmentConfig {
            length: self.max_x,
            width: self.max_y,
            interaction_radius: self.interaction_radius,
            max_movement: self.max_movement,
            tax_rate: self.tax_rate,
        }
    }

    pub fn step(&mut self) {
        for agent in self.agents.iter_mut().filter(|a| a.alive) {
            agent.move_randomly(
                self.max_movement,
                self.min_x as f64,
                self.min_y as f64,
                self.max_x as f64,
                self.max_y as f64,
            );
        }

        self.handle_interactions();
        self.update_agents();
    }

    fn handle_interactions(&mut self) {
        let alive_ids: Vec<usize> = self
            .agents
            .iter()
            .enumerate()
            .filter(|(_, a)| a.alive)
            .map(|(i, _)| i)
            .collect();

        for i in 0..alive_ids.len() {
            for j in (i + 1)..alive_ids.len() {
                let a_id = alive_ids[i];
                let b_id = alive_ids[j];
                let (ax, ay, bx, by);
                {
                    let a = &self.agents[a_id];
                    let b = &self.agents[b_id];
                    ax = a.x;
                    ay = a.y;
                    bx = b.x;
                    by = b.y;
                }
                let dist = ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt();
                if dist < self.interaction_radius {
                    let tax = self.tax_rate / 100.0;
                    let (winner, loser) = self.decide_transaction_by_id(a_id, b_id);
                    let amount = 0.05 * loser.wealth.min(winner.wealth);
                    let taxed = amount * tax;
                    winner.wealth += amount - taxed;
                    loser.wealth -= amount;
                }
            }
        }
    }

    fn decide_transaction_by_id(&mut self, a_id: usize, b_id: usize) -> (&mut Agent, &mut Agent) {
        let (score_a, score_b);
        {
            let a = &self.agents[a_id];
            let b = &self.agents[b_id];
            score_a = a.education + a.age as f64;
            score_b = b.education + b.age as f64;
        }
        if rand::random::<f64>() < score_a / (score_a + score_b) {
            self.get_pair_mut(a_id, b_id)
        } else {
            self.get_pair_mut(b_id, a_id)
        }
    }

    fn get_pair_mut(&mut self, winner_id: usize, loser_id: usize) -> (&mut Agent, &mut Agent) {
        let (left, right) = self.agents.split_at_mut(std::cmp::max(winner_id, loser_id));
        if winner_id < loser_id {
            (&mut left[winner_id], &mut right[0])
        } else {
            (&mut right[0], &mut left[loser_id])
        }
    }

    fn update_agents(&mut self) {
        let size = self.agents.len();
        let mut new_agents = Vec::new();

        for i in 0..size {
            if self.agents[i].alive && self.agents[i].age_and_check_death() {
                let (p1, p2) = self.select_parents();
                let child = self.create_offspring(p1, p2);
                new_agents.push(child);
            }
        }

        self.agents.extend(new_agents);
    }

    fn select_parents(&self) -> (&Agent, &Agent) {
        let mut rng = rand::thread_rng();
        let reproductive: Vec<_> = self
            .agents
            .iter()
            .filter(|a| a.age > 20 && a.age < 60 && a.alive)
            .collect();
        let p1 = reproductive[rng.gen_range(0..reproductive.len())];
        let p2 = reproductive[rng.gen_range(0..reproductive.len())];
        (p1, p2)
    }

    fn create_offspring(&self, p1: &Agent, p2: &Agent) -> Agent {
        let mut rng = rand::thread_rng();
        let id = self.agents.len();
        let wealth = 0.2 * p1.wealth + 0.2 * p2.wealth + rng.gen_range(0.0..10.0);
        Agent {
            id,
            x: rng.gen_range(0.0..100.0),
            y: rng.gen_range(0.0..100.0),
            wealth,
            education: 0.0,
            age: 0,
            alive: true,
        }
    }
}
