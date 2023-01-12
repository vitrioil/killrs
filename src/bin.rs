use clap::Parser;
use tracing::info;

use killrs::resource;
use killrs::tui;

/// Kill process if a system resource crosses threshold
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// TUI mode
    #[command(subcommand)]
    mode: Option<Mode>,

}

#[derive(clap::Subcommand, Debug)]
enum Mode {
    TUI, CLI {
    /// Process ID to kill
    #[arg(short, long)]
    pid: i32,

    /// Aggression level to end the process
    ///
    /// Level escalates after a signal is sent
    #[arg(short, long, default_value = "interrupt")]
    aggression: resource::Aggression,

    /// System property to monitor
    #[arg(value_enum)]
    resource: resource::ResourceOptions,

    /// Decide if threshold crossed above or below should kill a process
    ///
    /// By default, process is killed if resource value is more than threshold
    #[arg(short, long, default_value_t = false)]
    lower_threshold: bool,

    /// Resource threshold
    #[arg(short, long)]
    threshold: i32,
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    match &args.mode  {
        Some(Mode::TUI) => {
            let _ = tui::killrs_tui();
        }
        Some(Mode::CLI { lower_threshold, pid, aggression, resource, threshold }) => {
            let invert = if *lower_threshold { -1 } else { 1 };
            let mut resource = resource::Resource::new(
                *pid,
                resource::Aggression::Kill,
                resource::ResourceOptions::SysMem,
                invert,
                *threshold,
            );
            resource.killrs();
        }
        None => {
            panic!("");
        }
    }
    // info!("Started monitoring PID {}", resource.pid());
    // if false {
    //     let _ = tui::killrs_tui(&mut resource);
    // } else {
    //     resource.killrs();
    // }
    // info!("Ending monitoring PID {}", resource.pid());
    // info!("PID {} not alive", resource.pid());
}
