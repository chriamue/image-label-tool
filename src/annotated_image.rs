use crate::Annotation;
use image::DynamicImage;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

/// Image with Annotations
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct AnnotatedImage {
    image: Arc<Mutex<DynamicImage>>,
    annotations: Arc<Mutex<Vec<Annotation>>>,
}

impl AnnotatedImage {
    /// Create a new AnnotatedImage
    pub fn new() -> AnnotatedImage {
        AnnotatedImage {
            image: Arc::new(Mutex::new(DynamicImage::new_rgb8(1, 1))),
            annotations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// sets the image
    pub fn set_image(&self, image: DynamicImage) {
        *self.image.lock().unwrap() = image;
    }

    /// adds an annotation
    pub fn push(&self, annotation: Annotation) {
        self.annotations.lock().unwrap().push(annotation);
    }

    /// removes all annotations
    pub fn clear(&self) {
        self.annotations.lock().unwrap().clear();
    }

    /// len of all annotations
    pub fn len(&self) -> usize {
        self.annotations.lock().unwrap().len()
    }

    /// returns all annotations
    pub fn get_annotations(&self) -> Vec<Annotation> {
        self.annotations.lock().unwrap().clone()
    }

    /// returns the image
    pub fn get_image(&self) -> DynamicImage {
        self.image.lock().unwrap().clone()
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
