use clap::Parser;
use std::{env, path::PathBuf};

#[derive(Parser)]
#[clap(bin_name = "nn")]
pub struct Args {
    #[clap(parse(from_os_str))]
    path: Option<PathBuf>,

    #[clap(long, short = 'D')]
    pub remove_lock: bool,
}

const NODE_MODULES: &str = "node_modules";

const LOCK_FILE_NAMES: &[&str] = &["package-lock.json", "yarn.lock", "pnpm-lock.yaml"];

impl Args {
    fn get_path(&self) -> PathBuf {
        let cwd = env::current_dir().expect("Couldn't get CWD for some reason");
        self.path.clone().unwrap_or(cwd)
    }

    pub fn get_node_modules_path(&self) -> Option<PathBuf> {
        let node_modules_path = self.get_path().join(NODE_MODULES);
        (node_modules_path.exists() && node_modules_path.is_dir()).then(|| node_modules_path)
    }

    pub fn get_lock_file_paths(&self) -> Vec<PathBuf> {
        LOCK_FILE_NAMES
            .iter()
            .filter_map(|file_nm| {
                let lock_file = self.get_path().join(file_nm);
                lock_file.exists().then(|| lock_file)
            })
            .collect::<Vec<PathBuf>>()
    }
}
