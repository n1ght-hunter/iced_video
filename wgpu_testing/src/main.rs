use gpu_controller::State;
use gst::{
    traits::{ElementExt, PadExt},
    Buffer,
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

use video_player::{VideoPlayer, VideoFormat};

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
        "https://video-weaver.syd03.hls.ttvnw.net/v1/playlist/CpwFUvU2cqd8q-8W6ZlI9HJ6f3GDKrsgy-w5tJK-0HtxZDjNmiqlyTI3-Aihg4ZSsXXNT1NJXBYt7MdJ2R9qPRjBrW3JaDeq4YXlC0tpfvB9Me-4X4IFXkpH_Nfef39GoCQ6olWQj_LdPPbJsD5ADGlmMbVUYqv9scB1ycwYteEi1NcZHZ-CPS7EXBdAdylYyGkrqn1NHjqD55FECGPJX_Jc84M_arW8gcBN7vM_fRxPtIO9sGvgN806xnK803WiHCuJqw4wkR_5MhZ_HR3wbZdIxwfWQlQ58IEgCj_2YKdtDWPYH4pYZIJvUvKOzS9NOK8Ry0-ecBjXlndI0B19rINDAdOMPlZt8eTQgR8Fdg1VLWIP414uHsaSTDgaVPiH9mB6X2KuZzscDlhpM3Mlhc4PB_VyK57eMeUnYqKl-CuC99SswKFPsTxCiQhYX3RJOEk6yRcRfdq_XKPSjXjFwJuIUPbUOv8NIKXRggr2AqLlDx1yRdRnkGqD4X_xU8VvQwliHD1JntN9nXfi7Z7pPzgFatrHEBCdEGH7K_H2h6Thx1J0KC4zyIOUZwfX_PS_teIqOUZ0UypWCdany56QrxbewUqYC5nMFMilZegOYIgh3mrwDEdu4QDd9RXlU9tWEjmcnDtv9l93tQv7FDJNUcaEmZDNzaGLrzlCmm_8r3J48trsdX-kFNzT-M977EfLfooK4zwwyTydAJBrrTP2aEZbbDRvmzP1_2OEeEOVUqvC3DSwKKA2nI7Ar1DNNfKV_2Ydo4GFjhz_pXAcvcPdcQsmD7yr72gV_M7fNS9WG6ze1Nv5E0fgDOAZdcGT1duL87z06-psE1LZ9tYWLSiC7GYa_Cj5SvJDX8zpHzd3FWN--7Q7keDiaUT-2bWGzmEaDPlJQZ9cmjiFVYfpRyABKgl1cy13ZXN0LTIwhwU.m3u8",
        true,
        VideoFormat::Rgba,
        move |sink| {
            let sample = sink.pull_sample().map_err(|_| gst::FlowError::Eos)?;

            let pad = sink.static_pad("sink").ok_or(gst::FlowError::Error)?;

            let caps = pad.current_caps().ok_or(gst::FlowError::Error)?;
            let s = caps.structure(0).ok_or(gst::FlowError::Error)?;
            let width = s.get::<i32>("width").map_err(|_| gst::FlowError::Error)?;
            let height = s.get::<i32>("height").map_err(|_| gst::FlowError::Error)?;

            if let Some(buffer) = sample.buffer_owned() {
                clone_loop
                    .send_event(EventLoopMSG::Image(width, height, buffer))
                    .map_err(|_| gst::FlowError::Error)?;
            }

            Ok(gst::FlowSuccess::Ok)
        },
        |_,_| gst::prelude::Continue(true),
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
                                ElementState::Pressed => player.set_paused(!player.paused()),
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
