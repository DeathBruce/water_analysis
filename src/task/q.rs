//! This module contains the function to compute the tetrahedral order parameter q.
//!

use crate::task::cov::find_cov_oneatom;
use crate::task::{get_angle, get_distance_pbc};
use crate::{Atom, Frame};
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

/// Find the nearest four O atoms for one O atom.
pub fn find_neighbour<'a>(
    atom_oc: &Atom,  // center atom O
    coord_O: &'a Vec<&Atom>, 
    cell: &Vec<f64>
) -> Result<Vec<&'a Atom>, Box<dyn Error>> {
    let mut neighbour: Vec<&Atom> = vec![];
    let mut distance: Vec<(&Atom, f64)> = vec![];
    for atom_o in coord_O.iter() {
        let d = get_distance_pbc(atom_oc.coordination, atom_o.coordination, &cell);
        if d > 0.1 && d <= 5.0 {
            distance.push( (&atom_o, d) );
        }
    }
    // println!("{:?}", distance);
    distance.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
    // println!("{:?}", distance);
    for i in 0..4 {
        neighbour.push(distance[i].0)
    }
    //println!("atom {} has {} neighbours", atom_oc.index,neighbour.len());
    if neighbour.len() < 4 {
        panic!("# of neighbours is less than 4!")
    }
    Ok(neighbour)
}


/// Compute the q in one frame
/// and the answer is stored
pub fn q_oneframe(frame: &Frame, output: &str) -> Result<(f64), Box<dyn Error>> {
        
    let mut o = OpenOptions::new()
        .append(true)
        .open(output)
        .expect("cannot open file");
    //  ------Collect the information the frame------
    let cell: &Vec<f64> = &frame.cell;
    let mut numb_O: i32 = 0;
    let mut coord_O: Vec<&Atom> = vec![];
    for i in frame.atom.iter() {
        if i.type_name == "O" || i.type_name == "1" {  // O,H for vasp and 1,2 for lammps
            coord_O.push(i);
            numb_O += 1;
        }
    }

    // ---------------main loop-----------------------
    let mut tmp_q: f64 = 0.0;
    for atom_oc in coord_O.iter() {  //  atom_oc means center atom O
        let neighbour: Vec<&Atom> = find_neighbour(atom_oc, &coord_O, &cell)?;
        let mut answer: f64 = 1.0;
        for i in 0..neighbour.len()-1 {
            for j in (i+1)..neighbour.len() {
                let tmp_rad: f64 = get_angle(neighbour[i].coordination,
                    atom_oc.coordination,
                    neighbour[j].coordination,
                    &cell,
                );
                answer -= 0.375 * ( tmp_rad.cos() + 1.0/3.0 ).powi(2);
            }
        }
        o.write( (format!("{:.8}", answer) + "\n").as_bytes() )
            .expect("write q to file failed");
        tmp_q += answer;
    }


    Ok(tmp_q/(numb_O as f64))
}


pub fn q(system: &Vec<Frame>, output: &str) -> Result<(), Box<dyn Error>> {
    fs::File::create(&output).unwrap();
    let mut q_answer: f64 = 0.0;
    let mut nframe: i32 = 0;
    for i in system.iter() {
        nframe += 1;
        q_answer += q_oneframe(i, &output)?;
    }
    println!("Averate q is {:.8}", q_answer / (nframe as f64));
    Ok(())
}
