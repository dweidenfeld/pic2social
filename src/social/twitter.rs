use egg_mode::{self, KeyPair, Token};
use egg_mode::media::{media_types, UploadBuilder};
use egg_mode::tweet::DraftTweet;
use std::io;
use std::cell::RefCell;
use std::path::Path;
use super::errors;
use super::errors::ResultExt;
use super::read_file;
use super::SocialUpload;
use super::super::Config;
use tokio_core::reactor;

/// Twitter is the Twitter social plugin implementation.
/// It will enable you to do posts against the Twitter API.
pub struct Twitter {
    core: RefCell<reactor::Core>,
    token: Token,
}

impl Twitter {
    /// Create a new Twitter social plugin instance.
    pub fn new(core: RefCell<reactor::Core>, config: &Config) -> errors::Result<Twitter> {
        let token = authorize(&core, config)?;

        Ok(Twitter { core, token })
    }
}

impl SocialUpload for Twitter {
    fn post(&self, text: &str, path: &Path) -> errors::Result<()> {
        let mut core = self.core.borrow_mut();

        let image = read_file(path)?;
        let handle = core.handle();

        let builder = UploadBuilder::new(image, media_types::image_jpg());
        let media_handle = core.run(builder.call(&self.token, &handle))
            .chain_err(|| errors::ErrorKind::UploadError(path.to_path_buf()))?;

        let draft = DraftTweet::new(text).media_ids(&[media_handle.id]);
        let _ = core.run(draft.send(&self.token, &handle))
            .chain_err(|| errors::ErrorKind::PostError(path.to_path_buf()))?;

        Ok(())
    }
}

fn authorize(core: &RefCell<reactor::Core>, config: &Config) -> errors::Result<Token> {
    let consumer_key = match &config.consumer_key {
        Some(value) => value.clone(),
        None => return Err(errors::ErrorKind::MissingConfigurationError("consumer_key".to_string()))?
    };

    let consumer_secret = match &config.consumer_secret {
        Some(value) => value.clone(),
        None => return Err(errors::ErrorKind::MissingConfigurationError("consumer_secret".to_string()))?
    };

    if config.access_token.is_none() && config.access_token_secret.is_none() {
        let mut mut_core = core.borrow_mut();
        let handle = mut_core.handle();

        let con_token = KeyPair::new(consumer_key.to_string(), consumer_secret.to_string());
        let request_token = mut_core.run(egg_mode::request_token(&con_token, "oob", &handle))
            .chain_err(|| errors::ErrorKind::AuthorizationError)?;
        println!("Go to the following URL, sign in, and give me the PIN that comes back:");
        println!("{}", egg_mode::authorize_url(&request_token));

        let mut pin = String::new();
        io::stdin().read_line(&mut pin)
            .chain_err(|| errors::ErrorKind::AuthorizationError)?;
        println!();

        let tok_result = mut_core.run(egg_mode::access_token(con_token, &request_token, pin, &handle))
            .chain_err(|| errors::ErrorKind::AuthorizationError)?;
        let token = tok_result.0;

        mut_core.run(egg_mode::verify_tokens(&token, &handle))
            .chain_err(|| errors::ErrorKind::AuthorizationError)?;

        return Ok(token);
    }

    Ok(Token::Access {
        consumer: KeyPair::new(consumer_key, consumer_secret),
        access: KeyPair::new(config.access_token.clone().unwrap(), config.access_token_secret.clone().unwrap()),
    })
}

#[cfg(test)]
mod tests {
    use dotenv;
    use std::env;
    use std::path::PathBuf;
    use std::cell::RefCell;
    use tokio_core::reactor;
    use egg_mode::Token;

    #[test]
    pub fn authorize_with_all_tokens() {
        let _ = dotenv::dotenv();
        let core = RefCell::new(reactor::Core::new().unwrap());

        let consumer_key = env::var("P2S_CONSUMER_KEY").ok();
        let consumer_secret = env::var("P2S_CONSUMER_SECRET").ok();
        let access_token = env::var("P2S_ACCESS_TOKEN").ok();
        let access_token_secret = env::var("P2S_ACCESS_TOKEN_SECRET").ok();

        let config = super::Config {
            plugin: "twitter".to_string(),
            message: "".to_string(),
            directory: PathBuf::from(""),
            consumer_key: consumer_key.clone(),
            consumer_secret: consumer_secret.clone(),
            access_token: access_token.clone(),
            access_token_secret: access_token_secret.clone()
        };

        let token = super::authorize(&core, &config).unwrap();
        match token {
            Token::Access{consumer, access} => {
                assert_eq!(consumer_key.unwrap(), consumer.key);
                assert_eq!(consumer_secret.unwrap(), consumer.secret);
                assert_eq!(access_token.unwrap(), access.key);
                assert_eq!(access_token_secret.unwrap(), access.secret);
            },
            _ => panic!("Expected access token")
        }
    }

    #[test]
    pub fn authorize_with_missing_tokens() {
        let core = RefCell::new(reactor::Core::new().unwrap());

        let config = super::Config {
            plugin: "twitter".to_string(),
            message: "".to_string(),
            directory: PathBuf::from(""),
            consumer_key: None,
            consumer_secret: None,
            access_token: None,
            access_token_secret: None
        };

        if super::authorize(&core, &config).is_ok() {
            panic!("Expected authorization to fail, because of missing tokens");
        }
    }
}