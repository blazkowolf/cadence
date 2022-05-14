use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "Node Nuke")]
#[clap(bin_name = "nn", author, version, about, long_about = None)]
pub struct Args {
    /// Path to a directory containing a `node_modules` folder.
    /// Defaults to the current directory if not provided.
    #[clap(parse(from_os_str), default_value = ".")]
    pub path: PathBuf,
    /// Remove all associated lock files
    /// (e.g. `package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`).
    #[clap(long, short = 'D')]
    pub remove_lock: bool,
}

const NODE_MODULES: &str = "node_modules";

const LOCK_FILE_NAMES: &[&str] = &["package-lock.json", "yarn.lock", "pnpm-lock.yaml"];

impl Args {
    pub fn get_node_modules_path(&self) -> Option<PathBuf> {
        let node_modules_path = self.path.join(NODE_MODULES);
        (node_modules_path.exists() && node_modules_path.is_dir()).then(|| node_modules_path)
    }

    pub fn get_lock_file_paths(&self) -> Vec<PathBuf> {
        LOCK_FILE_NAMES
            .iter()
            .filter_map(|file_nm| {
                let lock_file = self.path.join(file_nm);
                lock_file.exists().then(|| lock_file)
            })
            .collect::<Vec<PathBuf>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_get_node_modules_path() {
        // Arrange
        let expected = None;
        let data = Args {
            path: PathBuf::from("/some/test/path"),
            remove_lock: true,
        };
        // Act
        let actual = data.get_node_modules_path();
        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn when_get_lock_file_paths() {
        // Arrange
        let expected: Vec<PathBuf> = Vec::new();
        let data = Args {
            path: PathBuf::from("/some/test/path"),
            remove_lock: true,
        };
        // Act
        let actual = data.get_lock_file_paths();
        // Assert
        assert_eq!(actual, expected);
    }
}
