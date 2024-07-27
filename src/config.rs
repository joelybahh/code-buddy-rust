use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
    pub llm_parameters: LlmParameters,
}

#[derive(Deserialize)]
pub struct LlmParameters {
    pub model: String,
    pub temperature: f32,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(".codebuddy")?;
    let config: Config = serde_json::from_str(&config_content)?;
    Ok(config)
}
