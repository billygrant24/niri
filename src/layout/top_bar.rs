use std::iter::zip;

use niri_config::Color;
use smithay::backend::renderer::element::Kind;
use smithay::utils::{Logical, Point, Rectangle, Size};

use crate::niri_render_elements;
use crate::render_helpers::border::BorderRenderElement;
use crate::render_helpers::renderer::NiriRenderer;
use crate::render_helpers::solid_color::{SolidColorBuffer, SolidColorRenderElement};

/// The height of the top bar in logical pixels
pub const TOP_BAR_HEIGHT: f64 = 24.0;
/// The size of buttons in the top bar
pub const BUTTON_SIZE: f64 = 18.0;
/// The spacing between buttons
pub const BUTTON_SPACING: f64 = 4.0;
/// The padding from the right edge
pub const RIGHT_PADDING: f64 = 6.0;

#[derive(Debug)]
pub struct TopBar {
    /// The background of the top bar
    background_buffer: SolidColorBuffer,
    /// Buffers for the three buttons
    button_buffers: [SolidColorBuffer; 3],
    /// The full size of the top bar
    size: Size<f64, Logical>,
    /// The locations of the buttons
    button_locations: [Point<f64, Logical>; 3],
    /// The button colors
    button_colors: [Color; 3],
}

niri_render_elements! {
    TopBarRenderElement => {
        SolidColor = SolidColorRenderElement,
        Border = BorderRenderElement,
    }
}

impl TopBar {
    pub fn new() -> Self {
        // Default colors for background and buttons
        let background_color = [0.2, 0.2, 0.2, 0.9]; // Dark gray with some transparency
        let button_colors = [
            Color::new_unpremul(1.0, 0.3, 0.3, 1.0), // Red - button 1
            Color::new_unpremul(1.0, 0.7, 0.2, 1.0), // Orange - button 2
            Color::new_unpremul(0.3, 0.8, 0.3, 1.0), // Green - button 3
        ];
        
        Self {
            background_buffer: SolidColorBuffer::new(Size::default(), background_color),
            button_buffers: Default::default(),
            size: Default::default(),
            button_locations: Default::default(),
            button_colors,
        }
    }

    /// Update the top bar based on the window size
    pub fn update(&mut self, win_size: Size<f64, Logical>) {
        // Set the top bar size (full width, fixed height)
        let size = Size::from((win_size.w, TOP_BAR_HEIGHT));
        self.size = size;
        
        // Update the background buffer
        self.background_buffer.resize(size);
        
        // Calculate button positions (aligned to the right)
        let mut right_offset = win_size.w - RIGHT_PADDING;
        
        for i in (0..3).rev() {
            // Position button from right to left
            right_offset -= BUTTON_SIZE;
            let button_location = Point::from((right_offset, (TOP_BAR_HEIGHT - BUTTON_SIZE) / 2.0));
            self.button_locations[i] = button_location;
            
            // Resize button buffer
            let button_size = Size::from((BUTTON_SIZE, BUTTON_SIZE));
            self.button_buffers[i].resize(button_size);
            self.button_buffers[i].set_color(self.button_colors[i].to_array_premul());
            
            // Add spacing between buttons
            right_offset -= BUTTON_SPACING;
        }
    }

    /// Check if a point is inside one of the buttons
    /// Returns the button index (0, 1, or 2) if hit, None otherwise
    pub fn hit_test(&self, point: Point<f64, Logical>) -> Option<usize> {
        // First check if we're in the top bar area
        if point.y < 0.0 || point.y > TOP_BAR_HEIGHT {
            return None;
        }
        
        // Check each button
        for (i, loc) in self.button_locations.iter().enumerate() {
            let button_rect = Rectangle::new(*loc, Size::from((BUTTON_SIZE, BUTTON_SIZE)));
            if button_rect.contains(point) {
                return Some(i);
            }
        }
        
        None
    }

    /// Render the top bar and its buttons
    pub fn render<'a, R: NiriRenderer + 'a>(
        &'a self,
        renderer: &mut R,
        location: Point<f64, Logical>,
    ) -> impl Iterator<Item = TopBarRenderElement> + 'a {
        // First render the background
        let background = SolidColorRenderElement::from_buffer(
            &self.background_buffer,
            location,
            1.0,
            Kind::Unspecified,
        );
        
        // Then render each button
        let buttons = self.button_buffers.iter().enumerate().map(move |(i, buf)| {
            let button_loc = location + self.button_locations[i];
            SolidColorRenderElement::from_buffer(
                buf,
                button_loc,
                1.0,
                Kind::Unspecified,
            )
        });
        
        // Combine the background with the buttons
        std::iter::once(background.into())
            .chain(buttons.map(Into::into))
    }
}