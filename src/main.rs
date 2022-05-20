#![feature(vec_into_raw_parts)]

use iced::{Element, Settings, Sandbox, Column, Text, Alignment};
use dxgcap::DXGIManager;

struct ScreenViewer {
    size: (u32, u32)
}

impl Sandbox for ScreenViewer {
    type Message = String;

    fn new() -> Self {
        let screen = &mut DXGIManager::new(1000 * 60).unwrap();
        let temp = screen.capture_frame().unwrap();
        ScreenViewer{
            size: (temp.1.0 as u32, temp.1.1 as u32),
        }
    }

    fn title(&self) -> String {
        String::from("deskmirror")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            _ => {

            }
        }
        let screen = &mut DXGIManager::new(1000 * 60).unwrap();
        let temp = screen.capture_frame().unwrap();
        println!("each frame of the monitor has: {:?} bytes", temp.0.len());
        println!("its resolution is: {:?} * {:?}", temp.1.0, temp.1.1);
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let screen = &mut DXGIManager::new(1000 * 60).unwrap();
        let temp = screen.capture_frame().unwrap();

        let pixels = unsafe {
            let (a, b, c) = temp.0.into_raw_parts();
            let ptr = a as *mut u8;
            Vec::from_raw_parts(ptr, 4*b, 4*c)
        };
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(Text::new(self.size.0.to_string()).size(50))
            .push(Text::new(self.size.1.to_string()).size(50))
            .push(iced::Image::new(iced::image::Handle::from_pixels(self.size.0, self.size.1,  pixels)))
            .into()
    }
}

fn main() -> iced::Result {
    ScreenViewer::run(Settings::default())
}
