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
struct Config {
    #[structopt(short = "d", long = "directory", parse(from_os_str))]
    directory: PathBuf,

    #[structopt(short = "m", long = "message")]
    message: String,
}

fn main() {
    let conf = Config::from_args();
    let _ = dotenv::dotenv();

    let core = reactor::Core::new()
        .expect("could not initialize core");

    let upload = social::twitter::Twitter::new(RefCell::new(core))
        .expect("Twitter client could not be initialized");

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