
#![feature(globs)]

extern crate uuid;
extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use std::rc::Rc;

use opengl_graphics::{
    Gl,
    Texture,
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

pub use action::Action;
pub use sprite::Sprite;
pub use scene::Scene;

pub mod action;
pub mod scene;
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

    let mut main_scene = Scene::new();

    // load resources
    let bg_tex = Rc::new(Texture::from_path(&asset_store.path("img/bg_0.png").unwrap()).unwrap());
    let mut bg = Sprite::from_texture(bg_tex.clone());
    bg.set_anchor(0.0, 0.0);
    main_scene.add_child(bg);

    let cloud_tex = Rc::new(Texture::from_path(&asset_store.path("img/cloud_0.png").unwrap()).unwrap());
    let mut cloud = Sprite::from_texture(cloud_tex.clone());
    cloud.set_anchor(0.5, 0.0);
    cloud.set_position(width as f64 / 2.0, 0.0);
    main_scene.add_child(cloud);

    let land_tex = Rc::new(Texture::from_path(&asset_store.path("img/land.png").unwrap()).unwrap());
    let (_, land_tex_height) = land_tex.get_size();
    let mut land = Sprite::from_texture(land_tex.clone());
    land.set_anchor(0.0, 0.0);
    land.set_position(0.0, (height - land_tex_height) as f64);
    main_scene.add_child(land);

    let mut wheel_textures = Vec::new();
    for i in range(0i, 4) {
        let path = asset_store.path(format!("img/wheels_{}.png", i).as_slice()).unwrap();
        wheel_textures.push(Rc::new(Texture::from_path(&path).unwrap()));
    }
    let mut wheel_texture_index = 0.0;
    let ref wheel_tex = wheel_textures[wheel_texture_index as uint];
    let mut wheel = Sprite::from_texture(wheel_tex.clone());
    wheel.set_anchor(0.5, 0.3);

    let copter_tex = Rc::new(Texture::from_path(&asset_store.path("img/bear_0_0_0.png").unwrap()).unwrap());
    let mut copter = Sprite::from_texture(copter_tex.clone());
    copter.set_anchor(0.5, 0.3);
    copter.set_position(width as f64 / 2.0, height as f64 / 2.0);
    copter.set_rotation(10.0);

    let wheel_id = copter.add_child(wheel);
    let copter_id = main_scene.add_child(copter);

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

                /*
                bg.draw(&c, gl);
                cloud.draw(&c, gl);
                land.draw(&c, gl);
                copter.draw(&c, gl);
                */
                main_scene.draw(&c, gl);
            },
            Update(args) => {
                wheel_texture_index = wheel_texture_index + args.dt * 20.0;
                let ref tex = wheel_textures[wheel_texture_index as uint % wheel_textures.len()];
                main_scene.child_mut(wheel_id).unwrap().set_texture(tex.clone());
            },
            Input(input::Press(_)) => {
                let copter = main_scene.child_mut(copter_id).unwrap();
                //copter.scale = [copter.scale[0] * -1.0, 1.0];
                let flip_x = !copter.flip_x();
                copter.set_flip_x(flip_x);
                if !copter.flip_x() {
                    copter.set_rotation(10.0);
                } else {
                    copter.set_rotation(-10.0);
                }
            },
            _ => {},
        }
    }
}

