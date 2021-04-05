use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::default::Default;

/// SearchTree struct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ST {
    pub v: Option<(u16, u8)>, //value
    pub c: Option<Vec<ST>>,   // children
}

impl Default for ST {
    fn default() -> Self {
        ST { v: None, c: None }
    }
}

impl ST {
    fn show(&self) {
        &self;
    }

    // let mut st = ST { ..ST::default() };
    pub fn new() -> ST {
        ST { ..ST::default() }
    }

    pub fn add_child(&mut self, child: Vec<ST>) {
        match &mut self.c {
            None => self.c = Some(child),
            Some(cv) => {
                cv.extend(child);
            }
        };
    }

    // used for testing
    #[allow(dead_code)]
    fn add_st(&mut self, child: ST) {
        match &mut self.c {
            None => {
                self.c = Some(vec![child]);
            }
            Some(cv) => {
                cv.push(child);
            }
        };
    }

    pub fn len(&self) -> usize {
        (&self).len_loop(0_usize)
    }

    fn len_loop(&self, depth: usize) -> usize {
        match &self.c {
            Some(c) => {
                for e in c.iter() {
                    e.len_loop(depth + 1);
                }
                depth + 1
            }
            None => 0,
        }
    }

    #[allow(dead_code)]
    pub fn dumper(&self) {
        &self.dump_loop(1_usize);
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        &self.dump_loop(0_usize);
    }

    fn dump_loop(&self, depth: usize) {
        match &self.v {
            Some(v) => match depth {
                0 => eprintln!("{}{:?}", "".repeat(depth), v),
                _ if depth >= 1 => eprintln!("{}{:?}", "\t".repeat(depth - 1), v),
                _ => (),
            },
            None => (),
        }
        match &self.c {
            Some(c) => {
                for e in c.iter() {
                    e.dump_loop(depth + 1);
                }
            }
            None => (),
        }
    }
}

#[derive(Debug)]
pub struct STItem(Vec<u16>, Vec<(u16, u8)>); // does this have to be a Vec or can one or both be an array?

impl Iterator for ST {
    //type Item = (Vec<u16>, Vec<(u16, u8)>); //abstract that for use in the other Iterators
    type Item = STItem;
    fn next(&mut self) -> Option<Self::Item> {
        let mut board: Vec<(u16, u8)> = vec![]; //empty board; we skip the root node because it just exists to hold all of the first pieces
        let path: Vec<u16> = match &mut self.c {
            Some(children) => {
                match self.v {
                    // add the current value to the board (if there is one)
                    Some(_value) => board.push(self.v.unwrap()),
                    None => {}
                };
                // iterate children and call next(child) on each of them NTS to be written once we grok recursive struct [iter,&iter_mut,&iter]
                let iter_index = 0;
                let step_on_the_path = match &(*children)[iter_index] {
                    ST {
                        v: Some(_v),
                        c: Some(_c),
                    } => iter_index,
                    //ST {Some(v),Some(c) } => self.v.unwrap(),
                    _ => 0,
                };
                //self.c = Some(children[1..].to_vec());  // cannot assign to `self.c` because it is borrowed
                //self.v = *(children)[0].v;
                self.v = Some((children)[0].v.unwrap());
                vec![step_on_the_path.try_into().unwrap()]
            }
            None => vec![],
        }; // let path:
        Some(STItem(path, board))
    }
}

pub mod st_or_to_disk;
