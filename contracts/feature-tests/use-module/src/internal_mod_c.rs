dharitri_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[dharitri_wasm_derive::module]
pub trait InternalModuleC {
	#[view]
	fn call_mod_c(&self) {}
}
