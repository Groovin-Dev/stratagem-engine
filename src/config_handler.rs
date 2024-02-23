use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_A, VK_D, VK_LCONTROL, VK_S, VK_W};

use crate::input_handler::prompt_for_key;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub initkey: u16,
    pub up: u16,
    pub down: u16,
    pub left: u16,
    pub right: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            initkey: VK_LCONTROL.0,
            up: VK_W.0,
            down: VK_S.0,
            left: VK_A.0,
            right: VK_D.0,
        }
    }
}

pub struct ConfigHandler {
    pub config: Config,
}

impl ConfigHandler {
    pub fn new(generate: bool) -> io::Result<Self> {
        // If generate is true, generate a new config file
        if generate {
            Self::generate_config();
            std::process::exit(0);
        }

        // If the config file does not exist, generate the default config
        if !PathBuf::from("config.toml").exists() {
            let config = Config::default();
            let toml = toml::to_string(&config).unwrap();
            let mut file = File::create("config.toml")?;
            file.write_all(toml.as_bytes())?;
        }

        // Load the config file
        let mut file = File::open("config.toml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config =
            toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Self { config })
    }

    fn generate_config() {
        println!("Generating new configuration. Please enter the requested keys:");

        let initkey = prompt_for_key("initkey (default LControl): ").0;
        let up = prompt_for_key("up (default W): ").0;
        let down = prompt_for_key("down (default S): ").0;
        let left = prompt_for_key("left (default A): ").0;
        let right = prompt_for_key("right (default D): ").0;

        let config = Config {
            initkey,
            up,
            down,
            left,
            right,
        };

        let toml = toml::to_string(&config).unwrap();
        let mut file = File::create("config.toml").unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }
}
