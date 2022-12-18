use crate::{annotated_image::AnnotatedImage, Annotation};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::*;

/// struct of label tool that manages annotated images
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct LabelTool {
    annotated_images: Arc<Mutex<Vec<AnnotatedImage>>>,
}

impl LabelTool {
    /// adds new annotated image
    pub fn push(&self, annotated_image: AnnotatedImage) {
        self.annotated_images.lock().unwrap().push(annotated_image);
    }

    /// get annotated images
    pub fn annotated_images(&self) -> Arc<Mutex<Vec<AnnotatedImage>>> {
        self.annotated_images.clone()
    }

    /// adds annotation to annotated image selected by index
    pub fn add_annotation(&self, index: usize, annotation: Annotation) {
        let mut locked = self.annotated_images.lock().unwrap();
        let annotations = locked.get_mut(index).unwrap();
        annotations.push(annotation);
    }

    /// gets annotated image selected by index
    pub fn get_annotated_image(&self, index: usize) -> Option<AnnotatedImage> {
        let locked = self.annotated_images.lock().unwrap();
        locked.get(index).cloned()
    }

    /// clears all annotated images
    pub fn clear(&self) {
        let mut locked = self.annotated_images.lock().unwrap();
        locked.clear();
    }

    /// size of annotated images
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
    /// constructor of new label tool
    #[wasm_bindgen(constructor)]
    pub fn new() -> LabelTool {
        LabelTool {
            annotated_images: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for LabelTool {
    fn default() -> Self {
        let annotated_image = AnnotatedImage::new();
        let label_tool = Self::new();
        label_tool.push(annotated_image);
        label_tool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let annotations = AnnotatedImage::default();
        let label_tool = LabelTool::default();
        label_tool.push(annotations);
        assert_eq!(2, label_tool.len());
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
