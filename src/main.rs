extern crate clap;
use std::io::{Read, Seek, SeekFrom, Write};
use clap::{crate_version, App, Arg};
use std::fs::OpenOptions;
use rand::Rng;

fn main() -> std::io::Result<()> {

    let matches = App::new("corrupter")
        .version(crate_version!())
        .author("Ronald B. <robaldwin9@gmail.com>")
        .about("Corrupts files for testing against software")
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("file path"),
        )
        .get_matches();

    // File path argument
    let file_path = matches.value_of("INPUT").unwrap();
    let mut byte_buff: Vec<u8> = Vec::new();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(file_path)
        .unwrap();

    file.read_to_end(&mut byte_buff)?;

    let mut rng = rand::thread_rng();

    for i in 0..byte_buff.len() {
        byte_buff[i] = rng.gen();
    }

    // Move file pointer back to begining and write bytes
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&byte_buff)?;

    println!("\tAll bytes of {} changed", file_path);
    std::io::Result::Ok(())
}
