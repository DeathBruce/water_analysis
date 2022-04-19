//! This module contains the function to compute the tetrahedral order parameter q.
//!

use crate::task::cov::find_cov_oneatom;
use crate::task::{get_angle, get_distance_pbc};
use crate::{Atom, Frame};
use std::error::Error;
use std::fs;
use std::io::Write;
