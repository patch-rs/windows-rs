pub trait IActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl {
    fn Kind(&self) -> windows_core::Result<ActivationKind>;
    fn PreviousExecutionState(&self) -> windows_core::Result<ApplicationExecutionState>;
    fn SplashScreen(&self) -> windows_core::Result<SplashScreen>;
}
impl windows_core::RuntimeName for IActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgs";
}
impl IActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IActivatedEventArgs_Impl, const OFFSET: isize>() -> IActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Kind<Identity: IActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ActivationKind) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActivatedEventArgs_Impl::Kind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviousExecutionState<Identity: IActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ApplicationExecutionState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActivatedEventArgs_Impl::PreviousExecutionState(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SplashScreen<Identity: IActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActivatedEventArgs_Impl::SplashScreen(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivatedEventArgs, OFFSET>(),
            Kind: Kind::<Identity, OFFSET>,
            PreviousExecutionState: PreviousExecutionState::<Identity, OFFSET>,
            SplashScreen: SplashScreen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait IActivatedEventArgsWithUser_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn User(&self) -> windows_core::Result<super::super::System::User>;
}
#[cfg(feature = "System")]
impl windows_core::RuntimeName for IActivatedEventArgsWithUser {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
}
#[cfg(feature = "System")]
impl IActivatedEventArgsWithUser_Vtbl {
    pub const fn new<Identity: IActivatedEventArgsWithUser_Impl, const OFFSET: isize>() -> IActivatedEventArgsWithUser_Vtbl {
        unsafe extern "system" fn User<Identity: IActivatedEventArgsWithUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActivatedEventArgsWithUser_Impl::User(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivatedEventArgsWithUser, OFFSET>(), User: User::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivatedEventArgsWithUser as windows_core::Interface>::IID
    }
}
pub trait IApplicationViewActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IApplicationViewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
}
impl IApplicationViewActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IApplicationViewActivatedEventArgs_Impl, const OFFSET: isize>() -> IApplicationViewActivatedEventArgs_Vtbl {
        unsafe extern "system" fn CurrentlyShownApplicationViewId<Identity: IApplicationViewActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IApplicationViewActivatedEventArgs_Impl::CurrentlyShownApplicationViewId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IApplicationViewActivatedEventArgs, OFFSET>(),
            CurrentlyShownApplicationViewId: CurrentlyShownApplicationViewId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationViewActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IAppointmentsProviderActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IAppointmentsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs";
}
impl IAppointmentsProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Verb<Identity: IAppointmentsProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppointmentsProviderActivatedEventArgs_Impl::Verb(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderActivatedEventArgs, OFFSET>(), Verb: Verb::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn AddAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl windows_core::RuntimeName for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
        unsafe extern "system" fn AddAppointmentOperation<Identity: IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppointmentsProviderAddAppointmentActivatedEventArgs_Impl::AddAppointmentOperation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderAddAppointmentActivatedEventArgs, OFFSET>(),
            AddAppointmentOperation: AddAppointmentOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderAddAppointmentActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn RemoveAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl windows_core::RuntimeName for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
        unsafe extern "system" fn RemoveAppointmentOperation<Identity: IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Impl::RemoveAppointmentOperation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderRemoveAppointmentActivatedEventArgs, OFFSET>(),
            RemoveAppointmentOperation: RemoveAppointmentOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderRemoveAppointmentActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub trait IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn ReplaceAppointmentOperation(&self) -> windows_core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>;
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl windows_core::RuntimeName for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ReplaceAppointmentOperation<Identity: IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Impl::ReplaceAppointmentOperation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderReplaceAppointmentActivatedEventArgs, OFFSET>(),
            ReplaceAppointmentOperation: ReplaceAppointmentOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderReplaceAppointmentActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn InstanceStartDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn LocalId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn RoamingId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
        unsafe extern "system" fn InstanceStartDate<Identity: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl::InstanceStartDate(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalId<Identity: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl::LocalId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoamingId<Identity: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Impl::RoamingId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, OFFSET>(),
            InstanceStartDate: InstanceStartDate::<Identity, OFFSET>,
            LocalId: LocalId::<Identity, OFFSET>,
            RoamingId: RoamingId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IAppointmentsProviderActivatedEventArgs_Impl {
    fn TimeToShow(&self) -> windows_core::Result<super::super::Foundation::DateTime>;
    fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan>;
}
impl windows_core::RuntimeName for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: isize>() -> IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
        unsafe extern "system" fn TimeToShow<Identity: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl::TimeToShow(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Identity: IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppointmentsProviderShowTimeFrameActivatedEventArgs_Impl::Duration(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAppointmentsProviderShowTimeFrameActivatedEventArgs, OFFSET>(),
            TimeToShow: TimeToShow::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppointmentsProviderShowTimeFrameActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Background")]
pub trait IBackgroundActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl {
    fn TaskInstance(&self) -> windows_core::Result<super::Background::IBackgroundTaskInstance>;
}
#[cfg(feature = "ApplicationModel_Background")]
impl windows_core::RuntimeName for IBackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Background")]
impl IBackgroundActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IBackgroundActivatedEventArgs_Impl, const OFFSET: isize>() -> IBackgroundActivatedEventArgs_Vtbl {
        unsafe extern "system" fn TaskInstance<Identity: IBackgroundActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBackgroundActivatedEventArgs_Impl::TaskInstance(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundActivatedEventArgs, OFFSET>(),
            TaskInstance: TaskInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IBarcodeScannerPreviewActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn ConnectionId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs";
}
impl IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IBarcodeScannerPreviewActivatedEventArgs_Impl, const OFFSET: isize>() -> IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ConnectionId<Identity: IBarcodeScannerPreviewActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBarcodeScannerPreviewActivatedEventArgs_Impl::ConnectionId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBarcodeScannerPreviewActivatedEventArgs, OFFSET>(),
            ConnectionId: ConnectionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBarcodeScannerPreviewActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Provider")]
pub trait ICachedFileUpdaterActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn CachedFileUpdaterUI(&self) -> windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI>;
}
#[cfg(feature = "Storage_Provider")]
impl windows_core::RuntimeName for ICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs";
}
#[cfg(feature = "Storage_Provider")]
impl ICachedFileUpdaterActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ICachedFileUpdaterActivatedEventArgs_Impl, const OFFSET: isize>() -> ICachedFileUpdaterActivatedEventArgs_Vtbl {
        unsafe extern "system" fn CachedFileUpdaterUI<Identity: ICachedFileUpdaterActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICachedFileUpdaterActivatedEventArgs_Impl::CachedFileUpdaterUI(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICachedFileUpdaterActivatedEventArgs, OFFSET>(),
            CachedFileUpdaterUI: CachedFileUpdaterUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICachedFileUpdaterActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait ICameraSettingsActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn VideoDeviceController(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn VideoDeviceExtension(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl windows_core::RuntimeName for ICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs";
}
impl ICameraSettingsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: isize>() -> ICameraSettingsActivatedEventArgs_Vtbl {
        unsafe extern "system" fn VideoDeviceController<Identity: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICameraSettingsActivatedEventArgs_Impl::VideoDeviceController(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceExtension<Identity: ICameraSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICameraSettingsActivatedEventArgs_Impl::VideoDeviceExtension(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICameraSettingsActivatedEventArgs, OFFSET>(),
            VideoDeviceController: VideoDeviceController::<Identity, OFFSET>,
            VideoDeviceExtension: VideoDeviceExtension::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICameraSettingsActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait ICommandLineActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Operation(&self) -> windows_core::Result<CommandLineActivationOperation>;
}
impl windows_core::RuntimeName for ICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs";
}
impl ICommandLineActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ICommandLineActivatedEventArgs_Impl, const OFFSET: isize>() -> ICommandLineActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Operation<Identity: ICommandLineActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommandLineActivatedEventArgs_Impl::Operation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommandLineActivatedEventArgs, OFFSET>(), Operation: Operation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandLineActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IContactActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IContactActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactActivatedEventArgs";
}
impl IContactActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Verb<Identity: IContactActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactActivatedEventArgs_Impl::Verb(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactActivatedEventArgs, OFFSET>(), Verb: Verb::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactCallActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactCallActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ServiceId<Identity: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactCallActivatedEventArgs_Impl::ServiceId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactCallActivatedEventArgs_Impl::ServiceUserId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactCallActivatedEventArgs_Impl::Contact(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactCallActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactCallActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMapActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn Address(&self) -> windows_core::Result<super::Contacts::ContactAddress>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactMapActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactMapActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactMapActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Address<Identity: IContactMapActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactMapActivatedEventArgs_Impl::Address(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactMapActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactMapActivatedEventArgs_Impl::Contact(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactMapActivatedEventArgs, OFFSET>(),
            Address: Address::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactMapActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactMessageActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactMessageActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactMessageActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ServiceId<Identity: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactMessageActivatedEventArgs_Impl::ServiceId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactMessageActivatedEventArgs_Impl::ServiceUserId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactMessageActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactMessageActivatedEventArgs_Impl::Contact(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactMessageActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactMessageActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPanelActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl {
    fn ContactPanel(&self) -> windows_core::Result<super::Contacts::ContactPanel>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactPanelActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactPanelActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactPanelActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ContactPanel<Identity: IContactPanelActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactPanelActivatedEventArgs_Impl::ContactPanel(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactPanelActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactPanelActivatedEventArgs_Impl::Contact(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactPanelActivatedEventArgs, OFFSET>(),
            ContactPanel: ContactPanel::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactPanelActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub trait IContactPickerActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn ContactPickerUI(&self) -> windows_core::Result<super::Contacts::Provider::ContactPickerUI>;
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl windows_core::RuntimeName for IContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
impl IContactPickerActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactPickerActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactPickerActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ContactPickerUI<Identity: IContactPickerActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactPickerActivatedEventArgs_Impl::ContactPickerUI(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactPickerActivatedEventArgs, OFFSET>(),
            ContactPickerUI: ContactPickerUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactPickerActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactPostActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactPostActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactPostActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ServiceId<Identity: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactPostActivatedEventArgs_Impl::ServiceId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactPostActivatedEventArgs_Impl::ServiceUserId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactPostActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactPostActivatedEventArgs_Impl::Contact(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactPostActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactPostActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Contacts")]
pub trait IContactVideoCallActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IContactActivatedEventArgs_Impl {
    fn ServiceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ServiceUserId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Contact(&self) -> windows_core::Result<super::Contacts::Contact>;
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl windows_core::RuntimeName for IContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Contacts")]
impl IContactVideoCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactVideoCallActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ServiceId<Identity: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactVideoCallActivatedEventArgs_Impl::ServiceId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUserId<Identity: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactVideoCallActivatedEventArgs_Impl::ServiceUserId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contact<Identity: IContactVideoCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactVideoCallActivatedEventArgs_Impl::Contact(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactVideoCallActivatedEventArgs, OFFSET>(),
            ServiceId: ServiceId::<Identity, OFFSET>,
            ServiceUserId: ServiceUserId::<Identity, OFFSET>,
            Contact: Contact::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactVideoCallActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IContactsProviderActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IContactsProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContactsProviderActivatedEventArgs";
}
impl IContactsProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContactsProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> IContactsProviderActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Verb<Identity: IContactsProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactsProviderActivatedEventArgs_Impl::Verb(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactsProviderActivatedEventArgs, OFFSET>(), Verb: Verb::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactsProviderActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IContinuationActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IContinuationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs";
}
#[cfg(feature = "Foundation_Collections")]
impl IContinuationActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IContinuationActivatedEventArgs_Impl, const OFFSET: isize>() -> IContinuationActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ContinuationData<Identity: IContinuationActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContinuationActivatedEventArgs_Impl::ContinuationData(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContinuationActivatedEventArgs, OFFSET>(),
            ContinuationData: ContinuationData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContinuationActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IDeviceActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn DeviceInformationId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs";
}
impl IDeviceActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IDeviceActivatedEventArgs_Impl, const OFFSET: isize>() -> IDeviceActivatedEventArgs_Vtbl {
        unsafe extern "system" fn DeviceInformationId<Identity: IDeviceActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceActivatedEventArgs_Impl::DeviceInformationId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verb<Identity: IDeviceActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceActivatedEventArgs_Impl::Verb(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDeviceActivatedEventArgs, OFFSET>(),
            DeviceInformationId: DeviceInformationId::<Identity, OFFSET>,
            Verb: Verb::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Devices_Enumeration")]
pub trait IDevicePairingActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn DeviceInformation(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
#[cfg(feature = "Devices_Enumeration")]
impl windows_core::RuntimeName for IDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs";
}
#[cfg(feature = "Devices_Enumeration")]
impl IDevicePairingActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IDevicePairingActivatedEventArgs_Impl, const OFFSET: isize>() -> IDevicePairingActivatedEventArgs_Vtbl {
        unsafe extern "system" fn DeviceInformation<Identity: IDevicePairingActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDevicePairingActivatedEventArgs_Impl::DeviceInformation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDevicePairingActivatedEventArgs, OFFSET>(),
            DeviceInformation: DeviceInformation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDevicePairingActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IDialReceiverActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn AppName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs";
}
impl IDialReceiverActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IDialReceiverActivatedEventArgs_Impl, const OFFSET: isize>() -> IDialReceiverActivatedEventArgs_Vtbl {
        unsafe extern "system" fn AppName<Identity: IDialReceiverActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDialReceiverActivatedEventArgs_Impl::AppName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDialReceiverActivatedEventArgs, OFFSET>(), AppName: AppName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDialReceiverActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
pub trait IFileActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Files(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>;
    fn Verb(&self) -> windows_core::Result<windows_core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
impl windows_core::RuntimeName for IFileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
impl IFileActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IFileActivatedEventArgs_Impl, const OFFSET: isize>() -> IFileActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Files<Identity: IFileActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileActivatedEventArgs_Impl::Files(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Verb<Identity: IFileActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileActivatedEventArgs_Impl::Verb(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileActivatedEventArgs, OFFSET>(),
            Files: Files::<Identity, OFFSET>,
            Verb: Verb::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IFileActivatedEventArgsWithCallerPackageFamilyName_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithCallerPackageFamilyName";
}
impl IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
    pub const fn new<Identity: IFileActivatedEventArgsWithCallerPackageFamilyName_Impl, const OFFSET: isize>() -> IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: IFileActivatedEventArgsWithCallerPackageFamilyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileActivatedEventArgsWithCallerPackageFamilyName_Impl::CallerPackageFamilyName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileActivatedEventArgsWithCallerPackageFamilyName, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileActivatedEventArgsWithCallerPackageFamilyName as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
pub trait IFileActivatedEventArgsWithNeighboringFiles_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IFileActivatedEventArgs_Impl {
    fn NeighboringFilesQuery(&self) -> windows_core::Result<super::super::Storage::Search::StorageFileQueryResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
impl windows_core::RuntimeName for IFileActivatedEventArgsWithNeighboringFiles {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
impl IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
    pub const fn new<Identity: IFileActivatedEventArgsWithNeighboringFiles_Impl, const OFFSET: isize>() -> IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
        unsafe extern "system" fn NeighboringFilesQuery<Identity: IFileActivatedEventArgsWithNeighboringFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileActivatedEventArgsWithNeighboringFiles_Impl::NeighboringFilesQuery(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileActivatedEventArgsWithNeighboringFiles, OFFSET>(),
            NeighboringFilesQuery: NeighboringFilesQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileActivatedEventArgsWithNeighboringFiles as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileOpenPickerActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn FileOpenPickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl windows_core::RuntimeName for IFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs";
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl IFileOpenPickerActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IFileOpenPickerActivatedEventArgs_Impl, const OFFSET: isize>() -> IFileOpenPickerActivatedEventArgs_Vtbl {
        unsafe extern "system" fn FileOpenPickerUI<Identity: IFileOpenPickerActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileOpenPickerActivatedEventArgs_Impl::FileOpenPickerUI(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileOpenPickerActivatedEventArgs, OFFSET>(),
            FileOpenPickerUI: FileOpenPickerUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileOpenPickerActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IFileOpenPickerActivatedEventArgs2_Impl: Sized + windows_core::IUnknownImpl {
    fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IFileOpenPickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2";
}
impl IFileOpenPickerActivatedEventArgs2_Vtbl {
    pub const fn new<Identity: IFileOpenPickerActivatedEventArgs2_Impl, const OFFSET: isize>() -> IFileOpenPickerActivatedEventArgs2_Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: IFileOpenPickerActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileOpenPickerActivatedEventArgs2_Impl::CallerPackageFamilyName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileOpenPickerActivatedEventArgs2, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileOpenPickerActivatedEventArgs2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFileOpenPickerContinuationEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn Files(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl windows_core::RuntimeName for IFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl IFileOpenPickerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: IFileOpenPickerContinuationEventArgs_Impl, const OFFSET: isize>() -> IFileOpenPickerContinuationEventArgs_Vtbl {
        unsafe extern "system" fn Files<Identity: IFileOpenPickerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileOpenPickerContinuationEventArgs_Impl::Files(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileOpenPickerContinuationEventArgs, OFFSET>(), Files: Files::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileOpenPickerContinuationEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Pickers_Provider")]
pub trait IFileSavePickerActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn FileSavePickerUI(&self) -> windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI>;
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl windows_core::RuntimeName for IFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs";
}
#[cfg(feature = "Storage_Pickers_Provider")]
impl IFileSavePickerActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IFileSavePickerActivatedEventArgs_Impl, const OFFSET: isize>() -> IFileSavePickerActivatedEventArgs_Vtbl {
        unsafe extern "system" fn FileSavePickerUI<Identity: IFileSavePickerActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSavePickerActivatedEventArgs_Impl::FileSavePickerUI(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileSavePickerActivatedEventArgs, OFFSET>(),
            FileSavePickerUI: FileSavePickerUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSavePickerActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IFileSavePickerActivatedEventArgs2_Impl: Sized + windows_core::IUnknownImpl {
    fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn EnterpriseId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IFileSavePickerActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2";
}
impl IFileSavePickerActivatedEventArgs2_Vtbl {
    pub const fn new<Identity: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: isize>() -> IFileSavePickerActivatedEventArgs2_Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSavePickerActivatedEventArgs2_Impl::CallerPackageFamilyName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseId<Identity: IFileSavePickerActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSavePickerActivatedEventArgs2_Impl::EnterpriseId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileSavePickerActivatedEventArgs2, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, OFFSET>,
            EnterpriseId: EnterpriseId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSavePickerActivatedEventArgs2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFileSavePickerContinuationEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn File(&self) -> windows_core::Result<super::super::Storage::StorageFile>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl windows_core::RuntimeName for IFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl IFileSavePickerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: IFileSavePickerContinuationEventArgs_Impl, const OFFSET: isize>() -> IFileSavePickerContinuationEventArgs_Vtbl {
        unsafe extern "system" fn File<Identity: IFileSavePickerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSavePickerContinuationEventArgs_Impl::File(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileSavePickerContinuationEventArgs, OFFSET>(), File: File::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSavePickerContinuationEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait IFolderPickerContinuationEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn Folder(&self) -> windows_core::Result<super::super::Storage::StorageFolder>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl windows_core::RuntimeName for IFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl IFolderPickerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: IFolderPickerContinuationEventArgs_Impl, const OFFSET: isize>() -> IFolderPickerContinuationEventArgs_Vtbl {
        unsafe extern "system" fn Folder<Identity: IFolderPickerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFolderPickerContinuationEventArgs_Impl::Folder(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFolderPickerContinuationEventArgs, OFFSET>(), Folder: Folder::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderPickerContinuationEventArgs as windows_core::Interface>::IID
    }
}
pub trait ILaunchActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn TileId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for ILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs";
}
impl ILaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> ILaunchActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Arguments<Identity: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILaunchActivatedEventArgs_Impl::Arguments(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileId<Identity: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILaunchActivatedEventArgs_Impl::TileId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILaunchActivatedEventArgs, OFFSET>(),
            Arguments: Arguments::<Identity, OFFSET>,
            TileId: TileId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILaunchActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait ILaunchActivatedEventArgs2_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn TileActivatedInfo(&self) -> windows_core::Result<TileActivatedInfo>;
}
impl windows_core::RuntimeName for ILaunchActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2";
}
impl ILaunchActivatedEventArgs2_Vtbl {
    pub const fn new<Identity: ILaunchActivatedEventArgs2_Impl, const OFFSET: isize>() -> ILaunchActivatedEventArgs2_Vtbl {
        unsafe extern "system" fn TileActivatedInfo<Identity: ILaunchActivatedEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILaunchActivatedEventArgs2_Impl::TileActivatedInfo(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILaunchActivatedEventArgs2, OFFSET>(),
            TileActivatedInfo: TileActivatedInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILaunchActivatedEventArgs2 as windows_core::Interface>::IID
    }
}
pub trait ILockScreenActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Info(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl windows_core::RuntimeName for ILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs";
}
impl ILockScreenActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ILockScreenActivatedEventArgs_Impl, const OFFSET: isize>() -> ILockScreenActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Info<Identity: ILockScreenActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILockScreenActivatedEventArgs_Impl::Info(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ILockScreenActivatedEventArgs, OFFSET>(), Info: Info::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILockScreenActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Calls")]
pub trait ILockScreenCallActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + ILaunchActivatedEventArgs_Impl {
    fn CallUI(&self) -> windows_core::Result<super::Calls::LockScreenCallUI>;
}
#[cfg(feature = "ApplicationModel_Calls")]
impl windows_core::RuntimeName for ILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Calls")]
impl ILockScreenCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ILockScreenCallActivatedEventArgs_Impl, const OFFSET: isize>() -> ILockScreenCallActivatedEventArgs_Vtbl {
        unsafe extern "system" fn CallUI<Identity: ILockScreenCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILockScreenCallActivatedEventArgs_Impl::CallUI(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ILockScreenCallActivatedEventArgs, OFFSET>(), CallUI: CallUI::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILockScreenCallActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IPhoneCallActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn LineId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs";
}
impl IPhoneCallActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPhoneCallActivatedEventArgs_Impl, const OFFSET: isize>() -> IPhoneCallActivatedEventArgs_Vtbl {
        unsafe extern "system" fn LineId<Identity: IPhoneCallActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPhoneCallActivatedEventArgs_Impl::LineId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPhoneCallActivatedEventArgs, OFFSET>(), LineId: LineId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhoneCallActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IPickerReturnedActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn PickerOperationId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IPickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPickerReturnedActivatedEventArgs";
}
impl IPickerReturnedActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPickerReturnedActivatedEventArgs_Impl, const OFFSET: isize>() -> IPickerReturnedActivatedEventArgs_Vtbl {
        unsafe extern "system" fn PickerOperationId<Identity: IPickerReturnedActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPickerReturnedActivatedEventArgs_Impl::PickerOperationId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPickerReturnedActivatedEventArgs, OFFSET>(),
            PickerOperationId: PickerOperationId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPickerReturnedActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IPrelaunchActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn PrelaunchActivated(&self) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IPrelaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs";
}
impl IPrelaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPrelaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> IPrelaunchActivatedEventArgs_Vtbl {
        unsafe extern "system" fn PrelaunchActivated<Identity: IPrelaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrelaunchActivatedEventArgs_Impl::PrelaunchActivated(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrelaunchActivatedEventArgs, OFFSET>(),
            PrelaunchActivated: PrelaunchActivated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrelaunchActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrint3DWorkflowActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Workflow(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl windows_core::RuntimeName for IPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs";
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl IPrint3DWorkflowActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPrint3DWorkflowActivatedEventArgs_Impl, const OFFSET: isize>() -> IPrint3DWorkflowActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Workflow<Identity: IPrint3DWorkflowActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrint3DWorkflowActivatedEventArgs_Impl::Workflow(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrint3DWorkflowActivatedEventArgs, OFFSET>(), Workflow: Workflow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrint3DWorkflowActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Devices_Printers_Extensions")]
pub trait IPrintTaskSettingsActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Configuration(&self) -> windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>;
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl windows_core::RuntimeName for IPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs";
}
#[cfg(feature = "Devices_Printers_Extensions")]
impl IPrintTaskSettingsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IPrintTaskSettingsActivatedEventArgs_Impl, const OFFSET: isize>() -> IPrintTaskSettingsActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Configuration<Identity: IPrintTaskSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintTaskSettingsActivatedEventArgs_Impl::Configuration(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintTaskSettingsActivatedEventArgs, OFFSET>(),
            Configuration: Configuration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTaskSettingsActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IProtocolActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri>;
}
impl windows_core::RuntimeName for IProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs";
}
impl IProtocolActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IProtocolActivatedEventArgs_Impl, const OFFSET: isize>() -> IProtocolActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Uri<Identity: IProtocolActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProtocolActivatedEventArgs_Impl::Uri(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtocolActivatedEventArgs, OFFSET>(), Uri: Uri::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtocolActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Data(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData";
}
#[cfg(feature = "Foundation_Collections")]
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
    pub const fn new<Identity: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: isize>() -> IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
        unsafe extern "system" fn CallerPackageFamilyName<Identity: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl::CallerPackageFamilyName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Impl::Data(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, OFFSET>(),
            CallerPackageFamilyName: CallerPackageFamilyName::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait IProtocolForResultsActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn ProtocolForResultsOperation(&self) -> windows_core::Result<super::super::System::ProtocolForResultsOperation>;
}
#[cfg(feature = "System")]
impl windows_core::RuntimeName for IProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs";
}
#[cfg(feature = "System")]
impl IProtocolForResultsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IProtocolForResultsActivatedEventArgs_Impl, const OFFSET: isize>() -> IProtocolForResultsActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ProtocolForResultsOperation<Identity: IProtocolForResultsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProtocolForResultsActivatedEventArgs_Impl::ProtocolForResultsOperation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtocolForResultsActivatedEventArgs, OFFSET>(),
            ProtocolForResultsOperation: ProtocolForResultsOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtocolForResultsActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IRestrictedLaunchActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn SharedContext(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl windows_core::RuntimeName for IRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs";
}
impl IRestrictedLaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IRestrictedLaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> IRestrictedLaunchActivatedEventArgs_Vtbl {
        unsafe extern "system" fn SharedContext<Identity: IRestrictedLaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRestrictedLaunchActivatedEventArgs_Impl::SharedContext(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRestrictedLaunchActivatedEventArgs, OFFSET>(),
            SharedContext: SharedContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRestrictedLaunchActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait ISearchActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Language(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for ISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgs";
}
impl ISearchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ISearchActivatedEventArgs_Impl, const OFFSET: isize>() -> ISearchActivatedEventArgs_Vtbl {
        unsafe extern "system" fn QueryText<Identity: ISearchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchActivatedEventArgs_Impl::QueryText(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: ISearchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchActivatedEventArgs_Impl::Language(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISearchActivatedEventArgs, OFFSET>(),
            QueryText: QueryText::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Search")]
pub trait ISearchActivatedEventArgsWithLinguisticDetails_Impl: Sized + windows_core::IUnknownImpl {
    fn LinguisticDetails(&self) -> windows_core::Result<super::Search::SearchPaneQueryLinguisticDetails>;
}
#[cfg(feature = "ApplicationModel_Search")]
impl windows_core::RuntimeName for ISearchActivatedEventArgsWithLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails";
}
#[cfg(feature = "ApplicationModel_Search")]
impl ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
    pub const fn new<Identity: ISearchActivatedEventArgsWithLinguisticDetails_Impl, const OFFSET: isize>() -> ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
        unsafe extern "system" fn LinguisticDetails<Identity: ISearchActivatedEventArgsWithLinguisticDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchActivatedEventArgsWithLinguisticDetails_Impl::LinguisticDetails(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISearchActivatedEventArgsWithLinguisticDetails, OFFSET>(),
            LinguisticDetails: LinguisticDetails::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchActivatedEventArgsWithLinguisticDetails as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub trait IShareTargetActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn ShareOperation(&self) -> windows_core::Result<super::DataTransfer::ShareTarget::ShareOperation>;
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl windows_core::RuntimeName for IShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
impl IShareTargetActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IShareTargetActivatedEventArgs_Impl, const OFFSET: isize>() -> IShareTargetActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ShareOperation<Identity: IShareTargetActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IShareTargetActivatedEventArgs_Impl::ShareOperation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IShareTargetActivatedEventArgs, OFFSET>(),
            ShareOperation: ShareOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShareTargetActivatedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IStartupTaskActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn TaskId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs";
}
impl IStartupTaskActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IStartupTaskActivatedEventArgs_Impl, const OFFSET: isize>() -> IStartupTaskActivatedEventArgs_Vtbl {
        unsafe extern "system" fn TaskId<Identity: IStartupTaskActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStartupTaskActivatedEventArgs_Impl::TaskId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IStartupTaskActivatedEventArgs, OFFSET>(), TaskId: TaskId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStartupTaskActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IToastNotificationActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Argument(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn UserInput(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs";
}
#[cfg(feature = "Foundation_Collections")]
impl IToastNotificationActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IToastNotificationActivatedEventArgs_Impl, const OFFSET: isize>() -> IToastNotificationActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Argument<Identity: IToastNotificationActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IToastNotificationActivatedEventArgs_Impl::Argument(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserInput<Identity: IToastNotificationActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IToastNotificationActivatedEventArgs_Impl::UserInput(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IToastNotificationActivatedEventArgs, OFFSET>(),
            Argument: Argument::<Identity, OFFSET>,
            UserInput: UserInput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToastNotificationActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub trait IUserDataAccountProviderActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Operation(&self) -> windows_core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>;
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl windows_core::RuntimeName for IUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
impl IUserDataAccountProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IUserDataAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> IUserDataAccountProviderActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Operation<Identity: IUserDataAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUserDataAccountProviderActivatedEventArgs_Impl::Operation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserDataAccountProviderActivatedEventArgs, OFFSET>(),
            Operation: Operation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserDataAccountProviderActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "UI_ViewManagement")]
pub trait IViewSwitcherProvider_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn ViewSwitcher(&self) -> windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher>;
}
#[cfg(feature = "UI_ViewManagement")]
impl windows_core::RuntimeName for IViewSwitcherProvider {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IViewSwitcherProvider";
}
#[cfg(feature = "UI_ViewManagement")]
impl IViewSwitcherProvider_Vtbl {
    pub const fn new<Identity: IViewSwitcherProvider_Impl, const OFFSET: isize>() -> IViewSwitcherProvider_Vtbl {
        unsafe extern "system" fn ViewSwitcher<Identity: IViewSwitcherProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewSwitcherProvider_Impl::ViewSwitcher(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IViewSwitcherProvider, OFFSET>(), ViewSwitcher: ViewSwitcher::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewSwitcherProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Media_SpeechRecognition")]
pub trait IVoiceCommandActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Result(&self) -> windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
#[cfg(feature = "Media_SpeechRecognition")]
impl windows_core::RuntimeName for IVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs";
}
#[cfg(feature = "Media_SpeechRecognition")]
impl IVoiceCommandActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IVoiceCommandActivatedEventArgs_Impl, const OFFSET: isize>() -> IVoiceCommandActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Result<Identity: IVoiceCommandActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVoiceCommandActivatedEventArgs_Impl::Result(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IVoiceCommandActivatedEventArgs, OFFSET>(), Result: Result::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVoiceCommandActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
pub trait IWalletActionActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn ItemId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ActionKind(&self) -> windows_core::Result<super::Wallet::WalletActionKind>;
    fn ActionId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
#[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
impl windows_core::RuntimeName for IWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
impl IWalletActionActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>() -> IWalletActionActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ItemId<Identity: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWalletActionActivatedEventArgs_Impl::ItemId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionKind<Identity: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWalletActionActivatedEventArgs_Impl::ActionKind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionId<Identity: IWalletActionActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWalletActionActivatedEventArgs_Impl::ActionId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWalletActionActivatedEventArgs, OFFSET>(),
            ItemId: ItemId::<Identity, OFFSET>,
            ActionKind: ActionKind::<Identity, OFFSET>,
            ActionId: ActionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWalletActionActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub trait IWebAccountProviderActivatedEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl {
    fn Operation(&self) -> windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>;
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl windows_core::RuntimeName for IWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs";
}
#[cfg(feature = "Security_Authentication_Web_Provider")]
impl IWebAccountProviderActivatedEventArgs_Vtbl {
    pub const fn new<Identity: IWebAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>() -> IWebAccountProviderActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Operation<Identity: IWebAccountProviderActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebAccountProviderActivatedEventArgs_Impl::Operation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccountProviderActivatedEventArgs, OFFSET>(),
            Operation: Operation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccountProviderActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
pub trait IWebAuthenticationBrokerContinuationEventArgs_Impl: Sized + windows_core::IUnknownImpl + IActivatedEventArgs_Impl + IContinuationActivatedEventArgs_Impl {
    fn WebAuthenticationResult(&self) -> windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl windows_core::RuntimeName for IWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Authentication_Web"))]
impl IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
    pub const fn new<Identity: IWebAuthenticationBrokerContinuationEventArgs_Impl, const OFFSET: isize>() -> IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
        unsafe extern "system" fn WebAuthenticationResult<Identity: IWebAuthenticationBrokerContinuationEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebAuthenticationBrokerContinuationEventArgs_Impl::WebAuthenticationResult(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAuthenticationBrokerContinuationEventArgs, OFFSET>(),
            WebAuthenticationResult: WebAuthenticationResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAuthenticationBrokerContinuationEventArgs as windows_core::Interface>::IID
    }
}
