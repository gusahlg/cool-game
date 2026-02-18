/// Viewport scaling that fits the game world to the screen.
///
/// Scales the world to fill the screen vertically and centers it
/// horizontally, leaving letterbox space on the sides.
///
/// Usage:
/// ```
/// let viewport = Viewport::new(160.0, 160.0); // 10 tiles * 16px
///
/// // each frame:
/// viewport.update();
/// viewport.apply();
/// // ... draw your world ...
/// Viewport::reset(); // switch back to screen-space for UI
/// ```
use macroquad::prelude::*;

pub struct Viewport {
    world_width: f32,
    world_height: f32,
    camera: Camera2D,
}

impl Viewport {
    pub fn new(world_width: f32, world_height: f32) -> Self {
        let mut viewport = Self {
            world_width,
            world_height,
            camera: Camera2D::default(),
        };
        viewport.update();
        viewport
    }

    /// Recalculate the camera for the current screen size.
    /// Call once per frame before `apply()`.
    pub fn update(&mut self) {
        let scale = 2.0 / self.world_height;
        let aspect = screen_width() / screen_height();

        self.camera = Camera2D {
            target: vec2(self.world_width / 2.0, self.world_height / 2.0),
            zoom: vec2(scale / aspect, -scale),
            ..Default::default()
        };
    }

    /// Set this viewport's camera as the active camera.
    pub fn apply(&self) {
        set_camera(&self.camera);
    }

    /// Reset to macroquad's default screen-space camera.
    pub fn reset() {
        set_default_camera();
    }
}
