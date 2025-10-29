// Imports
use std::{path::Path, fs::File, io::Write};
use async_std::fs;
use serde::{Serialize, Deserialize};
use serde_yaml;
use colored::{Colorize, ColoredString};

// Macros
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
        pub(crate) struct $name {
            $(pub $field: $t),*
        }
    };
}

// Templates
pub_struct!(DefaultConfig {
    server: ServerConfig,
    eula_acceptance: bool,
});

pub_struct!(ServerConfig {
    host: String,
    port: u16,
});

pub fn default_config() -> DefaultConfig {
    return DefaultConfig {
        server: ServerConfig { host: "localhost".to_string(), port: 0814 },
        eula_acceptance: false
    }
}

// Helpers
async fn generate_config(config_path: &Path) {
    let config_tag: ColoredString = "Config".green();
    println!("[{}] Generating Default Configuration...", config_tag);

    let comment: &'static str =r#"
# =================================================
# Oxide Configuration File
# Generated Dynamically - documentation available on github
# Edit carefully - changes take effect on restart.
# =================================================

"#;

    let yaml_str = serde_yaml::to_string(&default_config())
        .expect("Failed to serialize default config");
    let combo_str = format!("{}{}", comment, yaml_str);
    let mut file = File::create(config_path).expect("Failed to create config file");
    file.write_all(combo_str.as_bytes()).expect("Failed to write to config file");

    println!("[{}] Default Configuration Generated (@{}).", config_tag, config_path.display());
}

// Define Config
pub struct ConfigController {

}

impl ConfigController {
    pub async fn read_config() -> Result<(), Box<dyn std::error::Error>> {
        let config_tag: ColoredString = "Config".green();
        
        // Locate Configuration File
        let config_path = Path::new("config.yml");
        println!("[{}] Locating Configuration (@ {})...", config_tag, config_path.display());
        if !config_path.exists() {
            generate_config(&config_path).await;
        }

        // Load Configuration File
        println!("[{}] Loading Configuration...", config_tag);
        let file = fs::read_to_string(config_path)
            .await.expect("Failed to read config file");
        let _config: DefaultConfig = serde_yaml::from_str(&file)
            .expect("Failed to parse config file");

        

        Ok(())
    }
}