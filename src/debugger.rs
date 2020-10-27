use pancurses::{initscr,Input,endwin};

pub fn debugger_mainloop(program: Vec<u8>) {
    let window = initscr();
    pancurses::noecho();
    window.printw("gaming");
    window.refresh();
    window.keypad(true);
    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                window.addch(c);
            },
            Some(Input::KeyDC) => break,
            Some(input) => {
                window.addstr(&format!("{:?}",input));
            },
            None => ()

        }
    }
    endwin();
}