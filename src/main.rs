mod components;
mod refrigerant;
mod simulation_context;
mod air_conditioner;

use components::{Compressor,Evaporator,ExpansionValve,Condenser};
use refrigerant::Refrigerant;
use simulation_context::SimulationContext;
use air_conditioner::AirConditioner;

fn main() {
	let simulation_context = SimulationContext::new(25,35, 120);
	let air_conditioner = build_air_conditioner();
	air_conditioner.set_target_temperature(20);
	air_conditioner.start();
}

fn build_air_conditioner() -> AirConditioner {
	let compressor = Compressor::new();
	let condenser = Condenser::new();
	let expansion_valve = ExpansionValve::new();
	let evaporator = Evaporator::new();

	compressor.set_condenser(condenser);
	evaporator.set_expansion_valve(expansion_valve);
	expansion_valve.set_evaporator(evaporator);
	condenser.set_compressor(compressor);

	AirConditioner {
		compressor,
		condenser,
		expansion_valve,
		evaporator
	}
}
