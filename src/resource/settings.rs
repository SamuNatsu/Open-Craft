/* Settings module */
#![allow(dead_code)]

// Crates using
use serde::{ Deserialize, Serialize };
use std::{ fs, path::Path };
use toml;

// Structures
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub graphics: Graphics
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Graphics {
    pub resolution: [u32; 2]
}

// Implementations
impl Settings {
    // Read settings from file
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Settings, String> {
        // Read file
        match fs::read_to_string(&path) {
            // Deserialize contents
            Ok(contents)    => match toml::from_str(&contents) {
                Ok(ret) => Ok(ret),
                Err(_)  => Err("Fail to parse settings file".to_string())
            },

            // Fail to read
            Err(_)          => Err("Fail to read settings file".to_string())
        }
    }

    // Write settings to file
    pub fn dump<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        // Serialize contents
        match toml::to_string(self) {
            // Write to file
            Ok(contents)    => match fs::write(&path, &contents) {
                Ok(_)   => Ok(()),
                Err(_)  => Err("Fail to write settings file".to_string())
            },

            // Fail to dump
            Err(_)          => Err("Fail to dump settings".to_string())
        }
    }
}

