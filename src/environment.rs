use crate::values::RuntimeVal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeVal>,
    constants: Vec<String>,
}
impl Environment {
    pub fn new(parent_env: Option<Box<Environment>>) -> Self {
        return Environment {
            parent: parent_env,
            variables: HashMap::new(),
            constants: Vec::new(),
        }
    }

    pub fn declare_var(&mut self, varname: String, value: RuntimeVal, isconstant: bool) -> RuntimeVal {
        if self.variables.contains_key(&varname) {
            panic!("Cannot declare variable {}. It has alreeady been initialized.", varname)
        }
        self.variables.insert(varname.clone(), value.clone());
        if isconstant {
            self.constants.push(varname)
        }
        return value
    }
    pub fn assign_var(&mut self, varname: String, value: RuntimeVal) -> RuntimeVal {
        let env = self.resolve(varname.clone());
        if env.constants.contains(&varname) {
            panic!("Cannot reasign to variable {} as it is a constant.", varname)
        }
        env.variables.remove(&varname);
        env.variables.insert(varname, value.clone());
        return value
    }

    pub fn lookup_var(&mut self, varname: String) -> RuntimeVal {
        let env = self.resolve(varname.clone());
        return env.variables.get(&varname).expect("literally cant be none and if this does end up being none then shist").clone()
    }
    pub fn resolve(&mut self, varname: String) -> &mut Environment {
        if self.variables.contains_key(&varname) {
            return self
        }
        match self.parent {
            Some(_) => {
                //let environment = &self.parent;
                //let mut env = environment.as_ref().expect("environment should exist");
                return self.parent.as_mut().expect("environment should exist")
            },
            None => {
                panic!("cannot resolve '{}', as it does not exist", varname)
            },
        }
    }
}