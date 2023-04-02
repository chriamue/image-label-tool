use crate::annotated_image::AnnotatedImage;
use std::cell::RefCell;
use std::rc::Rc;
/// A store for managing a list of annotated images.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageStore {
    /// A reference-counted list of annotated images.
    pub images: Rc<RefCell<Vec<AnnotatedImage>>>,
}

/// Messages for updating the `ImageStore`.
pub enum ImageStoreMsg {
    /// Add an annotated image to the store.
    AddImage(AnnotatedImage),
}

impl ImageStore {
    /// Creates a new `ImageStore` with an empty list of annotated images.
    pub fn new() -> Self {
        Self {
            images: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Updates the `ImageStore` based on the given message.
    pub fn update(&mut self, msg: ImageStoreMsg) {
        match msg {
            ImageStoreMsg::AddImage(image) => {
                let mut images = self.images.borrow_mut();
                images.push(image);
            }
        }
    }

    /// Returns the number of images in the `ImageStore`.
    ///
    /// # Example
    ///
    /// ```
    /// # use image_label_tool::prelude::ImageStore;
    /// let image_store = ImageStore::default();
    /// assert_eq!(image_store.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.images.borrow().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the length of the images list after adding an annotated image to the store.
    #[test]
    fn test_len() {
        let mut image_store = ImageStore::new();
        assert_eq!(image_store.images.borrow().len(), 0);
        let image = AnnotatedImage::default();
        image_store.update(ImageStoreMsg::AddImage(image));
        assert_eq!(image_store.images.borrow().len(), 1);
        assert_eq!(image_store.len(), 1);
    }
}
