use std::error::Error;
use std::path::PathBuf;

use crate::service::ServiceManager;

pub struct LinuxService{}

impl ServiceManager for LinuxService {
    fn install(_search_paths: Vec<PathBuf>, _interval: u64, _threshold: u64) -> Result<PathBuf, Box<dyn Error>> {
        Err("Not yet supported on this platform.".into())
    }

    fn uninstall() -> Result<(), Box<dyn Error>> {
        Err("Not yet supported on this platform.".into())
    }

    fn status() -> Option<String> {
        println!("Not yet supported on this platform.");

        None
    }

    fn logs() -> Result<String, Box<dyn Error>> {
        Err("Not yet supported on this platform.".into())
    }
}
