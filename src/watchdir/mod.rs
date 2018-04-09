use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

pub mod errors {
    use std::path::PathBuf;

    error_chain!{
        errors {
            WatchStartFailed(path: PathBuf) {
                description("Starting watch on fs failed")
                display("Starting watch on fs failed for path: {}", path.display())
            }
        }
    }
}
use self::errors::ResultExt;

/// Watch for directory and get exectued whenever a file is created.
/// The callback is executed everytime a file has been created
/// ```
/// watch(PathBuf::from("pic/"), |file| {
///     println!("File created: {}", file.display());
/// })
/// ```
pub fn watch<F: Fn(PathBuf)>(directory: &PathBuf, cb: F) -> errors::Result<()> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))
        .chain_err(|| errors::ErrorKind::WatchStartFailed(directory.to_path_buf()))?;
    watcher.watch(directory, RecursiveMode::Recursive)
        .chain_err(|| errors::ErrorKind::WatchStartFailed(directory.to_path_buf()))?;

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Create(path)) => cb(path),
            Ok(_) => (),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}