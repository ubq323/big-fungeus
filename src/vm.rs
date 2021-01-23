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

    pub fn refresh_bounds(&mut self) {
        // the only time we actually need to properly check the current
        // min and max, instead of just keeping track of the greatest
        // ones we have seen so far, is in the y command.
        let (mut maxx,mut maxy,mut minx,mut miny) = (0,0,0,0);
        for (coord, _) in self.cells.iter() {
            // there will never be any 32s in here
            let (x,y) = *coord;
            maxx = max(maxx,x);
            maxy = max(maxy,y);
            minx = min(minx,x);
            miny = min(miny,y);
        }
        self.maxx=maxx;
        self.maxy=maxy;
        self.minx=minx;
        self.miny=miny;
    }

    pub fn get(&mut self, xy: (FungeCell,FungeCell)) -> FungeCell {
        self.cells.get(&xy).copied().unwrap_or(32)
    }
    pub fn set(&mut self, xy: (FungeCell,FungeCell), value: FungeCell) {
        if value == 32 {
            self.cells.remove(&xy);
        } else {
            self.cells.insert(xy,value);
            let (x,y) = xy;
            self.maxx = max(self.maxx,x);
            self.maxy = max(self.maxy,y);
            self.minx = min(self.minx,x);
            self.miny = min(self.miny,y);
        }
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
