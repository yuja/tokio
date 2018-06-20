
#[derive(Debug)]
pub struct Context {
    parent: Option<usize>,
}

impl Context {
    pub(crate) fn new(parent: Option<usize>) -> Context {
        Context {
            parent,
        }
    }

    pub fn parent(&self) -> Option<usize> {
        self.parent
    }
}
