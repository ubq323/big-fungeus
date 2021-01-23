use std::fs;
use std::io::Read;

#[derive(Debug)]
struct CmdLineArgs {
    filename: String,
}

fn get_args() -> Result<CmdLineArgs,String> {
    let mut argobj = CmdLineArgs { 
        filename: "".to_string(),
    };

    let mut found_filename = false;

    let args: Vec<String> = std::env::args().collect();
    let mut idx = 1;
    while idx < args.len() {
        match &args[idx] {
            arg if arg.chars().next() == Some('-') => {
                match arg.as_str() {
                    _ => {
                        return Err(format!("unknown argument: {}",arg).into());
                    }
                }
            },
            arg => {
                // must be filename
                argobj.filename = arg.to_string();
                found_filename = true;
                break;
            }
        }
        idx += 1;
    }
    if found_filename {
        Ok(argobj)
    } else {
        Err("please supply an input filename".to_string())
    }
}

fn main() {
    match get_args() {
        Err(msg) => {
            eprintln!("{}",msg);
            std::process::exit(1);
        },
        Ok(args) => {
            println!("{:?}",args);
            let mut file = fs::File::open(args.filename).expect("couldn't open file");
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).expect("couldn't read file");

        
            bigfungeus::mainloop(buf);
            /*
            let mut vm = vm::VM::new();
            let mut ip = ip::IP::new();
        
            vm.space.load(buf);
            
            let mut iters = 0;
            
            
            
            let mut h = String::new();
            loop {
                iters += 1;
                if debug {
                    eprintln!("{} {:?} - {:?} {} +{:?}",iters,ip.pos,ip.stack,ip.string_mode,ip.delta);
                    eprintln!("+---V---+");
                    for dy in -3..=3 {
                        eprint!("{}",if dy == 0 { "-" } else {"|"} );
                        for dx in -3..=3 {
                            eprint!("{}",std::char::from_u32(vm.space.get((ip.pos.x+dx,ip.pos.y+dy)) as u32).unwrap());
                        }
                        eprintln!("{}",if dy == 0 { "-" } else {"|"} );
                    }
                    eprintln!("+---^---+");
                    std::io::stdin().read_line(&mut h).unwrap();
                }
                if !ip.go(&mut vm) {
                    break;
                }
            }
            */
            println!("ok")
        }
    }
}
