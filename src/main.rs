#[macro_use]
extern crate cinnabar;
extern crate cinnabar_term;

use cinnabar::{App, Template};
use cinnabar_term::run;

fn main() {
    pub struct Store {
        points: usize,
    }

    pub enum Message {}

    pub enum Action {
        None,
        Increment,
    }

    elements_for!(Store, Message, Action);

    let store = Store { points: 0 };

    let simple = Template::new(|store: &Store, _message: &Option<Message>| {
        let points = format!("{} points", store.points);
        text(points).on_click(|_event| Action::Increment).done()
    });

    // let counter = Template::new(|store: &Store, _message: &Option<Message>| {
    //     let points = format!("{} points", store.points);
    //     panel()
    //         .child(text(points))
    //         .child(button().child(text("Increment")))
    //         .done()
    // });

    let mut app = App::new(
        store,
        simple.clone(),
        |store: Store, action: Action| match action {
            Action::Increment => Store {
                points: store.points + 1,
            },
            Action::None => store,
        },
    );

    run(app);
}
