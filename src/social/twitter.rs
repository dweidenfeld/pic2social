use egg_mode::{self, KeyPair, Token};
use egg_mode::media::{media_types, UploadBuilder};
use egg_mode::tweet::DraftTweet;
use std::{env, io};
use std::cell::RefCell;
use std::path::Path;
use super::errors;
use super::errors::ResultExt;
use super::read_file;
use super::SocialUpload;
use tokio_core::reactor;

/// Twitter is the Twitter social plugin implementation.
/// It will enable you to do posts against the Twitter API.
pub struct Twitter {
    core: RefCell<reactor::Core>,
    token: Token,
}


impl Twitter {
    /// Create a new Twitter social plugin instance.
    pub fn new(core: RefCell<reactor::Core>) -> errors::Result<Twitter> {
        let consumer_key = env::var("TWITTER_CONSUMER_KEY")
            .chain_err(|| errors::ErrorKind::MissingEnvironmentVariableError("TWITTER_CONSUMER_KEY".to_string()))?;

        let consumer_secret = env::var("TWITTER_CONSUMER_SECRET")
            .chain_err(|| errors::ErrorKind::MissingEnvironmentVariableError("TWITTER_CONSUMER_SECRET".to_string()))?;

        let access_token = env::var("TWITTER_ACCESS_TOKEN").ok();
        let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET").ok();

        if access_token.is_none() && access_token_secret.is_none() {
            let token = authorize(&core, consumer_key, consumer_secret)?;

            return Ok(Twitter { core, token });
        }

        let token = Token::Access {
            consumer: KeyPair::new(consumer_key, consumer_secret),
            access: KeyPair::new(access_token.unwrap(), access_token_secret.unwrap()),
        };

        Ok(Twitter { core, token })
    }
}

impl SocialUpload for Twitter {
    fn post<I: AsRef<Path>>(&self, text: String, path: I) -> errors::Result<()> {
        let mut core = self.core.borrow_mut();

        let image = read_file(path.as_ref())?;
        let handle = core.handle();

        let builder = UploadBuilder::new(image, media_types::image_jpg());
        let media_handle = core.run(builder.call(&self.token, &handle))
            .chain_err(|| errors::ErrorKind::UploadError(path.as_ref().to_path_buf()))?;

        let draft = DraftTweet::new(text).media_ids(&[media_handle.id]);
        let _ = core.run(draft.send(&self.token, &handle))
            .chain_err(|| errors::ErrorKind::PostError(path.as_ref().to_path_buf()))?;

        Ok(())
    }
}

fn authorize(core: &RefCell<reactor::Core>, consumer_key: String, consumer_secret: String) -> errors::Result<Token> {
    let mut mut_core = core.borrow_mut();
    let handle = mut_core.handle();

    let con_token = KeyPair::new(consumer_key, consumer_secret);
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

    Ok(token)
}