use poise::Command;
use crate::Data;
use crate::events::error::Error;
use crate::commands::ping::hello;

pub fn load_commands() -> Vec<Command<Data, Error>> {
    vec![
        hello()
    ]
}