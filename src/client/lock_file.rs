use std::str::FromStr;

#[derive(Clone)]
pub struct LCULockFile {
    pub process: String,
    pub pid: usize,
    pub port: u16,
    pub password: String,
    pub protocol: String,
}

impl LCULockFile {
    pub fn parse(str: impl AsRef<str>) -> Self {
        let mut args = str.as_ref().split(':');
        Self {
            process: args.next().map(|s| s.to_string()).unwrap_or_default(),
            pid: args
                .next()
                .map(|s| usize::from_str(s).unwrap_or_default())
                .unwrap_or_default(),
            port: args
                .next()
                .map(|s| u16::from_str(s).unwrap_or_default())
                .unwrap_or_default(),
            password: args.next().map(|s| s.to_string()).unwrap_or_default(),
            protocol: args.next().map(|s| s.to_string()).unwrap_or_default(),
        }
    }
}
