pub struct SimulationContext {
	inside_temperature: f64,
	outside_temperature: f64,

	inside_space_volume_m3: i32 // cubic meters
}

impl SimulationContext {
	pub fn new(&mut self, inside_temperature: f64, outside_temperature: f64, inside_space_volume_m3: i32)

	pub fn get_inside_temperature(&self) -> f64 {
		self.inside_temperature
	}

	pub fn get_outside_temperature(&self) -> f64 {
		self.outside_temperature
	}
}