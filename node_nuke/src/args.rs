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
        self.path
            .clone()
            .unwrap_or_else(|| env::current_dir().expect("Couldn't get CWD for some reason"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_get_path() {
        // Arrange
        let expected = PathBuf::from("/some/test/path");
        let data = Args {
            path: Some(PathBuf::from("/some/test/path")),
            remove_lock: false,
        };
        // Act
        let actual = data.get_path();
        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn when_get_path_default() {
        // Arrange
        let expected = env::current_dir().expect("Couldn't get CWD for some reason");
        let data = Args {
            path: None,
            remove_lock: true,
        };
        // Act
        let actual = data.get_path();
        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn when_get_node_modules_path() {
        // Arrange
        let expected = env::current_dir().expect("Couldn't get CWD for some reason");
        let data = Args {
            path: None,
            remove_lock: true,
        };
        // Act
        let actual = data.get_path();
        // Assert
        assert_eq!(actual, expected);
    }
}
