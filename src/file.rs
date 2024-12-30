use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Error, Write};

use crate::codewars::kata::Kata;

pub async fn kata_file(kata: &Kata, content: Vec<String>) -> Result<(), Error> {
    let dir_path = format!("src/katas/{}", kata.rank());
    let file_path = format!("{}/{}.rs", dir_path, kata.slug.replace("-", "_"));

    fs::create_dir_all(&dir_path)?;

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    let mut buf_file = BufWriter::new(file);

    writeln!(buf_file)?;
    writeln!(
        buf_file,
        "// {}\n// {}",
        kata.name,
        kata.train_url().unwrap()
    )?;
    writeln!(buf_file)?;

    for line in content {
        writeln!(buf_file, "{}", line)?;
    }

    buf_file.flush()?;

    Ok(())
}

pub async fn markdown_file(kata: &Kata) -> Result<(), Error> {
    let dir_path = format!("src/katas/{}", kata.rank());
    let file_path = format!("{}/{}.md", dir_path, kata.slug.replace("-", "_"));

    fs::create_dir_all(&dir_path)?;

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    let mut buf_file = BufWriter::new(file);

    writeln!(buf_file, "# {}", kata.name)?;
    writeln!(buf_file, "{}", kata.description)?;

    buf_file.flush()?;

    Ok(())
}
