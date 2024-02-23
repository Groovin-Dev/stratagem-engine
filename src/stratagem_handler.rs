use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use enigo::{Enigo, Key, KeyboardControllable};
use rand::Rng;
use serde::Deserialize;

use crate::config_handler::Config;

#[derive(Debug, Deserialize, Clone)]
pub enum Action {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Stratagem {
    pub name: String,
    pub sequence: Vec<Action>,
}

#[derive(Debug, Deserialize)]
struct StratagemsConfig {
    stratagems: Vec<Stratagem>,
}

pub struct StratagemHandler {
    pub stratagems: Vec<Stratagem>,
}

impl StratagemHandler {
    // Constructs a new StratagemHandler and loads the stratagems from the given file path
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: StratagemsConfig =
            toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Self {
            stratagems: config.stratagems,
        })
    }

    // Executes the stratagem with the given name
    pub fn execute(&self, name: &str, config: &Config) {
        if let Some(stratagem) = self.stratagems.iter().find(|s| s.name == name) {
            let mut enigo = Enigo::new();

            info!("Executing stratagem: {}", name);

            // Press the init key
            enigo.key_click(Key::Raw(config.initkey));

            debug!("Init key pressed: {}", config.initkey);

            for action in &stratagem.sequence {
                match action {
                    Action::Up => enigo.key_click(Key::Raw(config.up)),
                    Action::Down => enigo.key_click(Key::Raw(config.down)),
                    Action::Left => enigo.key_click(Key::Raw(config.left)),
                    Action::Right => enigo.key_click(Key::Raw(config.right)),
                }

                debug!("Action: {:?}", action);

                // Add a random delay to avoid macro detection. Idk what the exact times are but 100ms each was breaking so I added the random deviation
                let delay = StratagemHandler::random_deviation(100, 100);

                debug!("Delay: {}", delay);

                std::thread::sleep(std::time::Duration::from_millis(delay as u64));
            }
        } else {
            info!("Stratagem not found: {}", name);
        }
    }

    // random_deviation
    // Takes a number and a deviation and returns a random number within the deviation of the given number (if the number is 100 and the deviation is 100, the result will be between 0 and 200)
    pub fn random_deviation(num: u16, deviation: u16) -> u16 {
        let mut rng = rand::thread_rng();
        let deviation = rng.gen_range(0..deviation);
        let num = num as i16;
        let deviation = deviation as i16;
        let result = num + deviation - (deviation / 2);
        if result < 0 {
            0
        } else {
            result as u16
        }
    }
}
