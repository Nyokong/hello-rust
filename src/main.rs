use iced::widget::{button, text,column, container, row};
use iced::{Fill, Element};

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(counter: &mut u64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
    }
}

fn view(counter: &u64) -> Element<Message> {
    button(text(counter)).on_press(Message::Increment).into();
}


pub fn main() -> iced::Result {
    iced::run("A cool counter", update, view)
}


// struct DrawingApp {
//     points: Vec<Point>,
// }

// impl Sandbox for DrawingApp {
//     type Message = ();

//     fn new() -> Self {
//         Self { points: Vec::new() }
//     }

//     fn title(&self) -> String {
//         String::from("Drawing App")
//     }

//     fn update(&mut self, _message: Self::Message) {}

//     fn view(&mut self) -> Element<Self::Message> {
//         Canvas::new(self)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .into()
//     }
// }

// impl iced::canvas::Program<()> for DrawingApp {
//     fn draw(&self, bounds: iced::Rectangle, _cursor: iced::canvas::Cursor) -> Vec<iced::canvas::Geometry> {
//         let mut frame = iced::canvas::Frame::new(bounds.size());
//         for point in &self.points {
//             frame.fill(&Path::circle(*point, 2.0), Color::BLACK);
//         }
//         vec![frame.into_geometry()]
//     }
// }
