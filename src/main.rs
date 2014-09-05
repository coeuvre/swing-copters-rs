
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use opengl_graphics::{
    Gl,
};
use sdl2_game_window::WindowSDL2;
use graphics::*;
use piston::{
    AssetStore,
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
    Update,
    Input,
};
use piston::input;
use piston::image;
use piston::image::{
    GenericImage,
};

use sprite::Sprite;

pub mod sprite;

fn get_window_size(asset_store: &AssetStore) -> (u32, u32) {
    let path = asset_store.path("img/bg_0.png").unwrap();
    image::open(&path).unwrap().dimensions()
}

fn main() {
    let asset_store = AssetStore::from_folder("../bin/assets");
    let (width, height) = get_window_size(&asset_store);
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "Swing Copters (Rust Clone)".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    // load resources
    let mut bg = Sprite::from_texture_path(&asset_store.path("img/bg_0.png").unwrap()).unwrap();
    bg.anchor = [0.0, 0.0];

    let mut cloud = Sprite::from_texture_path(&asset_store.path("img/cloud_0.png").unwrap()).unwrap();
    cloud.anchor = [0.5, 0.0];
    cloud.position = [width as f64 / 2.0, 0.0];

    let mut land = Sprite::from_texture_path(&asset_store.path("img/land.png").unwrap()).unwrap();
    land.anchor = [0.0, 0.0];
    land.position = [0.0, height as f64 - land.texture_size()[1]];

    let mut copter = Sprite::from_texture_path(&asset_store.path("img/bear_0_0_0.png").unwrap()).unwrap();
    copter.anchor = [0.5, 0.8];
    copter.position = [width as f64 / 2.0 + 100.0, height as f64 / 2.0];


    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let ref mut gl = Gl::new();
    for e in EventIterator::new(&mut window, &event_settings) {
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.height as f64);

                bg.draw(&c, gl);
                cloud.draw(&c, gl);
                land.draw(&c, gl);
                copter.draw(&c, gl);
            },
            Update(args) => {
                //copter.rotation += args.dt * 30.0;
            },
            Input(input::Press(_)) => {
                //copter.scale = [copter.scale[0] * -1.0, 1.0];
                copter.flip_x = !copter.flip_x;
            },
            _ => {},
        }
    }
}

