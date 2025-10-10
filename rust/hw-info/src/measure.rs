use serde::Serialize;

#[derive(Serialize)]
pub struct Measure {
    pub value: f32,
    pub unit: String,
}
