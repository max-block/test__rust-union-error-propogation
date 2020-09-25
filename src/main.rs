use std::{fs, io::Error};

#[derive(Debug)]
enum AppError {
    EmptyFile,
    IOError(Error),
}

fn analyze_file(path: String) -> Result<usize, AppError> {
    let res = fs::read_to_string(path).map_err(|e| AppError::IOError(e))?;
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
