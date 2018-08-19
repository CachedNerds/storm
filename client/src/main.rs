extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::{Relm, Widget};
use relm::Update;
use gtk::{Button, DrawingArea, Window, WindowType, WidgetExt};

#[derive(Clone)]
struct Model {
}

#[derive(Msg)]
enum Msg {
    SomeEvent,
    Quit,
}

#[derive(Clone)]
struct Win {
    // …
    model: Model,
    window: Window,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
        }
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    // Futures and streams can be connected to send a message when a value is ready.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::SomeEvent => {
//                let future = create_future();
//                relm.connect_exec_ignore_err(future, SomeEvent);
            },
            Msg::Quit => gtk::main_quit(),
        }
    }

    // The next method is optional.
    // Futures and streams can be connected when the `Widget` is created in the
    // `subscriptions()` method.
    // fn subscriptions(&mut self, relm: &Relm<Self>) {
    //     let stream = Interval::new(Duration::from_secs(1));
    //     relm.connect_exec_ignore_err(stream, Tick);
    // }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);

        // Connect the signal `delete_event` to send the `Quit` message.
//        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();


        Win {
            model,
            window: window,
        }
    }
}

fn main() {
    println!("Hello, storm client!");
    fn main() {
        Win::run(()).unwrap();
    }
}
