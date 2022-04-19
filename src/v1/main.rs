
extern crate clap;

use std::env;
use std::process;
use clap::{Arg, App, SubCommand};


fn main() {
    let config = App::new("Water_analysis")
                      .version("0.1")
                      .author("Weiyu Li <liwy@shanghaitech.edu.cn>")
                      .about("Does some water analysis")
                      .arg(Arg::with_name("inputfile")
                           .short('i')
                           .long("in")
                           .help("Sets the input file to use")
                           .value_name("FILENAME")
                           .required(true)
                           .takes_value(true) )
                      .arg(Arg::with_name("input fmt")
                           .long("infmt")
                           .help("Sets the fomat of input file")
                           .value_name("FILE FORMAT")
                           .required(true)
                           .takes_value(true) )
                      .arg(Arg::with_name("task")
                           .long("task")
                           .help("Sets the interval of frames to be used")
                           .value_name("TASK")
                           .required(true)
                           .takes_value(true) ) 
                      .arg(Arg::with_name("taskopt")
                           .long("taskopt")
                           .help("Sets the interval of frames to be used")
                           .value_name("\"opt1 opt2\"")
                           .required(true)
                           .takes_value(true) ) 
                      .arg(Arg::with_name("outputfile")
                           .short('o')
                           .long("out")
                           .help("Sets the output file")
                           .value_name("FILENAME")
                           .required(true)
                           .takes_value(true) ) 
                      .arg(Arg::with_name("interval")
                           .long("interval")
                           .help("Sets the interval of frames to be used")
                           .value_name("\"start stop step\"")
                           .required(false)
                           .takes_value(true) )
                      .get_matches();


    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match config.occurrences_of("taskopt") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }


    if let Err(e) = water_analysis::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}



