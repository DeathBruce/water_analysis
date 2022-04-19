/// The main lib for the water_analysis program
/// The main run() function is defined in this file.
//  use std::env;
use std::error::Error;
//  use std::fs;
//use cgmath::Vector3;
use std::fmt;

pub mod load;
pub mod task;
pub mod convert;

extern crate clap;
use clap::{Arg, ArgMatches, App, SubCommand};

///  save the parameter read from terminal
/*
pub struct Config {
    pub filename  : String,
    pub filetype  : String,
    pub frameopt  : Vec<String>, // start stop step
    pub task      : String,
    pub taskopt   : Vec<String>, // different for different task
    pub output    : String,
}*/

///  save the information of an atom
pub struct Atom {
    pub index       : i32,
    pub type_name   : String,
    pub coordination : [f64; 3],
}

///  save the information of one frame
pub struct Frame {
    pub frame_idx    : i32,
    pub cell         : Vec<f64>,
    pub atom_type    : Vec<String>,
    pub atom_numb    : Vec<i32>,
    pub natom        : i32,
    pub atom         : Vec<Atom>,
}
/*
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let filetype = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file type"),
        };

        let mut frameopt = vec![];
        if filetype == "vasp/xdatcar" || filetype == "qe/traj" || filetype == "lammps/traj" {
            //  frameopt = vec!["startFrameNumb", "endFrameNumb", "step"];
            frameopt.push(args.next().unwrap());
            frameopt.push(args.next().unwrap());
            frameopt.push(args.next().unwrap());
        }

        let task = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a task"),
        };

        let mut taskopt: Vec<String> = vec![];
        match &task as &str {
            "rdf" => {
                //  taskopt = vec!["elementA", "elementB", "rcut", "numb_bins"];
                taskopt.push(args.next().unwrap());
                taskopt.push(args.next().unwrap());
                taskopt.push(args.next().unwrap());
                taskopt.push(args.next().unwrap()); },
            "msd" => {
                //  taskopt = vec!["elementA", "direction_type", "stepstart", "max step", "d step"];
                taskopt.push(args.next().unwrap());
                taskopt.push(args.next().unwrap());
                taskopt.push(args.next().unwrap());
                taskopt.push(args.next().unwrap());
                taskopt.push(args.next().unwrap()); },
            "dist" => {
                // taskopt = vec!["atom1_index, atom2_index"];
                taskopt.push(args.next().unwrap());
                taskopt.push(args.next().unwrap()); },
            "convert" => {
                //  taskopt = "qe2xdatcar", etc.
                taskopt.push(args.next().unwrap());
                taskopt.push(args.next().unwrap()); },
            _ => {}
        }

        let output = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an output"),
        };

        Ok(Config {
            filename,
            filetype,
            frameopt,
            task,
            taskopt,
            output,
        })
    }
}
*/
impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {:?})",
            self.index, self.type_name, self.coordination
        )
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {:?}",
            self.index, self.type_name, self.coordination
        )
    }
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "frame: {}\n cell: {:?}\n atom_type:{:?}\n atom_numb:{:?}\n natom:{}\n",
            self.frame_idx, self.cell, self.atom_type, self.atom_numb, self.natom
        )
    }
}

/// The main process of this program
/// contain the information of parameter, file to be processed
/// and what will be done.
/// And run the process.
pub fn run(config: ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("input file: {}", config.value_of("inputfile").unwrap());
    println!("filetype: {}", config.value_of("input fmt").unwrap());

    let task: &str = &config.value_of("task").unwrap();
    let mut taskopt: Vec<&str> = config.value_of("taskopt").unwrap()
                  .split_whitespace().collect();
    println!("task    : {}", task);
    println!("task option: {:?}", taskopt);
    let mut frameopt: Vec<&str> = config.value_of("interval").unwrap()
                  .split_whitespace().collect();

    // load input file
    
    if task != "convert" {
        let mut system: Vec<Frame>;
        match &config.value_of("input fmt").unwrap() as &str {
            "vasp/poscar" => {
                println!("Loading input files, please wait...");
                system = load::vasp::read_poscar(config.value_of("inputfile").unwrap())?;
            }
            "vasp/xdatcar" => {
                println!("Loading input files, please wait...");
                system = load::vasp::read_xdatcar(config.value_of("inputfile").unwrap(), &frameopt)?;
            }
            "qe/traj" => {
                println!("Loading input files, please wait...");
                system = load::qe::read_traj(config.value_of("inputfile").unwrap(), &frameopt)?;
            }
            "lammps/traj" => {
                println!("Loading input files, please wait...");
                system = load::lammps::read_lammpstrj(config.value_of("inputfile").unwrap(), &frameopt)?;
            }
            _ => panic!("can't read filetype, please check your filetype!"),
        };
        //println!("{:?}", system);
    
        match task as &str {
            "rdf" => {
                println!("Running task, please wait...");
                task::rdf::rdf(&mut system, &config.value_of("taskopt").unwrap(), &config.value_of("outputfile").unwrap())?;
            }
            "cov" => {
                println!("Running task, please wait...");
                task::cov::cov(&mut system, config.value_of("outputfile").unwrap())?;
            }
            "hb" => {
                println!("Running task, please wait...");
                task::hb::compute_HBs(&mut system, config.value_of("outputfile").unwrap())?;
            }
            "q" => {
                println!("Running task, please wait...");
            }
            "msd" => {
                println!("Running task, please wait...");
                task::msd::msd(&mut system, &taskopt, config.value_of("outputfile").unwrap())?;
            }
            "dist" => {
                println!("Running task, please wait...");
                task::distance::compute_distance(&mut system, &taskopt, config.value_of("outputfile").unwrap())?;
            }
            _ => panic!("unknown task, please check your task!"),
        };
    } else {
        match &config.value_of("taskopt").unwrap() as &str {
            "qe2xdatcar" => {
                println!("Running task, please wait...");
                convert::qe2xdatcar::qe2xdatcar(&config.filename, &config.value_of("outputfile").unwrap())?;
            }
            "xdatcar_joint" => {
                println!("Running task, please wait...");
                convert::xdatcar_joint::joint(&config.filename, &config.value_of("taskopt").unwrap(), &config.value_of("outputfile").unwrap())?;
            }
            _ => panic!("unknown convert option, please check!"),
        }
    }
     
    println!("Job is successfully finished!");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut i = 0;
        let mut a = 0;
        while i < 10 {
            if i == 5 {
                a = 5;
                i += 1;
            } else {
                i += 1;
            }
        }
        assert_eq!(a, 5);
    }
}
