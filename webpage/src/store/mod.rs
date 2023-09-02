use yewdux::store::Store;

#[derive(PartialEq, Default, Store)]
struct InputContent{
    text: String
}