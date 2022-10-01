use clap::Parser;
mod mem;

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
}

fn main() {
    let args = Args::parse();
    mem::read_smap(args.pid);
}
