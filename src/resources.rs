use std::collections::{hash_map::Entry, HashMap};
use std::ffi::OsString;
use std::path;

use ggez::{
    graphics::Image, Context, GameResult,
};

#[derive(Default)]
pub struct Resources {
    handles: HashMap<OsString, usize>,
    images: Vec<Image>,
}

impl Resources {
    pub fn new() -> Self {
        Resources::default()
    }

    pub fn add_image<P: AsRef<path::Path>>(
        &mut self,
        ctx: &mut Context,
        path: P,
    ) -> GameResult<usize> {
        match self.handles.entry(path.as_ref().as_os_str().to_owned()) {
            Entry::Occupied(e) => Ok(*e.get()),
            Entry::Vacant(e) => {
                let id = self.images.len();
                self.images.push(Image::new(ctx, path)?);
                e.insert(id);
                Ok(id)
            }
        }
    }

    pub fn get_image(&self, id: usize) -> &Image {
        &self.images[id]
    }
}
