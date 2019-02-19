#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use env_logger::Builder;
use env_logger::Env;
use mcp23x17::Expander;
use mcp23x17::IoValue;
use reqwest::Client;
use std::error::Error;

type Result<T = ()> = std::result::Result<T, Box<Error>>;

#[derive(Deserialize)]
struct Config {
    expander_device: String,
    client: ClientSettings,
    input_pins: InputPinsSettings,
}

#[derive(Deserialize)]
struct ClientSettings {
    endpoint: String,
}

#[derive(Deserialize)]
struct InputPinsSettings {
    switches: Vec<u8>,
}

fn main() -> Result {
    // initialize logger w/ log level "info"
    Builder::from_env(Env::new().default_filter_or("info")).init();

    // read config and run noorton
    run(&read_config()?)
}

fn read_config() -> Result<Config> {
    let conf_str = std::fs::read_to_string("noorton.toml")?;
    toml::from_str(&conf_str).map_err(From::from)
}

fn run(conf: &Config) -> Result {
    let rest_client = Client::new();

    let expander = Expander::new(&conf.expander_device)?;

    let switches: Vec<_> = conf
        .input_pins
        .switches
        .iter()
        .map(|&pin| expander.input(pin))
        .collect();

    let mut last_vals = vec![IoValue::High; switches.len()];

    loop {
        for (last_val, switch) in last_vals.iter_mut().zip(switches.iter()) {
            let next_val = switch.read_value().unwrap();
            if let (IoValue::High, IoValue::Low) = (*last_val, next_val) {
                if let Err(err) = rest_client.post(&conf.client.endpoint).send() {
                    error!("HTTP request failed: {:?}", err);
                }
            }
            *last_val = next_val;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
