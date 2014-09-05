
use std::rc::Rc;

use opengl_graphics::{
    Gl,
    Texture,
};
use graphics::*;
use graphics::internal::{
    Vec2d,
    Rectangle,
};

pub struct Sprite {
    /// Normalized
    pub anchor: Vec2d,

    pub position: Vec2d,
    /// In degree
    pub rotation: f64,
    pub scale: Vec2d,

    pub flip_x: bool,
    pub flip_y: bool,

    texture: Rc<Texture>,
}

impl Sprite {
    pub fn from_texture_path(path: &Path) -> Option<Sprite> {
        match Texture::from_path(path) {
            Ok(tex) => {
                Some(Sprite::from_texture(Rc::new(tex)))
            },
            _ => { None },
        }
    }

    pub fn from_texture(texture: Rc<Texture>) -> Sprite {
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

    pub fn set_texture(&mut self, texture: Rc<Texture>) {
        self.texture = texture;
    }

    pub fn draw(&self, c: &Context, gl: &mut Gl) {
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

        // for debug
        //model.rgb(1.0, 0.0, 0.0).draw(gl);

        model.image(&*self.texture).draw(gl);

        // for debug
        //c.trans(self.position[0], self.position[1]).rect(-5.0, -5.0, 10.0, 10.0).rgb(0.0, 0.0, 1.0).draw(gl);
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

    pub fn texture(&self) -> Rc<Texture> {
        self.texture.clone()
    }
}

