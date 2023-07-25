use crate::compressor::Compressor;

pub struct Evaporator {
	compressor: Option<Compressor>
}

impl Evaporator {
	pub fn new() -> Evaporator {
		Evaporator {
			compressor: None
		}
	}

	pub fn set_compressor(&mut self, compressor: Compressor) {
		self.compressor = compressor;
	}
}
