use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rand_pcg::Pcg64; // Using PCG algorithm for good statistical properties
use std::io;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    // Simulation parameters
    let annual_contribution = 10000.0; // Victor contributes $10,000 per year
    let mean_return = 0.08; // 8% mean annual return
    let std_dev_return = 0.16; // 16% standard deviation (typical for stocks)
    let years = 30; // 30-year investment horizon
    let num_simulations = 10000; // Number of simulations to run
    let seed = 12345; // Fixed seed for reproducibility

    println!("Running Monte Carlo simulation for Victor's portfolio");
    println!("Parameters:");
    println!("  Annual Contribution: ${:.2}", annual_contribution);
    println!("  Mean Annual Return: {:.1}%", mean_return * 100.0);
    println!("  Standard Deviation: {:.1}%", std_dev_return * 100.0);
    println!("  Investment Horizon: {} years", years);
    println!("  Number of Simulations: {}", num_simulations);
    println!("  Random Seed: {}", seed);

    // Start timing
    let start = Instant::now();

    // Create seeded random number generator
    let mut rng = Pcg64::seed_from_u64(seed);

    // Create normal distribution
    let normal = Normal::new(mean_return, std_dev_return).unwrap();

    // Store final portfolio values from each simulation
    let mut final_values = Vec::with_capacity(num_simulations);

    // Run simulations
    for _ in 0..num_simulations {
        let mut portfolio_value = 0.0;

        for _ in 0..years {
            // Generate random annual return from normal distribution
            let annual_return = normal.sample(&mut rng);

            // Apply annual return to existing portfolio
            portfolio_value *= 1.0 + annual_return;

            // Add annual contribution
            portfolio_value += annual_contribution;
        }

        final_values.push(portfolio_value);
    }

    // Sort for percentile calculations
    final_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Calculate statistics
    let mean = final_values.iter().sum::<f64>() / final_values.len() as f64;

    // Median (50th percentile)
    let median = if final_values.len() % 2 == 0 {
        (final_values[final_values.len() / 2 - 1] + final_values[final_values.len() / 2]) / 2.0
    } else {
        final_values[final_values.len() / 2]
    };

    // Standard deviation
    let variance = final_values
        .iter()
        .map(|&v| (v - mean).powi(2))
        .sum::<f64>()
        / final_values.len() as f64;
    let std_dev = variance.sqrt();

    // Percentiles
    let p05 = final_values[(final_values.len() as f64 * 0.05) as usize];
    let p25 = final_values[(final_values.len() as f64 * 0.25) as usize];
    let p75 = final_values[(final_values.len() as f64 * 0.75) as usize];
    let p95 = final_values[(final_values.len() as f64 * 0.95) as usize];

    // Calculate elapsed time
    let duration = start.elapsed();

    // Print results
    println!("\nResults after {} years:", years);
    println!("  Mean portfolio value: ${:.2}", mean);
    println!("  Median portfolio value: ${:.2}", median);
    println!("  Standard deviation: ${:.2}", std_dev);
    println!("\nPercentiles:");
    println!("  5th  percentile: ${:.2} (worst case scenario)", p05);
    println!("  25th percentile: ${:.2}", p25);
    println!("  75th percentile: ${:.2}", p75);
    println!("  95th percentile: ${:.2} (best case scenario)", p95);
    println!("\nSimulation completed in {:.2?}", duration);

    // Prompt the user to press any key before exiting
    println!("\nPress any key to exit...");
    let _ = io::stdin().read(&mut [0u8]).unwrap(); // Read a single byte (any key press)
}
