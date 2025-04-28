use regex::Regex;

use crate::vectors::BVec;

use super::enums::{Rotation, Shapes};

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
        let tetr_offset_data = re
            .captures_iter(tetr_offset_data_str)
            .map(|caps| {
                let n1_s = caps.name("n1").unwrap().as_str();
                let n2_s = caps.name("n2").unwrap().as_str();
                BVec::new(n1_s.trim().parse().unwrap(), n2_s.trim().parse().unwrap())
            })
            .collect();

        let tetr_offset_data_i = re
            .captures_iter(tetr_offset_data_i_str)
            .map(|caps| {
                let n1_s = caps.name("n1").unwrap().as_str();
                let n2_s = caps.name("n2").unwrap().as_str();
                BVec::new(n1_s.trim().parse().unwrap(), n2_s.trim().parse().unwrap())
            })
            .collect();

        let tetr_offset_data_o = re
            .captures_iter(tetr_offset_data_o_str)
            .map(|caps| {
                let n1_s = caps.name("n1").unwrap().as_str();
                let n2_s = caps.name("n2").unwrap().as_str();
                BVec::new(n1_s.trim().parse().unwrap(), n2_s.trim().parse().unwrap())
            })
            .collect();

        Tables {
            tetr_offset_data,
            tetr_offset_data_i,
            tetr_offset_data_o,
        }
    }

    fn get_offset_gen(&self, r: Rotation, offset: usize) -> BVec {
        let r_index = match r {
            Rotation::Up => 0,
            Rotation::Right => 5,
            Rotation::Down => 10,
            Rotation::Left => 15,
        };
        return self.tetr_offset_data[(r_index + offset) as usize];
    }
    fn get_offset_i(&self, r: Rotation, offset: usize) -> BVec {
        let r_index = match r {
            Rotation::Up => 0,
            Rotation::Right => 5,
            Rotation::Down => 10,
            Rotation::Left => 15,
        };
        return self.tetr_offset_data_i[(r_index + offset) as usize];
    }
    fn get_offset_o(&self, r: Rotation) -> BVec {
        self.tetr_offset_data_o[match r {
            Rotation::Up => 0,
            Rotation::Right => 1,
            Rotation::Down => 2,
            Rotation::Left => 3,
        }]
    }
    pub fn get_offset(&self, shape: Shapes, r: Rotation, offset: usize) -> BVec {
        match shape {
            Shapes::I => self.get_offset_i(r, offset),
            Shapes::O => {
                if offset > 0 {
                    return BVec::new(0, 0);
                }
                self.get_offset_o(r)
            }
            _ => self.get_offset_gen(r, offset),
        }
    }
    pub fn get_kicks(&self, r_i: &Rotation, r_f: &Rotation, shape: &Shapes) -> Vec<BVec> {
        let mut v = Vec::new();
        for offset in 0..5 as usize {
            v.push(&self.get_offset(*shape, *r_i, offset) - &self.get_offset(*shape, *r_f, offset))
        }
        return v;
    }
}
