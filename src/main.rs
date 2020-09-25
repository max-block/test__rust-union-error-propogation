use std::{fs, io::Error as IOError};

#[derive(Debug)]
enum AppError {
    EmptyFile,
    IOError(IOError),
}

impl From<IOError> for AppError {
    fn from(e: IOError) -> Self {
        AppError::IOError(e)
    }
}

fn analyze_file(path: String) -> Result<usize, AppError> {
    let res = fs::read_to_string(path)?;
    if res.len() == 0 {
        Err(AppError::EmptyFile)
    } else {
        Ok(res.len())
    }
}

fn main() -> Result<(), AppError> {
    let res = analyze_file("/tmp/123".to_string())?;
    println!("{}", res);
    Ok(())
}
