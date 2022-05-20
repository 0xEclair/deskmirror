#![feature(vec_into_raw_parts)]

use iced::{Element, Settings, Column, Text, Alignment, image, Application, Subscription, Command};
use dxgcap::DXGIManager;

struct ScreenViewer {
    size: (u32, u32),
    dxgi: DXGIManager
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Default(u8)
}

impl Application for ScreenViewer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Message>) {
        let mut dxgi_manager = DXGIManager::new(1000 * 60).unwrap();
        let temp = dxgi_manager.capture_frame().unwrap();
        let width = temp.1.0 as u32;
        let height = temp.1.1 as u32;
        (
            ScreenViewer{
                size: (width, height),
                dxgi: dxgi_manager
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("deskmirror")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            _ => {

            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(16)).map(|_|{
            Message::Default(5)
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(Text::new(self.size.0.to_string()).size(50))
            .push(Text::new(self.size.1.to_string()).size(50))
            .push(image::Image::new(
                image::Handle::from_pixels(
                    self.size.0,
                    self.size.1,
                    self.dxgi.capture_frame_components().unwrap().0
                ))
            )
            .into()
    }
}

fn main() -> iced::Result {
    ScreenViewer::run(Settings::default())
}