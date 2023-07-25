use crate::{Compressor,Condenser,ExpansionValve,Evaporator,TemperatureSensor}

pub struct AirConditioner {
	target_temperature: f64,
	mode: AirConditioningMode,

	compressor: Compressor,
	condenser: Condenser,
	expansion_valve: ExpansionValve,
	evaporator: Evaporator
}

impl AirConditioner {
	pub fn new(compressor: Compressor, condenser: Condenser, expansion_valve: ExpansionValve, evaporator: Evaporator) -> AirConditioner {
		AirConditioner {
			target_temperature: Default::default(),
			compressor,
			condenser,
			expansion_valve,
			evaporator
		}
	}

	fn set_target_temperature(&mut self, temperature: f64) {
		self.target_temperature = temperature;
	}

	fn set_mode(&mut self, mode: AirConditioningMode) {
		self.mode = mode;
	}

	fn start() {
		compressor.set_speed(100);
		compressor.start();
	}

	fn stop() {
	
	}
}

enum AirConditioningMode {
	Auto,
	Cooling,
	Heating,
	//Dry,
	//Fan
}
