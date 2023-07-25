use crate::Refrigerant;

pub trait Component {
	fn get_refrigerant(&self) -> &Refrigerant;
	fn process_refrigerant(&mut self, refrigerant: Refrigerant);
}
