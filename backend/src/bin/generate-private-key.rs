use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::paserk::FormatAsPaserk;
use pasetors::version4::V4;

fn main() {
    let sk = AsymmetricKeyPair::<V4>::generate().unwrap();

    let mut public = String::new();
    sk.public.fmt(&mut public).unwrap();

    let mut private = String::new();
    sk.secret.fmt(&mut private).unwrap();

    println!("Public Key: {}", public);
    println!("Private Key: {}", private);
}
