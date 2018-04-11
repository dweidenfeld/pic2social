use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use self::errors::ResultExt;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

pub mod errors {
    use std::path::PathBuf;

    error_chain! {
        errors {
            WatchStartError(path: PathBuf) {
                description("Starting watch on fs failed")
                display("Starting watch on fs failed for path: {}", path.display())
            }

            Abort {
                description("The file watching will stop")
                display("The file watching should be aborted by internal decision")
            }
        }
    }
}

/// Watch for directory and get exectued whenever a file is created.
/// The callback is executed everytime a file has been created
/// ```
/// watch(PathBuf::from("pic/"), |file| {
///     println!("File created: {}", file.display());
/// })
/// ```
pub fn watch<F: Fn(PathBuf) -> errors::Result<()>>(directory: &PathBuf, cb: F) -> errors::Result<()> {
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))
        .chain_err(|| errors::ErrorKind::WatchStartError(directory.to_path_buf()))?;
    watcher.watch(directory, RecursiveMode::Recursive)
        .chain_err(|| errors::ErrorKind::WatchStartError(directory.to_path_buf()))?;

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Create(path)) => match cb(path) {
                Ok(_) => (),
                Err(e) => {
                    println!("Got an error: {}", e);
                    return Err(e);
                }
            },
            Ok(_) => (),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::thread;
    use std::fs::File;

    #[test]
    pub fn watch_while_creating_some_file() {
        let tmp_dir = env::temp_dir();
        let mut tmp_file = tmp_dir.clone();
        tmp_file.push("foo.txt");
        let check_file = tmp_file.clone();

        thread::spawn(move || {
            let watch_result =  super::watch(&tmp_dir, |path| {
                assert_eq!(tmp_file.as_path(), &path);

                Err(super::errors::ErrorKind::Abort)?
            });

            if watch_result.is_ok() {
                panic!("The watching should abort");
            }
        });

        File::create(check_file).unwrap();
    }
}