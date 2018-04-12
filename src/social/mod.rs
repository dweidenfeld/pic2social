mod social_upload;
pub mod twitter;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

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

            UnknownPluginError {
                description("Plugin type unknown")
                display("Please use one of the available plugin types")
            }
        }
    }
}
use self::errors::ResultExt;

#[derive(Eq, Ord, PartialOrd, PartialEq)]
pub enum Plugin {
    Twitter,
}

impl FromStr for Plugin {
    type Err = errors::ErrorKind;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s.to_lowercase().as_str() {
            "twitter" => Ok(Plugin::Twitter),
            _ => Err(errors::ErrorKind::UnknownPluginError)?
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