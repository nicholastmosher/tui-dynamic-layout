use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::prelude::*;

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DynamicLayout {
    pub vertical: bool,
    pub parent: Rect,
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DynamicLayoutState {
    pub vertical: bool,
    pub last_area: Rect,
    pub offset: Option<u16>,
    pub dragging: bool,
}

impl DynamicLayout {
    pub fn vertical(parent: Rect) -> Self {
        Self {
            parent,
            vertical: true,
        }
    }

    pub fn horizontal(parent: Rect) -> Self {
        Self {
            parent,
            vertical: false,
        }
    }

    pub fn areas(&self, state: &mut DynamicLayoutState) -> (Rect, Rect) {
        state.vertical = self.vertical;
        state.last_area = self.parent;
        let offset = match state.offset {
            Some(offset) => offset,
            None => {
                // Set initial value to split at midpoint
                if self.vertical {
                    self.parent.height / 2
                } else {
                    self.parent.width / 2
                }
            }
        };
        state.offset = Some(offset);

        let (first, second) = if self.vertical {
            let mut first = self.parent;
            first.height = first.y + offset - 1;

            let mut second = self.parent;
            second.y += offset;
            second.height -= offset;

            (first, second)
        } else {
            let mut first = self.parent;
            first.width = first.x + offset - 1;

            let mut second = self.parent;
            second.x += offset;
            second.width -= offset;

            (first, second)
        };

        (first, second)
    }
}

impl DynamicLayoutState {
    pub fn new() -> Self {
        Self {
            vertical: true,
            last_area: Rect::default(),
            offset: None,
            dragging: false,
        }
    }

    pub fn handle_mouse_event(&mut self, event: &MouseEvent) {
        let offset = match self.offset {
            Some(offset) => offset,
            None => {
                // Set initial value to split at midpoint
                if self.vertical {
                    self.last_area.height / 2
                } else {
                    self.last_area.width / 2
                }
            }
        };
        self.offset = Some(offset);

        match (self.dragging, event.kind) {
            (true, MouseEventKind::Up(MouseButton::Left)) => {
                self.dragging = false;
            }
            (false, MouseEventKind::Down(MouseButton::Left)) => {
                if !mouse_in_rect(self.last_area, event) {
                    return;
                }

                // Detect if the click is on the split
                let clicked_split = if self.vertical {
                    event.row == self.last_area.y + offset
                } else {
                    event.column == self.last_area.x + offset
                };

                // If user clicked on the split line, go to dragging mode
                if clicked_split {
                    self.dragging = true;
                }
            }
            (true, MouseEventKind::Drag(MouseButton::Left)) => {
                let split_offset = if self.vertical {
                    // Mouse above rect
                    if event.row <= self.last_area.y {
                        0
                    }
                    // Mouse below rect
                    else if event.row > self.last_area.y + self.last_area.height {
                        self.last_area.height
                    }
                    // Mouse inside rect
                    else {
                        event.row - self.last_area.y
                    }
                } else {
                    // Mouse outside of rect to left
                    if event.column <= self.last_area.x {
                        0
                    }
                    // Mouse outside of rect to right
                    else if event.column > self.last_area.x + self.last_area.width {
                        self.last_area.width
                    }
                    // Mouse inside rect
                    else {
                        event.column - self.last_area.x
                    }
                };
                self.offset = Some(split_offset);
            }
            _ => {
                //
            }
        }
    }
}

fn mouse_in_rect(area: Rect, event: &MouseEvent) -> bool {
    if event.row < area.y {
        return false;
    }
    if event.row > area.y + area.height {
        return false;
    }
    if event.column < area.x {
        return false;
    }
    if event.column > area.x + area.width {
        return false;
    }
    true
}
