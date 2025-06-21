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
/// The padding from the right/left edge
pub const EDGE_PADDING: f64 = 6.0;

/// Button indices
pub const BUTTON_SCREENSHOT: usize = 0;
pub const BUTTON_PRESET_WIDTH: usize = 1;
pub const BUTTON_CLOSE: usize = 2;
pub const BUTTON_MINIMIZE: usize = 3;
pub const BUTTON_MAXIMIZE: usize = 4;

/// Nord theme colors
const NORD_POLAR_NIGHT_0: Color = Color::new_unpremul(0.18, 0.2, 0.25, 1.0);  // #2e3440
const NORD_POLAR_NIGHT_1: Color = Color::new_unpremul(0.22, 0.24, 0.29, 1.0);  // #3b4252
const NORD_POLAR_NIGHT_2: Color = Color::new_unpremul(0.25, 0.28, 0.33, 1.0);  // #434c5e
const NORD_POLAR_NIGHT_3: Color = Color::new_unpremul(0.3, 0.34, 0.38, 1.0);   // #4c566a
const NORD_SNOW_STORM_0: Color = Color::new_unpremul(0.86, 0.87, 0.9, 1.0);    // #d8dee9
const NORD_SNOW_STORM_1: Color = Color::new_unpremul(0.9, 0.91, 0.92, 1.0);    // #e5e9f0
const NORD_SNOW_STORM_2: Color = Color::new_unpremul(0.94, 0.95, 0.96, 1.0);   // #eceff4
const NORD_FROST_0: Color = Color::new_unpremul(0.57, 0.73, 0.82, 1.0);        // #8fbcbb
const NORD_FROST_1: Color = Color::new_unpremul(0.54, 0.75, 0.81, 1.0);        // #88c0d0
const NORD_FROST_2: Color = Color::new_unpremul(0.51, 0.63, 0.75, 1.0);        // #81a1c1
const NORD_FROST_3: Color = Color::new_unpremul(0.51, 0.59, 0.76, 1.0);        // #5e81ac
const NORD_AURORA_0: Color = Color::new_unpremul(0.74, 0.38, 0.42, 1.0);       // #bf616a
const NORD_AURORA_1: Color = Color::new_unpremul(0.83, 0.51, 0.42, 1.0);       // #d08770
const NORD_AURORA_2: Color = Color::new_unpremul(0.92, 0.8, 0.55, 1.0);        // #ebcb8b
const NORD_AURORA_3: Color = Color::new_unpremul(0.65, 0.75, 0.57, 1.0);       // #a3be8c
const NORD_AURORA_4: Color = Color::new_unpremul(0.7, 0.55, 0.74, 1.0);        // #b48ead

#[derive(Debug)]
pub struct TopBar {
    /// The background of the top bar
    background_buffer: SolidColorBuffer,
    /// Buffers for the buttons
    button_buffers: [SolidColorBuffer; 5],
    /// The full size of the top bar
    size: Size<f64, Logical>,
    /// The locations of the buttons
    button_locations: [Point<f64, Logical>; 5],
    /// The button colors
    button_colors: [Color; 5],
}

niri_render_elements! {
    TopBarRenderElement => {
        SolidColor = SolidColorRenderElement,
        Border = BorderRenderElement,
    }
}

impl TopBar {
    pub fn new() -> Self {
        // Use darker background color with some transparency
        let background_color = [0.2, 0.2, 0.2, 0.9];

        // Use brighter, more visible colors for buttons
        let button_colors = [
            Color::new_unpremul(0.2, 0.6, 1.0, 1.0),  // Screenshot - Bright blue
            Color::new_unpremul(0.4, 0.8, 1.0, 1.0),  // Preset Column Width - Light blue
            Color::new_unpremul(1.0, 0.3, 0.3, 1.0),  // Close - Red
            Color::new_unpremul(1.0, 0.7, 0.2, 1.0),  // Minimize - Orange
            Color::new_unpremul(0.3, 0.8, 0.3, 1.0),  // Maximize - Green
        ];
        
        // Create button buffers with initial size and explicitly premultiplied colors
        let button_size = Size::from((BUTTON_SIZE, BUTTON_SIZE));
        let button_buffers = [
            SolidColorBuffer::new(button_size, [0.2, 0.6, 1.0, 1.0]),  // Bright blue
            SolidColorBuffer::new(button_size, [0.4, 0.8, 1.0, 1.0]),  // Light blue
            SolidColorBuffer::new(button_size, [1.0, 0.3, 0.3, 1.0]),  // Red
            SolidColorBuffer::new(button_size, [1.0, 0.7, 0.2, 1.0]),  // Orange
            SolidColorBuffer::new(button_size, [0.3, 0.8, 0.3, 1.0]),  // Green
        ];
        
        Self {
            background_buffer: SolidColorBuffer::new(Size::default(), background_color),
            button_buffers,
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
        self.background_buffer.set_color([0.2, 0.2, 0.2, 0.9]);
        
        // Common Y position for all buttons
        let button_y = (TOP_BAR_HEIGHT - BUTTON_SIZE) / 2.0;
        
        // LEFT SIDE: Screenshot button
        let left_offset = EDGE_PADDING;
        self.button_locations[BUTTON_SCREENSHOT] = Point::from((left_offset, button_y));
        
        // RIGHT SIDE: Control buttons
        // Start from the right edge and work leftwards
        let mut right_offset = win_size.w - EDGE_PADDING;
        
        // Window control buttons (close, minimize, maximize) - rightmost
        for i in (BUTTON_CLOSE..=BUTTON_MAXIMIZE).rev() {
            right_offset -= BUTTON_SIZE;
            self.button_locations[i] = Point::from((right_offset, button_y));
            right_offset -= BUTTON_SPACING;
        }
        
        // Preset column width button - to the left of window control buttons
        right_offset -= BUTTON_SIZE;
        self.button_locations[BUTTON_PRESET_WIDTH] = Point::from((right_offset, button_y));
        
        // Resize and color all button buffers
        let button_size = Size::from((BUTTON_SIZE, BUTTON_SIZE));
        
        // Direct color assignment - these are already premultiplied
        let button_colors = [
            [0.2, 0.6, 1.0, 1.0],  // Screenshot - Bright blue
            [0.4, 0.8, 1.0, 1.0],  // Preset Column Width - Light blue
            [1.0, 0.3, 0.3, 1.0],  // Close - Red
            [1.0, 0.7, 0.2, 1.0],  // Orange
            [0.3, 0.8, 0.3, 1.0],  // Green
        ];
        
        for i in 0..self.button_buffers.len() {
            self.button_buffers[i].resize(button_size);
            self.button_buffers[i].set_color(button_colors[i]);
        }
    }

    /// Check if a point is inside one of the buttons
    /// Returns the button index if hit, None otherwise
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
