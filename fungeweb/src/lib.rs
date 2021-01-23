use wasm_bindgen::prelude::*;

pub struct Poop(pub bool, pub bool, pub bool);


#[wasm_bindgen]
pub fn perform() -> Poop {
    let mut vm = bigfungeus::vm::VM::new();
    let mut ip = bigfungeus::ip::IP::new();
    vm.space.set((0,0),'v' as bigfungeus::FungeCell);
    vm.space.set((0,1),'@' as bigfungeus::FungeCell);
    let a = ip.go(&mut vm);
    let b = ip.go(&mut vm);
    let c = ip.go(&mut vm);
    Poop(a,b,c)
    
    
}
