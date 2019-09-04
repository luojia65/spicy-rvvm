mod isa;

use clap::*;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Cursor};

fn main() {
    let matches = App::new("spicy-rvvm")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("elf-input-path")
            .value_name("PATH")
            .takes_value(true)
            .required(true)
            .help("path to the input RISC-V elf file"))
        .get_matches();
    let input_path_str = matches.value_of("elf-input-path").unwrap();
    let mut file = OpenOptions::new().read(true).open(input_path_str)
        .expect("open input file");
    let input_elf = elf::File::open_stream(&mut file)
        .expect("open file as ELF input stream");
    let text_section = input_elf.get_section(".text")
        .expect("find text section");
    let data = &text_section.data;
    let cursor = Cursor::new(data);
    let input = ReadSeekInput::new(cursor);
}

struct ReadSeekInput<T> {
    inner: T
}

impl<T> ReadSeekInput<T>
where 
    T: Read + Seek
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> ReadSeekInput<T> 
where 
    T: Read + Seek 
{
    pub fn next(&self) {
        
    }
}
