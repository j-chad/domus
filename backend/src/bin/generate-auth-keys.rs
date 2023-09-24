use const_format::concatcp;
use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::paserk::FormatAsPaserk;
use pasetors::version4::V4;
use std::fs::OpenOptions;

const ENV_PREFIX: &str = "DOMUS_AUTH.";
const ENV_PRIVATE_KEY: &str = concatcp!(ENV_PREFIX, "PRIVATE_KEY");
const ENV_PUBLIC_KEY: &str = concatcp!(ENV_PREFIX, "PUBLIC_KEY");

fn main() {
    let sk = AsymmetricKeyPair::<V4>::generate().unwrap();

    let mut public = String::new();
    sk.public.fmt(&mut public).unwrap();

    let mut private = String::new();
    sk.secret.fmt(&mut private).unwrap();

    write_to_env_file(&private, &public);

    println!("Public Key: {}", public);
    println!("Private Key: {}", private);
    println!("Tokens have been added to your .env file")
}

fn write_to_env_file(private: &str, public: &str) {
    use std::io::prelude::*;

    // create .env file if it doesn't exist
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .read(true)
        .create(true)
        .open(".env")
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // if either key is already in the file (with any value):
    // - remove the line
    contents = contents
        .lines()
        .filter(|line| !line.starts_with(ENV_PRIVATE_KEY) && !line.starts_with(ENV_PUBLIC_KEY))
        .map(|line| format!("{}\n", line))
        .collect();

    contents = contents.trim().to_string();

    let template = format!(
        "{}={}\n{}={}\n",
        ENV_PRIVATE_KEY, private, ENV_PUBLIC_KEY, public
    );

    if !contents.is_empty() {
        contents.push('\n');
    }

    contents.push_str(&template);
    file.set_len(0).unwrap();
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
