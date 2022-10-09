use clap::Parser;

use killrs::{resource, monitor};

/// Stake process if a resource crosses a threshold.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// Process ID to stake.
    #[arg(short, long)]
    pid: i32,

    /// Aggression level to end the process.
    #[arg(short, long, default_value_t = 0)]
    aggression: i32,

    /// System property to monitor.
    #[arg(value_enum)]
    resource: resource::ResourceOptions,

    /// To stake a process above or below a threshold.
    #[arg(short, long, default_value_t = false)]
    lower_threshold: bool,

    /// Threshold
    #[arg(short, long)]
    threshold: i32,
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let invert = if args.lower_threshold { -1 } else { 1 };
    let mut resource = resource::Resource::new(args.pid);
    monitor::monitor(&mut resource, args.aggression, args.resource, invert, args.threshold);
}
