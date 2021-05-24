use chrono::Timelike;

use std::process;

use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
    executor, time,
    window::Settings as WindowSettings,
    Application, Color, Column, Command, Container, Element, Length, Point, Rectangle, Row,
    Settings, Subscription, Vector,
};
use iced_native::event::Event;
use iced_native::keyboard::Event as KeyboardEvent;

pub fn main() -> iced::Result {
    Visualizer::run(Settings {
        window: WindowSettings {
            size: (400, 400),
            ..WindowSettings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}

struct Visualizer {
    data: Box<[u32]>,
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
                data: Box::new([1, 7, 4, 3, 2, 9, 5]),
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
                .width(Length::Units(100))
                .height(Length::Units(100)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .center_y();

        let row = Row::new().push(canvas).width(Length::Fill);
        Column::new().push(row).into()
    }
}

impl canvas::Program<Message> for Visualizer {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let clock = self.clock.draw(bounds.size(), |frame| {
            let center = frame.center();

            let color = Color::from_rgb8(0xc2, 0x23, 0x30);
            frame.translate(Vector::new(center.x, center.y));

            let stroke = Stroke {
                width: 5f32,
                color,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            let bar = Path::line(Point::new(0f32, 0f32), Point::new(400f32, 200f32));
            let bar2 = Path::line(Point::new(0f32, 0f32), Point::new(20f32, 20f32));

            frame.with_save(|frame| {
                frame.stroke(&bar2, stroke);
                frame.stroke(&bar, stroke);
            })
        });

        vec![clock]
    }
}
