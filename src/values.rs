#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    Null,
    Number,
    Boolean,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    //abstract values
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeVal {
    NullVal(NullVal),
    NumberVal(NumberVal),
    BooleanVal(BooleanVal),
}

impl RuntimeVal {
    pub fn get_value_type(&self) -> ValueType {
        match self {
            RuntimeVal::NullVal(_) => ValueType::Null,
            RuntimeVal::NumberVal(_) => ValueType::Number,
            RuntimeVal::BooleanVal(_) => ValueType::Boolean,
        }
    }
    pub fn get_number_value(&self) -> i64 {
        if let RuntimeVal::NumberVal(number_val) = self {
            return number_val.value
        } else {
            panic!("Not a NumberVal");
        }
    }
    pub fn get_null_value(&self) -> Value {
        if let RuntimeVal::NullVal(null_val) = self {
            return null_val.value.clone()
        } else {
            panic!("Not a NullVall")
        }
    }
    pub fn get_bool_val(&self) -> Bool {
        if let RuntimeVal::BooleanVal(bool_val) = self {
            return bool_val.value.clone()
        } else {
            panic!("Not a BooleanVal")
        }
    }
    pub fn to_number_val(&self) -> NumberVal {
        return NumberVal {
            value_type: ValueType::Number,
            value: self.get_number_value(),
        }
    }
    pub fn to_null_val(&self) -> NullVal {
        return NullVal {
            value_type: ValueType::Null,
            value: self.get_null_value(),
        }
    }
    pub fn to_boolean_val(&self) -> BooleanVal {
        return BooleanVal {
            value_type: ValueType::Boolean,
            value: self.get_bool_val(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Bool {
    True,
    False,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NullVal {
    pub value_type: ValueType,
    pub value: Value,
}

impl NullVal {
    pub fn to_runtime_val(self) -> RuntimeVal {
        return RuntimeVal::NullVal(self)
    }
    pub fn make_null() -> NullVal {
        return NullVal {
            value_type : ValueType::Null,
            value: Value::Null,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanVal {
    pub value_type: ValueType,
    pub value: Bool,
}
impl BooleanVal {
    pub fn to_runtime_val(self) -> RuntimeVal {
        return RuntimeVal::BooleanVal(self)
    }
    pub fn make_bool(value: Bool) -> BooleanVal {
        return BooleanVal {
            value_type: ValueType::Boolean,
            value
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumberVal {
    pub value_type: ValueType,
    pub value: i64,
}

impl NumberVal {
    pub fn to_runtime_val(self) -> RuntimeVal {
        return RuntimeVal::NumberVal(self)
    }
    pub fn make_number(value: i64) -> NumberVal {
        return NumberVal {
            value_type: ValueType::Number,
            value
        }
    }
}