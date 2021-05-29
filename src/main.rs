#[allow(unused_imports)]
use chrono::Timelike;

use std::process;

mod sorters;

use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
    executor, time,
    window::Settings as WindowSettings,
    Application, Color, Column, Command, Container, Element, Length, Point, Rectangle, Row,
    Settings, Subscription,
};
use iced_native::event::Event;
use iced_native::keyboard::Event as KeyboardEvent;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

pub fn main() -> iced::Result {
    Visualizer::run(Settings {
        window: WindowSettings {
            size: (WIDTH, HEIGHT),
            ..WindowSettings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}

struct Visualizer {
    data: Vec<u32>,
    sorter: sorters::BubbleSort,
    clock: Cache,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
    EventOccured(iced_native::Event),
}

impl Application for Visualizer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Visualizer {
                data: vec![15, 22, 25, 1, 7, 33, 4, 3, 22, 2, 15, 18, 19, 9, 5],
                sorter: sorters::BubbleSort::new(),
                clock: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Sort-rs")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let _now = local_time;
                self.update_data();
                self.clock.clear();
            }
            Message::EventOccured(event) => {
                if let Event::Keyboard(keyboard_event) = event {
                    if let KeyboardEvent::CharacterReceived(ch) = keyboard_event {
                        match ch {
                            'q' => {
                                process::exit(0);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced_native::subscription::events().map(Message::EventOccured),
            time::every(std::time::Duration::from_millis(500))
                .map(|_| Message::Tick(chrono::Local::now())),
        ])
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Container::new(
            Canvas::new(self)
                .width(Length::Units(WIDTH as u16))
                .height(Length::Units(HEIGHT as u16)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .center_y();

        let row = Row::new().push(canvas).width(Length::Fill);
        Column::new().push(row).into()
    }
}

impl Visualizer {
    fn update_data(&mut self) {
        self.sorter.tick(&mut self.data);
    }
}

impl canvas::Program<Message> for Visualizer {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let program = self.clock.draw(bounds.size(), |frame| {
            let shift: f32 = WIDTH as f32 / self.data.len() as f32;
            let mut position = 0f32;
            for data_point in self.data.iter() {
                let line = Path::line(
                    Point::new(position, 0f32),
                    Point::new(position, (data_point.clone() * 10) as f32),
                );
                frame.stroke(&line, stroke_setup("red", 3f32));
                position += shift
            }
        });

        vec![program]
    }
}

// utils {{{
fn stroke_setup(color: &str, width: f32) -> Stroke {
    let color = match color {
        "red" => Color::from_rgb8(0xc2, 0x23, 0x30),
        "blue" => Color::from_rgb8(0x12, 0x13, 0xc0),
        "green" => Color::from_rgb8(0x12, 0xf3, 0x10),
        _ => Color::BLACK,
    };
    Stroke {
        width,
        color,
        line_cap: LineCap::Round,
        ..Stroke::default()
    }
}
// }}}
