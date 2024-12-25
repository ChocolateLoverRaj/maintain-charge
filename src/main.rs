use clap::Parser;
use maintain_charge::maintain_charge;
use std::time::Duration;

mod maintain_charge;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(default_value = "41")]
    min_percent: u8,
    #[clap(default_value = "59")]
    max_percent: u8,
    #[clap(value_parser = humantime::parse_duration, default_value = "15s")]
    check_frequency: Duration,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("Using options: {:#?}", cli);
    maintain_charge(cli.min_percent, cli.max_percent, cli.check_frequency)
}
