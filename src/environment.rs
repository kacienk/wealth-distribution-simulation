use crate::agent::Agent;
use crate::environment_config::{EnvironmentConfig, Wealth};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

pub struct Environment {
    pub agents: Vec<Agent>,
    pub iteration: usize,
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
    pub next_agent_id: usize,
    pub config: EnvironmentConfig,
}

impl Environment {
    pub fn new(config: &EnvironmentConfig) -> Self {
        let mut agents: Vec<Agent> = (0..config.num_agents)
            .map(|id| {
                Agent::new(
                    id,
                    0.0,
                    config.length as f64,
                    0.0,
                    config.width as f64,
                    &config.age_and_death,
                    &config.education,
                    &config.wealth,
                )
            })
            .collect();

        let mut rng = rand::thread_rng();
        let mut by_age: Vec<usize> = (0..agents.len()).collect();
        by_age.sort_by(|&a, &b| agents[a].age.cmp(&agents[b].age));

        for i in 0..agents.len() {
            let age = agents[i].age;

            let age_years = age as f64 / 12.0;
            let prob_two_parents =
                (1.0 - (age_years / (config.age_and_death.max_start_age - 30.0))).clamp(0.0, 1.0); // 1.0 at age 0, 0.0 at age 60
            let prob_one_parent = (1.0 - prob_two_parents)
                * (1.0 - 0.5 * (age_years / (config.age_and_death.max_start_age - 18.0)))
                    .clamp(0.0, 1.0)
                    .min(1.0 - prob_two_parents);
            let roll: f64 = rng.gen();
            let num_parents = if roll < prob_two_parents {
                2
            } else if roll < prob_two_parents + prob_one_parent {
                1
            } else {
                0
            };

            let possible_parents: Vec<usize> = by_age
                .iter()
                .filter(|&&idx| {
                    agents[idx].age + 18 * 12 <= age && agents[idx].age + 45 * 12 > age && idx != i
                })
                .cloned()
                .collect();
            if possible_parents.len() >= num_parents {
                let selected = possible_parents
                    .choose_multiple(&mut rng, num_parents)
                    .cloned()
                    .collect::<Vec<_>>();
                for parent_idx in &selected {
                    agents[*parent_idx].children.push(i);
                }
            } else {
                possible_parents
                    .iter()
                    .for_each(|&parent_idx| agents[parent_idx].children.push(i));
            }
        }

        Self {
            agents,
            iteration: 0,
            min_x: 0,
            max_x: config.length,
            min_y: 0,
            max_y: config.width,
            next_agent_id: config.num_agents,
            config: config.clone(),
        }
    }

    pub fn config(&self) -> EnvironmentConfig {
        self.config
    }

    pub fn step(&mut self) {
        let mut agents_with_parents: HashMap<usize, u32> = HashMap::new();
        for agent in self.agents.iter() {
            if !agent.children.is_empty() {
                for child_id in &agent.children {
                    *agents_with_parents.entry(*child_id).or_insert(0) += 1;
                }
            }
        }

        for agent in self.agents.iter_mut().filter(|a| a.alive) {
            agent.move_randomly(
                self.config.max_movement,
                self.min_x as f64,
                self.min_y as f64,
                self.max_x as f64,
                self.max_y as f64,
            );

            Environment::handle_learning(&self.config, agent);
            if !agent.is_adult() {
                continue;
            }
            Environment::handle_income_and_consumption(&self.config, agent);
        }

        self.handle_interactions();

        self.update_agents();
        self.agents.retain(|a| a.alive); // Remove dead agents
        self.iteration += 1;
    }

    fn handle_interactions(&mut self) {
        // Select agents that are eligible for interaction based on age and interaction probability
        let interaction_eligable_ids: Vec<usize> = self
            .agents
            .iter()
            .enumerate()
            .filter(|(_, a)| a.alive)
            .filter(|(_, a)| a.is_adult())
            .filter(|(_, _)| {
                rand::random::<f64>() < self.config.transaction.transaction_probability
            })
            .map(|(i, _)| i)
            .collect();

        let tax_rate = self.config.transaction.tax_rate;
        let amount_rate = self.config.transaction.amount_rate;

        for i in 0..interaction_eligable_ids.len() {
            for j in (i + 1)..interaction_eligable_ids.len() {
                let a_id = interaction_eligable_ids[i];
                let b_id = interaction_eligable_ids[j];
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

                if dist < self.config.interaction_radius {
                    let (winner, loser) = self.decide_transaction_by_id(a_id, b_id);
                    let amount = amount_rate * loser.wealth.min(winner.wealth);
                    winner.wealth += (1.0 - tax_rate) * amount;
                    loser.wealth -= amount;
                }
            }
        }
    }

    fn handle_learning(config: &EnvironmentConfig, agent: &mut Agent) {
        let learning_rate = rand::thread_rng()
            .gen_range(config.education.learning_rate_min..config.education.learning_rate_max);
        let max_education = config.education.max;
        if agent.education < max_education {
            agent.education += learning_rate * (1.0 - agent.education / max_education);
        }
    }

    fn handle_income_and_consumption(config: &EnvironmentConfig, agent: &mut Agent) {
        let baseline_consumption = config.income_and_consumption.base_consumption;
        let additional_consumption = config.income_and_consumption.aditional_consumption_rate;
        let income_age_parameter = config.income_and_consumption.income_age_parameter;
        let income_education_parameter = config.income_and_consumption.income_education_parameter;

        let consumption = baseline_consumption
            + additional_consumption * (agent.wealth - baseline_consumption).max(0.0);
        let income = agent.income(income_education_parameter, income_age_parameter);
        agent.wealth += income;
        agent.wealth -= consumption;
    }

    fn decide_transaction_by_id(&mut self, a_id: usize, b_id: usize) -> (&mut Agent, &mut Agent) {
        let age_parameter = self.config.transaction.age_parameter;
        let education_parameter = self.config.transaction.education_parameter;
        let (score_a, score_b);
        {
            let a = &self.agents[a_id];
            let b = &self.agents[b_id];
            score_a = education_parameter * a.education + age_parameter * a.age as f64;
            score_b = education_parameter * b.education + age_parameter * b.age as f64;
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
        let mut inheritance: HashMap<usize, f64> = HashMap::new();

        let min_inheritance_at_birth_rate = self.config.wealth.min_inheritance_at_birth_rate;
        let max_inheritance_at_birth_rate = self.config.wealth.max_inheritance_at_birth_rate;

        for i in 0..size {
            if self.agents[i].alive && self.agents[i].age_and_check_death() {
                self.resolve_inheritance(&mut inheritance, i);

                // Create offspring (new agent) after inheritance logic
                let id = self.next_agent_id;
                self.next_agent_id += 1;
                let (min_x, max_x, min_y, max_y) = self.bounds();
                let (p1, p2) = self.select_parents();
                let child = Environment::create_offspring(
                    p1,
                    p2,
                    id,
                    min_x,
                    max_x,
                    min_y,
                    max_y,
                    min_inheritance_at_birth_rate,
                    max_inheritance_at_birth_rate,
                );
                new_agents.push(child);
            }
        }

        for agent in self.agents.iter_mut() {
            if let Some(amount) = inheritance.get(&agent.id) {
                agent.wealth += amount;
            }
        }

        self.agents.extend(new_agents);
    }

    fn resolve_inheritance(
        &mut self,
        inheritance: &mut HashMap<usize, f64>,
        agent_id: usize,
    ) -> () {
        let agent = &mut self.agents[agent_id];
        let dead_agent_wealth = agent.wealth;
        let children_ids = agent.children.clone();
        let num_children = children_ids.len();

        if num_children > 0 {
            let share = dead_agent_wealth / num_children as f64;
            for child_id in children_ids {
                *inheritance.entry(child_id).or_insert(0.0) += share;
            }
        }

        agent.wealth = 0.0;
    }

    fn select_parents(&mut self) -> (&mut Agent, &mut Agent) {
        let mut rng = rand::thread_rng();
        let reproductive_indices: Vec<_> = self
            .agents
            .iter()
            .enumerate()
            .filter(|(_, a)| a.age > 20 * 12 && a.age < 60 * 12 && a.alive)
            .map(|(i, _)| i)
            .collect();

        let len = reproductive_indices.len();
        assert!(len >= 2, "Not enough agents to select parents");

        let idx1 = rng.gen_range(0..len);
        let mut idx2 = rng.gen_range(0..len);
        while idx2 == idx1 {
            idx2 = rng.gen_range(0..len);
        }

        let a_idx = reproductive_indices[idx1];
        let b_idx = reproductive_indices[idx2];

        let (first, second) = if a_idx < b_idx {
            let (left, right) = self.agents.split_at_mut(b_idx);
            (&mut left[a_idx], &mut right[0])
        } else {
            let (left, right) = self.agents.split_at_mut(a_idx);
            (&mut right[0], &mut left[b_idx])
        };
        (first, second)
    }

    fn create_offspring(
        p1: &mut Agent,
        p2: &mut Agent,
        id: usize,
        min_x: usize,
        max_x: usize,
        min_y: usize,
        max_y: usize,
        min_inheritance_at_birth_rate: f64,
        max_inheritance_at_birth_rate: f64,
    ) -> Agent {
        let mut rng = rand::thread_rng();
        let parent1_inheritance =
            p1.wealth * rng.gen_range(min_inheritance_at_birth_rate..max_inheritance_at_birth_rate);
        let parent2_inheritance =
            p2.wealth * rng.gen_range(min_inheritance_at_birth_rate..max_inheritance_at_birth_rate);
        let wealth = parent1_inheritance + parent2_inheritance;
        p1.wealth -= parent1_inheritance;
        p2.wealth -= parent2_inheritance;
        p1.children.push(id);
        p2.children.push(id);

        Agent {
            id,
            children: Vec::new(),
            x: rng.gen_range(min_x as f64..max_x as f64),
            y: rng.gen_range(min_y as f64..max_y as f64),
            wealth,
            education: 0.0,
            age: 0,
            alive: true,
            mid_age: p1.mid_age,
            steepness: p1.steepness,
        }
    }

    pub fn bounds(&self) -> (usize, usize, usize, usize) {
        (self.min_x, self.max_x, self.min_y, self.max_y)
    }
}
