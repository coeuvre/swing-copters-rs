
use std::collections::hashmap::HashMap;
use uuid::Uuid;

use graphics::*;

use Sprite;
use Action;

pub struct Scene<I: ImageSize> {
    children: Vec<Sprite<I>>,
    children_index: HashMap<Uuid, uint>,
}

impl<I: ImageSize> Scene<I> {
    pub fn new() -> Scene<I> {
        Scene {
            children: Vec::new(),
            children_index: HashMap::new(),
        }
    }

    pub fn draw<B: BackEnd<I>>(&self, c: &Context, b: &mut B) {
        for child in self.children.iter() {
            child.draw(c, b);
        }
    }

    //pub fn run_action(&mut self, sprite_id: Uuid, action: &Action) {
    //}

    pub fn add_child(&mut self, sprite: Sprite<I>) -> Uuid {
        let id = sprite.id();
        self.children.push(sprite);
        self.children_index.insert(id, self.children.len() - 1);
        id
    }

    pub fn child(&self, id: Uuid) -> Option<&Sprite<I>> {
        match self.children_index.find(&id) {
            Some(i) => { Some(&self.children[*i]) },
            None => {
                for child in self.children.iter() {
                    match child.child(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        }
    }

    pub fn child_mut(&mut self, id: Uuid) -> Option<&mut Sprite<I>> {
        match self.children_index.find(&id) {
            Some(i) => { Some(self.children.get_mut(*i)) },
            None => {
                for child in self.children.mut_iter() {
                    match child.child_mut(id) {
                        Some(c) => {
                            return Some(c);
                        }
                        _ => {}
                    }
                }

                None
            }
        }
    }
}

