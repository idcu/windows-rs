#![allow(non_snake_case)]

use windows_core::*;

pub const E_POINTER: HRESULT = HRESULT(0x80004003_u32 as _);
pub const S_OK: HRESULT = HRESULT(0);

#[interface("09428a59-5b40-4e4c-9175-e7a78514316d")]
unsafe trait ITest: IUnknown {
    unsafe fn nothing(&self);
    unsafe fn hresult(&self) -> HRESULT;
    unsafe fn usize(&self, input: usize, output: *mut usize) -> HRESULT;
    unsafe fn hstring(&self, input: Borrow<HSTRING>, output: *mut HSTRING) -> HRESULT;
}

// TODO: interface macro should generate callside methods with AsBorrow traits?

#[implement(ITest)]
struct Test;

impl ITest_Impl for Test {
    unsafe fn nothing(&self) {}
    unsafe fn hresult(&self) -> HRESULT {
        S_OK
    }
    unsafe fn usize(&self, input: usize, output: *mut usize) -> HRESULT {
        if output.is_null() {
            return E_POINTER;
        }
        *output = input;
        S_OK
    }
    unsafe fn hstring(&self, input: Borrow<HSTRING>, output: *mut HSTRING) -> HRESULT {
        if output.is_null() {
            return E_POINTER;
        }
        *output = input.clone();
        S_OK
    }
}

#[test]
fn test() {
    unsafe {
        let test: ITest = Test.into();
        test.nothing();
        assert_eq!(test.hresult(), S_OK);

        assert_eq!(test.usize(0, std::ptr::null_mut()), E_POINTER);
        let mut output = 0;
        assert_eq!(test.usize(123, &mut output), S_OK);
        assert_eq!(output, 123);

        assert_eq!(test.hstring(h!("hello"), std::ptr::null_mut()), E_POINTER);
        let mut output = HSTRING::new();
        assert_eq!(test.hstring(h!("hello"), &mut output), S_OK);
        assert_eq!(&output, h!("hello"));
    }
}
