mod social_upload;
pub mod twitter;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub use self::social_upload::SocialUpload;

pub mod errors {
    use std::path::PathBuf;

    error_chain! {
        errors {
            MissingConfigurationError(variable: String) {
                description("Missing configuration")
                display("Missing configuration for: {}", variable)
            }

            ImageReadError(path: PathBuf) {
                description("Could not read image file")
                display("Could not read image file from: {}", path.display())
            }

            UploadError(path: PathBuf) {
                description("Could not upload image")
                display("Could not upload image: {}", path.display())
            }

            PostError(path: PathBuf) {
                description("Could not publish post")
                display("Could not publish post for image: {}", path.display())
            }

            AuthorizationError {
                description("Authorization failed")
                display("Authorization failed")
            }
        }
    }
}
use self::errors::ResultExt;

#[derive(Eq, Ord, PartialOrd, PartialEq)]
pub enum Plugin {
    Unknown,
    Twitter,
}

impl From<String> for Plugin {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            _ => Plugin::Twitter,
        }
    }
}

fn read_file<F: AsRef<Path>>(path: F) -> errors::Result<Vec<u8>> {
    let mut file = File::open(path.as_ref())
        .chain_err(|| errors::ErrorKind::ImageReadError(path.as_ref().to_path_buf()))?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .chain_err(|| errors::ErrorKind::ImageReadError(path.as_ref().to_path_buf()))?;

    Ok(bytes)
}