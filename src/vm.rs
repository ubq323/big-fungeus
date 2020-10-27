use std::collections::HashMap;
use std::cmp::{min,max};
use crate::FungeCell;
use std::default::Default;

#[derive(Debug)]
pub struct Space {
    cells: HashMap<(FungeCell,FungeCell),FungeCell>,
    maxx: FungeCell,
    maxy: FungeCell,
    minx: FungeCell,
    miny: FungeCell,
}

impl Space {
    pub fn maxx(&self) -> FungeCell {self.maxx}
    pub fn maxy(&self) -> FungeCell {self.maxy}
    pub fn minx(&self) -> FungeCell {self.minx}
    pub fn miny(&self) -> FungeCell {self.miny}

    pub fn get(&mut self, xy: (FungeCell,FungeCell)) -> FungeCell {
        *self.cells.entry(xy).or_insert(32)
    }
    pub fn set(&mut self, xy: (FungeCell,FungeCell), value: FungeCell) {
        self.cells.insert(xy,value);
        let (x,y) = xy;
        self.maxx = max(self.maxx,x);
        self.maxy = max(self.maxy,y);
        self.minx = min(self.minx,x);
        self.miny = min(self.miny,y);
    }
    pub fn new() -> Space {
        Space {
            cells: Default::default(),
            maxx: 0,
            maxy: 0,
            minx: 0,
            miny: 0,
        }
    }
    pub fn load(&mut self, content: Vec<u8>) {
        self.loadat((0,0),content);
    }
    pub fn loadat(&mut self, xy: (FungeCell,FungeCell), content: Vec<u8>) {
        let (mut x,mut y) = xy;
        // some "interesting" newline handling behaviour
        // because we have to treat (10), (13) and (13 10) as a newline
        let mut last13 = false;
        for byte in content {
            let ch = byte as char;
            if ch == '\x0c' {
                continue;
            } else if ch == '\r' {
                x = 0;
                y += 1;
                last13 = true;
            } else if ch == '\n' {
                if last13 {
                    last13 = false;
                } else {
                    x = 0;
                    y += 1;
                }
            } else {
                if ch != ' ' {
                    self.set((x,y),ch as FungeCell);
                }
                x += 1;
            }
        }
    }
}


pub struct VM {
    pub space: Space,
}
impl VM {
    pub fn new() -> VM {
        VM { space: Space::new() }
    }
}