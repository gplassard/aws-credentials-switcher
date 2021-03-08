use std::fs;
use std::env;
use std::path::Path;
use std::process;
use std::io;
use dirs;
use ini::Ini;
use exitcode;

fn main() {
    let home = dirs::home_dir().expect("Can't get home directory");
    let credentials = get_credentials().expect("Can't retrieve current credentials from aws config file");
    let aws = home.join(".aws");
    let aws_v1 = home.join(".aws.v1");
    let aws_v2 = home.join(".aws.v2");

    if !aws.is_dir() || !aws_v1.is_dir() || !aws_v2.is_dir() {
        eprintln!("Can't find .aws, .aws.v1, and .aws.v2 directories in users home");
        process::exit(exitcode::CONFIG);
    }

    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();

    if command == "use-v1" {
        replace_directory_with(aws.as_path(), aws_v1.as_path()).unwrap();
    }
    else if command == "use-v2" {
        replace_directory_with(aws.as_path(), aws_v2.as_path()).unwrap();
    }
    else {
        eprintln!("Unrecognized command {} expected use-v1 / use-v2", command);
        process::exit(1);
    }
}

#[derive(Debug)]
struct AWSCredentials {
    access_key: String,
    secret_key: String,
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

fn replace_directory_with(old: &Path, new: &Path) -> io::Result<u64> {
    fs::remove_dir_all(old)
        .and_then(|_| fs::copy(new, old))
}
