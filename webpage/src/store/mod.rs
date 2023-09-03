use yewdux::store::Store;

#[derive(PartialEq, Eq, Default, Store, Clone)]
pub struct InputContent{
    pub text: String
}