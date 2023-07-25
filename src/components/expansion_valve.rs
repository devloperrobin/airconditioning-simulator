use crate::evaporator::Evaporator;

pub struct ExpansionValve {
	evaporator: Option<Evaporator>
}

impl ExpansionValve {
	pub fn new() -> ExpansionValve {
		ExpansionValve {
			evaporator: None
		}
	}

	pub fn set_evaporator(&mut self, evaporator: Evaporator) {
		self.evaporator = evaporator;
	}
}
