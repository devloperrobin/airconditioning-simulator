use crate::{Component,Refrigerant};

pub struct Compressor {
	speed: u8, // percentage
	output: Component;
	input_refrigerant: Option<Refrigerant>;
}

impl Compressor {
	pub fn new() -> Compressor {
		Compressor {
			condensor: None
		}
	}

	pub fn set_speed(&mut self, speed: u8) -> Result<(), String> {
		// Check if the speed is within the valid range (0 to 100)
		if speed > 100 {
			return Err(String::from("Speed must be between 0 and 100."));
		}

		// If the speed is within the valid range, update the compressor speed
		self.speed = speed;
		Ok(())
	}
}

impl Component for Compressor {
	fn get_refrigerant(&self) -> &Refrigerant {

	}

	fn process_refrigerant(&self, &mut refrigerant: Refrigerant) {
		println!("TODO Compress");
	}

	fn set_ouput(&mut self, condenser: Condenser) {
		self.condenser = condenser;
	}
}