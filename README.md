# Wealth Distribution Simulation

This project simulates the evolution of wealth and education in a population of agents over time. It models inheritance, education, interactions, and demographic processes, and provides tools for visualizing the resulting distributions.

## Features

- **Agent-based simulation** of wealth, education, and demographic changes.
- **Inheritance mechanism**: Wealth is passed to children upon death.
- **Education growth**: Agents increase their education over time, affecting their income.
- **Interactions**: Agents interact and transact wealth based on proximity and probability.
- **Demographic realism**: Agents have parents, children, and age-dependent death probability.
- **Metrics logging**: Wealth and education statistics are logged at each iteration.
- **Visualization**: Jupyter notebook for plotting Gini coefficient, wealth/education percentiles, and more.

## Getting Started

### Prerequisites

- Rust (edition 2021)
- Python 3 with `matplotlib` and `pandas` for visualization

### Build & Run

1. **Build the simulation:**

    ```sh
    cargo build --release
    ```

2. **Run the simulation:**

    ```sh
    cargo run --release
    ```

    By default, configuration is loaded from `config/default.json`. You can provide a custom config file as an argument.

    ```sh
    cargo run --release -- path/to/your/config.json
    ```

    If you want to run a simulation without a visualisation window, provide an environment variable `VISUALISE = false`:

    ```sh
    VISUALISE=false cargo run --release -- path/to/your/config.json
    ```

3. **View results:**cargo run --release
    - Simulation metrics are saved to `visualisation/metrics.csv`.
    - Open `visualisation/visualisation.ipynb` in Jupyter to plot and analyze results.

## Configuration

Simulation parameters are set in `config/default.json`, including:

- Number of agents
- World size
- Interaction parameters
- Tax rate
- Age and death curve parameters
- Education and wealth parameters

Example:

```json
{
  "num_iterations": 5000,
  "num_agents": 1000,
  "length": 1000,
  "width": 1000,
  "interaction_radius": 50.0,
  "max_movement": 15.0,
  "age_and_death": {
    "mean_age": 30.0,
    "stddev_age": 10.0,
    "mid_age": 80.0,
    "max_start_age": 90.0,
    "steepness": 0.02
  },
  "education": {
    "initial_adult_min": 4.0,
    "initial_adult_max": 10.0,
    "elemental_education_threshold": 4.0,
    "children_education_jitter": 2.0,
    "learning_rate_min": 0.005,
    "learning_rate_max": 0.05,
    "max": 10.0
  },
  "income_and_consumption": {
    "income_age_parameter": 0.05,
    "income_education_parameter": 2.0,
    "base_consumption": 10.0,
    "aditional_consumption_rate": 0.2
  },
  "transaction": {
    "transaction_probability": 0.3,
    "education_parameter": 1.0,
    "age_parameter": 0.001,
    "tax_rate": 0.05,
    "amount_rate": 0.05
  },
  "wealth": {
    "min_initial_wealth": 10.0,
    "max_initial_wealth": 100.0,
    "min_inheritance_at_birth_rate": 0.1,
    "max_inheritance_at_birth_rate": 0.3
  }
}
```

## Visualization

The `visualisation/visualisation.ipynb` notebook provides plots for:

- Gini coefficient over time
- Wealth percentiles over time
- Total wealth and adult agent count
- Education mean and percentiles
- Death probability curve

## File Structure

- `src/agent.rs` - Agent definition and behavior
- `src/environment.rs` - Simulation environment and logic
- `src/metrics.rs` - Logging of simulation metrics (wealth and education)
- `src/environment_config.rs` - Configuration structs and loading
- `visualisation/metrics.csv` - Output metrics for plotting
- `visualisation/visualisation.ipynb` - Jupyter notebook for analysis

## License

MIT License

---

**Authors:**  
Kacper Cienkosz - [GitHub](https://github.com/kacienk)  
Miłosz Dubiel - [GitHub](https://github.com/dubielel)
