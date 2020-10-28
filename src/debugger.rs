use pancurses::{initscr,Input,endwin,COLOR_PAIR};

use crate::vm::VM;
use crate::ip::IP;

use std::{thread,time};

pub fn debugger_mainloop(program: Vec<u8>) {
    let ten_millis = time::Duration::from_millis(10);
    let window = initscr();
    pancurses::noecho();
    window.keypad(true);

    let mut vm = VM::new();
    let mut ip = IP::new();

    vm.space.load(program);

    pancurses::start_color();
    pancurses::init_pair(1,pancurses::COLOR_WHITE,pancurses::COLOR_RED);
    pancurses::init_pair(2,pancurses::COLOR_CYAN,pancurses::COLOR_BLACK);

    window.attron(COLOR_PAIR(2));
    window.mvaddstr(0,0,"┌");
    window.mvaddstr(0,52,"┐");
    window.mvaddstr(52,0,"└");
    window.mvaddstr(52,52,"┘");

    for i in 1..52 {
        window.mvaddstr(0,i,"─");
        window.mvaddstr(52,i,"─");
        window.mvaddstr(i,0,"│");
        window.mvaddstr(i,52,"│");
    }
    window.attroff(COLOR_PAIR(2));

    window.refresh();
    window.getch();

    'outer: loop {
        for dx in 0..50 {
            for dy in 0..50 {
                let ch = vm.space.get((ip.pos.x - 25 + dx as i64,ip.pos.y - 25 + dy as i64));
                let ch = std::char::from_u32(ch as u32).unwrap_or('\u{FFFD}');
                if dx == 25 && dy == 25 {
                    window.attron(COLOR_PAIR(1));
                    window.mvaddch(dx,dy,ch);
                    window.attroff(COLOR_PAIR(1));
                } else {
                    window.mvaddch(dy+1,dx+1,ch);
                }
            }
        }
        window.refresh();
        thread::sleep(ten_millis);
        //window.getch();
        //for _ in 1..100 {
            if !ip.go(&mut vm) {
                break 'outer;
            }
        //}
    }
    endwin();
}