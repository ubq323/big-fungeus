
type FungeCell =  i64;


pub mod vm;
pub mod ip;
mod util;
#[cfg(feature = "debugger")]
pub mod debugger;

// very temporary
pub fn mainloop(program: Vec<u8>) {
    use crate::vm::VM;
    use crate::ip::IP;

    let mut vm = VM::new();
    let mut ip = IP::new();

    vm.space.load(program);

    loop {
        if !ip.go(&mut vm) {
            break;
        }
    }
}
