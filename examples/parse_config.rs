/// Example: Parse a deck configuration file
///
/// Usage: cargo run --example parse_config [config_file]

use deck::DeckConfig;
use std::{env, fs};

fn main() {
    // Get config file path from command line or use default
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "examples/simple_config.json".to_string());

    // Read the example configuration
    let config_json = fs::read_to_string(&config_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read config file '{}': {}", config_path, e);
            std::process::exit(1);
        });

    // Parse the JSON into our DeckConfig type
    match serde_json::from_str::<DeckConfig>(&config_json) {
        Ok(config) => {
            println!("✓ Successfully parsed configuration!");
            println!("\nRoutes defined: {}", config.routes.len());

            for (i, route) in config.routes.iter().enumerate() {
                println!("\nRoute {}:", i + 1);
                println!("  Path: {}", route.path);
                println!("  Method: {:?}", route.method);
                println!("  Pipeline steps: {}", route.pipeline.len());

                for (j, step) in route.pipeline.iter().enumerate() {
                    println!("    Step {}: {:?}", j + 1, step.name.as_deref().unwrap_or("<unnamed>"));
                }
            }

            // Pretty-print the parsed structure
            println!("\n--- Parsed Configuration ---");
            println!("{:#?}", config);
        }
        Err(e) => {
            eprintln!("✗ Failed to parse configuration:");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
