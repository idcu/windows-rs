use super::*;

// TODO: maybe Borrow is what windows-bindgen should use rather than ManuallyDrop for ABI fields and parameters
// and then have the Param trait become AsBorrow and ParamValue become BorrowValue for the temporary to cover required interfaces...
// ParamValue.abi would then return a Borrow value rather than t::Abi - maybe Borrow should be called Abi?

// TODO: maybe use this instead of Param for generated bindings - except it doesn't
// handle ownership for WinRT required interfaces 🙄 but maybe we want to drop
// that anyway to avoid the implicit QI cost being hidden...

/// A wrapper type for borrowed Windows values providing transparent representation.


// TODO: rename Abi
#[repr(transparent)]
pub struct Borrow<'a, T: Type<T>>(T::Abi, std::marker::PhantomData<&'a T>); // TODO: should track lifetime

impl<'a, T: Type<T>> std::ops::Deref for Borrow<'a, T> {
    type Target = T::Default;

    fn deref(&self) -> &T::Default {
        unsafe { std::mem::transmute(&self.0) }
    }
}

impl<'a, T: Type<T>> Borrow<'a, T> {
    /// Returns the borrowed value as a cloned result.
    pub fn ok(&self) -> Result<T> {
        T::from_default(self)
    }
}

// TODO: the interface macro can generate AsRef<Borrow<T>> for the callable interface type?

// TODO: provide a bunch of AsRef for handling common types
// impl<'a, T: Type<T>> AsRef<Borrow<'a, T>> for Option<&'a T> {
//     fn as_ref(&self) -> &Borrow<'a, T>
// }