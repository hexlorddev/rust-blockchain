use wasmer::{Store, Module, Instance, imports, Value};
use parity_wasm::elements::Internal;
use std::error::Error;

pub struct SmartContractEngine {
    store: Store,
}

impl SmartContractEngine {
    pub fn new() -> Self {
        let store = Store::default();
        SmartContractEngine { store }
    }

    pub fn deploy_contract(&self, wasm_bytes: &[u8]) -> Result<Instance, Box<dyn Error>> {
        let module = Module::new(&self.store, wasm_bytes)?;
        let import_object = imports! {};
        let instance = Instance::new(&module, &import_object)?;
        Ok(instance)
    }

    pub fn execute_contract(
        &self,
        instance: &Instance,
        method: &str,
        args: Vec<Value>
    ) -> Result<Vec<Value>, Box<dyn Error>> {
        let func = instance.exports.get_function(method)?;
        let result = func.call(&args)?;
        Ok(result.to_vec())
    }

    pub fn validate_contract(wasm_bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        let module = parity_wasm::deserialize_buffer(wasm_bytes)?;
        
        // Check for forbidden operations
        for section in module.sections() {
            if let parity_wasm::elements::Section::Code(code_section) = section {
                for body in code_section.bodies() {
                    for op in body.code().elements() {
                        if let Internal::I32TruncSF64 = op {
                            return Err("Floating point operations not allowed".into());
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}