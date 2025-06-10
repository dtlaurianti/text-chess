use ncurses;

fn main() {
    ncurses::initscr();
    ncurses::addstr("Hello, world!").unwrap();
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();
}
