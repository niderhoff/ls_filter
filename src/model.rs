
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Annotation {
    pub points: Vec<(f64, f64)>,
    pub polygonlabels: Vec<String>,
    pub original_width: Option<u32>,
    pub original_height: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub image: String,
    pub id: usize,
    pub label: Vec<Annotation>,
    pub annotator: usize,
    pub annotation_id: usize,
    pub created_at: String,
    pub updated_at: String,
    pub lead_time: f32,
}