// In the scenario I'm trying to emulate here, `BType` would be a real type
// and BTypeExperimental would be the mock of `BType`.

#[cfg(not(feature = "experimental"))]
pub struct BType {}

#[cfg(feature = "experimental")]
pub struct BTypeExperimental {}
