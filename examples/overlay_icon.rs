// Copyright 2014-2021 The winit contributors
// Copyright 2021-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use std::env::current_dir;
use tao::{
  event::{ElementState, Event, KeyEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::{Key, ModifiersState},
  window::WindowBuilder,
};

#[allow(clippy::single_match)]
fn main() {
  env_logger::init();
  let event_loop = EventLoop::new();

  let window = WindowBuilder::new().build(&event_loop).unwrap();

  let mut modifiers = ModifiersState::default();

  eprintln!("Key mappings:");
  eprintln!("  [any key]: Show the Overlay Icon");
  eprintln!("  Ctrl+1: Clear");

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::ModifiersChanged(new_state) => {
          modifiers = new_state;
        }
        WindowEvent::KeyboardInput {
          event:
            KeyEvent {
              logical_key: Key::Character(key_str),
              state: ElementState::Released,
              ..
            },
          ..
        } => {
          if modifiers.is_empty() {
            let mut path = current_dir().unwrap();
            path.push("./examples/icon.ico");

            window.set_overlay_icon(Some(path.to_str().unwrap().to_string()));
          } else if modifiers.control_key() && key_str == "1" {
            window.set_overlay_icon(None);
          }
        }
        _ => {}
      },
      _ => {}
    }
  });
}
