mod bindings;

use bindings::*;
use windows_core::*;

#[implement(ITest)]
struct Test(std::sync::RwLock<(HSTRING, i32, Option<ITest>)>);

impl ITest_Impl for Test {
    fn MethodString(&self, test: &HSTRING) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.0 = test.clone();
        Ok(())
    }
    fn MethodStringN(&self, test: &HSTRING) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.0 = test.clone();
        Ok(())
    }
    fn MethodInt32(&self, test: i32) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.1 = test;
        Ok(())
    }
    fn MethodInt32N(&self, test: i32) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.1 = test;
        Ok(())
    }
    fn MethodTest(&self, test: Option<&ITest>) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.2 = test.cloned();
        Ok(())
    }
    fn MethodTestN(&self, test: Option<&ITest>) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.2 = test.cloned();
        Ok(())
    }
    fn String(&self) -> Result<HSTRING> {
        let this = self.0.read().unwrap();
        Ok(this.0.clone())
    }
    fn SetString(&self, value: &HSTRING) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.0 = value.clone();
        Ok(())
    }
    fn StringN(&self) -> Result<HSTRING> {
        let this = self.0.read().unwrap();
        Ok(this.0.clone())
    }
    fn SetStringN(&self, value: &HSTRING) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.0 = value.clone();
        Ok(())
    }
    fn Int32(&self) -> Result<i32> {
        let this = self.0.read().unwrap();
        Ok(this.1)
    }
    fn SetInt32(&self, value: i32) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.1 = value;
        Ok(())
    }
    fn Int32N(&self) -> Result<i32> {
        let this = self.0.read().unwrap();
        Ok(this.1)
    }
    fn SetInt32N(&self, value: i32) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.1 = value;
        Ok(())
    }
    fn Test(&self) -> Result<ITest> {
        let this = self.0.read().unwrap();
        this.2.clone().ok_or_else(||Error::empty())
    }
    fn SetTest(&self, value: Option<&ITest>) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.2 = value.cloned();
        Ok(())
    }
    fn TestN(&self) -> Result<ITest> {
        let this = self.0.read().unwrap();
        this.2.clone().ok_or_else(||Error::empty())
    }
    fn SetTestN(&self, value: Option<&ITest>) -> Result<()> {
        let mut this = self.0.write().unwrap();
        this.2 = value.cloned();
        Ok(())
    }
}
