use clap::Parser;
use console::Key;

#[derive(Parser, Debug)]
pub struct Cli
{
    #[arg(short, long, default_value_t = 25)]
    pub work_in_minutes: u32,
    #[arg(short, long, default_value_t = 5)]
    pub break_in_minutes: u32,
    #[arg(short, long, default_value_t = 4)]
    pub sessions_per_cycle: u32,
    #[arg(short, long, default_value_t = 2)]
    pub cycles: u32,
}

#[derive(Debug)]
pub enum Signal {
    Pause
}

impl Signal {
 pub fn get_signal(character: &Key) -> Option<Signal> {
    match character {
        Key::Char(' ') => Some(Signal::Pause),
        _ => None
    }
 }


}