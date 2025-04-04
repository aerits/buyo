use std::collections::HashMap;

use regex::Regex;

use crate::vectors::BVec;

use super::enums::Rotation;

pub struct Tables {
    tetr_offset_data: Vec<BVec>,
    tetr_offset_data_i: Vec<BVec>,
    tetr_offset_data_o: Vec<BVec>,
}
impl Tables {
    pub fn new() -> Tables {
        let tetr_offset_data_str = "
        ( 0, 0) 	( 0, 0) 	( 0, 0) 	( 0, 0) 	( 0, 0)
        ( 0, 0) 	(+1, 0) 	(+1,-1) 	( 0,+2) 	(+1,+2)
        ( 0, 0) 	( 0, 0) 	( 0, 0) 	( 0, 0) 	( 0, 0)
        ( 0, 0) 	(-1, 0) 	(-1,-1) 	( 0,+2) 	(-1,+2)
        ";
        let tetr_offset_data_i_str = "
        ( 0, 0) 	(-1, 0) 	(+2, 0) 	(-1, 0) 	(+2, 0)
 	    (-1, 0) 	( 0, 0) 	( 0, 0) 	( 0,+1) 	( 0,-2)
 	    (-1,+1) 	(+1,+1) 	(-2,+1) 	(+1, 0) 	(-2, 0)
 	    ( 0,+1) 	( 0,+1) 	( 0,+1) 	( 0,-1) 	( 0,+2)
        ";
        let tetr_offset_data_o_str = "
        ( 0, 0) 
 	    ( 0,-1)
 	    (-1,-1)
 	    (-1, 0)
        ";
        let re = Regex::new(r"(?<n1>.[0-9]).(?<n2>.[0-9])").unwrap();
        let tetr_offset_data = re.captures_iter(tetr_offset_data_str).map(|caps| {
            let n1: i32 = caps.name("n1").unwrap().as_str().parse().unwrap();
            let n2: i32 = caps.name("n2").unwrap().as_str().parse().unwrap();
            BVec::new(n1, n2)
        }).collect();

        let tetr_offset_data_i = re.captures_iter(tetr_offset_data_i_str).map(|caps| {
            let n1: i32 = caps.name("n1").unwrap().as_str().parse().unwrap();
            let n2: i32 = caps.name("n2").unwrap().as_str().parse().unwrap();
            BVec::new(n1, n2)
        }).collect();

        let tetr_offset_data_o = re.captures_iter(tetr_offset_data_o_str).map(|caps| {
            let n1: i32 = caps.name("n1").unwrap().as_str().parse().unwrap();
            let n2: i32 = caps.name("n2").unwrap().as_str().parse().unwrap();
            BVec::new(n1, n2)
        }).collect();

        Tables {
            tetr_offset_data,
            tetr_offset_data_i,
            tetr_offset_data_o,
        }
    }

    pub fn get_offset(&self, r: Rotation, offset: usize) -> BVec {
        let r_index = match r {
            Rotation::Up => 0,
            Rotation::Right => 5,
            Rotation::Down => 10,
            Rotation::Left => 15,
        };
        return self.tetr_offset_data[(r_index + offset) as usize];
    }
    pub fn get_offset_i(&self, r: Rotation, offset: usize) -> BVec {
        let r_index = match r {
            Rotation::Up => 0,
            Rotation::Right => 5,
            Rotation::Down => 10,
            Rotation::Left => 15,
        };
        return self.tetr_offset_data_i[(r_index + offset) as usize];
    }
    pub fn get_offset_o(&self, r: Rotation) -> BVec {
        self.tetr_offset_data_o[match r {
            Rotation::Up => 0,
            Rotation::Right => 1,
            Rotation::Down => 2,
            Rotation::Left => 3,
        }]
    }
}
