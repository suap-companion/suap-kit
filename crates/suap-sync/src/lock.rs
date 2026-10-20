//! The single-instance lock that keeps two synchronization runs from overlapping.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use crate::SyncError;

/// A lock older than this is considered abandoned by a crashed run.
const STALE_LOCK: Duration = Duration::from_secs(10 * 60);

/// Keeps two synchronization runs from overlapping; released when dropped.
#[derive(Debug)]
pub struct SyncLock {
    path: PathBuf,
}

impl SyncLock {
    /// Takes the lock at `path`; `Ok(None)` if another run holds it.
    pub fn acquire(path: &Path) -> Result<Option<Self>, SyncError> {
        fs::create_dir_all(path.parent().unwrap_or(Path::new(".")))?;
        if is_stale(path) {
            fs::remove_file(path)?;
        }
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
        {
            Ok(_) => Ok(Some(Self {
                path: path.to_path_buf(),
            })),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(None),
            Err(error) => Err(error.into()),
        }
    }
}

impl Drop for SyncLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn is_stale(path: &Path) -> bool {
    let age = fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| SystemTime::now().duration_since(modified).ok());
    age.is_some_and(|age| age > STALE_LOCK)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn the_lock_admits_one_run_at_a_time_and_recovers_from_stale_files() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("sub").join("sync.lock");
        let first = SyncLock::acquire(&path)
            .unwrap()
            .expect("first run gets the lock");
        assert!(SyncLock::acquire(&path).unwrap().is_none());
        drop(first);
        assert!(!path.exists());
        let again = SyncLock::acquire(&path).unwrap();
        assert!(again.is_some());
        drop(again);

        // A lock left by a crashed run, long ago, is taken over.
        fs::write(&path, "").unwrap();
        let old = SystemTime::now() - Duration::from_secs(3600);
        fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_modified(old)
            .unwrap();
        assert!(SyncLock::acquire(&path).unwrap().is_some());
        // A file name the system refuses reports the I/O error; a parent that is a file does too.
        assert!(SyncLock::acquire(&directory.path().join("x".repeat(300))).is_err());
        let blocker = directory.path().join("arquivo");
        fs::write(&blocker, "").unwrap();
        assert!(SyncLock::acquire(&blocker.join("sync.lock")).is_err());
    }
}
