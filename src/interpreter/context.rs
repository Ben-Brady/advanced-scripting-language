use std::collections::HashMap;
use super::{Value, Error};

pub struct Context {
    variables: HashMap<String, Value>,
    pub output: String,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: HashMap::new(),
            output: String::new(),
        }
    }

    pub fn get_variable(&mut self, name: String) -> Result<Value, Error>{
        let value_option = self.variables.get(&name);
        if let Some(value) = value_option {
            Ok(value.to_owned())
        } else {
            Err(Error::VariableDoesNotExist(name))
        }
    }

    pub fn declare_variable(&mut self, name: String, value: Value) -> Result<(), Error> {
        if self.variables.contains_key(&name) {
            return Err(Error::VariableAlreadyDeclared(name));
        }
        
        self.variables.insert(name, value);
        Ok(())
    }

    pub fn assign_variable(&mut self, name: String, value: Value) -> Result<(), Error> {
        if !self.variables.contains_key(&name) {
            return Err(Error::VariableDoesNotExist(name));
        }
        
        // Updates variable inplace
        self.variables.insert(name, value); 
        Ok(())
    }
}
