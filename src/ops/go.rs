use crate::go;

pub fn init() -> Vec<engine::OpDecl> {
    vec![go::external_function::decl()]
}
