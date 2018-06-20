
/// Span creation context
#[derive(Debug)]
pub struct Description {
    name: &'static str,
    module_path: Option<&'static str>,
}

impl Description {
    pub fn new(name: &'static str) -> Description {
        Description {
            name,
            module_path: None,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn module_path(&self) -> Option<&'static str> {
        self.module_path.as_ref()
            .map(|s| &**s)
    }

    pub fn set_module_path(&mut self, val: &'static str) {
        self.module_path = Some(val);
    }
}
