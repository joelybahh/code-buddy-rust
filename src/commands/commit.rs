use crate::{cli::CommitArgs, config};

pub async fn run(args: CommitArgs) {
    println!("Command line arguments: {:?}", args);
    match config::load_config() {
        Ok(config) => {
            println!("API Key: {}", config.api_key);
            println!("LLM Model: {}", config.llm_parameters.model);
            println!("LLM Temperature: {}", config.llm_parameters.temperature);
            // Implement the commit logic here
        }
        Err(e) => eprintln!("Error loading config: {}", e),
    }
}
