pub trait IWaaSAssessor_Impl: Sized + windows_core::IUnknownImpl {
    fn GetOSUpdateAssessment(&self) -> windows_core::Result<OSUpdateAssessment>;
}
impl windows_core::RuntimeName for IWaaSAssessor {}
impl IWaaSAssessor_Vtbl {
    pub const fn new<Identity: IWaaSAssessor_Impl, const OFFSET: isize>() -> IWaaSAssessor_Vtbl {
        unsafe extern "system" fn GetOSUpdateAssessment<Identity: IWaaSAssessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: *mut OSUpdateAssessment) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWaaSAssessor_Impl::GetOSUpdateAssessment(this) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOSUpdateAssessment: GetOSUpdateAssessment::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWaaSAssessor as windows_core::Interface>::IID
    }
}
