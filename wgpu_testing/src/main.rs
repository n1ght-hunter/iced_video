// currently broken

use gpu_controller::State;
use video_player::player::{
    Buffer, Continue, ElementExt, FlowError, FlowSuccess, PadExt, VideoFormat, VideoPlayer,
    VideoSettings,
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

mod gpu_controller;
mod texture;

fn main() {
    pollster::block_on(run());
}

enum EventLoopMSG {
    Image(i32, i32, Buffer),
}

async fn run() {
    env_logger::init();
    let event_loop = EventLoopBuilder::with_user_event().build();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(&window).await;

    let clone_loop = event_loop.create_proxy();

    let mut player = VideoPlayer::new(
        VideoSettings {
            uri:
                Some("http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_string()),
            ..Default::default()
        },
        VideoFormat::Rgba,
        move |sink| {
            let sample = sink.pull_sample().map_err(|_| FlowError::Eos)?;

            let pad = sink.static_pad("sink").ok_or(FlowError::Error)?;

            let caps = pad.current_caps().ok_or(FlowError::Error)?;
            let s = caps.structure(0).ok_or(FlowError::Error)?;
            let width = s.get::<i32>("width").map_err(|_| FlowError::Error)?;
            let height = s.get::<i32>("height").map_err(|_| FlowError::Error)?;

            if let Some(buffer) = sample.buffer_owned() {
                clone_loop
                    .send_event(EventLoopMSG::Image(width, height, buffer))
                    .map_err(|_| FlowError::Error)?;
            }

            Ok(FlowSuccess::Ok)
        },
        |_, _| Continue(true),
    )
    .unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so we have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        WindowEvent::MouseInput {
                            device_id: _,
                            state,
                            button,
                            modifiers: _,
                        } => match button {
                            MouseButton::Left => match state {
                                ElementState::Pressed => player.set_paused_state(!player.paused()),
                                ElementState::Released => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            Event::UserEvent(event) => {
                match event {
                    EventLoopMSG::Image(width, height, buffer) => {
                        state.update(buffer, height as u32, width as u32);
                        match state.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                state.resize(state.size)
                            }
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            // We're ignoring timeouts
                            Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                        }
                    }
                }
            }
            _ => {}
        }
    });
}
