use std::error::Error;

pub struct WindowsService {}

impl ServiceManager for WindowsService {
    fn install(search_paths: Vec<PathBuf>, interval: u64, threshold: u64) -> Result<PathBuf, Box<dyn Error>> {
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
