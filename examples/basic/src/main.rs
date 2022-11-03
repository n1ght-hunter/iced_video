use std::{cell::Cell, sync::mpsc};

use iced::{
    executor, subscription,
    widget::{button, column, container, text, Image, Text},
    Application, Command, Event, Length, Settings, Theme,
};
use iced_native::{image, window};
use video_player::{GSTMessage, VideoFormat, VideoPlayer, FlowError, FlowSuccess, MessageView, ElementExt, PadExt, Continue};

fn main() {
    std::env::set_var("GST_DEBUG", "3");
    App::run(Settings {
        exit_on_close_request: false,
        ..Default::default()
    })
    .unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Video(SubMSG),
    PausePlay,
    EventOccurred(iced_native::Event),
}

struct App {
    receiver: Cell<Option<mpsc::Receiver<SubMSG>>>,
    player: VideoPlayer,
    frame: Option<image::Handle>,
    should_exit: bool,
    starting_exit: bool,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (player, receiver) = subscription();
        (
            App {
                receiver: Some(receiver).into(),
                player,
                frame: None,
                should_exit: false,
                starting_exit: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Video(event) => match event {
                SubMSG::Image(image) => {
                    self.frame = Some(image);
                }
                SubMSG::Message(message) => match message {
                    GSTMessage::Eos => {
                        if self.starting_exit == true {
                            self.should_exit = true;
                        }
                    }
                    _ => (),
                },
            },
            Message::PausePlay => {
                let player = &mut self.player;
                if player.paused() {
                    player.set_paused(false);
                } else {
                    player.set_paused(true);
                }
            }
            Message::EventOccurred(event) => {
                println!("event: {:?}", event);
                if let Event::Window(window::Event::CloseRequested) = event {
                    println!("close");
                    self.player.exit().unwrap();
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let receiver = self.receiver.replace(None);

        iced::Subscription::batch(vec![
            iced_native::subscription::events().map(Message::EventOccurred),
            subscription::unfold("subscription", receiver, |mut stream| async move {
                let item = stream.as_mut().unwrap().recv().unwrap();
        
                (Some(item), stream)
            }).map(Message::Video),
        ])
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let (width, height) = self.player.size();
        let image = if self.frame.is_some() {
            Image::new(self.frame.clone().unwrap())
                .height(Length::Units(height as u16))
                .width(Length::Units(width as u16))
        } else {
            Image::new(image::Handle::from_pixels(0, 0, vec![]))
        };
        let button = button(text("pause/play")).on_press(Message::PausePlay);

        let text = Text::new(format!(
            "{:#?}s / {:#?}s",
            self.player.position().as_secs(),
            self.player.duration().as_secs()
        ));
        container(column![image, text, button])
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Clone, Debug)]
enum SubMSG {
    Image(image::Handle),
    Message(GSTMessage),
}

fn subscription() -> (VideoPlayer, mpsc::Receiver<SubMSG>) {
    let (sender, receiver) = mpsc::channel::<SubMSG>();

    let sender1 = sender.clone();

    let player = VideoPlayer::new(
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm",
        false,
        VideoFormat::Bgra,
        move |sink| {
            let sample = sink.pull_sample().map_err(|_| FlowError::Eos)?;
            let buffer = sample.buffer().ok_or(FlowError::Error)?;
            let map = buffer.map_readable().map_err(|_| FlowError::Error)?;

            let pad = sink.static_pad("sink").ok_or(FlowError::Error)?;

            let caps = pad.current_caps().ok_or(FlowError::Error)?;
            let s = caps.structure(0).ok_or(FlowError::Error)?;
            let width = s.get::<i32>("width").map_err(|_| FlowError::Error)?;
            let height = s.get::<i32>("height").map_err(|_| FlowError::Error)?;

            if let Some(buffer) = sample.buffer_owned() {
                let image = image::Handle::from_pixels(
                    width as u32,
                    height as u32,
                    map.as_slice().to_owned(),
                );
                sender.send(SubMSG::Image(image)).expect("unable to send");
            }

            Ok(FlowSuccess::Ok)
        },
        move |_bus, msg| {
            let view = msg.view();

            let message = match view {
                MessageView::Eos(_) => GSTMessage::Eos,
                MessageView::Error(_) => GSTMessage::Error,
                MessageView::Warning(_) => GSTMessage::Warning,
                MessageView::Info(_) => GSTMessage::Info,
                MessageView::Tag(_) => GSTMessage::Tag,
                MessageView::Buffering(_) => GSTMessage::Buffering,
                MessageView::StateChanged(_) => GSTMessage::StateChanged,
                MessageView::StateDirty(_) => GSTMessage::StateDirty,
                MessageView::StepDone(_) => GSTMessage::StepDone,
                MessageView::ClockProvide(_) => GSTMessage::ClockProvide,
                MessageView::ClockLost(_) => GSTMessage::ClockLost,
                MessageView::NewClock(_) => GSTMessage::NewClock,
                MessageView::StructureChange(_) => GSTMessage::StructureChange,
                MessageView::StreamStatus(_) => GSTMessage::StreamStatus,
                MessageView::Application(_) => GSTMessage::Application,
                MessageView::Element(_) => GSTMessage::Element,
                MessageView::SegmentStart(_) => GSTMessage::SegmentStart,
                MessageView::SegmentDone(_) => GSTMessage::SegmentDone,
                MessageView::DurationChanged(_) => GSTMessage::DurationChanged,
                MessageView::Latency(_) => GSTMessage::Latency,
                MessageView::AsyncStart(_) => GSTMessage::AsyncStart,
                MessageView::AsyncDone(_) => GSTMessage::AsyncDone,
                MessageView::RequestState(_) => GSTMessage::RequestState,
                MessageView::StepStart(_) => GSTMessage::StepStart,
                MessageView::Qos(_) => GSTMessage::Qos,
                MessageView::Progress(_) => GSTMessage::Progress,
                MessageView::Toc(_) => GSTMessage::Toc,
                MessageView::ResetTime(_) => GSTMessage::ResetTime,
                MessageView::StreamStart(_) => GSTMessage::StreamStart,
                MessageView::NeedContext(_) => GSTMessage::NeedContext,
                MessageView::HaveContext(_) => GSTMessage::HaveContext,
                MessageView::DeviceAdded(_) => GSTMessage::DeviceAdded,
                MessageView::DeviceRemoved(_) => GSTMessage::DeviceRemoved,
                MessageView::PropertyNotify(_) => GSTMessage::PropertyNotify,
                MessageView::StreamCollection(_) => GSTMessage::StreamCollection,
                MessageView::StreamsSelected(_) => GSTMessage::StreamsSelected,
                MessageView::Redirect(_) => GSTMessage::Redirect,
                MessageView::Other => GSTMessage::Other,
                _ => GSTMessage::Other,
            };

            sender1
                .send(SubMSG::Message(message))
                .expect("unable to send message");

            // Tell the mainloop to continue executing this callback.
            Continue(true)
        },
    )
    .unwrap();
    (player, receiver)
}
