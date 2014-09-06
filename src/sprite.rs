
use std::rc::Rc;

use graphics::*;
use graphics::internal::{
    Vec2d,
    Rectangle,
};

pub struct Sprite<I: ImageSize> {
    /// Normalized
    pub anchor: Vec2d,

    pub position: Vec2d,
    /// In degree
    pub rotation: f64,
    pub scale: Vec2d,

    pub flip_x: bool,
    pub flip_y: bool,

    texture: Rc<I>,
}

impl<I: ImageSize> Sprite<I> {
    pub fn from_texture(texture: Rc<I>) -> Sprite<I> {
        Sprite {
            anchor: [0.5, 0.5],

            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],

            flip_x: false,
            flip_y: false,

            texture: texture,
        }
    }

    pub fn set_texture(&mut self, texture: Rc<I>) {
        self.texture = texture;
    }

    pub fn draw<B: BackEnd<I>>(&self, c: &Context, b: &mut B) {
        let size = self.texture_size();
        let anchor = [self.anchor[0] * size[0], self.anchor[1] * size[1]];

        let transformed = c.trans(self.position[0], self.position[1])
                           .rot_deg(self.rotation)
                           .scale(self.scale[0], self.scale[1]);

        let mut model = transformed.rect(-anchor[0],
                                         -anchor[1],
                                          size[0],
                                          size[1]);

        if self.flip_x {
            model = model.trans(size[0] - 2.0 * anchor[0], 0.0).flip_h();
        }

        if self.flip_y {
            model = model.trans(0.0, size[1] - 2.0 * anchor[1]).flip_v();
        }

        // for debug: bounding_box
        //model.rgb(1.0, 0.0, 0.0).draw(b);

        model.image(&*self.texture).draw(b);

        // for debug: anchor point
        //c.trans(self.position[0], self.position[1]).rect(-5.0, -5.0, 10.0, 10.0).rgb(0.0, 0.0, 1.0).draw(b);
    }

    pub fn bounding_box(&self) -> Rectangle {
        let (w, h) = self.texture.get_size();
        let w = w as f64 * self.scale[0];
        let h = h as f64 * self.scale[1];

        [
            self.position[0] - self.anchor[0] * w,
            self.position[1] - self.anchor[1] * h,
            w,
            h
        ]
    }

    pub fn texture_size(&self) -> Vec2d {
        let (w, h) = self.texture.get_size();
        [w as f64, h as f64]
    }

    pub fn texture(&self) -> Rc<I> {
        self.texture.clone()
    }
}

