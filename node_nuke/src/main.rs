use std::{fs, io, path::PathBuf};

use clap::Parser;

use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[clap(bin_name = "nn")]
struct Args {
    #[clap(parse(from_os_str))]
    path: Option<PathBuf>,

    #[clap(long, short = 'D')]
    remove_lock: bool,
}

const FRAMES: &[&str] = &[
    "🤜\u{3000}\u{3000}\u{3000}\u{3000}🤛 ",
    "🤜\u{3000}\u{3000}\u{3000}\u{3000}🤛 ",
    "🤜\u{3000}\u{3000}\u{3000}\u{3000}🤛 ",
    "\u{3000}🤜\u{3000}\u{3000}🤛\u{3000} ",
    "\u{3000}\u{3000}🤜🤛\u{3000}\u{3000} ",
    "\u{3000}🤜✨🤛\u{3000}\u{3000} ",
    "🤜\u{3000}✨\u{3000}🤛\u{3000} ",
];

const LOCK_FILE_NAMES: &[&str] = &["package-lock.json", "yarn.lock", "pnpm-lock.yaml"];

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Use cwd if no path provided
    let path = args.path.unwrap_or_else(|| PathBuf::from("."));

    let node_modules_path = path.join("node_modules");
    if !node_modules_path.exists() || !node_modules_path.is_dir() {
        panic!("No node_modules here!");
    }

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(120);
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .tick_strings(FRAMES),
    );

    let lock_files = LOCK_FILE_NAMES
        .iter()
        .filter_map(|file_nm| {
            let lock_file = path.join(file_nm);
            lock_file.exists().then(|| lock_file)
        })
        .collect::<Vec<PathBuf>>();

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
