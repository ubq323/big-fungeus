use std::io::Read;

use crate::vm::Space;
use crate::FungeCell;
use crate::util::Vec2D;
use crate::vm::VM;

type Stack = Vec<FungeCell>;
type StackStack = Vec<Stack>;

const NORTH: Vec2D = Vec2D::new( 0,-1);
const SOUTH: Vec2D = Vec2D::new( 0, 1);
const WEST:  Vec2D = Vec2D::new(-1, 0);
const EAST:  Vec2D = Vec2D::new( 1, 0);

pub struct IP {
    // pub only for temp debugging
    pub pos: Vec2D,
    pub delta: Vec2D,
    pub stack: Stack, // or TOSS
    pub string_mode: bool,
    last_space: bool,

    storage_offset: Vec2D,

    stack_stack: StackStack, // end is SOSS, because TOSS is stored separately
}

impl IP {
    pub fn new() -> IP {
       IP { 
           pos: Vec2D::zero(),
           delta: EAST,
           stack: Stack::default(),
           string_mode: false,
           last_space: false,
           storage_offset: Vec2D::zero(),
           stack_stack: StackStack::default(),
       } 
    }
    pub fn go(&mut self, vm: &mut VM) -> bool {
        let instr = vm.space.get(self.pos.into());
        let res = self.exec_instr(instr,vm);
        self.proceed_to_next(vm);
        res
    }
    pub fn popstack(&mut self) -> FungeCell {
        self.stack.pop().unwrap_or(0)
    }

    fn proceed_to_next(&mut self, vm: &mut VM) {
        let mut semicolon_mode = false;
        loop {
            self.pos += self.delta;
            self.wrap_around(&mut vm.space);
            let instr = vm.space.get(self.pos.into());
            if semicolon_mode {
                if instr == (';' as i64) {
                    semicolon_mode = false;
                }
            } else {
                if self.string_mode { break; }
                if instr == ' ' as i64 { 
                    continue;
                } else if instr == ';' as i64 {
                    semicolon_mode = true;
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    fn wrap_around(&mut self, space: &mut Space) {
        fn in_bounds(pos: Vec2D, space: &Space) -> bool {
            pos.x >= space.minx() && pos.x <= space.maxx() && pos.y >= space.miny() && pos.y <= space.maxy()
        }
        if in_bounds(self.pos,space) {
            return;
        } else {
            self.delta = self.delta.reflect();
            while !in_bounds(self.pos, space) {
                self.pos += self.delta;
            }
            while in_bounds(self.pos, space) {
                self.pos += self.delta;
            }
            self.delta = self.delta.reflect();
        }
    }

    fn exec_instr(&mut self,instr: FungeCell,vm: &mut VM) -> bool {
        let mut running = true;
        if let Some(instr_char) = std::char::from_u32(instr as u32) {
            if self.string_mode {
                if instr_char == '"' {
                    self.string_mode = false;
                } else {
                    if instr_char == ' ' {
                        if self.last_space {
                            return true;
                        } else {
                            self.stack.push(instr);
                            self.last_space = true;
                        }
                    } else {
                        self.stack.push(instr);
                        self.last_space = false;
                    }
                }
            } else {
                match instr_char {
                    ' ' => (),
                    '.' => print!("{} ",self.popstack()),
                    ',' => print!("{}",std::char::from_u32(self.popstack() as u32).unwrap_or('?')),
                    '~' => {
                        let byte = std::io::stdin().bytes().next().unwrap_or(Ok('\n' as u8)).expect("stdin error");
                        self.stack.push(byte as FungeCell);
                    },
                    /*'&' => {
                        
                    }*/
                    
                    '0'..='9' => {
                        let digit = ((instr_char as u32) - ('0' as u32)) as FungeCell;
                        self.stack.push(digit); 
                    },
                    'a'..='f' => {
                        let digit = ((instr_char as u32) - ('a' as u32)) as FungeCell;
                        self.stack.push(digit+10);
                    },

                    'y' => {
                        let n = self.popstack();
                        let orig_stack_size = self.stack.len() as FungeCell;

                        // 20. envvars (skipped)
                        self.stack.push(0);
                        self.stack.push(0);

                        // 19. cmdline (skipped);
                        self.stack.push(0);
                        self.stack.push(0);

                        // 18. stackstack sizes
                        for stack in &self.stack_stack {
                            self.stack.push(stack.len() as FungeCell);
                        }
                        self.stack.push(orig_stack_size);

                        // 17. size of stackstack
                        self.stack.push(self.stack_stack.len() as FungeCell + 1);

                        // not doing datetime for now bc effort
                        // so hardcoded value instead
                        // 16. time
                        let (hour,minute,second) = (23,15,59);
                        self.stack.push((hour*256*256)+(minute*256)+second);

                        // 15. date
                        let (year,month,day) = (2020,10,25);
                        self.stack.push(((year-1900)*256*256)+(month*256)+day);

                        // board size
                        vm.space.refresh_bounds();
                        let (minx,miny,maxx,maxy) = (vm.space.minx(),vm.space.miny(),vm.space.maxx(),vm.space.maxy());
                        let (lx,ly) = (maxx-minx,maxy-miny);

                        // 14. greatest point rel. least point
                        self.stack.push(lx);
                        self.stack.push(ly);

                        // 13. least point rel. origin
                        self.stack.push(minx);
                        self.stack.push(miny);

                        // 12. current storage offset
                        self.stack.push(self.storage_offset.x);
                        self.stack.push(self.storage_offset.y);

                        // 11. current delta
                        self.stack.push(self.delta.x);
                        self.stack.push(self.delta.y);

                        // 10. current position
                        self.stack.push(self.pos.x);
                        self.stack.push(self.pos.y);

                        // 9. team number. skipping for now
                        self.stack.push(0);

                        // 8. ip id. skipping for now
                        self.stack.push(0);

                        // 7. number of dimensions. currently we only support befunge
                        self.stack.push(2);

                        // 6. path separator, for some reason
                        self.stack.push(std::path::MAIN_SEPARATOR as FungeCell);

                        // 5. operating paradigm id (semantics for = command)
                        // unavailable for now, probably into the future as well
                        self.stack.push(0);

                        // 4. version number
                        self.stack.push(0);

                        // 3. handprint
                        self.stack.push(0x42464753); // b"BFGS"

                        // 2. cell size.
                        self.stack.push(8); // bytes

                        // 1. env flags 
                        // 000u=oit where u is unbuffered io
                        // we don't have any of those things (yet)
                        //            000u=oit
                        let flags = 0b00000000;
                        self.stack.push(flags);

                        let stack_size_now = self.stack.len();
                        if n > 0 {
                            let c = self.stack[stack_size_now-n as usize];
                            self.stack.truncate(orig_stack_size as usize);
                            self.stack.push(c);
                        }

                    },

                    '?' => {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        self.delta = match rng.gen_range(0,4) {
                            0 => NORTH,
                            1 => EAST,
                            2 => SOUTH,
                            _ => WEST,
                        };
                    },
                    
                    '<' => self.delta = WEST,
                    '>' => self.delta = EAST,
                    '^' => self.delta = NORTH,
                    'v' => self.delta = SOUTH,
                    'r' => self.delta = self.delta.reflect(),
                    '#' => self.pos += self.delta,
                    '[' => self.delta = self.delta.rotl(),
                    ']' => self.delta = self.delta.rotr(),
                    'k' => {
                        let count = self.popstack();
                        if count == 0 {
                            self.proceed_to_next(vm);
                            self.proceed_to_next(vm);
                        } else {
                            let oldpos = self.pos;
                            self.proceed_to_next(vm);
                            let instr = vm.space.get(self.pos.into());
                            self.pos = oldpos;
                            for _ in 0..count {
                                self.exec_instr(instr, vm);
                            }
                        }
                    },
                    'j' => {
                        let s = self.popstack();
                        self.pos += self.delta * s;
                    },
                    'x' => {
                        let y = self.popstack();
                        let x = self.popstack();
                        self.delta = (x,y).into();
                    },

                    '{' => {
                        let n = self.popstack();
                        let l = self.stack.len();
                        let moved_items = if n == 0 {
                            vec!()
                        } else if n < 0 {
                            self.stack.append(&mut vec![0;-n as usize]);
                            vec!()
                        } else if n > (l as i64) {
                            // need to transfer l, with n-l 0's before it
                            let mut tmp = vec![0;n as usize-l];
                            tmp.append(&mut self.stack);
                            tmp
                        } else {
                            self.stack.split_off(l - n as usize)
                        };

                        let old_stack = std::mem::replace(&mut self.stack, moved_items);
                        self.stack_stack.push(old_stack);
                        let old_stack = self.stack_stack.last_mut().unwrap(); // can unwrap because we just pushed it

                        let (sx,sy) = self.storage_offset.into();
                        old_stack.push(sx);
                        old_stack.push(sy);
                        self.storage_offset = self.pos + self.delta;

                    },
                    '}' => {
                        let n = self.popstack();

                        if self.stack_stack.len() == 0 {
                            self.delta = self.delta.reflect();
                        } else {
                            let mut soss = self.stack_stack.pop().unwrap(); // can unwrap since we just checked

                            let y = soss.pop().unwrap_or(0);
                            let x = soss.pop().unwrap_or(0);
                            self.storage_offset = (x,y).into();

                            let l = self.stack.len();
                            if n < 0 {
                                soss.truncate(l+n as usize);
                            } else {
                                let mut moved_items = if n == 0 {
                                    vec!()
                                } else if n > (l as i64) {
                                    // want to move too many
                                    let mut tmp = vec![0;n as usize-l];
                                    tmp.append(&mut self.stack);
                                    tmp
                                } else {
                                    self.stack.split_off(l - n as usize)
                                };
                                soss.append(&mut moved_items);
                            }
                            self.stack = soss;
                        }
                    },
                    'u' => {
                        if self.stack_stack.len() == 0 {
                            self.delta = self.delta.reflect();
                        } else {
                            let n = self.popstack();
                            let soss = self.stack_stack.last_mut().unwrap();
                            if n > 0 {
                                for _ in 0..n {
                                    self.stack.push(soss.pop().unwrap_or(0));
                                }
                            } else if n < 0 {
                                for _ in 0..n {
                                    soss.push(self.stack.pop().unwrap_or(0));
                                }
                            }
                                
                        }
                    },
                    
                    '@' => running = false,
                    
                    '$' => {self.stack.pop();},
                    ':' => {
                        let v = self.popstack();
                        self.stack.push(v);
                        self.stack.push(v);
                        // yes we have to do it like that
                    },
                    '\\' => {
                        let b = self.popstack();
                        let a = self.popstack();
                        self.stack.push(b);
                        self.stack.push(a);
                    },
                    'n' => {
                        self.stack.clear();
                    },
                    
                    '"' => { 
                        self.string_mode = true;
                        self.last_space = false;
                    },

                    '_' => self.delta = if self.popstack() == 0 {EAST} else {WEST},
                    '|' => self.delta = if self.popstack() == 0 {SOUTH} else {NORTH},
                    'w' => {
                        let b = self.popstack();
                        let a = self.popstack();
                        if a < b {
                            self.exec_instr('[' as FungeCell, vm);
                        } else if a > b {
                            self.exec_instr(']' as FungeCell, vm);
                        }
                    }

                    '+' => {
                        let b = self.popstack();
                        let a = self.popstack();
                        self.stack.push(a+b);
                    },
                    '-' => {
                        let b = self.popstack();
                        let a = self.popstack();
                        self.stack.push(a-b);
                    },
                    '*' => {
                        let b = self.popstack();
                        let a = self.popstack();
                        self.stack.push(a*b);
                    },
                    '/' => {
                        let b = self.popstack();
                        let a = self.popstack();
                        if b == 0 {
                            self.stack.push(0);
                        } else {
                            self.stack.push(a/b);
                        }
                    },
                    '%' => {
                        let b = self.popstack();
                        let a = self.popstack();
                        if b == 0 {
                            self.stack.push(0);
                        } else {
                            self.stack.push(a%b);
                        }
                    },
                    '`' => {
                        let b = self.popstack();
                        let a = self.popstack();
                        self.stack.push(if a > b {1} else {0});
                    },
                    '!' => {
                        let v = self.popstack();
                        let nv = if v == 0 {1} else {0};
                        self.stack.push(nv);
                    },


                    'p' => {
                        let y = self.popstack();
                        let x = self.popstack();
                        let pos = (self.storage_offset + (x,y).into()).into();
                        let v = self.popstack();
                        vm.space.set(pos,v);
                    },
                    'g' => {
                        let y = self.popstack();
                        let x = self.popstack();
                        let pos = (self.storage_offset + (x,y).into()).into();
                        self.stack.push(vm.space.get(pos));
                    },
                    '\'' => {
                        let ch = vm.space.get((self.pos+self.delta).into());
                        self.stack.push(ch);
                        self.pos += self.delta;
                    },
                    's' => {
                        let ch = self.popstack();
                        vm.space.set((self.pos+self.delta).into(),ch);
                        self.pos += self.delta;
                    },
                    'z' => (),

                    '(' => {
                        // don't actually have any fingerprint support yet
                        // so this doesn't do much
                        let count = self.popstack();
                        for _ in 0..count {
                            self.popstack();
                        }
                        self.delta = self.delta.reflect();
                    },
                    ')' => {
                        let count = self.popstack();
                        for _ in 0..count {
                            self.popstack();
                        }
                        self.delta = self.delta.reflect();
                    },

                    'q' => {
                        // probably not ultimately ideal
                        std::process::exit(self.popstack() as i32);
                    },

                    '\x0c' => (), // form feed

                    
                    _ => {
                        eprintln!("unknown instruction char: {} ({})",instr_char,instr);
                        self.delta = self.delta.reflect();
                        //return false;
                    }  
                } 
            }
        } else {
            eprintln!("unknown instruction value: {}",instr);
            self.delta = self.delta.reflect();
        }
        return running;
    }
}
