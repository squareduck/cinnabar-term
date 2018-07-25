use std::io::{self, Stdin, Stdout, Write};

use cinnabar::event::ClickEvent;
use cinnabar::render::{RenderCommand, RenderItem, RenderList, RenderText};

use termion::clear;
use termion::cursor;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

// use std::io::{self, Stdin, Write};

pub type Refs<A> = Vec<Vec<Option<RenderItem<A>>>>;

fn empty_refs<A>(width: u16, height: u16) -> Refs<A> {
    let mut row = Vec::with_capacity(width as usize);
    for x in 0..width {
        let mut col = Vec::with_capacity(height as usize);
        for y in 0..height {
            col.push(None)
        }
        row.push(col);
    }

    row
}

pub struct Screen<A> {
    width: u16,
    height: u16,
    refs: Refs<A>,
    output: MouseTerminal<RawTerminal<Stdout>>,
}

impl<A> Screen<A> {
    pub fn new() -> Self {
        let (width, height) = terminal_size().unwrap();
        let stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

        Screen {
            width,
            height,
            refs: empty_refs(width, height),
            output: stdout,
        }
    }

    pub fn reset_refs(&mut self) {
        self.refs = empty_refs(self.width, self.height);
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn render_from(&mut self, list: RenderList<A>) {
        write!(self.output, "{}", clear::All);
        for item in list.iter() {
            match item {
                RenderCommand::Text(RenderText {
                    position: (x, y),
                    size: (width, height),
                    node,
                }) => {
                    for p_x in (x + 1)..(width + 1) {
                        for p_y in (y + 1)..(height + 1) {
                            self.refs[p_x as usize][p_y as usize] =
                                Some(RenderItem::Text(node.clone()))
                        }
                    }
                    write!(
                        self.output,
                        "{}{}",
                        cursor::Goto(x + 1, y + 1),
                        node.content()
                    )
                }
            };
        }
        self.output.flush().unwrap();
    }

    //
    // # Events
    //
    pub fn click(&self, x: u16, y: u16) -> Option<A> {
        match self.refs[x as usize][y as usize] {
            Some(RenderItem::Text(ref text)) => {
                if let Some(ref handler) = text.handlers().click {
                    Some(handler(ClickEvent {}))
                } else {
                    None
                }
            }
            Some(_) => None,
            None => None,
        }
    }
}
