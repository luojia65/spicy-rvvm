use clap::*;
use std::fs::OpenOptions;

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
    // println!("{}", input_path_str);
    let file = OpenOptions::new().read(true).open(input_path_str)
        .expect("open input file");
    // todo
    let _ = file;
}
