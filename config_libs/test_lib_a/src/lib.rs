#[allow(dead_code)]
struct TestStruct {
    // test_lib_a wants to use an experimental feature from lib_b
    b_type_experimental: lib_b::BTypeExperimental,

    // test_lib_a aims to test lib_a so include `AType` here
    a_type: lib_a::AType,
}
