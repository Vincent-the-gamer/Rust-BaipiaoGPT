use yewdux::store::Store;

#[derive(PartialEq, Eq, Clone, Default, Store)]
pub struct IsRequesting {
    pub value: bool
}

impl IsRequesting {
    pub fn set(&mut self, val: bool) {
        self.value = val;
    }
}