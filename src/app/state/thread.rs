use std::collections::HashMap;

pub struct PiVariables {
    /// Contains a list of variables and their internal values.
    /// Although variables will have different names, only their
    /// internal values will be displayed.
    variables: HashMap<String, String>,
}

impl PiVariables {
    pub fn new() -> Self {
        Self { variables: HashMap::new() }
    }
}

pub struct PiThread {
    //
    local: PiVariables,
    
}