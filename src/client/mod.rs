mod credentials;
mod lock_file;
mod perks;

pub use perks::*;

pub use credentials::*;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::Client;

#[macro_export]
macro_rules! declare_get_endpoint {
    ($name:ident, $path:literal, text) => {
        pub fn $name(credentials: &LCUCredentials) -> Result<String, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::GET,
                Url::from_str(&format!("https://{}/{}", credentials.address, $path)).map_err(Box::new)?,
            );
            credentials.insert_auth_header(&mut request)?;
            let response = CLIENT.execute(request).map_err(Box::new)?;
            Ok(response.text().unwrap_or_default())
        }
    };
    ($name:ident, $path:literal, $ret:ty) => {
        pub fn $name(credentials: &LCUCredentials) -> Result<$ret, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::GET,
                Url::from_str(&format!("https://{}/{}", credentials.address, $path)).map_err(Box::new)?,
            );
            credentials.insert_auth_header(&mut request)?;
            let response = CLIENT.execute(request).map_err(Box::new)?;
            let text = response.text().map_err(Box::new)?;
            Ok(serde_json::from_str::<$ret>(&text).unwrap_or_default())
        }
    };
    ($name:ident, $path:literal, $ret:ty, $($param:ident : $param_t:ty),+) => {
        pub fn $name(credentials: &LCUCredentials $(, $param: $param_t)+) -> Result<$ret, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::GET,
                Url::from_str(&format!("https://{}/{}", credentials.address, format_args!($path $(, $param)+))).map_err(Box::new)?,
            );
            credentials.insert_auth_header(&mut request)?;
            let response = CLIENT.execute(request).map_err(Box::new)?;
            let text = response.text().map_err(Box::new)?;
            Ok(serde_json::from_str::<$ret>(&text).unwrap_or_default())
        }
    }
}

#[macro_export]
macro_rules! declare_del_endpoint {
    ($name:ident, $path:literal, text) => {
        pub fn $name(credentials: &LCUCredentials) -> Result<String, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::DELETE,
                Url::from_str(&format!("https://{}/{}", credentials.address, $path)).map_err(Box::new)?,
            );
            credentials.insert_auth_header(&mut request)?;
            let response = CLIENT.execute(request).map_err(Box::new)?;
            Ok(response.text().unwrap_or_default())
        }
    };
    ($name:ident, $path:literal, $ret:ty) => {
        pub fn $name(credentials: &LCUCredentials) -> Result<$ret, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::DELETE,
                Url::from_str(&format!("https://{}/{}", credentials.address, $path)).map_err(Box::new)?,
            );
            credentials.insert_auth_header(&mut request)?;
            let response = CLIENT.execute(request).map_err(Box::new)?;
            let text = response.text().map_err(Box::new)?;
            Ok(serde_json::from_str::<$ret>(&text).unwrap_or_default())
        }
    };
    ($name:ident, $path:literal, $ret:ty, $($param:ident : $param_t:ty),+) => {
        pub fn $name(credentials: &LCUCredentials $(, $param: $param_t)+) -> Result<$ret, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::DELETE,
                Url::from_str(&format!("https://{}/{}", credentials.address, format_args!($path $(, $param)+))).map_err(Box::new)?,
            );
            credentials.insert_auth_header(&mut request)?;
            let response = CLIENT.execute(request).map_err(Box::new)?;
            let text = response.text().map_err(Box::new)?;
            Ok(serde_json::from_str::<$ret>(&text).unwrap_or_default())
        }
    }
}

#[macro_export]
macro_rules! declare_put_endpoint {
    ($name:ident, $path:literal, $input:ty, text) => {
        pub fn $name(credentials: &LCUCredentials, input: &$input) -> Result<String, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::PUT,
                Url::from_str(&format!("https://{}/{}", credentials.address, $path)).map_err(Box::new).unwrap(),
            );
            credentials.insert_auth_header(&mut request).unwrap();
            let body = serde_json::to_string(input).unwrap_or_default();
            let _ = request.body_mut().insert(body.into());
            let response = CLIENT.execute(request).map_err(Box::new).unwrap();
            Ok(response.text().unwrap_or_default())
        }
    };
    ($name:ident, $path:literal, $input:ty, $ret:ty) => {
        pub fn $name(credentials: &LCUCredentials, input: &$input) -> Result<$ret, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::PUT,
                Url::from_str(&format!("https://{}/{}", credentials.address, $path)).map_err(Box::new).unwrap(),
            );
            credentials.insert_auth_header(&mut request).unwrap();
            let body = serde_json::to_string(input).unwrap_or_default();
            let _ = request.body_mut().insert(body.into());
            let response = CLIENT.execute(request).map_err(Box::new).unwrap();
            let text = response.text().map_err(Box::new)?;
            Ok(serde_json::from_str::<$ret>(&text).unwrap_or_default())
        }
    };
    ($name:ident, $path:literal, $input:ty, $ret:ty, $($param:ident : $param_t:ty),+) => {
        pub fn $name(credentials: &LCUCredentials, input: &$input $(, $param: $param_t)+) -> Result<$ret, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::PUT,
                Url::from_str(&format!("https://{}/{}", credentials.address, format_args!($path $(, $param)+))).map_err(Box::new)?,
            );
            credentials.insert_auth_header(&mut request)?;
            let body = serde_json::to_string(input).unwrap_or_default();
            let _ = request.body_mut().insert(body.into());
            let response = CLIENT.execute(request).map_err(Box::new).unwrap();
            let text = response.text().map_err(Box::new)?;
            Ok(serde_json::from_str::<$ret>(&text).unwrap_or_default())
        }
    }
}

#[macro_export]
macro_rules! declare_post_endpoint {
    ($name:ident, $path:literal, $input:ty, text) => {
        pub fn $name(credentials: &LCUCredentials, input: &$input) -> Result<String, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::POST,
                Url::from_str(&format!("https://{}/{}", credentials.address, $path)).map_err(Box::new).unwrap(),
            );
            credentials.insert_auth_header(&mut request).unwrap();
            let body = serde_json::to_string(input).unwrap_or_default();
            let _ = request.body_mut().insert(body.into());
            let response = CLIENT.execute(request).map_err(Box::new).unwrap();
            Ok(response.text().unwrap_or_default())
        }
    };
    ($name:ident, $path:literal, $input:ty, $ret:ty) => {
        pub fn $name(credentials: &LCUCredentials, input: &$input) -> Result<$ret, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::POST,
                Url::from_str(&format!("https://{}/{}", credentials.address, $path)).map_err(Box::new).unwrap(),
            );
            credentials.insert_auth_header(&mut request).unwrap();
            let body = serde_json::to_string(input).unwrap_or_default();
            let _ = request.body_mut().insert(body.into());
            let response = CLIENT.execute(request).map_err(Box::new).unwrap();
            let text = response.text().map_err(Box::new)?;
            Ok(serde_json::from_str::<$ret>(&text).unwrap_or_default())
        }
    };
    ($name:ident, $path:literal, $input:ty, $ret:ty, $($param:ident : $param_t:ty),+) => {
        pub fn $name(credentials: &LCUCredentials, input: &$input $(, $param: $param_t)+) -> Result<$ret, Box<dyn Error>> {
            let mut request = reqwest::blocking::Request::new(
                Method::POST,
                Url::from_str(&format!("https://{}/{}", credentials.address, format_args!($path $(, $param)+))).map_err(Box::new)?,
            );
            credentials.insert_auth_header(&mut request)?;
            let body = serde_json::to_string(input).unwrap_or_default();
            let _ = request.body_mut().insert(body.into());
            let response = CLIENT.execute(request).map_err(Box::new).unwrap();
            let text = response.text().map_err(Box::new)?;
            Ok(serde_json::from_str::<$ret>(&text).unwrap_or_default())
        }
    }
}

lazy_static! {
    pub(crate) static ref CLIENT: Client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
}

#[cfg(target_os = "windows")]
lazy_static! {
    static ref REGEX: Regex = Regex::new("\"--install-directory=(.*?)\"").unwrap();
}

#[cfg(target_os = "macos")]
lazy_static! {
    static ref REGEX: Regex = Regex::new("--install-directory=(.*?)( --|\n|$)").unwrap();
}
