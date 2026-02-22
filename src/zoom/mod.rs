/// Shows a 10-tile (160px) window into the world, pinned to the right
/// edge of the screen. Scroll vertically with the mouse wheel and
/// horizontally by holding Ctrl + scroll.
use macroquad::prelude::*;

const VIEW_SIZE: f32 = 160.0; // 10 tiles * 16px
const SCROLL_SPEED: f32 = 32.0; // 2 tiles per scroll tick

pub struct Viewport {
    world_width: f32,
    world_height: f32,
    cam_x: f32, // top-left corner of the visible window (world pixels)
    cam_y: f32,
    camera: Camera2D,
}

impl Viewport {
    pub fn new(world_width: f32, world_height: f32) -> Self {
        let mut viewport = Self {
            world_width,
            world_height,
            cam_x: 0.0,
            cam_y: (world_height - VIEW_SIZE).max(0.0),
            camera: Camera2D::default(),
        };
        viewport.rebuild_camera();
        viewport
    }

    /// Handle scroll input and recalculate the camera each frame.
    pub fn update(&mut self) {
        let (_wheel_x, wheel_y) = mouse_wheel();

        if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            // Ctrl + scroll → horizontal panning
            self.cam_x -= wheel_y * SCROLL_SPEED;
        } else {
            // Normal scroll → vertical panning
            self.cam_y -= wheel_y * SCROLL_SPEED;
        }

        // Clamp so the view stays within world bounds
        self.cam_x = self.cam_x.clamp(0.0, (self.world_width - VIEW_SIZE).max(0.0));
        self.cam_y = self.cam_y.clamp(0.0, (self.world_height - VIEW_SIZE).max(0.0));

        self.rebuild_camera();
    }

    /// Set this viewport's camera as the active camera.
    pub fn apply(&self) {
        set_camera(&self.camera);
    }

    /// Reset to macroquad's default screen-space camera.
    pub fn reset() {
        set_default_camera();
    }

    fn rebuild_camera(&mut self) {
        let scale = 2.0 / VIEW_SIZE;
        let aspect = screen_width() / screen_height();
        let visible_w = aspect * VIEW_SIZE;

        // Center of the visible window, pinned so the right edge of the
        // view aligns with the right edge of the screen.
        let center_x = self.cam_x + VIEW_SIZE - visible_w / 2.0;
        let center_y = self.cam_y + VIEW_SIZE / 2.0;

        self.camera = Camera2D {
            target: vec2(center_x, center_y),
            zoom: vec2(scale / aspect, scale),
            ..Default::default()
        };
    }
}
