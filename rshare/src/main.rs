mod other;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::other::state::AppState;
use clap::Parser;
use std::fs::{create_dir_all, remove_dir_all};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: Option<Vec<PathBuf>>,
    #[arg(long, short = 'c', help = "Clean up share directory")]
    clean: bool,
}

fn app_dir() -> PathBuf {
    env::var_os("RSHARE_APP_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/opt/rshare"))
}

fn clean(app_dir: &Path) {
    println!("Cleaning up...");
    let shared_dir = app_dir.join("shared");
    match remove_dir_all(&shared_dir) {
        Ok(_) => println!("Successfully clean /shared"),
        Err(e) => println!("Something went wrong: {}", e),
    };

    create_dir_all(&shared_dir).expect("Can not create a shared directory");
}

fn share_paths(app_dir: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let share_path = app_dir.join("shared");
    let dir = fs::read_dir(&share_path).expect("Can not read share directory");
    for path in dir {
        let path = path.unwrap().path();
        paths.push(path);
    }
    paths
}
fn main() {
    let app_dir = app_dir();
    let shared_dir = app_dir.join("shared");
    if !shared_dir.exists() {
        let _ = create_dir_all(&shared_dir);
    }
    let args = Args::parse();
    if args.clean {
        clean(&app_dir)
    }

    let files = args.path.unwrap_or(Vec::new());
    let mut state = AppState { app_dir, files };
    let share_paths = share_paths(&state.app_dir);
    for path in share_paths {
        state.files.push(path);
    }
    other::srv::run_srv(state);
}
