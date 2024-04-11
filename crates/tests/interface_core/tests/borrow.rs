#![allow(non_snake_case)]

use windows_core::*;

pub const S_OK: HRESULT = HRESULT(0);
pub const E_INVALIDARG: HRESULT = HRESULT(0x80070057_u32 as _);
pub const E_POINTER: HRESULT = HRESULT(0x80004003_u32 as _);

#[interface("09428a59-5b40-4e4c-9175-e7a78514316d")]
unsafe trait ITest: IUnknown {
    unsafe fn usize(&self, input: usize, output: *mut usize) -> HRESULT;
    unsafe fn hstring(&self, input: Borrow<HSTRING>, output: *mut HSTRING) -> HRESULT;
    unsafe fn interface(&self, input: Borrow<ITest>, output: *mut Option<ITest>) -> HRESULT;
}

// TODO: interface macro should generate callside methods with AsBorrow traits?

#[implement(ITest)]
struct Test;

impl ITest_Impl for Test {
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
    unsafe fn interface(&self, input: Borrow<ITest>, output: *mut Option<ITest>) -> HRESULT {
        if input.is_none() {
            return E_INVALIDARG;
        }
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

        assert_eq!(test.usize(0, std::ptr::null_mut()), E_POINTER);
        let mut output = 0;
        assert_eq!(test.usize(123, &mut output), S_OK);
        assert_eq!(output, 123);

        assert_eq!(test.hstring(h!("hello"), std::ptr::null_mut()), E_POINTER);
        let mut output = HSTRING::new();
        assert_eq!(test.hstring(h!("hello"), &mut output), S_OK);
        assert_eq!(&output, h!("hello"));

        assert_eq!(test.interface(None, std::ptr::null_mut()), E_INVALIDARG);
        assert_eq!(test.interface(&test, std::ptr::null_mut()), E_POINTER);
        let mut output = None;
        assert_eq!(test.interface(&test, &mut output), S_OK);
        assert_eq!(test.interface(Some(&test), &mut output), S_OK);
        assert_eq!(output.as_ref(), Some(&test));
    }
}
