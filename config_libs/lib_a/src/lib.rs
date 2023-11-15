// Here, lib_a depends on lib_b, only available when the
// "experimental" feature is _not_ enabled.

#[allow(dead_code)]
pub struct AType {
    #[cfg(not(feature = "experimental_a"))]
    pub b_type: lib_b::BType,

    #[cfg(feature = "experimental_a")]
    pub b_type: lib_b::BTypeExperimental,
}
