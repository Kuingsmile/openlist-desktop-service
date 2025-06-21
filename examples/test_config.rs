use anyhow::{Context, Result};
use std::{env, path::PathBuf};

// Get the configuration directory based on the platform
fn get_config_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let appdata = env::var("APPDATA")
            .or_else(|_| env::var("USERPROFILE").map(|p| format!("{}\\AppData\\Roaming", p)))
            .context("Could not determine Windows config directory")?;
        Ok(PathBuf::from(appdata).join("OpenListService"))
    }

    #[cfg(target_os = "macos")]
    {
        let home = env::var("HOME").context("Could not determine home directory")?;
        Ok(PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("OpenListService"))
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
            Ok(PathBuf::from(xdg_config).join("openlist-service"))
        } else {
            let home = env::var("HOME").context("Could not determine home directory")?;
            Ok(PathBuf::from(home).join(".config").join("openlist-service"))
        }
    }
}

fn main() -> Result<()> {
    println!("Testing configuration directory path...");

    let config_dir = get_config_dir()?;
    println!("Configuration directory: {:?}", config_dir);

    let config_file = config_dir.join("process_configs.json");
    println!("Configuration file would be: {:?}", config_file);

    // Try to create the directory to test permissions
    match std::fs::create_dir_all(&config_dir) {
        Ok(_) => {
            println!("✓ Configuration directory can be created");

            // Test writing a sample configuration
            let sample_config = r#"[
  {
    "id": "test-id",
    "name": "Test Process",
    "bin_path": "C:\\Windows\\System32\\notepad.exe",
    "args": [],
    "log_file": "test.log",
    "working_dir": null,
    "env_vars": null,
    "auto_restart": false,
    "run_as_admin": false,
    "created_at": 1640995100,
    "updated_at": 1640995100
  }
]"#;

            match std::fs::write(&config_file, sample_config) {
                Ok(_) => {
                    println!("✓ Sample configuration file written successfully");

                    // Try to read it back
                    match std::fs::read_to_string(&config_file) {
                        Ok(content) => {
                            println!("✓ Configuration file can be read back");
                            println!("File content length: {} bytes", content.len());
                        }
                        Err(e) => println!("✗ Failed to read configuration file: {}", e),
                    }

                    // Clean up
                    let _ = std::fs::remove_file(&config_file);
                }
                Err(e) => println!("✗ Failed to write configuration file: {}", e),
            }
        }
        Err(e) => println!("✗ Failed to create configuration directory: {}", e),
    }

    Ok(())
}
