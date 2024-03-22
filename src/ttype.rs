/// Carbide type
pub trait TType {
    type Native;
    const KEYWORD: &'static str;
}

pub struct Bool {}
impl TType for Bool {
    type Native = bool;
    const KEYWORD: &'static str = "bool";
}
