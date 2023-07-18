#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/accessors/pass/*.rs");
    t.compile_fail("tests/ui/accessors/fail/*.rs");

    t.pass("tests/ui/contract/pass/*.rs");
    t.compile_fail("tests/ui/contract/fail/*.rs");

    t.pass("tests/ui/modifier_definition/pass/*.rs");
    t.compile_fail("tests/ui/modifier_definition/fail/*.rs");

    t.pass("tests/ui/storage_derive/pass/*.rs");
    t.compile_fail("tests/ui/storage_derive/fail/*.rs");

    t.pass("tests/ui/trait_definition/pass/*.rs");
    t.compile_fail("tests/ui/trait_definition/fail/*.rs");

    t.pass("tests/ui/storage_item/pass/*.rs");

    t.pass("tests/ui/wrapper/pass/*.rs");
    t.compile_fail("tests/ui/wrapper/fail/*.rs");
}
