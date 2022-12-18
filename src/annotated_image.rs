use crate::Annotation;
use image::DynamicImage;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct AnnotatedImage {
    image: Arc<Mutex<DynamicImage>>,
    annotations: Arc<Mutex<Vec<Annotation>>>,
}

impl AnnotatedImage {
    pub fn new() -> AnnotatedImage {
        AnnotatedImage {
            image: Arc::new(Mutex::new(DynamicImage::new_rgb8(1, 1))),
            annotations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn set_image(&self, image: DynamicImage) {
        *self.image.lock().unwrap() = image;
    }

    pub fn push(&self, annotation: Annotation) {
        self.annotations.lock().unwrap().push(annotation);
    }

    pub fn clear(&self) {
        self.annotations.lock().unwrap().clear();
    }

    pub fn len(&self) -> usize {
        self.annotations.lock().unwrap().len()
    }

    pub fn get_annotations(&self) -> Vec<Annotation> {
        self.annotations.lock().unwrap().clone()
    }

    pub fn get_image(&self) -> DynamicImage {
        self.image.lock().unwrap().clone()
    }
}

#[wasm_bindgen]
impl AnnotatedImage {
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

    #[test]
    fn test_len() {
        let annotations = AnnotatedImage::new();
        let annotation = Annotation::default();
        annotations.push(annotation);
        assert_eq!(1, annotations.len());
    }

    #[test]
    fn test_clear() {
        let annotations = AnnotatedImage::new();
        let annotation = Annotation::default();
        annotations.push(annotation);
        assert_eq!(1, annotations.len());
        annotations.clear();
        assert_eq!(0, annotations.len());
    }

    #[test]
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
