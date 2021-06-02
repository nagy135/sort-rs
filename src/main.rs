#[allow(unused_imports)]
use chrono::{offset::Utc, DateTime, Timelike};

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
const DONE_DELAY: i64 = 3;

const DATA_SIZE: usize = 40;

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
    slides: Vec<Vec<u32>>,
    columns: usize,
    index: usize,
    max: u32,
    clock: Cache,
    done: i64,
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
        let data = generate_random_array(DATA_SIZE);
        (
            Visualizer {
                slides: sorters::Sorters::bubble_sort(&data),
                clock: Default::default(),
                columns: data.len(),
                max: *data.iter().max().unwrap(),
                done: 0,
                index: 0,
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
                self.tick();
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
            time::every(std::time::Duration::from_millis(30))
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
    fn tick(&mut self) {
        if self.index == self.slides.len() - 1 {
            let now = Utc::now().timestamp();
            if self.done == 0 {
                self.done = now;
            } else if (now - self.done) >= DONE_DELAY {
                self.index = 0;
                self.done = 0;
            }
        } else {
            self.index = (self.index + 1) % self.slides.len();
        }
    }
}
impl canvas::Program<Message> for Visualizer {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let program = self.clock.draw(bounds.size(), |frame| {
            frame.fill_rectangle(
                Point::new(0f32, 0f32),
                Size::new(WIDTH as f32, HEIGHT as f32),
                Color::WHITE,
            );
            let shift: f32 = (WIDTH as f32 - BAR_WIDTH / 2f32) / self.columns as f32;
            let mut position = 0f32;
            frame.fill_text(format!("Frame {}/{}", self.index + 1, self.slides.len()));
            for data_point in self.slides[self.index].iter() {
                let height = HEIGHT as f32 * (*data_point as f32 / self.max as f32) as f32;
                frame.fill_rectangle(
                    Point::new(position, HEIGHT as f32),
                    Size::new(WIDTH as f32 / self.columns as f32, -height),
                    Color::BLACK,
                );
                position += shift;
            }
        });

        vec![program]
    }
}

fn generate_random_array(size: usize) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::with_capacity(size);
    for _ in 0..result.capacity() {
        result.push(rand::random());
    }
    result
}
