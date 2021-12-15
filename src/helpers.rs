use path_clean::PathClean;
use std::path::{Path, PathBuf};

pub(crate) fn fixed_clean<P: AsRef<Path>>(path: P) -> PathBuf {
    // Workaround from https://github.com/danreeves/path-clean/issues/4 pending crate update
    PathBuf::from(path.as_ref().to_string_lossy().replace("\\", "/")).clean()
}
