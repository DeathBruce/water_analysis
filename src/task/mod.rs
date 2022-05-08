//! Basic task such as rdf, HBs, etc.
//!
//! Some basic functions is defined here such as
//! get_distance_pbc(), get_angle()

pub mod cov;
pub mod hb;
pub mod msd;
pub mod q;
pub mod rdf;
pub mod distance;

use crate::{Atom, Frame};
use std::error::Error;
use std::f64::consts::PI;
use std::fmt;

pub struct Water<'a> {
    O: &'a Atom,
    H: (&'a Atom, &'a Atom),
    distance: f64,
    deg: f64,
}

impl<'a> fmt::Debug for Water<'a> {
    //  print{:?} for struct Water
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            " O: {:?}\n H1: {:?}\n H2: {:?}\n dist_avg: {}\n cov_angle:{}\n",
            self.O.coordination, self.H.0.coordination, self.H.1.coordination, self.distance, self.deg
        )
    }
}

/// Compute the distance between two points a and b.
/// 
/// considering the periodic boundary condition.
pub fn get_distance_pbc(a: [f64; 3], b: [f64; 3], cell: &Vec<f64>) -> f64 {
    let vector1 = a.clone();
    let vector2 = b.clone();
    let mut delta = vec![0.0, 0.0, 0.0];
    for i in 0..3 {
        delta[i] = vector2[i] - vector1[i];
        delta[i] = delta[i] - (delta[i] / cell[i]).round() * cell[i];
    }
    let distance = (delta[0].powi(2) + delta[1].powi(2) + delta[2].powi(2)).sqrt();
    distance
}

/// Compute the angle of H-O-H, using the law of cosines.
/// Return the angle in Rad.
/// considering the periodic boundary condition.
pub fn get_angle(a: [f64; 3], o: [f64; 3], b: [f64; 3], cell: &Vec<f64>) -> f64 {
    let r_oa = get_distance_pbc(o, a, &cell);
    let r_ob = get_distance_pbc(o, b, &cell);
    let r_ab = get_distance_pbc(a, b, &cell);
    let rad = ((r_oa.powi(2) + r_ob.powi(2) - r_ab.powi(2)) / (2.0 * r_oa * r_ob)).acos();
    let deg = 180.0 * rad / PI;
    rad
}

pub fn unwrap(system: &Vec<Frame>) -> Result<(), Box<dyn Error>> {
    Ok(())
}