use crate::expansion_valve::ExpansionValve;

pub struct Condenser {
	expansion_valve: Option<ExpansionValve>
}

impl Condenser {
	pub fn new() -> Condenser {
		Condenser {
			expansion_valve: None
		}
	}

	pub fn set_expansion_valve(&mut self, expansion_valve: ExpansionValve) {
		self.expansion_valve = expansion_valve;
	}
}
