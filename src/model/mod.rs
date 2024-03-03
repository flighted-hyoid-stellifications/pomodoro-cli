use clap::Parser;

#[derive(Parser, Debug)]
struct Cli
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