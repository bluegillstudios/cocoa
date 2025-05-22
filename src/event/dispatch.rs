// Copyright 2025 Bluegill Studios.
// Licensed under the GNU General Public License, v2.0.

use crate::event::Event;
use crate::widget::Widget;

pub struct Dispatcher<'a> {
    root_widget: &'a mut dyn Widget,
}

impl<'a> Dispatcher<'a> {
    pub fn new(root_widget: &'a mut dyn Widget) -> Self {
        Self { root_widget }
    }

    pub fn dispatch(&mut self, event: &Event) -> bool {
        self.root_widget.on_event(event)
    }
}
