use chrono::Timelike;

use std::process;

use iced::{
    canvas::{self, Cache, Cursor, Geometry, Path},
    executor, time,
    window::Settings as WindowSettings,
    Application, Color, Column, Command, Element, Length, Rectangle, Row, Settings, Space,
    Subscription, Vector,
};
use iced_native::event::Event;
use iced_native::keyboard::Event as KeyboardEvent;

pub fn main() -> iced::Result {
    Sorter::run(Settings {
        window: WindowSettings {
            size: (400, 200),
            ..WindowSettings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}

struct Sorter {
    data: u32,
    clock: Cache,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
    EventOccured(iced_native::Event),
}

impl Application for Sorter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Sorter {
                data: 0,
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
        let row = Row::new()
            .push(Space::new(Length::Units(50), Length::Shrink))
            .width(Length::Fill);
        Column::new().padding(20).push(row).into()
    }
}

impl canvas::Program<Message> for Sorter {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let clock = self.clock.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let color = Color::from_rgb8(0xc2, 0x23, 0x30);

            let background = Path::circle(center, radius);
            frame.fill(&background, color);

            frame.translate(Vector::new(center.x, center.y));
        });

        vec![clock]
    }
}
