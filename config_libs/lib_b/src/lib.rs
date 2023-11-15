#[cfg(not(feature = "experimental"))]
pub struct BType {}

#[cfg(feature = "experimental")]
pub struct BTypeExperimental {}
