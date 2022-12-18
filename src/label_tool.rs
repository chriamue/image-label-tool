use crate::{annotated_image::AnnotatedImage, Annotation};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct LabelTool {
    annotated_images: Arc<Mutex<Vec<AnnotatedImage>>>,
}

impl LabelTool {
    pub fn push(&self, annotated_image: AnnotatedImage) {
        self.annotated_images.lock().unwrap().push(annotated_image);
    }

    pub fn annotated_images(&self) -> Arc<Mutex<Vec<AnnotatedImage>>> {
        self.annotated_images.clone()
    }

    pub fn add_annotation(&self, index: usize, annotation: Annotation) {
        let mut locked = self.annotated_images.lock().unwrap();
        let annotations = locked.get_mut(index).unwrap();
        annotations.push(annotation);
    }

    pub fn get_annotated_image(&self, index: usize) -> Option<AnnotatedImage> {
        let locked = self.annotated_images.lock().unwrap();
        match locked.get(index) {
            Some(annotated_image) => Some(annotated_image.clone()),
            None => None,
        }
    }

    pub fn clear(&self) {
        let mut locked = self.annotated_images.lock().unwrap();
        locked.clear();
    }

    pub fn len(&self) -> usize {
        self.annotated_images.lock().unwrap().len()
    }
}

impl PartialEq for LabelTool {
    fn eq(&self, other: &Self) -> bool {
        if ::core::ptr::eq(&self, &other) {
            true
        } else {
            match (
                other.annotated_images.try_lock(),
                self.annotated_images.try_lock(),
            ) {
                (Ok(a), Ok(b)) => a.deref() == b.deref(),
                _ => true,
            }
        }
    }
}

#[wasm_bindgen]
impl LabelTool {
    #[wasm_bindgen(constructor)]
    pub fn new() -> LabelTool {
        let annotated_image = AnnotatedImage::new();
        LabelTool {
            annotated_images: Arc::new(Mutex::new(vec![annotated_image])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let annotations = AnnotatedImage::new();
        let label_tool = LabelTool::new();
        label_tool.push(annotations);
        assert_eq!(1, label_tool.len());
    }

    #[test]
    fn test_clear() {
        let annotations = AnnotatedImage::new();
        let label_tool = LabelTool::new();
        label_tool.push(annotations);
        assert_eq!(1, label_tool.len());
        label_tool.clear();
        assert_eq!(0, label_tool.len());
    }
}
