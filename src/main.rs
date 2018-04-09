//! This crate is a image upload for social media portals.
//!
//! ```
//! pic2social 0.1.0
//! Dominik Weidenfeld <dominik@sh0k.de>
//! A social media image uploader, that watches a directory
//!
//! USAGE:
//!     pic2social --directory <directory> --message <message>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -d, --directory <directory>
//!     -m, --message <message>
//! ```
//!
//! You can run it with
//! ```
//! ./pic2social -d pic/ -m "#myHashTag"
//! ```
//!
//! Then it will watch the pic folder and everytime a picture is added
//! to the directory it will be uploaded with the provided message.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate egg_mode;
extern crate dotenv;
extern crate tokio_core;
extern crate notify;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate error_chain;

mod social;
mod watchdir;

use std::cell::RefCell;

use social::SocialUpload;
use std::path::PathBuf;
use tokio_core::reactor;
use structopt::StructOpt;

/// The configuration object that holds all external configuration definitions
/// It will be automatically parsed using `StructOpt`.
#[derive(StructOpt, Debug)]
#[structopt(name = "pic2social", about = "A social media image uploader, that watches a directory")]
struct Config {
    #[structopt(short = "d", long = "directory", parse(from_os_str))]
    directory: PathBuf,

    #[structopt(short = "m", long = "message")]
    message: String,
}

fn main() {
    let conf = Config::from_args();

    dotenv::dotenv()
        .expect("could not parse environment variables");

    let core = reactor::Core::new().ok()
        .expect("could not initialize core");

    let upload = social::twitter::Twitter::new(RefCell::new(core)).ok()
        .expect("Twitter client could not be initialized");

    println!("Watching for pictures in directory: {}", conf.directory.display());
    match watchdir::watch(&conf.directory, |path| {
        if path.ends_with(".jpg") {
            match upload.post(conf.message.clone(), path.as_path()) {
                Ok(_) => println!("Uploaded Image: {} ({})", conf.message, path.display()),
                Err(e) => println!("Error uploading image: {} ({}) {:?}", conf.message, path.display(), e)
         c   };
        }
    }) {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    };
}