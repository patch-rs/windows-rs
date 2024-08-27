pub trait IUserDataAccountProviderOperation_Impl: Sized + windows_core::IUnknownImpl {
    fn Kind(&self) -> windows_core::Result<UserDataAccountProviderOperationKind>;
}
impl windows_core::RuntimeName for IUserDataAccountProviderOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation";
}
impl IUserDataAccountProviderOperation_Vtbl {
    pub const fn new<Identity: IUserDataAccountProviderOperation_Impl, const OFFSET: isize>() -> IUserDataAccountProviderOperation_Vtbl {
        unsafe extern "system" fn Kind<Identity: IUserDataAccountProviderOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UserDataAccountProviderOperationKind) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUserDataAccountProviderOperation_Impl::Kind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserDataAccountProviderOperation, OFFSET>(), Kind: Kind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserDataAccountProviderOperation as windows_core::Interface>::IID
    }
}
