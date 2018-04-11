#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
#![feature(external_doc)]
#![doc(include="../README.md")]

extern crate egg_mode;
extern crate dotenv;
extern crate tokio_core;
extern crate notify;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate error_chain;

pub mod social;
mod watchdir;

use std::cell::RefCell;

use social::SocialUpload;
use std::path::PathBuf;
use tokio_core::reactor;
use structopt::StructOpt;

/// The configuration object that holds all external configuration definitions
/// It will be automatically parsed using `StructOpt`.
#[derive(StructOpt, Debug)]
#[structopt(name = "pic2social", about = "A social media image uploader with active directory watching")]
pub struct Config {
    #[structopt(short = "d", long = "directory", parse(from_os_str), env = "P2S_DIRECTORY")]
    directory: PathBuf,

    #[structopt(short = "m", long = "message", env = "P2S_MESSAGE")]
    message: String,

    #[structopt(short = "p", long = "plugin", env = "P2S_PLUGIN", default_value = "twitter")]
    plugin: String,

    #[structopt(long = "consumer_key", env = "P2S_CONSUMER_KEY")]
    consumer_key: Option<String>,

    #[structopt(long = "consumer_secret", env = "P2S_CONSUMER_SECRET")]
    consumer_secret: Option<String>,

    #[structopt(long = "access_token", env = "P2S_ACCESS_TOKEN")]
    access_token: Option<String>,

    #[structopt(long = "access_token_secret", env = "P2S_ACCESS_TOKEN_SECRET")]
    access_token_secret: Option<String>,
}

fn main() {
    let _ = dotenv::dotenv();
    let conf = Config::from_args();

    let plugin = social::Plugin::from(conf.plugin.clone());

    let core = reactor::Core::new()
        .expect("could not initialize core");

    let upload = match plugin {
        social::Plugin::Twitter => social::twitter::Twitter::new(RefCell::new(core), &conf),
        _ => unreachable!()
    }.expect("Social plugin could not be initialized");

    println!("Watching for pictures in directory: {}", conf.directory.display());
    match watchdir::watch(&conf.directory, |path| {
        if path.ends_with(".jpg") {
            match upload.post(conf.message.clone(), path.as_path()) {
                Ok(_) => println!("Uploaded Image: {} ({})", conf.message, path.display()),
                Err(e) => println!("Error uploading image: {} ({}) {:?}", conf.message, path.display(), e)
            };
        }
    }) {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    };
}