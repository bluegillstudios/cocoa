// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License v2.0.

pub mod button;
pub mod label;
pub mod panel;

pub use button::Button;
pub use label::Label;
pub use panel::Panel;

use skia_safe::Canvas;
use crate::event::Event;

pub trait Widget {
    fn draw(&mut self, canvas: &mut skia_safe::Canvas);
    fn on_event(&mut self, event: &Event) -> bool; 
    fn layout(&mut self, rect: skia_safe::Rect);
}
