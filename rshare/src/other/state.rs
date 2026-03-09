use std::path::PathBuf;

#[derive(Clone)]
pub struct AppState {
    pub app_dir: PathBuf,
    pub files: Vec<PathBuf>,
}
