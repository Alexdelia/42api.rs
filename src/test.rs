use std::io;

const LOG_FOLDER: &str = "test_log";

pub fn log_test(file: &str, content: &str) -> io::Result<()> {
    std::fs::create_dir_all(LOG_FOLDER)?;

    std::fs::write(format!("{LOG_FOLDER}/{file}.log"), content)
}
