mod args;
mod error;

use std::{fs, io};

use args::Args;
use clap::Parser;

use indicatif::{ProgressBar, ProgressStyle};

const FRAMES: &[&str] = &[
    "🤜\u{3000}\u{3000}\u{3000}\u{3000}🤛 ",
    "🤜\u{3000}\u{3000}\u{3000}\u{3000}🤛 ",
    "🤜\u{3000}\u{3000}\u{3000}\u{3000}🤛 ",
    "\u{3000}🤜\u{3000}\u{3000}🤛\u{3000} ",
    "\u{3000}\u{3000}🤜🤛\u{3000}\u{3000} ",
    "\u{3000}🤜✨🤛\u{3000}\u{3000} ",
    "🤜\u{3000}✨\u{3000}🤛\u{3000} ",
];

fn main() -> io::Result<()> {
    let args = Args::parse();

    let node_modules_path = args.get_node_modules_path().expect("No node_modules here!");

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(120);
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .tick_strings(FRAMES),
    );

    let lock_files = args.get_lock_file_paths();

    if args.remove_lock {
        spinner.set_message("Crushing lock files 👊");
        if lock_files.iter().try_for_each(fs::remove_file).is_err() {
            spinner.set_message(
                "Couldn't crush all lock files. Delete the rest of them yourself, homie  🤷‍♂️",
            );
        }
    }

    spinner.set_message("Nuking your node_modules 💣");
    fs::remove_dir_all(node_modules_path)?;

    spinner.finish_with_message("Nuclear winter imminent ⛄");

    Ok(())
}
