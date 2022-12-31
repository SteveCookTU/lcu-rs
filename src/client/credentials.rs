use crate::client::lock_file::LCULockFile;
use crate::client::REGEX;
use reqwest::blocking::Request;
use reqwest::header::HeaderValue;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct LCUCredentials {
    pub address: String,
    pub username: String,
    pub password: String,
}

impl LCUCredentials {
    pub fn new(address: String, username: String, password: String) -> Self {
        Self {
            address,
            username,
            password,
        }
    }

    pub fn from_fs() -> Result<Self, Box<dyn Error>> {
        let path = get_install_path()?;
        let mut file = File::open(path.join("lockfile")).map_err(Box::new)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(Box::new)?;
        let lock_file = LCULockFile::parse(&contents);
        Ok(Self {
            address: format!("127.0.0.1:{}", lock_file.port),
            username: "riot".to_string(),
            password: lock_file.password,
        })
    }

    pub fn insert_auth_header(&self, request: &mut Request) -> Result<(), Box<dyn Error>> {
        request.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&format!("Basic {}", self.auth_token())).map_err(Box::new)?,
        );
        Ok(())
    }

    pub fn auth_token(&self) -> String {
        base64::encode(format!("{}:{}", self.username, self.password))
    }
}

fn get_install_path() -> Result<PathBuf, Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        String::from_utf8(
            Command::new("wmic")
                .args([
                    "PROCESS",
                    "WHERE",
                    "name='LeagueClientUx.exe'",
                    "GET",
                    "commandline",
                ])
                .output()
                .map_err(Box::new)?
                .stdout,
        )
        .map_err(Box::new)?
    } else {
        String::from_utf8(
            Command::new("ps")
                .args(["x", "-o", "args", "|", "grep", "'LeagueClientUx'"])
                .output()
                .map_err(Box::new)?
                .stdout,
        )
        .map_err(Box::new)?
    };
    let captures = REGEX
        .captures(&output)
        .expect("Failed to capture installation path from process (0)");
    let path = captures
        .get(1)
        .expect("Failed to capture installation path from process (1)");
    Ok(PathBuf::from(path.as_str()))
}
