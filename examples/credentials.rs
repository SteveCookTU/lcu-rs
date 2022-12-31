use lcu_rs::client::LCUCredentials;

fn main() {
    let credentials = LCUCredentials::from_fs().expect("Failed to get credentials from FS");
    println!("{credentials:?}, {}", credentials.auth_token());
}
