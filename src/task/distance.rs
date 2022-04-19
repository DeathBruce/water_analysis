//! This module contains the main function of computing distance between two atoms.
//! For now, this program can only handle for pure water (H-O-H).
//!
//! A commonly command for running this process is
//! ```
//! execfile ./a.xdatcar vasp/xdatcar 1 2000 5 distance atom1_index atom2_index ./dist.dat
//! ```

use crate::task::get_distance_pbc;
use crate::Frame;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub fn compute_distance(
    system: &Vec<Frame>,
    rdfopt: &Vec<&str>,
    output: &str,
) -> Result<(), Box<dyn Error>> {

    let index1: usize = rdfopt[0].parse::<usize>().unwrap();
    let index2: usize = rdfopt[1].parse::<usize>().unwrap();
    let mut o = fs::File::create(output).unwrap();

    for (i, frame) in system.iter().enumerate() {
        let dist: f64 = get_distance_pbc(frame.atom[index1-1].coordination, 
                                    frame.atom[index2-1].coordination, 
                                    &frame.cell);
        o.write((format!("{:.4}  {:.8}", i, dist) + "\n").as_bytes())
            .expect("write rdf to file failed");
    }
    Ok(())
}
