extern crate clap;
use clap::{App, Arg};
use std::process::Command;

fn main() {
    let matches = App::new("mypipe")
        .version("1.0")
        .about("My own pipe implementation")
        .author("Luis Valdez")
        .arg(
            Arg::with_name("in")
                .required(true)
                .short("i")
                .long("--in")
                .takes_value(true)
                .help("input"),
        )
        .arg(
            Arg::with_name("out")
                .required(true)
                .short("o")
                .long("--out")
                .takes_value(true)
                .help("output"),
        )
        .get_matches();

    let _in = matches.value_of("in").unwrap_or("default");
    let _out = matches.value_of("out").unwrap_or("default");

    let output = Command::new(_in)
        .output()
        .expect("failed to execute process");

    let stdout = output.stdout;
    let arg = String::from_utf8_lossy(&stdout).to_string();

    let result = Command::new(_out).arg(arg)
        .output()
        .expect("failed to execute process");



    println!("{}", String::from_utf8_lossy(&result.stdout));
    //println!("test {:?}", String::from_utf8_lossy(&err));
}
