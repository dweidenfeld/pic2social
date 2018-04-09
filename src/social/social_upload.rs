use std::path::Path;
use super::errors;

pub trait SocialUpload {
    /// Publish a post with an image and a text to the given social media provider.
    /// All available providers should be listed on top of the documentation.
    fn post<I: AsRef<Path>>(&self, text: String, image: I) -> errors::Result<()>;
}