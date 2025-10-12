use serde::Serialize;


#[derive(Serialize)]
pub struct Measure(f32, String);
    /*{
    value: f32,
    unit: String,}*/
impl Measure {
	pub fn new(value: f32, unit: String) -> Self {
		Self(value, unit)
	}
	pub fn value(&self) -> f32 { self.0 }
	pub fn unit(&self) -> &String { &self.1 }
}
