//! # Introduction
//!
//! Mini rust projects that target specific features of rust

pub mod progbar;

use clap::{Args, Subcommand};
use progbar::ProgBarExt;

#[derive(Debug, Args)]
#[command(flatten_help = true, subcommand_required = true)]
pub struct MiniArgs {
    #[command(subcommand)]
    command: MiniCommands,
}

#[derive(Clone, Subcommand, Debug)]
#[command(arg_required_else_help = true)]
enum MiniCommands {
    /// Print a progress bar to an iterator.
    Progbar {
        #[command(subcommand)]
        command: ProgBarCommands,
    },
}

#[derive(Clone, Subcommand, Debug)]
#[command(arg_required_else_help = true)]
enum ProgBarCommands {
    UnBounded {
        #[arg(short, long, default_value_t = String::from("#"))]
        bar_style: String,
    },
    Bounded {
        #[arg(long, default_value_t = String::from("["))]
        delim_start: String,

        #[arg( long, default_value_t = String::from("]"))]
        delim_close: String,

        #[arg(long, default_value_t = String::from("#"))]
        bar_style: String,

        #[arg(short, long, default_value_t = 50)]
        iter_count: u8,
    },
}

impl MiniArgs {
    pub fn run(self) {
        match self.command {
            MiniCommands::Progbar { command } => {
                use std::thread::sleep;
                use std::time::Duration;

                match command {
                    ProgBarCommands::UnBounded { bar_style } => {
                        // test run UnBounded
                        for _ in (0..).progbar().bar_style(bar_style) {
                            sleep(Duration::from_millis(50))
                        }
                    }
                    ProgBarCommands::Bounded {
                        delim_start,
                        delim_close,
                        bar_style,
                        iter_count,
                    } => {
                        // test run Bounded
                        for _ in (0..iter_count)
                            .progbar()
                            .bar_style(bar_style)
                            .with_bounds(delim_start, delim_close)
                        {
                            sleep(Duration::from_millis(50))
                        }
                    }
                }
            }
        }
    }
}
