use std::io::{self, Stdin, Write};

use cinnabar::App;

use termion::cursor::{self, DetectCursorPos};
use termion::event::*;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use screen::Screen;

pub fn run<S, M, A>(initial_app: App<S, M, A>) {
    let mut app = initial_app;

    let mut screen = Screen::new();
    let stdin = io::stdin();

    let list = app.render_list((0, 0), screen.size());
    screen.render_from(list);

    for c in stdin.events() {
        let event = c.unwrap();
        match event {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(m) => match m {
                MouseEvent::Release(x, y) => {
                    if let Some(action) = screen.click(x, y) {
                        app = app.action(action);
                    }
                }
                _ => {}
            },
            _ => {}
        }

        let list = app.render_list((0, 0), screen.size());
        screen.render_from(list);
    }
}
