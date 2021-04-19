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
    let aws = home.join(".aws");
    let aws_v1 = home.join(".aws.v1");
    let aws_v2 = home.join(".aws.v2");

    if !aws.is_dir() || !aws_v1.is_dir() || !aws_v2.is_dir() {
        eprintln!("Can't find .aws, .aws.v1, and .aws.v2 directories in users home");
        process::exit(exitcode::CONFIG);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Unrecognized command expected use-v1 / use-v2");
        process::exit(1);
    }

    let command = args[1].as_str();

    if command == "use-v1" {
        println!("Retrieving credentials from {}", aws.to_str().unwrap());
        let credentials = get_credentials().expect("Can't retrieve current credentials from aws config file");
        println!("Copying {} to {}", aws_v1.to_str().unwrap(), aws.to_str().unwrap());
        replace_directory_with(aws.as_path(), aws_v1.as_path()).unwrap();
        println!("Updating credentials");
        set_credentials(credentials);
        println!("Success !");
    }
    else if command == "use-v2" {
        println!("Retrieving credentials from {}", aws.to_str().unwrap());
        let credentials = get_credentials().expect("Can't retrieve current credentials from aws config file");
        println!("Copying {} to {}", aws_v2.to_str().unwrap(), aws.to_str().unwrap());
        replace_directory_with(aws.as_path(), aws_v2.as_path()).unwrap();
        println!("Updating credentials");
        set_credentials(credentials);
        println!("Success !");
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

fn set_credentials(creds: AWSCredentials) -> Option<io::Result<()>> {
    dirs::home_dir()
        .map(|home| home.as_path().join(".aws").join("credentials"))
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .map(|p| Ini::load_from_file(p).expect("Ini parse error"))
        .map(|mut credentials| {

            credentials.mut_iter()
                .filter(|(_, prop)| prop.contains_key("aws_access_key_id"))
                .for_each(|(section_name, prop) | {
                    println!("Setting access_key / secret_key for profile {:?}", section_name);
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
