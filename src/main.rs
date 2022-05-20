#![feature(vec_into_raw_parts)]

use iced::{Element, Settings, Column, Text, Alignment, image, Application, Subscription, Command};
use dxgcap::{BGRA8, DXGIManager};

struct ScreenViewer {
    size: (u32, u32),
    screen: image::Handle,
    dxgi: DXGIManager
}

impl Application for ScreenViewer {
    type Executor = iced::executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<()>) {
        let mut dxgi_manager = DXGIManager::new(1000 * 60).unwrap();
        let temp = dxgi_manager.capture_frame().unwrap();
        let width = temp.1.0 as u32;
        let height = temp.1.1 as u32;
        (
            ScreenViewer{
                size: (width, height),
                screen: image::Handle::from_pixels(width, height, pixels_from(temp.0)),
                dxgi: dxgi_manager
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("deskmirror")
    }

    fn update(&mut self, message: Self::Message) -> Command<()> {
        match message {
            _ => {

            }
        }
        let temp = self.dxgi.capture_frame().unwrap();
        self.screen = image::Handle::from_pixels(self.size.0, self.size.1, pixels_from(temp.0));
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(Text::new(self.size.0.to_string()).size(50))
            .push(Text::new(self.size.1.to_string()).size(50))
            .push(image::Image::new(self.screen.clone()))
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(50)).map(|_|{
            ()
        })
    }
}

fn main() -> iced::Result {
    ScreenViewer::run(Settings::default())
}

fn pixels_from(vec: Vec<BGRA8>) -> Vec<u8> {
    unsafe {
        let(a, b, c) = vec.into_raw_parts();
        let ptr = a as *mut u8;
        Vec::from_raw_parts(ptr, 4*b, 4*c)
    }
}