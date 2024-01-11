use std::path::Path;

use playbins::BasicPlayer;

mod playbins;
mod player;

// fn main() {
//     // "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/TearsOfSteel.mp4".into(),
//     let (mut player, reciver) = player::Player::start();

//     let url = Path::new(
//         "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
//     );
//     player.set_source(url);

//     smol::block_on(async move {
//         while let Ok(event) = reciver.recv().await {
//             match event {
//                 player::Event::Frame(frame) => {
//                     println!("frame: {:?}", frame.width());
//                 }
//             }
//         }
//         print!("end")
//     });

//     std::thread::sleep(std::time::Duration::from_secs(10));
// }

use iced::{
    executor,
    futures::SinkExt,
    widget::{self, container, image},
    Application, Color, Command, Element, Length,
};
use player::video_frame::VideoFunctions;

fn main() {
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Frame(iced::widget::image::Handle),
}

struct App {
    frame: Option<iced::widget::image::Handle>,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App { frame: None }, Command::none())
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::channel("some rnado id", 100, |mut ouput| async move {
            let (mut player, reciver) = player::Player::start();

            let url = Path::new(
                "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            );
            player.set_source(url);

            while let Ok(frame) = reciver.recv().await {
                match frame {
                    player::Event::Frame(mut frame) => {
                        let pixels = test(&mut frame).unwrap();
                        if let Err(err) = ouput.send(Message::Frame(pixels)).await {
                            println!("error: {:?}", err);
                            break;
                        };
                    }
                }
            }
            println!("end");

            loop {
                iced::futures::pending!()
            }
        })
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Frame(frame) => {
                self.frame = Some(frame);
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        container(if let Some(image) = &self.frame {
            Element::from(
                widget::Image::new(image.clone())
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .explain(Color::BLACK)
        } else {
            Element::from(
                widget::Image::new(iced::widget::image::Handle::from_memory([]))
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
        })
        .center_x()
        .center_y()
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

pub fn test(frame: &ffmpeg::util::frame::Video) -> Result<image::Handle, anyhow::Error> {
    let mut buffer = vec![0; (frame.height() * frame.width() * 4) as usize];
    let ffmpeg_line_iter = frame.data(0).chunks_exact(frame.stride(0));

    let slint_pixel_line_iter = buffer.chunks_exact_mut(frame.stride(0));

    for (source_line, dest_line) in ffmpeg_line_iter.zip(slint_pixel_line_iter) {
        dest_line.copy_from_slice(&source_line[..])
    }

    Ok(image::Handle::from_pixels(
        frame.width() as u32,
        frame.height() as u32,
        buffer,
    ))
}

/// Convert a `ffmpeg::Frame` to a `Frame` struct.
pub fn convert_frame_to_image_handle(
    frame: &mut ffmpeg::Frame,
) -> Result<image::Handle, anyhow::Error> {
    let frame_width = frame.width();
    let frame_height = frame.height();
    let frame_format = frame.format();
    let src_data = frame.data();
    let src_linesize = frame.linesize();

    println!("frame: {:?}", frame.width());
    println!("frame: {:?}", frame.height());
    println!("frame: {:?}", frame.format());
    println!("frame: {:?}", frame.data());
    println!("frame: {:?}", frame.linesize());

    let frame_array = vec![0; (frame_height * frame_width * 4) as usize];

    let pixels = copy_image_to_buffer(
        frame_array,
        src_data,
        src_linesize,
        frame_format,
        frame_width,
        frame_height,
    )?;

    Ok(image::Handle::from_pixels(
        frame_width as u32,
        frame_height as u32,
        pixels,
    ))
}

fn copy_image_to_buffer(
    mut buffer: Vec<u8>,
    src_data: [*mut u8; 8],
    src_linesize: [i32; 8],
    frame_format: ffmpeg::ffi::AVPixelFormat,
    frame_width: i32,
    frame_height: i32,
) -> Result<Vec<u8>, anyhow::Error> {
    let bytes_copied = unsafe {
        ffmpeg::ffi::av_image_copy_to_buffer(
            buffer.as_mut_ptr(),
            buffer.len() as i32,
            src_data.as_ptr() as *const *const u8,
            src_linesize.as_ptr() as *const i32,
            frame_format,
            frame_width,
            frame_height,
            1,
        )
    };

    if bytes_copied == buffer.len() as i32 {
        Ok(buffer)
    } else {
        println!(
            "Failed to copy image to buffer: {} should be {}",
            bytes_copied,
            buffer.len()
        );
        tracing::error!(
            "Failed to copy image to buffer: {} should be {}",
            bytes_copied,
            buffer.len()
        );
        Err(anyhow::Error::msg(
            "Failed to copy image to buffer: {} should be {}",
        ))
    }
}
