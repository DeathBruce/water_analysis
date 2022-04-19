//! This module contains the main function of computing radial distribution function.
//!
//! A commonly command for running this process is
//! ```
//! execfile ./a.xdatcar vasp/xdatcar 1 2000 5 rdf O O 8 320 ./rdf.1ps.dat
//! ```

use crate::task::get_distance_pbc;
use crate::{Atom, Frame};
use std::error::Error;
use std::f64::consts::PI;
use std::fs;
use std::io::Write;

/// Compute the Rdf g(r)["a", "b"] in one frame,
/// and the answer is stored in the form of g(r) = vec![].
pub fn rdf_oneframe(
    frame: &Frame,
    rdf_type: &[&str; 2],
    rcut: f64,
    numb_bins: i32,
    vcell: &f64,
) -> Result<Vec<f64>, Box<dyn Error>> {
    
    //  ------Collect the information of b------
    let mut numb_a: i32 = 0;
    for i in 0..frame.atom_type.len() {
        if rdf_type[0] == &frame.atom_type[i] {
            numb_a = frame.atom_numb[i]
        }
    }
    let mut numb_b: i32 = 0;
    for i in 0..frame.atom_type.len() {
        if rdf_type[1] == &frame.atom_type[i] {
            numb_b = frame.atom_numb[i]
        }
    }
    //println!("numb_a = {} ; numb_b = {}", numb_a, numb_b);

    //  ------Collect the coordination of a and b------
    let mut coord_a: Vec<&Atom> = vec![];
    let mut coord_b: Vec<&Atom> = vec![];
    for i in 0..frame.atom.len() {
        if &frame.atom[i].type_name == rdf_type[0] {
            coord_a.push(&frame.atom[i])
        }
        if &frame.atom[i].type_name == rdf_type[1] {
            coord_b.push(&frame.atom[i])
        }
    }

    //  ------the main computation------
    let mut distance: Vec<f64> = vec![];
    let dr: f64 = rcut / numb_bins as f64;
    let mut count: Vec<f64> = vec![0.0; numb_bins as usize];

    for i in &coord_a {
        for j in &coord_b {
            let d: f64 = get_distance_pbc(i.coordination, j.coordination, &frame.cell);
            if d < rcut {
                let layer = (d / dr).floor() as usize;
                distance.push(d);
                count[layer] += 1.0;
            }
        }
    }
    //println!("{:?}", count);
    let rho_b: f64 = (numb_b) as f64 / vcell;
    for i in 0..numb_bins as usize {
        let n: f64 = i as f64;
        //count[i] = count[i] / ((numb_b as f64) * 4.0 * PI * (n * dr).powi(2) * dr * rho_b);
        // Using numb_b should be wrong.
        count[i] = count[i] / ((numb_a as f64) * 4.0 * PI * (n * dr).powi(2) * dr * rho_b);
    }
    //println!("{:?}", count);
    Ok(count)
}

/// Create the output file and put all covalence angle of all the frames
/// into the output file.
/// rdfopt = vec!["elementA", "elementB", "rcut", "numb_bins"];
pub fn rdf(
    system: &Vec<Frame>,
    rdfopt: &Vec<&str>,
    output: &str,
) -> Result<(), Box<dyn Error>> {

    //  ------load the task option------
    let rdf_type: [&str; 2] = [&rdfopt[0], &rdfopt[1]];
    let rcut: f64 = rdfopt[2].parse::<f64>().unwrap();
    let numb_bins: i32 = rdfopt[3].parse::<i32>().unwrap();

    //  ------Compute rdf and loop for frames------
    let mut gr: Vec<f64> = vec![0.0; numb_bins as usize];
    let mut nframe: i32 = 0;

    for i in system.iter() {
        //println!("{}",nframe);
        nframe += 1;
        let vcell: f64 = i.cell[0] * i.cell[1] * i.cell[2];
        let gr_oneframe = rdf_oneframe(i, &rdf_type, rcut, numb_bins, &vcell)?;
        for j in 0..numb_bins as usize {
            gr[j] = gr[j] + gr_oneframe[j];
        }
    }
    println!("nframe is {}", nframe);
    for j in 0..numb_bins as usize {
        gr[j] = gr[j] / nframe as f64;
    }

    //  ------write into output file------
    //  "r[i]   g[i]   \n"
    let mut r: Vec<f64> = vec![];
    for i in 0..numb_bins {
        r.push((rcut / numb_bins as f64) * i as f64);
    }
    let mut o = fs::File::create(output).unwrap();
    for i in 0..numb_bins as usize {
        o.write((format!("{:.4}  {:.8}", r[i], gr[i]) + "\n").as_bytes())
            .expect("write rdf to file failed");
    }
    Ok(())
}
