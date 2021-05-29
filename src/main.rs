#[allow(unused_imports)]
use chrono::Timelike;

use std::process;

mod sorters;

use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry},
    executor, time,
    window::Settings as WindowSettings,
    Application, Color, Column, Command, Container, Element, Length, Point, Rectangle, Row,
    Settings, Size, Subscription,
};
use iced_native::event::Event;
use iced_native::keyboard::Event as KeyboardEvent;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const BAR_WIDTH: f32 = 20f32;

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
                data: vec![15, 22, 25, 1],
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
            let shift: f32 = (WIDTH as f32 - BAR_WIDTH / 2f32) / self.data.len() as f32;
            let mut position = 0f32;
            let max = *self.data.iter().max().unwrap() as f32;
            for data_point in self.data.iter() {
                let height = HEIGHT as f32 * (*data_point as f32 / max) as f32;
                frame.fill_rectangle(
                    Point::new(position, 0f32),
                    Size::new(WIDTH as f32 / self.data.len() as f32, height),
                    Color::BLACK,
                );
                position += shift;
            }
        });

        vec![program]
    }
}
