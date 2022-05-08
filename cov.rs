//! This module contains the main function of computing covalence bond.
//! For now, this program can only handle for pure water (H-O-H).
//!
//! A commonly command for running this process is
//! ```
//! execfile ./a.xdatcar vasp/xdatcar 1 2000 5 cov ./cov.1ps.dat
//! ```

use crate::task::{get_angle, get_distance_pbc};
use crate::{Atom, Frame};
use std::f64::consts::PI;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

//  need to be adapted for numb_type >= 2
//  consulting the rdf.rs

/// Find the covalence bond for an O atom.
pub fn find_cov_oneatom<'a>(
    atom_o: &Atom,
    coord_H: &'a Vec<&Atom>,
    cell: &Vec<f64>,
) -> Result<Vec<&'a Atom>, Box<dyn Error>> {
    let mut neighbour: Vec<&Atom> = vec![];
    let mut distance: Vec<(usize, f64)> = vec![];
    for atom_h in coord_H.iter() {
        let d = get_distance_pbc(atom_o.coordination, atom_h.coordination, &cell);
        if d <= 1.5 {
            //distance.push( ( j.index , i.coordination.distance(j.coordination) ) );
            distance.push((atom_h.index.try_into().unwrap(), d));
            neighbour.push(atom_h)
        }
        //println!("{:?}", distance[i]);
        if distance.len() > 1 {
            // println!("There are more than 2 H close to O, bad!\n");
            break;
        }
    }
    //distance.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
    if neighbour.len() != 2 { panic!("neighbour # of atom {} is not equal to 2", atom_o.index) }
    Ok(neighbour)
}

/// Compute the covalence angles (H-O-H) for each water molecule in one frame,
/// and the each angle will be printed (append) to the output file.
/// For now, this function is only for pure water.
pub fn cov_oneframe(frame: &Frame, output: &str) -> Result<(), Box<dyn Error>> {
    let mut o = OpenOptions::new()
        .append(true)
        .open(output)
        .expect("cannot open file");
    // let coord: &Vec<Atom> = &frame.xyz;
    let mut coord_O: Vec<&Atom> = vec![];
    let mut coord_H: Vec<&Atom> = vec![];
    //println!("framenumber: {}, \n {:?}",frame.frame_idx,frame.xyz);
    //let numb_O = frame.atom_numb[0] as usize;
    //let numb_H = frame.atom_numb[1] as usize;
    for i in frame.atom.iter() {
        if i.type_name == "O" || i.type_name == "1" {  // O,H for vasp and 1,2 for lammps
            coord_O.push(i)
        }
        if i.type_name == "H" || i.type_name == "2" {
            coord_H.push(i)
        }
    }

    //  ------Collect the coordination of a and b------
    //let mut molecules: Vec<Water> = vec![];
    let cell: &Vec<f64> = &frame.cell;
    for atom_o in coord_O.iter() {
        //  atom_o: &Atom
        let neighbour: Vec<&Atom> = find_cov_oneatom(atom_o, &coord_H, &cell)?;
        let cov_ang = get_angle(
            neighbour[0].coordination,
            atom_o.coordination,
            neighbour[1].coordination,
            &cell,
        ) * 180.0 / PI;
        o.write((format!("{:.8}", cov_ang) + "\n").as_bytes())
            .expect("write cov_angle failed");
        /*
        let mut distance: Vec<(usize, f64)> = vec![];

        for atom_h in coord_H.iter() {
            let d = get_distance_pbc(atom_o.coordination , atom_h.coordination, &cell);
            if d <= 1.2 {
                //distance.push( ( j.index , i.coordination.distance(j.coordination) ) );
                distance.push( (atom_h.index.try_into().unwrap() , d) );
            }
            //println!("{:?}", distance[i]);
            if distance.len() > 1 {
                // println!("There are more than 2 H close to O, bad!\n");
                break
            }
        }
        //  println!("{:?}",distance);
        distance.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
        //  println!("{:?}", coord[distance[0].0-1]);
        let dist_avg = ( distance[0].1 + distance[1].1 ) / 2.0;
        //  println!("{:?}", dist_avg);
        let cov_ang = get_angle(atom_o.coordination, coord[distance[0].0-1].coordination, coord[distance[1].0-1].coordination, &cell);
        //  println!("{}, {:?}", atom_o,ang);
        o.write( (format!("{:.8}", cov_ang) + "\n" ).as_bytes() ).expect("write cov_angle failed");
        // molecules.push(Water{ O: atom_o,
        //                       H: (&coord[distance[0].0-1], &coord[distance[1].0-1]),
        //                       distance: dist_avg,
        //                       deg: cov_ang
        //                     });
        */
    }
    //println!("{:?}", molecules);
    //Ok( molecules )
    Ok(())
}

/// Create the output file and put all covalence angle of all the frames
/// into the output file.
pub fn cov(system: &Vec<Frame>, output: &str) -> Result<(), Box<dyn Error>> {
    fs::File::create(&output).unwrap();
    for i in system.iter() {
        cov_oneframe(i, &output)?;
    }
    Ok(())
}
