#[cfg(feature = "Foundation_Collections")]
pub trait IStorageProviderItemPropertySource_Impl: Sized + windows_core::IUnknownImpl {
    fn GetItemProperties(&self, itempath: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IStorageProviderItemPropertySource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderItemPropertySource";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageProviderItemPropertySource_Vtbl {
    pub const fn new<Identity: IStorageProviderItemPropertySource_Impl, const OFFSET: isize>() -> IStorageProviderItemPropertySource_Vtbl {
        unsafe extern "system" fn GetItemProperties<Identity: IStorageProviderItemPropertySource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itempath: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderItemPropertySource_Impl::GetItemProperties(this, core::mem::transmute(&itempath)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderItemPropertySource, OFFSET>(),
            GetItemProperties: GetItemProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderItemPropertySource as windows_core::Interface>::IID
    }
}
pub trait IStorageProviderKnownFolderSyncInfoSource_Impl: Sized + windows_core::IUnknownImpl {
    fn GetKnownFolderSyncInfo(&self) -> windows_core::Result<StorageProviderKnownFolderSyncInfo>;
    fn KnownFolderSyncInfoChanged(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IStorageProviderKnownFolderSyncInfoSource, windows_core::IInspectable>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKnownFolderSyncInfoChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IStorageProviderKnownFolderSyncInfoSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderKnownFolderSyncInfoSource";
}
impl IStorageProviderKnownFolderSyncInfoSource_Vtbl {
    pub const fn new<Identity: IStorageProviderKnownFolderSyncInfoSource_Impl, const OFFSET: isize>() -> IStorageProviderKnownFolderSyncInfoSource_Vtbl {
        unsafe extern "system" fn GetKnownFolderSyncInfo<Identity: IStorageProviderKnownFolderSyncInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderKnownFolderSyncInfoSource_Impl::GetKnownFolderSyncInfo(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnownFolderSyncInfoChanged<Identity: IStorageProviderKnownFolderSyncInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderKnownFolderSyncInfoSource_Impl::KnownFolderSyncInfoChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKnownFolderSyncInfoChanged<Identity: IStorageProviderKnownFolderSyncInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStorageProviderKnownFolderSyncInfoSource_Impl::RemoveKnownFolderSyncInfoChanged(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderKnownFolderSyncInfoSource, OFFSET>(),
            GetKnownFolderSyncInfo: GetKnownFolderSyncInfo::<Identity, OFFSET>,
            KnownFolderSyncInfoChanged: KnownFolderSyncInfoChanged::<Identity, OFFSET>,
            RemoveKnownFolderSyncInfoChanged: RemoveKnownFolderSyncInfoChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderKnownFolderSyncInfoSource as windows_core::Interface>::IID
    }
}
pub trait IStorageProviderKnownFolderSyncInfoSourceFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn GetKnownFolderSyncInfoSource(&self) -> windows_core::Result<IStorageProviderKnownFolderSyncInfoSource>;
}
impl windows_core::RuntimeName for IStorageProviderKnownFolderSyncInfoSourceFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderKnownFolderSyncInfoSourceFactory";
}
impl IStorageProviderKnownFolderSyncInfoSourceFactory_Vtbl {
    pub const fn new<Identity: IStorageProviderKnownFolderSyncInfoSourceFactory_Impl, const OFFSET: isize>() -> IStorageProviderKnownFolderSyncInfoSourceFactory_Vtbl {
        unsafe extern "system" fn GetKnownFolderSyncInfoSource<Identity: IStorageProviderKnownFolderSyncInfoSourceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderKnownFolderSyncInfoSourceFactory_Impl::GetKnownFolderSyncInfoSource(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderKnownFolderSyncInfoSourceFactory, OFFSET>(),
            GetKnownFolderSyncInfoSource: GetKnownFolderSyncInfoSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderKnownFolderSyncInfoSourceFactory as windows_core::Interface>::IID
    }
}
pub trait IStorageProviderPropertyCapabilities_Impl: Sized + windows_core::IUnknownImpl {
    fn IsPropertySupported(&self, propertycanonicalname: &windows_core::HSTRING) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IStorageProviderPropertyCapabilities {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderPropertyCapabilities";
}
impl IStorageProviderPropertyCapabilities_Vtbl {
    pub const fn new<Identity: IStorageProviderPropertyCapabilities_Impl, const OFFSET: isize>() -> IStorageProviderPropertyCapabilities_Vtbl {
        unsafe extern "system" fn IsPropertySupported<Identity: IStorageProviderPropertyCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertycanonicalname: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderPropertyCapabilities_Impl::IsPropertySupported(this, core::mem::transmute(&propertycanonicalname)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderPropertyCapabilities, OFFSET>(),
            IsPropertySupported: IsPropertySupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderPropertyCapabilities as windows_core::Interface>::IID
    }
}
pub trait IStorageProviderStatusUISource_Impl: Sized + windows_core::IUnknownImpl {
    fn GetStatusUI(&self) -> windows_core::Result<StorageProviderStatusUI>;
    fn StatusUIChanged(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IStorageProviderStatusUISource, windows_core::IInspectable>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUIChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IStorageProviderStatusUISource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusUISource";
}
impl IStorageProviderStatusUISource_Vtbl {
    pub const fn new<Identity: IStorageProviderStatusUISource_Impl, const OFFSET: isize>() -> IStorageProviderStatusUISource_Vtbl {
        unsafe extern "system" fn GetStatusUI<Identity: IStorageProviderStatusUISource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderStatusUISource_Impl::GetStatusUI(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUIChanged<Identity: IStorageProviderStatusUISource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderStatusUISource_Impl::StatusUIChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUIChanged<Identity: IStorageProviderStatusUISource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStorageProviderStatusUISource_Impl::RemoveStatusUIChanged(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderStatusUISource, OFFSET>(),
            GetStatusUI: GetStatusUI::<Identity, OFFSET>,
            StatusUIChanged: StatusUIChanged::<Identity, OFFSET>,
            RemoveStatusUIChanged: RemoveStatusUIChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderStatusUISource as windows_core::Interface>::IID
    }
}
pub trait IStorageProviderStatusUISourceFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn GetStatusUISource(&self, syncrootid: &windows_core::HSTRING) -> windows_core::Result<IStorageProviderStatusUISource>;
}
impl windows_core::RuntimeName for IStorageProviderStatusUISourceFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusUISourceFactory";
}
impl IStorageProviderStatusUISourceFactory_Vtbl {
    pub const fn new<Identity: IStorageProviderStatusUISourceFactory_Impl, const OFFSET: isize>() -> IStorageProviderStatusUISourceFactory_Vtbl {
        unsafe extern "system" fn GetStatusUISource<Identity: IStorageProviderStatusUISourceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncrootid: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderStatusUISourceFactory_Impl::GetStatusUISource(this, core::mem::transmute(&syncrootid)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderStatusUISourceFactory, OFFSET>(),
            GetStatusUISource: GetStatusUISource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderStatusUISourceFactory as windows_core::Interface>::IID
    }
}
pub trait IStorageProviderUICommand_Impl: Sized + windows_core::IUnknownImpl {
    fn Label(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Description(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Icon(&self) -> windows_core::Result<super::super::Foundation::Uri>;
    fn State(&self) -> windows_core::Result<StorageProviderUICommandState>;
    fn Invoke(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IStorageProviderUICommand {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderUICommand";
}
impl IStorageProviderUICommand_Vtbl {
    pub const fn new<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>() -> IStorageProviderUICommand_Vtbl {
        unsafe extern "system" fn Label<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderUICommand_Impl::Label(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderUICommand_Impl::Description(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Icon<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderUICommand_Impl::Icon(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut StorageProviderUICommandState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageProviderUICommand_Impl::State(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Identity: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStorageProviderUICommand_Impl::Invoke(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderUICommand, OFFSET>(),
            Label: Label::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Icon: Icon::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderUICommand as windows_core::Interface>::IID
    }
}
pub trait IStorageProviderUriSource_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPathForContentUri(&self, contenturi: &windows_core::HSTRING, result: Option<&StorageProviderGetPathForContentUriResult>) -> windows_core::Result<()>;
    fn GetContentInfoForPath(&self, path: &windows_core::HSTRING, result: Option<&StorageProviderGetContentInfoForPathResult>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IStorageProviderUriSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderUriSource";
}
impl IStorageProviderUriSource_Vtbl {
    pub const fn new<Identity: IStorageProviderUriSource_Impl, const OFFSET: isize>() -> IStorageProviderUriSource_Vtbl {
        unsafe extern "system" fn GetPathForContentUri<Identity: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenturi: core::mem::MaybeUninit<windows_core::HSTRING>, result: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStorageProviderUriSource_Impl::GetPathForContentUri(this, core::mem::transmute(&contenturi), windows_core::from_raw_borrowed(&result)).into()
        }
        unsafe extern "system" fn GetContentInfoForPath<Identity: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::HSTRING>, result: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStorageProviderUriSource_Impl::GetContentInfoForPath(this, core::mem::transmute(&path), windows_core::from_raw_borrowed(&result)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageProviderUriSource, OFFSET>(),
            GetPathForContentUri: GetPathForContentUri::<Identity, OFFSET>,
            GetContentInfoForPath: GetContentInfoForPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderUriSource as windows_core::Interface>::IID
    }
}
