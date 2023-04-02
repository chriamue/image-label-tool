use crate::Annotation;
use image::DynamicImage;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// Represents an image with associated annotations.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct AnnotatedImage {
    image: Rc<RefCell<DynamicImage>>,
    annotations: Rc<RefCell<Vec<Annotation>>>,
}

impl AnnotatedImage {
    /// Creates a new `AnnotatedImage` with an empty image and no annotations.
    pub fn new() -> AnnotatedImage {
        AnnotatedImage {
            image: Rc::new(RefCell::new(DynamicImage::new_rgb8(1, 1))),
            annotations: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Sets the image for the `AnnotatedImage`.
    pub fn set_image(&self, image: DynamicImage) {
        *self.image.borrow_mut() = image;
    }

    /// Adds an annotation to the `AnnotatedImage`.
    pub fn push(&self, annotation: Annotation) {
        self.annotations.borrow_mut().push(annotation);
    }

    /// Removes all annotations from the `AnnotatedImage`.
    pub fn clear(&self) {
        self.annotations.borrow_mut().clear();
    }

    /// Returns the number of annotations in the `AnnotatedImage`.
    pub fn len(&self) -> usize {
        self.annotations.borrow().len()
    }

    /// Returns a reference to the annotations in the `AnnotatedImage`.
    pub fn get_annotations(&self) -> Ref<Vec<Annotation>> {
        self.annotations.borrow()
    }

    /// returns the image
    pub fn get_image(&self) -> Ref<DynamicImage> {
        self.image.borrow()
    }
}

#[wasm_bindgen]
impl AnnotatedImage {
    /// constructor of AnnotatedImages for wasm
    #[wasm_bindgen(constructor)]
    pub fn constructor() -> AnnotatedImage {
        Self::new()
    }
}

impl PartialEq for AnnotatedImage {
    fn eq(&self, other: &Self) -> bool {
        ::core::ptr::eq(&self, &other)
    }
}

impl Default for AnnotatedImage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[crate::test]
    fn test_len() {
        let annotations = AnnotatedImage::new();
        let annotation = Annotation::default();
        annotations.push(annotation);
        assert_eq!(1, annotations.len());
    }

    #[crate::test]
    fn test_clear() {
        let annotations = AnnotatedImage::new();
        let annotation = Annotation::default();
        annotations.push(annotation);
        assert_eq!(1, annotations.len());
        annotations.clear();
        assert_eq!(0, annotations.len());
    }

    #[crate::test]
    fn test_default() {
        let annotations = AnnotatedImage::default();
        let annotation = Annotation::default();
        annotations.push(annotation);
        assert_eq!(1, annotations.len());
        let image = annotations.get_image();
        assert_eq!(image.width(), 1);
        assert_eq!(image.height(), 1);
    }
}
