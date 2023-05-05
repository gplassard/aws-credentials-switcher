use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};


use dirs;
use env_logger::Builder;

use ini::Ini;
use log::info;
use structopt::StructOpt;

use cli::Cli;

use crate::cli::Command;

mod cli;

fn main() {
    let home = dirs::home_dir().expect("Can't get home directory");

    let cli = Cli::from_args();
    Builder::new().filter_level(cli.log_level).init();

    match cli.command {
        Command::Use { alternative } => {
            let aws = home.join(".aws");
            let alt_dir = home.join(".aws".to_owned() + &alternative);
            switch(aws, alt_dir)
        },
    }
}

#[derive(Debug)]
struct AWSCredentials {
    profile_name: Option<String>,
    access_key: String,
    secret_key: String,
}

fn switch(default: PathBuf, replace: PathBuf) -> () {
    info!("Retrieving credentials from {}", default.to_str().unwrap());
    let credentials = get_credentials(&default);
    if credentials.len() >= 1 {
        let profile_names: Vec<String> = credentials.iter().map(|c| c.profile_name.unwrap_or("<no name>".to_string())).collect();
        info!("Retrieved credentials for following profiles {}", profile_names.join(", "));
    }
    else {
        info!("No credentials were found");
    }

    info!("Copying {} to {}", replace.to_str().unwrap(), default.to_str().unwrap());
    replace_directory_with(default.as_path(), replace.as_path()).unwrap();

    restore_credentials(&default, credentials);
    info!("Success !");
}

fn get_credentials(path: &PathBuf) -> Vec<AWSCredentials> {
    path.join("credentials").to_str()
        .map(|p| Ini::load_from_file(p).expect("Ini parse error"))
        .into_iter()
        .flat_map(|ini| {
            ini.iter()
                .filter_map(|(name, props)| {
                    let access_key = props.get("aws_access_key_id");
                    let secret_key = props.get("aws_secret_access_key");

                    match (access_key, secret_key) {
                        (Some(a), Some(b)) => Some(AWSCredentials {
                            profile_name: name.map(|t| t.to_string()),
                            access_key: a.to_string(),
                            secret_key: b.to_string()
                        }),
                        _ => None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn restore_credentials(path: &PathBuf, creds: Vec<AWSCredentials>) -> Option<io::Result<()>> {
    let indexed_credentials: HashMap<String, AWSCredentials> = creds.into_iter().map(|c| (c.profile_name.unwrap_or("".to_string()), c)).collect();

    path.join("credentials").to_str()
        .map(|p| Ini::load_from_file(p).expect("Ini parse error"))
        .map(|mut credentials| {

            credentials.iter_mut()
                .filter(|(_, prop)| prop.contains_key("aws_access_key_id"))
                .for_each(|(section_name, prop) | {
                    if let Some(cred) = indexed_credentials.get(section_name.unwrap_or("")) {
                        info!("Setting access_key / secret_key for profile {:?}", section_name);
                        prop.insert("aws_access_key_id".to_string(), cred.access_key.clone());
                        prop.insert("aws_secret_access_key".to_string(), cred.secret_key.clone());
                    }
                    else {
                        info!("No credentials to set for profile {:?}", section_name);
                    }
                });

            credentials.write_to_file( path.join("credentials"))
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
