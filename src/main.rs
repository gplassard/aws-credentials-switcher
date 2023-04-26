use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

use dirs;
use env_logger::Builder;
use exitcode;
use ini::Ini;
use log::{error, info};
use structopt::StructOpt;

use cli::Cli;

use crate::cli::Command;

mod cli;

fn main() {
    let home = dirs::home_dir().expect("Can't get home directory");
    let aws = home.join(".aws");
    let aws_v1 = home.join(".aws.v1");
    let aws_v2 = home.join(".aws.v2");
    let aws_v3 = home.join(".aws.v3");

    let cli = Cli::from_args();
    Builder::new().filter_level(cli.log_level).init();

    if !aws.is_dir() || !aws_v1.is_dir() || !aws_v2.is_dir() || !aws_v3.is_dir() {
        error!("Can't find .aws, .aws.v1, .aws.v2, and .aws.v3 directories in users home");
        process::exit(exitcode::CONFIG);
    }

    match cli.command {
        Command::UseV1 => switch(aws, aws_v1),
        Command::UseV2 => switch(aws, aws_v2),
        Command::UseV3 => switch(aws, aws_v3)
    }
}

#[derive(Debug)]
struct AWSCredentials {
    access_key: String,
    secret_key: String,
}

fn switch(default: PathBuf, replace: PathBuf) -> () {
    info!("Retrieving credentials from {}", default.to_str().unwrap());
    let credentials = get_credentials();
    info!("Copying {} to {}", replace.to_str().unwrap(), default.to_str().unwrap());
    replace_directory_with(default.as_path(), replace.as_path()).unwrap();
    match credentials {
        Some(c) => {
            info!("Updating credentials");
            set_credentials(c);
        }
        None => {
            info!("No credentials were found, skipping credentials update");
        }
    }
    info!("Success !");
}

fn get_credentials() -> Option<AWSCredentials> {
    dirs::home_dir()
        .map(|home| home.as_path().join(".aws").join("credentials") )
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .map(|p| Ini::load_from_file(p).expect("Ini parse error"))
        .and_then( |credentials| {
            let access_key = credentials.get_from(Some("default"), "aws_access_key_id");
            let secret_key = credentials.get_from(Some("default"), "aws_secret_access_key");
            match (access_key, secret_key) {
                (Some(a), Some(b)) => Some(AWSCredentials {
                    access_key: a.to_string(),
                    secret_key: b.to_string()
                }),
                _ => None
            }
        } )
}

fn set_credentials(creds: AWSCredentials) -> Option<io::Result<()>> {
    dirs::home_dir()
        .map(|home| home.as_path().join(".aws").join("credentials"))
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .map(|p| Ini::load_from_file(p).expect("Ini parse error"))
        .map(|mut credentials| {

            credentials.iter_mut()
                .filter(|(_, prop)| prop.contains_key("aws_access_key_id"))
                .for_each(|(section_name, prop) | {
                    info!("Setting access_key / secret_key for profile {:?}", section_name);
                    prop.insert("aws_access_key_id".to_string(), creds.access_key.clone());
                    prop.insert("aws_secret_access_key".to_string(), creds.secret_key.clone());
                });

            credentials.write_to_file( dirs::home_dir().unwrap().as_path().join(".aws").join("credentials"))
        })
}

fn replace_directory_with(old: &Path, new: &Path) -> io::Result<()> {
    fs::remove_dir_all(old)
        .and_then(|_| copy_dir_all(new, old))
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
