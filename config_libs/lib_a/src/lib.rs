// Here, lib_a depends on the specific type of lib_b, only available when the
// "experimental" feature isn't enabled.

#[allow(dead_code)]
pub struct AType {
    pub b_type: lib_b::BType,
}
