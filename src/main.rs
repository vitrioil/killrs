use clap::Parser;

mod mem;
mod monitor;

/// Simple script to monitor ps memory
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the process
    /// #[arg(short, long)]
    /// name: String,

    /// Process ID
    #[arg(short, long, default_value_t = 1)]
    pid: i32,

    /// Aggression level
    #[arg(short, long, default_value_t = 0)]
    aggression: i32,

    /// System memory threshold (in %)
    #[arg(short, long, default_value_t = 70)]
    system_threshold: i32,

    /// Process memory threshold (in kB)
    #[arg(short = 't', long, default_value_t = 2000)]
    process_threshold: i32,
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    monitor::monitor(args.pid, args.aggression, args.system_threshold, args.process_threshold);
}
