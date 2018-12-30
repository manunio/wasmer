// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/types.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;

use crate::runtime::types::Value;
use crate::webassembly::{compile, instantiate, ImportObject, Instance, ResultObject};

use super::_common::{spectest_importobject, NaNCheck};

// Line 3
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func))
      (type (;2;) (func (param i32)))
      (type (;3;) (func (param i32)))
      (type (;4;) (func (result i32)))
      (type (;5;) (func (param i32) (result i32)))
      (type (;6;) (func (param i32) (result i32)))
      (type (;7;) (func (param f32 f64)))
      (type (;8;) (func (param f32 f64)))
      (type (;9;) (func (param f32 f64)))
      (type (;10;) (func (param f32 f64)))
      (type (;11;) (func (param f32 f64)))
      (type (;12;) (func (param f32 f64 i32 f64 i32 i32)))
      (type (;13;) (func (param f32 f64 i32))))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 44
#[test]
fn c1_l44_assert_malformed() {
    let wasm_binary = [
        40, 116, 121, 112, 101, 32, 40, 102, 117, 110, 99, 32, 40, 114, 101, 115, 117, 108, 116,
        32, 105, 51, 50, 41, 32, 40, 112, 97, 114, 97, 109, 32, 105, 51, 50, 41, 41, 41,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is malformed"
    );
}

// Line 48
#[test]
fn c2_l48_assert_malformed() {
    let wasm_binary = [
        40, 116, 121, 112, 101, 32, 40, 102, 117, 110, 99, 32, 40, 114, 101, 115, 117, 108, 116,
        32, 36, 120, 32, 105, 51, 50, 41, 41, 41,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is malformed"
    );
}

// Line 53
#[test]
fn c3_l53_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 0, 2, 127, 127];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 57
#[test]
fn c4_l57_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 0, 2, 127, 127];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

#[test]
fn test_module_1() {
    let mut result_object = create_module_1();
    // We group the calls together
    start_module_1(&mut result_object);
}
