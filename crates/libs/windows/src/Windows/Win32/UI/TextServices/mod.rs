#[inline]
pub unsafe fn DoMsCtfMonitor(dwflags: u32, heventforservicestop: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    windows_targets::link!("msctfmonitor.dll" "system" fn DoMsCtfMonitor(dwflags : u32, heventforservicestop : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DoMsCtfMonitor(dwflags, heventforservicestop)
}
#[inline]
pub unsafe fn InitLocalMsCtfMonitor(dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("msctfmonitor.dll" "system" fn InitLocalMsCtfMonitor(dwflags : u32) -> windows_core::HRESULT);
    InitLocalMsCtfMonitor(dwflags).ok()
}
#[inline]
pub unsafe fn UninitLocalMsCtfMonitor() -> windows_core::Result<()> {
    windows_targets::link!("msctfmonitor.dll" "system" fn UninitLocalMsCtfMonitor() -> windows_core::HRESULT);
    UninitLocalMsCtfMonitor().ok()
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ANCHOR_CHANGE_HISTORY_FLAGS(pub u32);
impl ANCHOR_CHANGE_HISTORY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const AccClientDocMgr: windows_core::GUID = windows_core::GUID::from_u128(0xfc48cc30_4f3e_4fa1_803b_ad0e196a83b1);
pub const AccDictionary: windows_core::GUID = windows_core::GUID::from_u128(0x6572ee16_5fe5_4331_bb6d_76a49c56e423);
pub const AccServerDocMgr: windows_core::GUID = windows_core::GUID::from_u128(0x6089a37e_eb8a_482d_bd6f_f9f46904d16d);
pub const AccStore: windows_core::GUID = windows_core::GUID::from_u128(0x5440837f_4bff_4ae5_a1b1_7722ecc6332a);
pub const CAND_CANCELED: TfCandidateResult = TfCandidateResult(2i32);
pub const CAND_FINALIZED: TfCandidateResult = TfCandidateResult(0i32);
pub const CAND_SELECTED: TfCandidateResult = TfCandidateResult(1i32);
pub const CLSID_TF_CategoryMgr: windows_core::GUID = windows_core::GUID::from_u128(0xa4b544a1_438d_4b41_9325_869523e2d6c7);
pub const CLSID_TF_ClassicLangBar: windows_core::GUID = windows_core::GUID::from_u128(0x3318360c_1afc_4d09_a86b_9f9cb6dceb9c);
pub const CLSID_TF_DisplayAttributeMgr: windows_core::GUID = windows_core::GUID::from_u128(0x3ce74de4_53d3_4d74_8b83_431b3828ba53);
pub const CLSID_TF_InputProcessorProfiles: windows_core::GUID = windows_core::GUID::from_u128(0x33c53a50_f456_4884_b049_85fd643ecfed);
pub const CLSID_TF_LangBarItemMgr: windows_core::GUID = windows_core::GUID::from_u128(0xb9931692_a2b3_4fab_bf33_9ec6f9fb96ac);
pub const CLSID_TF_LangBarMgr: windows_core::GUID = windows_core::GUID::from_u128(0xebb08c45_6c4a_4fdc_ae53_4eb8c4c7db8e);
pub const CLSID_TF_ThreadMgr: windows_core::GUID = windows_core::GUID::from_u128(0x529a9e6b_6587_4f23_ab9e_9c7d683e3c50);
pub const CLSID_TF_TransitoryExtensionUIEntry: windows_core::GUID = windows_core::GUID::from_u128(0xae6be008_07fb_400d_8beb_337a64f7051f);
pub const CLSID_TsfServices: windows_core::GUID = windows_core::GUID::from_u128(0x39aedc00_6b60_46db_8d31_3642be0e4373);
pub const DCM_FLAGS_CTFMON: u32 = 2u32;
pub const DCM_FLAGS_LOCALTHREADTSF: u32 = 4u32;
pub const DCM_FLAGS_TASKENG: u32 = 1u32;
pub const DocWrap: windows_core::GUID = windows_core::GUID::from_u128(0xbf426f7e_7a5e_44d6_830c_a390ea9462a3);
pub const GETIF_DICTGRAM: TfSapiObject = TfSapiObject(4i32);
pub const GETIF_RECOCONTEXT: TfSapiObject = TfSapiObject(1i32);
pub const GETIF_RECOGNIZER: TfSapiObject = TfSapiObject(2i32);
pub const GETIF_RECOGNIZERNOINIT: TfSapiObject = TfSapiObject(5i32);
pub const GETIF_RESMGR: TfSapiObject = TfSapiObject(0i32);
pub const GETIF_VOICE: TfSapiObject = TfSapiObject(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_TEXT_AND_PROPERTY_UPDATES_FLAGS(pub u32);
impl GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GUID_APP_FUNCTIONPROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0x4caef01e_12af_4b0e_9db1_a6ec5b881208);
pub const GUID_COMPARTMENT_CONVERSIONMODEBIAS: windows_core::GUID = windows_core::GUID::from_u128(0x5497f516_ee91_436e_b946_aa2c05f1ac5b);
pub const GUID_COMPARTMENT_EMPTYCONTEXT: windows_core::GUID = windows_core::GUID::from_u128(0xd7487dbf_804e_41c5_894d_ad96fd4eea13);
pub const GUID_COMPARTMENT_ENABLED_PROFILES_UPDATED: windows_core::GUID = windows_core::GUID::from_u128(0x92c1fd48_a9ae_4a7c_be08_4329e4723817);
pub const GUID_COMPARTMENT_HANDWRITING_OPENCLOSE: windows_core::GUID = windows_core::GUID::from_u128(0xf9ae2c6b_1866_4361_af72_7aa30948890e);
pub const GUID_COMPARTMENT_KEYBOARD_DISABLED: windows_core::GUID = windows_core::GUID::from_u128(0x71a5b253_1951_466b_9fbc_9c8808fa84f2);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE: windows_core::GUID = windows_core::GUID::from_u128(0xb6592511_bcee_4122_a7c4_09f4b3fa4396);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_CONVERSION: windows_core::GUID = windows_core::GUID::from_u128(0xccf05dd8_4a87_11d7_a6e2_00065b84435c);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_SENTENCE: windows_core::GUID = windows_core::GUID::from_u128(0xccf05dd9_4a87_11d7_a6e2_00065b84435c);
pub const GUID_COMPARTMENT_KEYBOARD_OPENCLOSE: windows_core::GUID = windows_core::GUID::from_u128(0x58273aad_01bb_4164_95c6_755ba0b5162d);
pub const GUID_COMPARTMENT_SAPI_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x51af2086_cc6b_457d_b5aa_8b19dc290ab4);
pub const GUID_COMPARTMENT_SPEECH_CFGMENU: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c5c2d_4e83_4bb6_91a2_e019bff6762d);
pub const GUID_COMPARTMENT_SPEECH_DISABLED: windows_core::GUID = windows_core::GUID::from_u128(0x56c5c607_0703_4e59_8e52_cbc84e8bbe35);
pub const GUID_COMPARTMENT_SPEECH_GLOBALSTATE: windows_core::GUID = windows_core::GUID::from_u128(0x2a54fe8e_0d08_460c_a75d_87035ff436c5);
pub const GUID_COMPARTMENT_SPEECH_OPENCLOSE: windows_core::GUID = windows_core::GUID::from_u128(0x544d6a63_e2e8_4752_bbd1_000960bca083);
pub const GUID_COMPARTMENT_SPEECH_UI_STATUS: windows_core::GUID = windows_core::GUID::from_u128(0xd92016f0_9367_4fe7_9abf_bc59dacbe0e3);
pub const GUID_COMPARTMENT_TIPUISTATUS: windows_core::GUID = windows_core::GUID::from_u128(0x148ca3ec_0366_401c_8d75_ed978d85fbc9);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION: windows_core::GUID = windows_core::GUID::from_u128(0x8be347f5_c7a0_11d7_b408_00065b84435c);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_DOCUMENTMANAGER: windows_core::GUID = windows_core::GUID::from_u128(0x8be347f7_c7a0_11d7_b408_00065b84435c);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_PARENT: windows_core::GUID = windows_core::GUID::from_u128(0x8be347f8_c7a0_11d7_b408_00065b84435c);
pub const GUID_INTEGRATIONSTYLE_SEARCHBOX: windows_core::GUID = windows_core::GUID::from_u128(0xe6d1bd11_82f7_4903_ae21_1a6397cde2eb);
pub const GUID_LBI_INPUTMODE: windows_core::GUID = windows_core::GUID::from_u128(0x2c77a81e_41cc_4178_a3a7_5f8a987568e6);
pub const GUID_LBI_SAPILAYR_CFGMENUBUTTON: windows_core::GUID = windows_core::GUID::from_u128(0xd02f24a1_942d_422e_8d99_b4f2addee999);
pub const GUID_MODEBIAS_CHINESE: windows_core::GUID = windows_core::GUID::from_u128(0x7add26de_4328_489b_83ae_6493750cad5c);
pub const GUID_MODEBIAS_CONVERSATION: windows_core::GUID = windows_core::GUID::from_u128(0x0f4ec104_1790_443b_95f1_e10f939d6546);
pub const GUID_MODEBIAS_DATETIME: windows_core::GUID = windows_core::GUID::from_u128(0xf2bdb372_7f61_4039_92ef_1c35599f0222);
pub const GUID_MODEBIAS_FILENAME: windows_core::GUID = windows_core::GUID::from_u128(0xd7f707fe_44c6_4fca_8e76_86ab50c7931b);
pub const GUID_MODEBIAS_FULLWIDTHALPHANUMERIC: windows_core::GUID = windows_core::GUID::from_u128(0x81489fb8_b36a_473d_8146_e4a2258b24ae);
pub const GUID_MODEBIAS_FULLWIDTHHANGUL: windows_core::GUID = windows_core::GUID::from_u128(0xc01ae6c9_45b5_4fd0_9cb1_9f4cebc39fea);
pub const GUID_MODEBIAS_HALFWIDTHKATAKANA: windows_core::GUID = windows_core::GUID::from_u128(0x005f6b63_78d4_41cc_8859_485ca821a795);
pub const GUID_MODEBIAS_HANGUL: windows_core::GUID = windows_core::GUID::from_u128(0x76ef0541_23b3_4d77_a074_691801ccea17);
pub const GUID_MODEBIAS_HIRAGANA: windows_core::GUID = windows_core::GUID::from_u128(0xd73d316e_9b91_46f1_a280_31597f52c694);
pub const GUID_MODEBIAS_KATAKANA: windows_core::GUID = windows_core::GUID::from_u128(0x2e0eeddd_3a1a_499e_8543_3c7ee7949811);
pub const GUID_MODEBIAS_NAME: windows_core::GUID = windows_core::GUID::from_u128(0xfddc10f0_d239_49bf_b8fc_5410caaa427e);
pub const GUID_MODEBIAS_NONE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const GUID_MODEBIAS_NUMERIC: windows_core::GUID = windows_core::GUID::from_u128(0x4021766c_e872_48fd_9cee_4ec5c75e16c3);
pub const GUID_MODEBIAS_READING: windows_core::GUID = windows_core::GUID::from_u128(0xe31643a3_6466_4cbf_8d8b_0bd4d8545461);
pub const GUID_MODEBIAS_URLHISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x8b0e54d9_63f2_4c68_84d4_79aee7a59f09);
pub const GUID_PROP_ATTRIBUTE: windows_core::GUID = windows_core::GUID::from_u128(0x34b45670_7526_11d2_a147_00105a2799b5);
pub const GUID_PROP_COMPOSING: windows_core::GUID = windows_core::GUID::from_u128(0xe12ac060_af15_11d2_afc5_00105a2799b5);
pub const GUID_PROP_INPUTSCOPE: windows_core::GUID = windows_core::GUID::from_u128(0x1713dd5a_68e7_4a5b_9af6_592a595c778d);
pub const GUID_PROP_LANGID: windows_core::GUID = windows_core::GUID::from_u128(0x3280ce20_8032_11d2_b603_00105a2799b5);
pub const GUID_PROP_MODEBIAS: windows_core::GUID = windows_core::GUID::from_u128(0x372e0716_974f_40ac_a088_08cdc92ebfbc);
pub const GUID_PROP_READING: windows_core::GUID = windows_core::GUID::from_u128(0x5463f7c0_8e31_11d2_bf46_00105a2799b5);
pub const GUID_PROP_TEXTOWNER: windows_core::GUID = windows_core::GUID::from_u128(0xf1e2d520_0969_11d3_8df0_00105a2799b5);
pub const GUID_PROP_TKB_ALTERNATES: windows_core::GUID = windows_core::GUID::from_u128(0x70b2a803_968d_462e_b93b_2164c91517f7);
pub const GUID_SYSTEM_FUNCTIONPROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0x9a698bb0_0f21_11d3_8df1_00105a2799b5);
pub const GUID_TFCAT_CATEGORY_OF_TIP: windows_core::GUID = windows_core::GUID::from_u128(0x534c48c1_0607_4098_a521_4fc899c73e90);
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY: windows_core::GUID = windows_core::GUID::from_u128(0xb95f181b_ea4c_4af1_8056_7c321abbb091);
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0x046b8c80_1647_40f7_9b21_b93b81aabc1b);
pub const GUID_TFCAT_PROPSTYLE_STATIC: windows_core::GUID = windows_core::GUID::from_u128(0x565fb8d8_6bd4_4ca1_b223_0f2ccb8f4f96);
pub const GUID_TFCAT_PROP_AUDIODATA: windows_core::GUID = windows_core::GUID::from_u128(0x9b7be3a9_e8ab_4d47_a8fe_254fa423436d);
pub const GUID_TFCAT_PROP_INKDATA: windows_core::GUID = windows_core::GUID::from_u128(0x7c6a82ae_b0d7_4f14_a745_14f28b009d61);
pub const GUID_TFCAT_TIPCAP_COMLESS: windows_core::GUID = windows_core::GUID::from_u128(0x364215d9_75bc_11d7_a6ef_00065b84435c);
pub const GUID_TFCAT_TIPCAP_DUALMODE: windows_core::GUID = windows_core::GUID::from_u128(0x3af314a2_d79f_4b1b_9992_15086d339b05);
pub const GUID_TFCAT_TIPCAP_IMMERSIVEONLY: windows_core::GUID = windows_core::GUID::from_u128(0x3a4259ac_640d_4ad4_89f7_1eb67e7c4ee8);
pub const GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT: windows_core::GUID = windows_core::GUID::from_u128(0x13a016df_560b_46cd_947a_4c3af1e0e35d);
pub const GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT: windows_core::GUID = windows_core::GUID::from_u128(0xccf05dd7_4a87_11d7_a6e2_00065b84435c);
pub const GUID_TFCAT_TIPCAP_LOCALSERVER: windows_core::GUID = windows_core::GUID::from_u128(0x74769ee9_4a66_4f9d_90d6_bf8b7c3eb461);
pub const GUID_TFCAT_TIPCAP_SECUREMODE: windows_core::GUID = windows_core::GUID::from_u128(0x49d2f9ce_1f5e_11d7_a6d3_00065b84435c);
pub const GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT: windows_core::GUID = windows_core::GUID::from_u128(0x25504fb4_7bab_4bc1_9c69_cf81890f0ef5);
pub const GUID_TFCAT_TIPCAP_TSF3: windows_core::GUID = windows_core::GUID::from_u128(0x07dcb4af_98de_4548_bef7_25bd45979a1f);
pub const GUID_TFCAT_TIPCAP_UIELEMENTENABLED: windows_core::GUID = windows_core::GUID::from_u128(0x49d2f9cf_1f5e_11d7_a6d3_00065b84435c);
pub const GUID_TFCAT_TIPCAP_WOW16: windows_core::GUID = windows_core::GUID::from_u128(0x364215da_75bc_11d7_a6ef_00065b84435c);
pub const GUID_TFCAT_TIP_HANDWRITING: windows_core::GUID = windows_core::GUID::from_u128(0x246ecb87_c2f2_4abe_905b_c8b38add2c43);
pub const GUID_TFCAT_TIP_KEYBOARD: windows_core::GUID = windows_core::GUID::from_u128(0x34745c63_b2f0_4784_8b67_5e12c8701a31);
pub const GUID_TFCAT_TIP_SPEECH: windows_core::GUID = windows_core::GUID::from_u128(0xb5a73cd1_8355_426b_a161_259808f26b14);
pub const GUID_TFCAT_TRANSITORYEXTENSIONUI: windows_core::GUID = windows_core::GUID::from_u128(0x6302de22_a5cf_4b02_bfe8_4d72b2bed3c6);
pub const GUID_TS_SERVICE_ACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0xf9786200_a5bf_4a0f_8c24_fb16f5d1aabb);
pub const GUID_TS_SERVICE_ACTIVEX: windows_core::GUID = windows_core::GUID::from_u128(0xea937a50_c9a6_4b7d_894a_49d99b784834);
pub const GUID_TS_SERVICE_DATAOBJECT: windows_core::GUID = windows_core::GUID::from_u128(0x6086fbb5_e225_46ce_a770_c1bbd3e05d7b);
pub const GXFPF_NEAREST: u32 = 2u32;
pub const GXFPF_ROUND_NEAREST: u32 = 1u32;
windows_core::imp::define_interface!(IAccClientDocMgr, IAccClientDocMgr_Vtbl, 0x4c896039_7b6d_49e6_a8c1_45116a98292b);
windows_core::imp::interface_hierarchy!(IAccClientDocMgr, windows_core::IUnknown);
impl IAccClientDocMgr {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocuments(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocuments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LookupByHWND(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupByHWND)(windows_core::Interface::as_raw(self), hwnd, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LookupByPoint(&self, pt: super::super::Foundation::POINT, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupByPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(pt), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFocused(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFocused)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAccClientDocMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocuments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocuments: usize,
    pub LookupByHWND: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LookupByPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::POINT, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocused: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccClientDocMgr_Impl: windows_core::IUnknownImpl {
    fn GetDocuments(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn LookupByPoint(&self, pt: &super::super::Foundation::POINT, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetFocused(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccClientDocMgr_Vtbl {
    pub const fn new<Identity: IAccClientDocMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocuments<Identity: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccClientDocMgr_Impl::GetDocuments(this) {
                Ok(ok__) => {
                    enumunknown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Identity: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccClientDocMgr_Impl::LookupByHWND(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Identity: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccClientDocMgr_Impl::LookupByPoint(this, core::mem::transmute(&pt), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocused<Identity: IAccClientDocMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccClientDocMgr_Impl::GetFocused(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocuments: GetDocuments::<Identity, OFFSET>,
            LookupByHWND: LookupByHWND::<Identity, OFFSET>,
            LookupByPoint: LookupByPoint::<Identity, OFFSET>,
            GetFocused: GetFocused::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccClientDocMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAccClientDocMgr {}
windows_core::imp::define_interface!(IAccDictionary, IAccDictionary_Vtbl, 0x1dc4cb5f_d737_474d_ade9_5ccfc9bc1cc9);
windows_core::imp::interface_hierarchy!(IAccDictionary, windows_core::IUnknown);
impl IAccDictionary {
    pub unsafe fn GetLocalizedString(&self, term: *const windows_core::GUID, lcid: u32, presult: *mut windows_core::BSTR, plcid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocalizedString)(windows_core::Interface::as_raw(self), term, lcid, core::mem::transmute(presult), core::mem::transmute(plcid)).ok()
    }
    pub unsafe fn GetParentTerm(&self, term: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParentTerm)(windows_core::Interface::as_raw(self), term, &mut result__).map(|| result__)
    }
    pub unsafe fn GetMnemonicString(&self, term: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMnemonicString)(windows_core::Interface::as_raw(self), term, &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn LookupMnemonicTerm(&self, bstrmnemonic: &windows_core::BSTR) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupMnemonicTerm)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmnemonic), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ConvertValueToString(&self, term: *const windows_core::GUID, lcid: u32, varvalue: &super::super::System::Variant::VARIANT, pbstrresult: *mut windows_core::BSTR, plcid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertValueToString)(windows_core::Interface::as_raw(self), term, lcid, core::mem::transmute_copy(varvalue), core::mem::transmute(pbstrresult), core::mem::transmute(plcid)).ok()
    }
}
#[repr(C)]
pub struct IAccDictionary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLocalizedString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetParentTerm: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetMnemonicString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LookupMnemonicTerm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ConvertValueToString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ConvertValueToString: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAccDictionary_Impl: windows_core::IUnknownImpl {
    fn GetLocalizedString(&self, term: *const windows_core::GUID, lcid: u32, presult: *mut windows_core::BSTR, plcid: *mut u32) -> windows_core::Result<()>;
    fn GetParentTerm(&self, term: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID>;
    fn GetMnemonicString(&self, term: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn LookupMnemonicTerm(&self, bstrmnemonic: &windows_core::BSTR) -> windows_core::Result<windows_core::GUID>;
    fn ConvertValueToString(&self, term: *const windows_core::GUID, lcid: u32, varvalue: &super::super::System::Variant::VARIANT, pbstrresult: *mut windows_core::BSTR, plcid: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAccDictionary_Vtbl {
    pub const fn new<Identity: IAccDictionary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLocalizedString<Identity: IAccDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, term: *const windows_core::GUID, lcid: u32, presult: *mut *mut core::ffi::c_void, plcid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccDictionary_Impl::GetLocalizedString(this, core::mem::transmute_copy(&term), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&presult), core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn GetParentTerm<Identity: IAccDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, term: *const windows_core::GUID, pparentterm: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccDictionary_Impl::GetParentTerm(this, core::mem::transmute_copy(&term)) {
                Ok(ok__) => {
                    pparentterm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMnemonicString<Identity: IAccDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, term: *const windows_core::GUID, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccDictionary_Impl::GetMnemonicString(this, core::mem::transmute_copy(&term)) {
                Ok(ok__) => {
                    presult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupMnemonicTerm<Identity: IAccDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmnemonic: *mut core::ffi::c_void, pterm: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccDictionary_Impl::LookupMnemonicTerm(this, core::mem::transmute(&bstrmnemonic)) {
                Ok(ok__) => {
                    pterm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertValueToString<Identity: IAccDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, term: *const windows_core::GUID, lcid: u32, varvalue: super::super::System::Variant::VARIANT, pbstrresult: *mut *mut core::ffi::c_void, plcid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccDictionary_Impl::ConvertValueToString(this, core::mem::transmute_copy(&term), core::mem::transmute_copy(&lcid), core::mem::transmute(&varvalue), core::mem::transmute_copy(&pbstrresult), core::mem::transmute_copy(&plcid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLocalizedString: GetLocalizedString::<Identity, OFFSET>,
            GetParentTerm: GetParentTerm::<Identity, OFFSET>,
            GetMnemonicString: GetMnemonicString::<Identity, OFFSET>,
            LookupMnemonicTerm: LookupMnemonicTerm::<Identity, OFFSET>,
            ConvertValueToString: ConvertValueToString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccDictionary as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAccDictionary {}
windows_core::imp::define_interface!(IAccServerDocMgr, IAccServerDocMgr_Vtbl, 0xad7c73cf_6dd5_4855_abc2_b04bad5b9153);
windows_core::imp::interface_hierarchy!(IAccServerDocMgr, windows_core::IUnknown);
impl IAccServerDocMgr {
    pub unsafe fn NewDocument<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).NewDocument)(windows_core::Interface::as_raw(self), riid, punk.param().abi()).ok()
    }
    pub unsafe fn RevokeDocument<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).RevokeDocument)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn OnDocumentFocus<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnDocumentFocus)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAccServerDocMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDocumentFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAccServerDocMgr_Impl: windows_core::IUnknownImpl {
    fn NewDocument(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RevokeDocument(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnDocumentFocus(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IAccServerDocMgr_Vtbl {
    pub const fn new<Identity: IAccServerDocMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewDocument<Identity: IAccServerDocMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccServerDocMgr_Impl::NewDocument(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RevokeDocument<Identity: IAccServerDocMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccServerDocMgr_Impl::RevokeDocument(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn OnDocumentFocus<Identity: IAccServerDocMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccServerDocMgr_Impl::OnDocumentFocus(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NewDocument: NewDocument::<Identity, OFFSET>,
            RevokeDocument: RevokeDocument::<Identity, OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccServerDocMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAccServerDocMgr {}
windows_core::imp::define_interface!(IAccStore, IAccStore_Vtbl, 0xe2cd4a63_2b72_4d48_b739_95e4765195ba);
windows_core::imp::interface_hierarchy!(IAccStore, windows_core::IUnknown);
impl IAccStore {
    pub unsafe fn Register<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), riid, punk.param().abi()).ok()
    }
    pub unsafe fn Unregister<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Unregister)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocuments(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocuments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LookupByHWND(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupByHWND)(windows_core::Interface::as_raw(self), hwnd, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LookupByPoint(&self, pt: super::super::Foundation::POINT, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupByPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(pt), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OnDocumentFocus<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnDocumentFocus)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn GetFocused(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFocused)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAccStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocuments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocuments: usize,
    pub LookupByHWND: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LookupByPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::POINT, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDocumentFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocused: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccStore_Impl: windows_core::IUnknownImpl {
    fn Register(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Unregister(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDocuments(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn LookupByPoint(&self, pt: &super::super::Foundation::POINT, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn OnDocumentFocus(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetFocused(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccStore_Vtbl {
    pub const fn new<Identity: IAccStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Register<Identity: IAccStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccStore_Impl::Register(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn Unregister<Identity: IAccStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccStore_Impl::Unregister(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetDocuments<Identity: IAccStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccStore_Impl::GetDocuments(this) {
                Ok(ok__) => {
                    enumunknown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByHWND<Identity: IAccStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccStore_Impl::LookupByHWND(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupByPoint<Identity: IAccStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccStore_Impl::LookupByPoint(this, core::mem::transmute(&pt), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDocumentFocus<Identity: IAccStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccStore_Impl::OnDocumentFocus(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetFocused<Identity: IAccStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccStore_Impl::GetFocused(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            Unregister: Unregister::<Identity, OFFSET>,
            GetDocuments: GetDocuments::<Identity, OFFSET>,
            LookupByHWND: LookupByHWND::<Identity, OFFSET>,
            LookupByPoint: LookupByPoint::<Identity, OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Identity, OFFSET>,
            GetFocused: GetFocused::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAccStore {}
windows_core::imp::define_interface!(IAnchor, IAnchor_Vtbl, 0x0feb7e34_5a60_4356_8ef7_abdec2ff7cf8);
windows_core::imp::interface_hierarchy!(IAnchor, windows_core::IUnknown);
impl IAnchor {
    pub unsafe fn SetGravity(&self, gravity: TsGravity) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGravity)(windows_core::Interface::as_raw(self), gravity).ok()
    }
    pub unsafe fn GetGravity(&self) -> windows_core::Result<TsGravity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGravity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsEqual<P0>(&self, pawith: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<IAnchor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pawith.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Compare<P0>(&self, pawith: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IAnchor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), pawith.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Shift<P3>(&self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).Shift)(windows_core::Interface::as_raw(self), dwflags, cchreq, core::mem::transmute(pcch), pahaltanchor.param().abi()).ok()
    }
    pub unsafe fn ShiftTo<P0>(&self, pasite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).ShiftTo)(windows_core::Interface::as_raw(self), pasite.param().abi()).ok()
    }
    pub unsafe fn ShiftRegion(&self, dwflags: u32, dir: TsShiftDir) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShiftRegion)(windows_core::Interface::as_raw(self), dwflags, dir, &mut result__).map(|| result__)
    }
    pub unsafe fn SetChangeHistoryMask(&self, dwmask: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChangeHistoryMask)(windows_core::Interface::as_raw(self), dwmask).ok()
    }
    pub unsafe fn GetChangeHistory(&self) -> windows_core::Result<ANCHOR_CHANGE_HISTORY_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChangeHistory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ClearChangeHistory(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearChangeHistory)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IAnchor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAnchor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, TsGravity) -> windows_core::HRESULT,
    pub GetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TsGravity) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Shift: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *mut i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShiftTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShiftRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, TsShiftDir, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetChangeHistoryMask: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetChangeHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> windows_core::HRESULT,
    pub ClearChangeHistory: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAnchor_Impl: windows_core::IUnknownImpl {
    fn SetGravity(&self, gravity: TsGravity) -> windows_core::Result<()>;
    fn GetGravity(&self) -> windows_core::Result<TsGravity>;
    fn IsEqual(&self, pawith: Option<&IAnchor>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Compare(&self, pawith: Option<&IAnchor>) -> windows_core::Result<i32>;
    fn Shift(&self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: Option<&IAnchor>) -> windows_core::Result<()>;
    fn ShiftTo(&self, pasite: Option<&IAnchor>) -> windows_core::Result<()>;
    fn ShiftRegion(&self, dwflags: u32, dir: TsShiftDir) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetChangeHistoryMask(&self, dwmask: u32) -> windows_core::Result<()>;
    fn GetChangeHistory(&self) -> windows_core::Result<ANCHOR_CHANGE_HISTORY_FLAGS>;
    fn ClearChangeHistory(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IAnchor>;
}
impl IAnchor_Vtbl {
    pub const fn new<Identity: IAnchor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGravity<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gravity: TsGravity) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::SetGravity(this, core::mem::transmute_copy(&gravity)).into()
        }
        unsafe extern "system" fn GetGravity<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgravity: *mut TsGravity) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::GetGravity(this) {
                Ok(ok__) => {
                    pgravity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pawith: *mut core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::IsEqual(this, windows_core::from_raw_borrowed(&pawith)) {
                Ok(ok__) => {
                    pfequal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pawith: *mut core::ffi::c_void, plresult: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::Compare(this, windows_core::from_raw_borrowed(&pawith)) {
                Ok(ok__) => {
                    plresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shift<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::Shift(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), windows_core::from_raw_borrowed(&pahaltanchor)).into()
        }
        unsafe extern "system" fn ShiftTo<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::ShiftTo(this, windows_core::from_raw_borrowed(&pasite)).into()
        }
        unsafe extern "system" fn ShiftRegion<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::ShiftRegion(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dir)) {
                Ok(ok__) => {
                    pfnoregion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChangeHistoryMask<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::SetChangeHistoryMask(this, core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn GetChangeHistory<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::GetChangeHistory(this) {
                Ok(ok__) => {
                    pdwhistory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearChangeHistory<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchor_Impl::ClearChangeHistory(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaclone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAnchor_Impl::Clone(this) {
                Ok(ok__) => {
                    ppaclone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGravity: SetGravity::<Identity, OFFSET>,
            GetGravity: GetGravity::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Compare: Compare::<Identity, OFFSET>,
            Shift: Shift::<Identity, OFFSET>,
            ShiftTo: ShiftTo::<Identity, OFFSET>,
            ShiftRegion: ShiftRegion::<Identity, OFFSET>,
            SetChangeHistoryMask: SetChangeHistoryMask::<Identity, OFFSET>,
            GetChangeHistory: GetChangeHistory::<Identity, OFFSET>,
            ClearChangeHistory: ClearChangeHistory::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAnchor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAnchor {}
windows_core::imp::define_interface!(IClonableWrapper, IClonableWrapper_Vtbl, 0xb33e75ff_e84c_4dca_a25c_33b8dc003374);
windows_core::imp::interface_hierarchy!(IClonableWrapper, windows_core::IUnknown);
impl IClonableWrapper {
    pub unsafe fn CloneNewWrapper<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CloneNewWrapper)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IClonableWrapper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CloneNewWrapper: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IClonableWrapper_Impl: windows_core::IUnknownImpl {
    fn CloneNewWrapper(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IClonableWrapper_Vtbl {
    pub const fn new<Identity: IClonableWrapper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CloneNewWrapper<Identity: IClonableWrapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClonableWrapper_Impl::CloneNewWrapper(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CloneNewWrapper: CloneNewWrapper::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClonableWrapper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IClonableWrapper {}
windows_core::imp::define_interface!(ICoCreateLocally, ICoCreateLocally_Vtbl, 0x03de00aa_f272_41e3_99cb_03c5e8114ea0);
windows_core::imp::interface_hierarchy!(ICoCreateLocally, windows_core::IUnknown);
impl ICoCreateLocally {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CoCreateLocally<P5>(&self, rclsid: *const windows_core::GUID, dwclscontext: u32, riid: *const windows_core::GUID, punk: *mut Option<windows_core::IUnknown>, riidparam: *const windows_core::GUID, punkparam: P5, varparam: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>
    where
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CoCreateLocally)(windows_core::Interface::as_raw(self), rclsid, dwclscontext, riid, core::mem::transmute(punk), riidparam, punkparam.param().abi(), core::mem::transmute_copy(varparam)).ok()
    }
}
#[repr(C)]
pub struct ICoCreateLocally_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CoCreateLocally: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CoCreateLocally: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICoCreateLocally_Impl: windows_core::IUnknownImpl {
    fn CoCreateLocally(&self, rclsid: *const windows_core::GUID, dwclscontext: u32, riid: *const windows_core::GUID, punk: *mut Option<windows_core::IUnknown>, riidparam: *const windows_core::GUID, punkparam: Option<&windows_core::IUnknown>, varparam: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICoCreateLocally_Vtbl {
    pub const fn new<Identity: ICoCreateLocally_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CoCreateLocally<Identity: ICoCreateLocally_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwclscontext: u32, riid: *const windows_core::GUID, punk: *mut *mut core::ffi::c_void, riidparam: *const windows_core::GUID, punkparam: *mut core::ffi::c_void, varparam: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoCreateLocally_Impl::CoCreateLocally(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&riidparam), windows_core::from_raw_borrowed(&punkparam), core::mem::transmute(&varparam)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CoCreateLocally: CoCreateLocally::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoCreateLocally as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICoCreateLocally {}
windows_core::imp::define_interface!(ICoCreatedLocally, ICoCreatedLocally_Vtbl, 0x0a53eb6c_1908_4742_8cff_2cee2e93f94c);
windows_core::imp::interface_hierarchy!(ICoCreatedLocally, windows_core::IUnknown);
impl ICoCreatedLocally {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LocalInit<P0, P2>(&self, punklocalobject: P0, riidparam: *const windows_core::GUID, punkparam: P2, varparam: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).LocalInit)(windows_core::Interface::as_raw(self), punklocalobject.param().abi(), riidparam, punkparam.param().abi(), core::mem::transmute_copy(varparam)).ok()
    }
}
#[repr(C)]
pub struct ICoCreatedLocally_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LocalInit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LocalInit: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICoCreatedLocally_Impl: windows_core::IUnknownImpl {
    fn LocalInit(&self, punklocalobject: Option<&windows_core::IUnknown>, riidparam: *const windows_core::GUID, punkparam: Option<&windows_core::IUnknown>, varparam: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICoCreatedLocally_Vtbl {
    pub const fn new<Identity: ICoCreatedLocally_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LocalInit<Identity: ICoCreatedLocally_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punklocalobject: *mut core::ffi::c_void, riidparam: *const windows_core::GUID, punkparam: *mut core::ffi::c_void, varparam: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoCreatedLocally_Impl::LocalInit(this, windows_core::from_raw_borrowed(&punklocalobject), core::mem::transmute_copy(&riidparam), windows_core::from_raw_borrowed(&punkparam), core::mem::transmute(&varparam)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LocalInit: LocalInit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoCreatedLocally as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICoCreatedLocally {}
windows_core::imp::define_interface!(IDocWrap, IDocWrap_Vtbl, 0xdcd285fe_0be0_43bd_99c9_aaaec513c555);
windows_core::imp::interface_hierarchy!(IDocWrap, windows_core::IUnknown);
impl IDocWrap {
    pub unsafe fn SetDoc<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetDoc)(windows_core::Interface::as_raw(self), riid, punk.param().abi()).ok()
    }
    pub unsafe fn GetWrappedDoc(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWrappedDoc)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDocWrap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDoc: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWrappedDoc: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDocWrap_Impl: windows_core::IUnknownImpl {
    fn SetDoc(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetWrappedDoc(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl IDocWrap_Vtbl {
    pub const fn new<Identity: IDocWrap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDoc<Identity: IDocWrap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDocWrap_Impl::SetDoc(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetWrappedDoc<Identity: IDocWrap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDocWrap_Impl::GetWrappedDoc(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDoc: SetDoc::<Identity, OFFSET>,
            GetWrappedDoc: GetWrappedDoc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDocWrap as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDocWrap {}
windows_core::imp::define_interface!(IEnumITfCompositionView, IEnumITfCompositionView_Vtbl, 0x5efd22ba_7838_46cb_88e2_cadb14124f8f);
windows_core::imp::interface_hierarchy!(IEnumITfCompositionView, windows_core::IUnknown);
impl IEnumITfCompositionView {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumITfCompositionView> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, rgcompositionview: &mut [Option<ITfCompositionView>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgcompositionview.len().try_into().unwrap(), core::mem::transmute(rgcompositionview.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumITfCompositionView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumITfCompositionView_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumITfCompositionView>;
    fn Next(&self, ulcount: u32, rgcompositionview: *mut Option<ITfCompositionView>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumITfCompositionView_Vtbl {
    pub const fn new<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumITfCompositionView_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgcompositionview: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumITfCompositionView_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgcompositionview), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumITfCompositionView_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumITfCompositionView_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumITfCompositionView as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumITfCompositionView {}
windows_core::imp::define_interface!(IEnumSpeechCommands, IEnumSpeechCommands_Vtbl, 0x8c5dac4f_083c_4b85_a4c9_71746048adca);
windows_core::imp::interface_hierarchy!(IEnumSpeechCommands, windows_core::IUnknown);
impl IEnumSpeechCommands {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSpeechCommands> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, pspcmds: &mut [*mut u16], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pspcmds.len().try_into().unwrap(), core::mem::transmute(pspcmds.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumSpeechCommands_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u16, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumSpeechCommands_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumSpeechCommands>;
    fn Next(&self, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumSpeechCommands_Vtbl {
    pub const fn new<Identity: IEnumSpeechCommands_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSpeechCommands_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpeechCommands_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pspcmds), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpeechCommands_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumSpeechCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpeechCommands_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSpeechCommands as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSpeechCommands {}
windows_core::imp::define_interface!(IEnumTfCandidates, IEnumTfCandidates_Vtbl, 0xdefb1926_6c80_4ce8_87d4_d6b72b812bde);
windows_core::imp::interface_hierarchy!(IEnumTfCandidates, windows_core::IUnknown);
impl IEnumTfCandidates {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfCandidates> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ppcand: &mut [Option<ITfCandidateString>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppcand.len().try_into().unwrap(), core::mem::transmute(ppcand.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfCandidates_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfCandidates_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfCandidates>;
    fn Next(&self, ulcount: u32, ppcand: *mut Option<ITfCandidateString>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfCandidates_Vtbl {
    pub const fn new<Identity: IEnumTfCandidates_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfCandidates_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppcand: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfCandidates_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppcand), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfCandidates_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfCandidates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfCandidates_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfCandidates as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfCandidates {}
windows_core::imp::define_interface!(IEnumTfContextViews, IEnumTfContextViews_Vtbl, 0xf0c0f8dd_cf38_44e1_bb0f_68cf0d551c78);
windows_core::imp::interface_hierarchy!(IEnumTfContextViews, windows_core::IUnknown);
impl IEnumTfContextViews {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfContextViews> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, rgviews: &mut [Option<ITfContextView>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgviews.len().try_into().unwrap(), core::mem::transmute(rgviews.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfContextViews_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfContextViews_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfContextViews>;
    fn Next(&self, ulcount: u32, rgviews: *mut Option<ITfContextView>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfContextViews_Vtbl {
    pub const fn new<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfContextViews_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgviews: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContextViews_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgviews), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContextViews_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContextViews_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfContextViews as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfContextViews {}
windows_core::imp::define_interface!(IEnumTfContexts, IEnumTfContexts_Vtbl, 0x8f1a7ea6_1654_4502_a86e_b2902344d507);
windows_core::imp::interface_hierarchy!(IEnumTfContexts, windows_core::IUnknown);
impl IEnumTfContexts {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfContexts> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, rgcontext: &mut [Option<ITfContext>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgcontext.len().try_into().unwrap(), core::mem::transmute(rgcontext.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfContexts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfContexts_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfContexts>;
    fn Next(&self, ulcount: u32, rgcontext: *mut Option<ITfContext>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfContexts_Vtbl {
    pub const fn new<Identity: IEnumTfContexts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfContexts_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgcontext: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContexts_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgcontext), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContexts_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfContexts_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfContexts as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfContexts {}
windows_core::imp::define_interface!(IEnumTfDisplayAttributeInfo, IEnumTfDisplayAttributeInfo_Vtbl, 0x7cef04d7_cb75_4e80_a7ab_5f5bc7d332de);
windows_core::imp::interface_hierarchy!(IEnumTfDisplayAttributeInfo, windows_core::IUnknown);
impl IEnumTfDisplayAttributeInfo {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, rginfo: &mut [Option<ITfDisplayAttributeInfo>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rginfo.len().try_into().unwrap(), core::mem::transmute(rginfo.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfDisplayAttributeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfDisplayAttributeInfo_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn Next(&self, ulcount: u32, rginfo: *mut Option<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfDisplayAttributeInfo_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rginfo: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDisplayAttributeInfo_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rginfo), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDisplayAttributeInfo_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDisplayAttributeInfo_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfDisplayAttributeInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfDisplayAttributeInfo {}
windows_core::imp::define_interface!(IEnumTfDocumentMgrs, IEnumTfDocumentMgrs_Vtbl, 0xaa80e808_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(IEnumTfDocumentMgrs, windows_core::IUnknown);
impl IEnumTfDocumentMgrs {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfDocumentMgrs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, rgdocumentmgr: &mut [Option<ITfDocumentMgr>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgdocumentmgr.len().try_into().unwrap(), core::mem::transmute(rgdocumentmgr.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfDocumentMgrs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfDocumentMgrs_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfDocumentMgrs>;
    fn Next(&self, ulcount: u32, rgdocumentmgr: *mut Option<ITfDocumentMgr>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfDocumentMgrs_Vtbl {
    pub const fn new<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfDocumentMgrs_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDocumentMgrs_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgdocumentmgr), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDocumentMgrs_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfDocumentMgrs_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfDocumentMgrs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfDocumentMgrs {}
windows_core::imp::define_interface!(IEnumTfFunctionProviders, IEnumTfFunctionProviders_Vtbl, 0xe4b24db0_0990_11d3_8df0_00105a2799b5);
windows_core::imp::interface_hierarchy!(IEnumTfFunctionProviders, windows_core::IUnknown);
impl IEnumTfFunctionProviders {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfFunctionProviders> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ppcmdobj: &mut [Option<ITfFunctionProvider>], pcfetch: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppcmdobj.len().try_into().unwrap(), core::mem::transmute(ppcmdobj.as_ptr()), core::mem::transmute(pcfetch)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfFunctionProviders_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfFunctionProviders_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfFunctionProviders>;
    fn Next(&self, ulcount: u32, ppcmdobj: *mut Option<ITfFunctionProvider>, pcfetch: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfFunctionProviders_Vtbl {
    pub const fn new<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfFunctionProviders_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppcmdobj: *mut *mut core::ffi::c_void, pcfetch: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfFunctionProviders_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppcmdobj), core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfFunctionProviders_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfFunctionProviders_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfFunctionProviders as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfFunctionProviders {}
windows_core::imp::define_interface!(IEnumTfInputProcessorProfiles, IEnumTfInputProcessorProfiles_Vtbl, 0x71c6e74d_0f28_11d8_a82a_00065b84435c);
windows_core::imp::interface_hierarchy!(IEnumTfInputProcessorProfiles, windows_core::IUnknown);
impl IEnumTfInputProcessorProfiles {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfInputProcessorProfiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn Next(&self, pprofile: &mut [TF_INPUTPROCESSORPROFILE], pcfetch: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pprofile.len().try_into().unwrap(), core::mem::transmute(pprofile.as_ptr()), core::mem::transmute(pcfetch)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfInputProcessorProfiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TF_INPUTPROCESSORPROFILE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait IEnumTfInputProcessorProfiles_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn Next(&self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl IEnumTfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfInputProcessorProfiles_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfInputProcessorProfiles_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfInputProcessorProfiles_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfInputProcessorProfiles_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfInputProcessorProfiles as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for IEnumTfInputProcessorProfiles {}
windows_core::imp::define_interface!(IEnumTfLangBarItems, IEnumTfLangBarItems_Vtbl, 0x583f34d0_de25_11d2_afdd_00105a2799b5);
windows_core::imp::interface_hierarchy!(IEnumTfLangBarItems, windows_core::IUnknown);
impl IEnumTfLangBarItems {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfLangBarItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ppitem: &mut [Option<ITfLangBarItem>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppitem.len().try_into().unwrap(), core::mem::transmute(ppitem.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfLangBarItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfLangBarItems_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfLangBarItems>;
    fn Next(&self, ulcount: u32, ppitem: *mut Option<ITfLangBarItem>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfLangBarItems_Vtbl {
    pub const fn new<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfLangBarItems_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppitem: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLangBarItems_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppitem), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLangBarItems_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLangBarItems_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfLangBarItems as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfLangBarItems {}
windows_core::imp::define_interface!(IEnumTfLanguageProfiles, IEnumTfLanguageProfiles_Vtbl, 0x3d61bf11_ac5f_42c8_a4cb_931bcc28c744);
windows_core::imp::interface_hierarchy!(IEnumTfLanguageProfiles, windows_core::IUnknown);
impl IEnumTfLanguageProfiles {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfLanguageProfiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, pprofile: &mut [TF_LANGUAGEPROFILE], pcfetch: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pprofile.len().try_into().unwrap(), core::mem::transmute(pprofile.as_ptr()), core::mem::transmute(pcfetch)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfLanguageProfiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TF_LANGUAGEPROFILE, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfLanguageProfiles_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfLanguageProfiles>;
    fn Next(&self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfLanguageProfiles_Vtbl {
    pub const fn new<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfLanguageProfiles_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLanguageProfiles_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&pcfetch)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLanguageProfiles_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLanguageProfiles_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfLanguageProfiles as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfLanguageProfiles {}
windows_core::imp::define_interface!(IEnumTfLatticeElements, IEnumTfLatticeElements_Vtbl, 0x56988052_47da_4a05_911a_e3d941f17145);
windows_core::imp::interface_hierarchy!(IEnumTfLatticeElements, windows_core::IUnknown);
impl IEnumTfLatticeElements {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfLatticeElements> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, rgselements: &mut [TF_LMLATTELEMENT], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgselements.len().try_into().unwrap(), core::mem::transmute(rgselements.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfLatticeElements_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TF_LMLATTELEMENT, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfLatticeElements_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfLatticeElements>;
    fn Next(&self, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfLatticeElements_Vtbl {
    pub const fn new<Identity: IEnumTfLatticeElements_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfLatticeElements_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLatticeElements_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgselements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLatticeElements_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfLatticeElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfLatticeElements_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfLatticeElements as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfLatticeElements {}
windows_core::imp::define_interface!(IEnumTfProperties, IEnumTfProperties_Vtbl, 0x19188cb0_aca9_11d2_afc5_00105a2799b5);
windows_core::imp::interface_hierarchy!(IEnumTfProperties, windows_core::IUnknown);
impl IEnumTfProperties {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ppprop: &mut [Option<ITfProperty>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppprop.len().try_into().unwrap(), core::mem::transmute(ppprop.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfProperties_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfProperties>;
    fn Next(&self, ulcount: u32, ppprop: *mut Option<ITfProperty>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfProperties_Vtbl {
    pub const fn new<Identity: IEnumTfProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfProperties_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppprop: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfProperties_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppprop), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfProperties_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfProperties_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfProperties as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfProperties {}
windows_core::imp::define_interface!(IEnumTfPropertyValue, IEnumTfPropertyValue_Vtbl, 0x8ed8981b_7c10_4d7d_9fb3_ab72e9c75f72);
windows_core::imp::interface_hierarchy!(IEnumTfPropertyValue, windows_core::IUnknown);
impl IEnumTfPropertyValue {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfPropertyValue> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, rgvalues: &mut [TF_PROPERTYVAL], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvalues.len().try_into().unwrap(), core::mem::transmute(rgvalues.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfPropertyValue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TF_PROPERTYVAL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumTfPropertyValue_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfPropertyValue>;
    fn Next(&self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumTfPropertyValue_Vtbl {
    pub const fn new<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfPropertyValue_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfPropertyValue_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgvalues), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfPropertyValue_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfPropertyValue_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfPropertyValue as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumTfPropertyValue {}
windows_core::imp::define_interface!(IEnumTfRanges, IEnumTfRanges_Vtbl, 0xf99d3f40_8e32_11d2_bf46_00105a2799b5);
windows_core::imp::interface_hierarchy!(IEnumTfRanges, windows_core::IUnknown);
impl IEnumTfRanges {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfRanges> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, pprange: &mut [Option<ITfRange>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pprange.len().try_into().unwrap(), core::mem::transmute(pprange.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfRanges_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfRanges_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfRanges>;
    fn Next(&self, ulcount: u32, pprange: *mut Option<ITfRange>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfRanges_Vtbl {
    pub const fn new<Identity: IEnumTfRanges_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfRanges_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pprange: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfRanges_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pprange), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfRanges_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfRanges_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfRanges as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfRanges {}
windows_core::imp::define_interface!(IEnumTfUIElements, IEnumTfUIElements_Vtbl, 0x887aa91e_acba_4931_84da_3c5208cf543f);
windows_core::imp::interface_hierarchy!(IEnumTfUIElements, windows_core::IUnknown);
impl IEnumTfUIElements {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTfUIElements> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ppelement: &mut [Option<ITfUIElement>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelement.len().try_into().unwrap(), core::mem::transmute(ppelement.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumTfUIElements_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfUIElements_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfUIElements>;
    fn Next(&self, ulcount: u32, ppelement: *mut Option<ITfUIElement>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfUIElements_Vtbl {
    pub const fn new<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumTfUIElements_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppelement: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfUIElements_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppelement), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfUIElements_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumTfUIElements_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfUIElements as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfUIElements {}
windows_core::imp::define_interface!(IInternalDocWrap, IInternalDocWrap_Vtbl, 0xe1aa6466_9db4_40ba_be03_77c38e8e60b2);
windows_core::imp::interface_hierarchy!(IInternalDocWrap, windows_core::IUnknown);
impl IInternalDocWrap {
    pub unsafe fn NotifyRevoke(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyRevoke)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInternalDocWrap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyRevoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInternalDocWrap_Impl: windows_core::IUnknownImpl {
    fn NotifyRevoke(&self) -> windows_core::Result<()>;
}
impl IInternalDocWrap_Vtbl {
    pub const fn new<Identity: IInternalDocWrap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyRevoke<Identity: IInternalDocWrap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternalDocWrap_Impl::NotifyRevoke(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyRevoke: NotifyRevoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternalDocWrap as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternalDocWrap {}
pub const ILMCM_CHECKLAYOUTANDTIPENABLED: u32 = 1u32;
pub const ILMCM_LANGUAGEBAROFF: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSERT_TEXT_AT_SELECTION_FLAGS(pub u32);
pub const IS_ADDRESS_CITY: InputScope = InputScope(17i32);
pub const IS_ADDRESS_COUNTRYNAME: InputScope = InputScope(18i32);
pub const IS_ADDRESS_COUNTRYSHORTNAME: InputScope = InputScope(19i32);
pub const IS_ADDRESS_FULLPOSTALADDRESS: InputScope = InputScope(13i32);
pub const IS_ADDRESS_POSTALCODE: InputScope = InputScope(14i32);
pub const IS_ADDRESS_STATEORPROVINCE: InputScope = InputScope(16i32);
pub const IS_ADDRESS_STREET: InputScope = InputScope(15i32);
pub const IS_ALPHANUMERIC_FULLWIDTH: InputScope = InputScope(41i32);
pub const IS_ALPHANUMERIC_HALFWIDTH: InputScope = InputScope(40i32);
pub const IS_ALPHANUMERIC_PIN: InputScope = InputScope(65i32);
pub const IS_ALPHANUMERIC_PIN_SET: InputScope = InputScope(66i32);
pub const IS_BOPOMOFO: InputScope = InputScope(43i32);
pub const IS_CHAT: InputScope = InputScope(58i32);
pub const IS_CHAT_WITHOUT_EMOJI: InputScope = InputScope(68i32);
pub const IS_CHINESE_FULLWIDTH: InputScope = InputScope(54i32);
pub const IS_CHINESE_HALFWIDTH: InputScope = InputScope(53i32);
pub const IS_CURRENCY_AMOUNT: InputScope = InputScope(21i32);
pub const IS_CURRENCY_AMOUNTANDSYMBOL: InputScope = InputScope(20i32);
pub const IS_CURRENCY_CHINESE: InputScope = InputScope(42i32);
pub const IS_DATE_DAY: InputScope = InputScope(24i32);
pub const IS_DATE_DAYNAME: InputScope = InputScope(27i32);
pub const IS_DATE_FULLDATE: InputScope = InputScope(22i32);
pub const IS_DATE_MONTH: InputScope = InputScope(23i32);
pub const IS_DATE_MONTHNAME: InputScope = InputScope(26i32);
pub const IS_DATE_YEAR: InputScope = InputScope(25i32);
pub const IS_DEFAULT: InputScope = InputScope(0i32);
pub const IS_DIGITS: InputScope = InputScope(28i32);
pub const IS_EMAILNAME_OR_ADDRESS: InputScope = InputScope(60i32);
pub const IS_EMAIL_SMTPEMAILADDRESS: InputScope = InputScope(5i32);
pub const IS_EMAIL_USERNAME: InputScope = InputScope(4i32);
pub const IS_ENUMSTRING: InputScope = InputScope(-5i32);
pub const IS_FILE_FILENAME: InputScope = InputScope(3i32);
pub const IS_FILE_FULLFILEPATH: InputScope = InputScope(2i32);
pub const IS_FORMULA: InputScope = InputScope(51i32);
pub const IS_FORMULA_NUMBER: InputScope = InputScope(67i32);
pub const IS_HANGUL_FULLWIDTH: InputScope = InputScope(49i32);
pub const IS_HANGUL_HALFWIDTH: InputScope = InputScope(48i32);
pub const IS_HANJA: InputScope = InputScope(47i32);
pub const IS_HIRAGANA: InputScope = InputScope(44i32);
pub const IS_KATAKANA_FULLWIDTH: InputScope = InputScope(46i32);
pub const IS_KATAKANA_HALFWIDTH: InputScope = InputScope(45i32);
pub const IS_LOGINNAME: InputScope = InputScope(6i32);
pub const IS_MAPS: InputScope = InputScope(62i32);
pub const IS_NAME_OR_PHONENUMBER: InputScope = InputScope(59i32);
pub const IS_NATIVE_SCRIPT: InputScope = InputScope(55i32);
pub const IS_NUMBER: InputScope = InputScope(29i32);
pub const IS_NUMBER_FULLWIDTH: InputScope = InputScope(39i32);
pub const IS_NUMERIC_PASSWORD: InputScope = InputScope(63i32);
pub const IS_NUMERIC_PIN: InputScope = InputScope(64i32);
pub const IS_ONECHAR: InputScope = InputScope(30i32);
pub const IS_PASSWORD: InputScope = InputScope(31i32);
pub const IS_PERSONALNAME_FULLNAME: InputScope = InputScope(7i32);
pub const IS_PERSONALNAME_GIVENNAME: InputScope = InputScope(9i32);
pub const IS_PERSONALNAME_MIDDLENAME: InputScope = InputScope(10i32);
pub const IS_PERSONALNAME_PREFIX: InputScope = InputScope(8i32);
pub const IS_PERSONALNAME_SUFFIX: InputScope = InputScope(12i32);
pub const IS_PERSONALNAME_SURNAME: InputScope = InputScope(11i32);
pub const IS_PHRASELIST: InputScope = InputScope(-1i32);
pub const IS_PRIVATE: InputScope = InputScope(61i32);
pub const IS_REGULAREXPRESSION: InputScope = InputScope(-2i32);
pub const IS_SEARCH: InputScope = InputScope(50i32);
pub const IS_SEARCH_INCREMENTAL: InputScope = InputScope(52i32);
pub const IS_SRGS: InputScope = InputScope(-3i32);
pub const IS_TELEPHONE_AREACODE: InputScope = InputScope(34i32);
pub const IS_TELEPHONE_COUNTRYCODE: InputScope = InputScope(33i32);
pub const IS_TELEPHONE_FULLTELEPHONENUMBER: InputScope = InputScope(32i32);
pub const IS_TELEPHONE_LOCALNUMBER: InputScope = InputScope(35i32);
pub const IS_TEXT: InputScope = InputScope(57i32);
pub const IS_TIME_FULLTIME: InputScope = InputScope(36i32);
pub const IS_TIME_HOUR: InputScope = InputScope(37i32);
pub const IS_TIME_MINORSEC: InputScope = InputScope(38i32);
pub const IS_URL: InputScope = InputScope(1i32);
pub const IS_XML: InputScope = InputScope(-4i32);
pub const IS_YOMI: InputScope = InputScope(56i32);
windows_core::imp::define_interface!(ISpeechCommandProvider, ISpeechCommandProvider_Vtbl, 0x38e09d4c_586d_435a_b592_c8a86691dec6);
windows_core::imp::interface_hierarchy!(ISpeechCommandProvider, windows_core::IUnknown);
impl ISpeechCommandProvider {
    pub unsafe fn EnumSpeechCommands(&self, langid: u16) -> windows_core::Result<IEnumSpeechCommands> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumSpeechCommands)(windows_core::Interface::as_raw(self), langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProcessCommand(&self, pszcommand: &[u16], langid: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessCommand)(windows_core::Interface::as_raw(self), core::mem::transmute(pszcommand.as_ptr()), pszcommand.len().try_into().unwrap(), langid).ok()
    }
}
#[repr(C)]
pub struct ISpeechCommandProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumSpeechCommands: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessCommand: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u16) -> windows_core::HRESULT,
}
pub trait ISpeechCommandProvider_Impl: windows_core::IUnknownImpl {
    fn EnumSpeechCommands(&self, langid: u16) -> windows_core::Result<IEnumSpeechCommands>;
    fn ProcessCommand(&self, pszcommand: &windows_core::PCWSTR, cch: u32, langid: u16) -> windows_core::Result<()>;
}
impl ISpeechCommandProvider_Vtbl {
    pub const fn new<Identity: ISpeechCommandProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumSpeechCommands<Identity: ISpeechCommandProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechCommandProvider_Impl::EnumSpeechCommands(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessCommand<Identity: ISpeechCommandProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcommand: windows_core::PCWSTR, cch: u32, langid: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechCommandProvider_Impl::ProcessCommand(this, core::mem::transmute(&pszcommand), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&langid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumSpeechCommands: EnumSpeechCommands::<Identity, OFFSET>,
            ProcessCommand: ProcessCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechCommandProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpeechCommandProvider {}
windows_core::imp::define_interface!(ITextStoreACP, ITextStoreACP_Vtbl, 0x28888fe3_c2a0_483a_a3ea_8cb1ce51ff3d);
windows_core::imp::interface_hierarchy!(ITextStoreACP, windows_core::IUnknown);
impl ITextStoreACP {
    pub unsafe fn AdviseSink<P1>(&self, riid: *const windows_core::GUID, punk: P1, dwmask: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AdviseSink)(windows_core::Interface::as_raw(self), riid, punk.param().abi(), dwmask).ok()
    }
    pub unsafe fn UnadviseSink<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).UnadviseSink)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestLock)(windows_core::Interface::as_raw(self), dwlockflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TS_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryInsert)(windows_core::Interface::as_raw(self), acpteststart, acptestend, cch, core::mem::transmute(pacpresultstart), core::mem::transmute(pacpresultend)).ok()
    }
    pub unsafe fn GetSelection(&self, ulindex: u32, pselection: &mut [TS_SELECTION_ACP], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ulindex, pselection.len().try_into().unwrap(), core::mem::transmute(pselection.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn SetSelection(&self, pselection: &[TS_SELECTION_ACP]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), pselection.len().try_into().unwrap(), core::mem::transmute(pselection.as_ptr())).ok()
    }
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: &mut [u16], pcchplainret: *mut u32, prgruninfo: &mut [TS_RUNINFO], pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), acpstart, acpend, core::mem::transmute(pchplain.as_ptr()), pchplain.len().try_into().unwrap(), core::mem::transmute(pcchplainret), core::mem::transmute(prgruninfo.as_ptr()), prgruninfo.len().try_into().unwrap(), core::mem::transmute(pcruninforet), core::mem::transmute(pacpnext)).ok()
    }
    pub unsafe fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &[u16]) -> windows_core::Result<TS_TEXTCHANGE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::super::System::Com::IDataObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEmbedded(&self, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), acppos, rguidservice, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P3>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: P3) -> windows_core::Result<TS_TEXTCHANGE>
    where
        P3: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pdataobject.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &[u16], pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap(), core::mem::transmute(pacpstart), core::mem::transmute(pacpend), core::mem::transmute(pchange)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<P1>(&self, dwflags: u32, pdataobject: P1, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), dwflags, pdataobject.param().abi(), core::mem::transmute(pacpstart), core::mem::transmute(pacpend), core::mem::transmute(pchange)).ok()
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, pafilterattrs: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestSupportedAttrs)(windows_core::Interface::as_raw(self), dwflags, pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr())).ok()
    }
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, pafilterattrs: &[windows_core::GUID], dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestAttrsAtPosition)(windows_core::Interface::as_raw(self), acppos, pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, pafilterattrs: &[windows_core::GUID], dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(windows_core::Interface::as_raw(self), acppos, pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, pafilterattrs: &[windows_core::GUID], dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindNextAttrTransition)(windows_core::Interface::as_raw(self), acpstart, acphalt, pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr()), dwflags, core::mem::transmute(pacpnext), core::mem::transmute(pffound), core::mem::transmute(plfoundoffset)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, paattrvals: &mut [TS_ATTRVAL], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RetrieveRequestedAttrs)(windows_core::Interface::as_raw(self), paattrvals.len().try_into().unwrap(), core::mem::transmute(paattrvals.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn GetEndACP(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEndACP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetActiveView(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetACPFromPoint)(windows_core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), vcview, acpstart, acpend, core::mem::transmute(prc), core::mem::transmute(pfclipped)).ok()
    }
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
    }
    pub unsafe fn GetWnd(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITextStoreACP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut TS_SELECTION_ACP, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const TS_SELECTION_ACP) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PWSTR, u32, *mut u32, *mut TS_RUNINFO, u32, *mut u32, *mut i32) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, windows_core::PCWSTR, u32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::super::System::Com::FORMATETC, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryInsertEmbedded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, *mut core::ffi::c_void, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *mut i32, *mut i32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut i32, *mut i32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub FindNextAttrTransition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *const windows_core::GUID, u32, *mut i32, *mut super::super::Foundation::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TS_ATTRVAL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RetrieveRequestedAttrs: usize,
    pub GetEndACP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetACPFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::POINT, u32, *mut i32) -> windows_core::HRESULT,
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, *mut super::super::Foundation::RECT, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextStoreACP_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>, dwmask: u32) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::Result<()>;
    fn GetText(&self, acpstart: i32, acpend: i32, pchplain: windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn GetEndACP(&self) -> windows_core::Result<i32>;
    fn GetActiveView(&self) -> windows_core::Result<u32>;
    fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<i32>;
    fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::HWND>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextStoreACP_Vtbl {
    pub const fn new<Identity: ITextStoreACP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::UnadviseSink(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags)) {
                Ok(ok__) => {
                    phrsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::QueryInsert(this, core::mem::transmute_copy(&acpteststart), core::mem::transmute_copy(&acptestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpresultstart), core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::GetText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchplain), core::mem::transmute_copy(&cchplainreq), core::mem::transmute_copy(&pcchplainret), core::mem::transmute_copy(&prgruninfo), core::mem::transmute_copy(&cruninforeq), core::mem::transmute_copy(&pcruninforet), core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: windows_core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::SetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)) {
                Ok(ok__) => {
                    pchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetFormattedText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                Ok(ok__) => {
                    ppdataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetEmbedded(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                Ok(ok__) => {
                    pfinsertable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), windows_core::from_raw_borrowed(&pdataobject)) {
                Ok(ok__) => {
                    pchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchtext: windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdataobject: *mut core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acphalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacpnext), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacp: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetEndACP(this) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetActiveView(this) {
                Ok(ok__) => {
                    pvcview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP_Impl::GetWnd(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
            RequestLock: RequestLock::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            QueryInsert: QueryInsert::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, OFFSET>,
            GetEndACP: GetEndACP::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACP as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextStoreACP {}
windows_core::imp::define_interface!(ITextStoreACP2, ITextStoreACP2_Vtbl, 0xf86ad89f_5fe4_4b8d_bb9f_ef3797a84f1f);
windows_core::imp::interface_hierarchy!(ITextStoreACP2, windows_core::IUnknown);
impl ITextStoreACP2 {
    pub unsafe fn AdviseSink<P1>(&self, riid: *const windows_core::GUID, punk: P1, dwmask: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AdviseSink)(windows_core::Interface::as_raw(self), riid, punk.param().abi(), dwmask).ok()
    }
    pub unsafe fn UnadviseSink<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).UnadviseSink)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestLock)(windows_core::Interface::as_raw(self), dwlockflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TS_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryInsert)(windows_core::Interface::as_raw(self), acpteststart, acptestend, cch, core::mem::transmute(pacpresultstart), core::mem::transmute(pacpresultend)).ok()
    }
    pub unsafe fn GetSelection(&self, ulindex: u32, pselection: &mut [TS_SELECTION_ACP], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ulindex, pselection.len().try_into().unwrap(), core::mem::transmute(pselection.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn SetSelection(&self, pselection: &[TS_SELECTION_ACP]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), pselection.len().try_into().unwrap(), core::mem::transmute(pselection.as_ptr())).ok()
    }
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: &mut [u16], pcchplainret: *mut u32, prgruninfo: &mut [TS_RUNINFO], pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), acpstart, acpend, core::mem::transmute(pchplain.as_ptr()), pchplain.len().try_into().unwrap(), core::mem::transmute(pcchplainret), core::mem::transmute(prgruninfo.as_ptr()), prgruninfo.len().try_into().unwrap(), core::mem::transmute(pcruninforet), core::mem::transmute(pacpnext)).ok()
    }
    pub unsafe fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &[u16]) -> windows_core::Result<TS_TEXTCHANGE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::super::System::Com::IDataObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEmbedded(&self, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), acppos, rguidservice, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P3>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: P3) -> windows_core::Result<TS_TEXTCHANGE>
    where
        P3: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pdataobject.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &[u16], pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap(), core::mem::transmute(pacpstart), core::mem::transmute(pacpend), core::mem::transmute(pchange)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<P1>(&self, dwflags: u32, pdataobject: P1, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), dwflags, pdataobject.param().abi(), core::mem::transmute(pacpstart), core::mem::transmute(pacpend), core::mem::transmute(pchange)).ok()
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, pafilterattrs: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestSupportedAttrs)(windows_core::Interface::as_raw(self), dwflags, pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr())).ok()
    }
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, pafilterattrs: &[windows_core::GUID], dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestAttrsAtPosition)(windows_core::Interface::as_raw(self), acppos, pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, pafilterattrs: &[windows_core::GUID], dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(windows_core::Interface::as_raw(self), acppos, pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, pafilterattrs: &[windows_core::GUID], dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindNextAttrTransition)(windows_core::Interface::as_raw(self), acpstart, acphalt, pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr()), dwflags, core::mem::transmute(pacpnext), core::mem::transmute(pffound), core::mem::transmute(plfoundoffset)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, paattrvals: &mut [TS_ATTRVAL], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RetrieveRequestedAttrs)(windows_core::Interface::as_raw(self), paattrvals.len().try_into().unwrap(), core::mem::transmute(paattrvals.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn GetEndACP(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEndACP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetActiveView(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetACPFromPoint)(windows_core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), vcview, acpstart, acpend, core::mem::transmute(prc), core::mem::transmute(pfclipped)).ok()
    }
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITextStoreACP2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut TS_SELECTION_ACP, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const TS_SELECTION_ACP) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PWSTR, u32, *mut u32, *mut TS_RUNINFO, u32, *mut u32, *mut i32) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, windows_core::PCWSTR, u32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::super::System::Com::FORMATETC, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryInsertEmbedded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, *mut core::ffi::c_void, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *mut i32, *mut i32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut i32, *mut i32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub FindNextAttrTransition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *const windows_core::GUID, u32, *mut i32, *mut super::super::Foundation::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TS_ATTRVAL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RetrieveRequestedAttrs: usize,
    pub GetEndACP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetACPFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::POINT, u32, *mut i32) -> windows_core::HRESULT,
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, *mut super::super::Foundation::RECT, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextStoreACP2_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>, dwmask: u32) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::Result<()>;
    fn GetText(&self, acpstart: i32, acpend: i32, pchplain: windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn GetEndACP(&self) -> windows_core::Result<i32>;
    fn GetActiveView(&self) -> windows_core::Result<u32>;
    fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<i32>;
    fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::RECT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextStoreACP2_Vtbl {
    pub const fn new<Identity: ITextStoreACP2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::UnadviseSink(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags)) {
                Ok(ok__) => {
                    phrsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::QueryInsert(this, core::mem::transmute_copy(&acpteststart), core::mem::transmute_copy(&acptestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpresultstart), core::mem::transmute_copy(&pacpresultend)).into()
        }
        unsafe extern "system" fn GetSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::GetText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchplain), core::mem::transmute_copy(&cchplainreq), core::mem::transmute_copy(&pcchplainret), core::mem::transmute_copy(&prgruninfo), core::mem::transmute_copy(&cruninforeq), core::mem::transmute_copy(&pcruninforet), core::mem::transmute_copy(&pacpnext)).into()
        }
        unsafe extern "system" fn SetText<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: windows_core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::SetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)) {
                Ok(ok__) => {
                    pchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetFormattedText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                Ok(ok__) => {
                    ppdataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetEmbedded(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                Ok(ok__) => {
                    pfinsertable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), windows_core::from_raw_borrowed(&pdataobject)) {
                Ok(ok__) => {
                    pchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchtext: windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdataobject: *mut core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acphalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacpnext), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetEndACP<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacp: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetEndACP(this) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetActiveView(this) {
                Ok(ok__) => {
                    pvcview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACP2_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACP2_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
            RequestLock: RequestLock::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            QueryInsert: QueryInsert::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, OFFSET>,
            GetEndACP: GetEndACP::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACP2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextStoreACP2 {}
windows_core::imp::define_interface!(ITextStoreACPEx, ITextStoreACPEx_Vtbl, 0xa2de3bc2_3d8e_11d3_81a9_f753fbe61a00);
windows_core::imp::interface_hierarchy!(ITextStoreACPEx, windows_core::IUnknown);
impl ITextStoreACPEx {
    pub unsafe fn ScrollToRect(&self, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ScrollToRect)(windows_core::Interface::as_raw(self), acpstart, acpend, core::mem::transmute(rc), dwposition).ok()
    }
}
#[repr(C)]
pub struct ITextStoreACPEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ScrollToRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::super::Foundation::RECT, u32) -> windows_core::HRESULT,
}
pub trait ITextStoreACPEx_Impl: windows_core::IUnknownImpl {
    fn ScrollToRect(&self, acpstart: i32, acpend: i32, rc: &super::super::Foundation::RECT, dwposition: u32) -> windows_core::Result<()>;
}
impl ITextStoreACPEx_Vtbl {
    pub const fn new<Identity: ITextStoreACPEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ScrollToRect<Identity: ITextStoreACPEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPEx_Impl::ScrollToRect(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute(&rc), core::mem::transmute_copy(&dwposition)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollToRect: ScrollToRect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextStoreACPEx {}
windows_core::imp::define_interface!(ITextStoreACPServices, ITextStoreACPServices_Vtbl, 0xaa80e901_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITextStoreACPServices, windows_core::IUnknown);
impl ITextStoreACPServices {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0, P1, P3>(&self, pprop: P0, prange: P1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfProperty>,
        P1: windows_core::Param<ITfRange>,
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pprop.param().abi(), prange.param().abi(), core::mem::transmute(phdr), pstream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Unserialize<P0, P2, P3>(&self, pprop: P0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P2, ploader: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfProperty>,
        P2: windows_core::Param<super::super::System::Com::IStream>,
        P3: windows_core::Param<ITfPersistentPropertyLoaderACP>,
    {
        (windows_core::Interface::vtable(self).Unserialize)(windows_core::Interface::as_raw(self), pprop.param().abi(), phdr, pstream.param().abi(), ploader.param().abi()).ok()
    }
    pub unsafe fn ForceLoadProperty<P0>(&self, pprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfProperty>,
    {
        (windows_core::Interface::vtable(self).ForceLoadProperty)(windows_core::Interface::as_raw(self), pprop.param().abi()).ok()
    }
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRange)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITextStoreACPServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Unserialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Unserialize: usize,
    pub ForceLoadProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoreACPServices_Impl: windows_core::IUnknownImpl {
    fn Serialize(&self, pprop: Option<&ITfProperty>, prange: Option<&ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Unserialize(&self, pprop: Option<&ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Option<&super::super::System::Com::IStream>, ploader: Option<&ITfPersistentPropertyLoaderACP>) -> windows_core::Result<()>;
    fn ForceLoadProperty(&self, pprop: Option<&ITfProperty>) -> windows_core::Result<()>;
    fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoreACPServices_Vtbl {
    pub const fn new<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Serialize<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPServices_Impl::Serialize(this, windows_core::from_raw_borrowed(&pprop), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&phdr), windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void, ploader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPServices_Impl::Unserialize(this, windows_core::from_raw_borrowed(&pprop), core::mem::transmute_copy(&phdr), windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPServices_Impl::ForceLoadProperty(this, windows_core::from_raw_borrowed(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreACPServices_Impl::CreateRange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, OFFSET>,
            Unserialize: Unserialize::<Identity, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, OFFSET>,
            CreateRange: CreateRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextStoreACPServices {}
windows_core::imp::define_interface!(ITextStoreACPSink, ITextStoreACPSink_Vtbl, 0x22d44c94_a419_4542_a272_ae26093ececf);
windows_core::imp::interface_hierarchy!(ITextStoreACPSink, windows_core::IUnknown);
impl ITextStoreACPSink {
    pub unsafe fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTextChange)(windows_core::Interface::as_raw(self), dwflags, pchange).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnSelectionChange)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLayoutChange)(windows_core::Interface::as_raw(self), lcode, vcview).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttrsChange(&self, acpstart: i32, acpend: i32, paattrs: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAttrsChange)(windows_core::Interface::as_raw(self), acpstart, acpend, paattrs.len().try_into().unwrap(), core::mem::transmute(paattrs.as_ptr())).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLockGranted)(windows_core::Interface::as_raw(self), dwlockflags).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStartEditTransaction)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnEndEditTransaction)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITextStoreACPSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTextChange: unsafe extern "system" fn(*mut core::ffi::c_void, TEXT_STORE_TEXT_CHANGE_FLAGS, *const TS_TEXTCHANGE) -> windows_core::HRESULT,
    pub OnSelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void, TsLayoutCode, u32) -> windows_core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnAttrsChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnLockGranted: unsafe extern "system" fn(*mut core::ffi::c_void, TEXT_STORE_LOCK_FLAGS) -> windows_core::HRESULT,
    pub OnStartEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITextStoreACPSink_Impl: windows_core::IUnknownImpl {
    fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn OnSelectionChange(&self) -> windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OnAttrsChange(&self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> windows_core::Result<()>;
}
impl ITextStoreACPSink_Vtbl {
    pub const fn new<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTextChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnTextChange(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchange)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnSelectionChange(this).into()
        }
        unsafe extern "system" fn OnLayoutChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnLayoutChange(this, core::mem::transmute_copy(&lcode), core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnAttrsChange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&cattrs), core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnLockGranted(this, core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnStartEditTransaction(this).into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSink_Impl::OnEndEditTransaction(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTextChange: OnTextChange::<Identity, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextStoreACPSink {}
windows_core::imp::define_interface!(ITextStoreACPSinkEx, ITextStoreACPSinkEx_Vtbl, 0x2bdf9464_41e2_43e3_950c_a6865ba25cd4);
impl core::ops::Deref for ITextStoreACPSinkEx {
    type Target = ITextStoreACPSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextStoreACPSinkEx, windows_core::IUnknown, ITextStoreACPSink);
impl ITextStoreACPSinkEx {
    pub unsafe fn OnDisconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDisconnect)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITextStoreACPSinkEx_Vtbl {
    pub base__: ITextStoreACPSink_Vtbl,
    pub OnDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITextStoreACPSinkEx_Impl: ITextStoreACPSink_Impl {
    fn OnDisconnect(&self) -> windows_core::Result<()>;
}
impl ITextStoreACPSinkEx_Vtbl {
    pub const fn new<Identity: ITextStoreACPSinkEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDisconnect<Identity: ITextStoreACPSinkEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreACPSinkEx_Impl::OnDisconnect(this).into()
        }
        Self { base__: ITextStoreACPSink_Vtbl::new::<Identity, OFFSET>(), OnDisconnect: OnDisconnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPSinkEx as windows_core::Interface>::IID || iid == &<ITextStoreACPSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextStoreACPSinkEx {}
windows_core::imp::define_interface!(ITextStoreAnchor, ITextStoreAnchor_Vtbl, 0x9b2077b0_5f18_4dec_bee9_3cc722f5dfe0);
windows_core::imp::interface_hierarchy!(ITextStoreAnchor, windows_core::IUnknown);
impl ITextStoreAnchor {
    pub unsafe fn AdviseSink<P1>(&self, riid: *const windows_core::GUID, punk: P1, dwmask: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AdviseSink)(windows_core::Interface::as_raw(self), riid, punk.param().abi(), dwmask).ok()
    }
    pub unsafe fn UnadviseSink<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).UnadviseSink)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestLock)(windows_core::Interface::as_raw(self), dwlockflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TS_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueryInsert<P0, P1>(&self, pateststart: P0, patestend: P1, cch: u32, pparesultstart: *mut Option<IAnchor>, pparesultend: *mut Option<IAnchor>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).QueryInsert)(windows_core::Interface::as_raw(self), pateststart.param().abi(), patestend.param().abi(), cch, core::mem::transmute(pparesultstart), core::mem::transmute(pparesultend)).ok()
    }
    pub unsafe fn GetSelection(&self, ulindex: u32, pselection: &mut [TS_SELECTION_ANCHOR], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ulindex, pselection.len().try_into().unwrap(), core::mem::transmute(pselection.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn SetSelection(&self, pselection: &[TS_SELECTION_ANCHOR]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), pselection.len().try_into().unwrap(), core::mem::transmute(pselection.as_ptr())).ok()
    }
    pub unsafe fn GetText<P1, P2>(&self, dwflags: u32, pastart: P1, paend: P2, pchtext: &mut [u16], pcch: *mut u32, fupdateanchor: bool) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi(), core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap(), core::mem::transmute(pcch), fupdateanchor.into()).ok()
    }
    pub unsafe fn SetText<P1, P2>(&self, dwflags: u32, pastart: P1, paend: P2, pchtext: &[u16]) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi(), core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText<P0, P1>(&self, pastart: P0, paend: P1) -> windows_core::Result<super::super::System::Com::IDataObject>
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), pastart.param().abi(), paend.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEmbedded<P1>(&self, dwflags: u32, papos: P1, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P1: windows_core::Param<IAnchor>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), dwflags, papos.param().abi(), rguidservice, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P1, P2, P3>(&self, dwflags: u32, pastart: P1, paend: P2, pdataobject: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
        P3: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi(), pdataobject.param().abi()).ok()
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, pafilterattrs: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestSupportedAttrs)(windows_core::Interface::as_raw(self), dwflags, pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr())).ok()
    }
    pub unsafe fn RequestAttrsAtPosition<P0>(&self, papos: P0, pafilterattrs: &[windows_core::GUID], dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).RequestAttrsAtPosition)(windows_core::Interface::as_raw(self), papos.param().abi(), pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition<P0>(&self, papos: P0, pafilterattrs: &[windows_core::GUID], dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(windows_core::Interface::as_raw(self), papos.param().abi(), pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    pub unsafe fn FindNextAttrTransition<P0, P1>(&self, pastart: P0, pahalt: P1, pafilterattrs: &[windows_core::GUID], dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).FindNextAttrTransition)(windows_core::Interface::as_raw(self), pastart.param().abi(), pahalt.param().abi(), pafilterattrs.len().try_into().unwrap(), core::mem::transmute(pafilterattrs.as_ptr()), dwflags, core::mem::transmute(pffound), core::mem::transmute(plfoundoffset)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, paattrvals: &mut [TS_ATTRVAL], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RetrieveRequestedAttrs)(windows_core::Interface::as_raw(self), paattrvals.len().try_into().unwrap(), core::mem::transmute(paattrvals.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn GetStart(&self) -> windows_core::Result<IAnchor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStart)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEnd(&self) -> windows_core::Result<IAnchor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnd)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetActiveView(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAnchorFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<IAnchor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAnchorFromPoint)(windows_core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTextExt<P1, P2>(&self, vcview: u32, pastart: P1, paend: P2, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), vcview, pastart.param().abi(), paend.param().abi(), core::mem::transmute(prc), core::mem::transmute(pfclipped)).ok()
    }
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
    }
    pub unsafe fn GetWnd(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).map(|| result__)
    }
    pub unsafe fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &[u16], ppastart: *mut Option<IAnchor>, ppaend: *mut Option<IAnchor>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap(), core::mem::transmute(ppastart), core::mem::transmute(ppaend)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<P1>(&self, dwflags: u32, pdataobject: P1, ppastart: *mut Option<IAnchor>, ppaend: *mut Option<IAnchor>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), dwflags, pdataobject.param().abi(), core::mem::transmute(ppastart), core::mem::transmute(ppaend)).ok()
    }
}
#[repr(C)]
pub struct ITextStoreAnchor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut TS_SELECTION_ANCHOR, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const TS_SELECTION_ANCHOR) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub FindNextAttrTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32, *mut super::super::Foundation::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TS_ATTRVAL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RetrieveRequestedAttrs: usize,
    pub GetStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAnchorFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::POINT, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::RECT, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::super::System::Com::FORMATETC, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryInsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextStoreAnchor_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>, dwmask: u32) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, pateststart: Option<&IAnchor>, patestend: Option<&IAnchor>, cch: u32, pparesultstart: *mut Option<IAnchor>, pparesultend: *mut Option<IAnchor>) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> windows_core::Result<()>;
    fn GetText(&self, dwflags: u32, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, pchtext: windows_core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, pchtext: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
    fn GetFormattedText(&self, pastart: Option<&IAnchor>, paend: Option<&IAnchor>) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, dwflags: u32, papos: Option<&IAnchor>, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn InsertEmbedded(&self, dwflags: u32, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, papos: Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, papos: Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, pastart: Option<&IAnchor>, pahalt: Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn GetStart(&self) -> windows_core::Result<IAnchor>;
    fn GetEnd(&self) -> windows_core::Result<IAnchor>;
    fn GetActiveView(&self) -> windows_core::Result<u32>;
    fn GetAnchorFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<IAnchor>;
    fn GetTextExt(&self, vcview: u32, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&self, vcview: u32) -> windows_core::Result<super::super::Foundation::HWND>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &windows_core::PCWSTR, cch: u32, ppastart: *mut Option<IAnchor>, ppaend: *mut Option<IAnchor>) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>, ppastart: *mut Option<IAnchor>, ppaend: *mut Option<IAnchor>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITextStoreAnchor_Vtbl {
    pub const fn new<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UnadviseSink<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::UnadviseSink(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RequestLock<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags)) {
                Ok(ok__) => {
                    phrsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pateststart: *mut core::ffi::c_void, patestend: *mut core::ffi::c_void, cch: u32, pparesultstart: *mut *mut core::ffi::c_void, pparesultend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::QueryInsert(this, windows_core::from_raw_borrowed(&pateststart), windows_core::from_raw_borrowed(&patestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pparesultstart), core::mem::transmute_copy(&pparesultend)).into()
        }
        unsafe extern "system" fn GetSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetText<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pchtext: windows_core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::GetText(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&fupdateanchor)).into()
        }
        unsafe extern "system" fn SetText<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pchtext: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::SetText(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetFormattedText(this, windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend)) {
                Ok(ok__) => {
                    ppdataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, papos: *mut core::ffi::c_void, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetEmbedded(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&papos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papos: *mut core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::RequestAttrsAtPosition(this, windows_core::from_raw_borrowed(&papos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papos: *mut core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::RequestAttrsTransitioningAtPosition(this, windows_core::from_raw_borrowed(&papos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, pahalt: *mut core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const windows_core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::FindNextAttrTransition(this, windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&pahalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn GetStart<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppastart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetStart(this) {
                Ok(ok__) => {
                    ppastart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetEnd(this) {
                Ok(ok__) => {
                    ppaend.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetActiveView(this) {
                Ok(ok__) => {
                    pvcview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnchorFromPoint<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetAnchorFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppasite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::GetWnd(this, core::mem::transmute_copy(&vcview)) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextStoreAnchor_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                Ok(ok__) => {
                    pfinsertable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchtext: windows_core::PCWSTR, cch: u32, ppastart: *mut *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&ppastart), core::mem::transmute_copy(&ppaend)).into()
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdataobject: *mut core::ffi::c_void, ppastart: *mut *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchor_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&ppastart), core::mem::transmute_copy(&ppaend)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
            RequestLock: RequestLock::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            QueryInsert: QueryInsert::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, OFFSET>,
            GetStart: GetStart::<Identity, OFFSET>,
            GetEnd: GetEnd::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            GetAnchorFromPoint: GetAnchorFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITextStoreAnchor {}
windows_core::imp::define_interface!(ITextStoreAnchorEx, ITextStoreAnchorEx_Vtbl, 0xa2de3bc1_3d8e_11d3_81a9_f753fbe61a00);
windows_core::imp::interface_hierarchy!(ITextStoreAnchorEx, windows_core::IUnknown);
impl ITextStoreAnchorEx {
    pub unsafe fn ScrollToRect<P0, P1>(&self, pstart: P0, pend: P1, rc: super::super::Foundation::RECT, dwposition: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).ScrollToRect)(windows_core::Interface::as_raw(self), pstart.param().abi(), pend.param().abi(), core::mem::transmute(rc), dwposition).ok()
    }
}
#[repr(C)]
pub struct ITextStoreAnchorEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ScrollToRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::RECT, u32) -> windows_core::HRESULT,
}
pub trait ITextStoreAnchorEx_Impl: windows_core::IUnknownImpl {
    fn ScrollToRect(&self, pstart: Option<&IAnchor>, pend: Option<&IAnchor>, rc: &super::super::Foundation::RECT, dwposition: u32) -> windows_core::Result<()>;
}
impl ITextStoreAnchorEx_Vtbl {
    pub const fn new<Identity: ITextStoreAnchorEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ScrollToRect<Identity: ITextStoreAnchorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstart: *mut core::ffi::c_void, pend: *mut core::ffi::c_void, rc: super::super::Foundation::RECT, dwposition: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorEx_Impl::ScrollToRect(this, windows_core::from_raw_borrowed(&pstart), windows_core::from_raw_borrowed(&pend), core::mem::transmute(&rc), core::mem::transmute_copy(&dwposition)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollToRect: ScrollToRect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchorEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextStoreAnchorEx {}
windows_core::imp::define_interface!(ITextStoreAnchorSink, ITextStoreAnchorSink_Vtbl, 0xaa80e905_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITextStoreAnchorSink, windows_core::IUnknown);
impl ITextStoreAnchorSink {
    pub unsafe fn OnTextChange<P1, P2>(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: P1, paend: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).OnTextChange)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi()).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnSelectionChange)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLayoutChange)(windows_core::Interface::as_raw(self), lcode, vcview).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttrsChange<P0, P1>(&self, pastart: P0, paend: P1, paattrs: &[windows_core::GUID]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        (windows_core::Interface::vtable(self).OnAttrsChange)(windows_core::Interface::as_raw(self), pastart.param().abi(), paend.param().abi(), paattrs.len().try_into().unwrap(), core::mem::transmute(paattrs.as_ptr())).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLockGranted)(windows_core::Interface::as_raw(self), dwlockflags).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStartEditTransaction)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnEndEditTransaction)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITextStoreAnchorSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTextChange: unsafe extern "system" fn(*mut core::ffi::c_void, TEXT_STORE_CHANGE_FLAGS, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void, TsLayoutCode, u32) -> windows_core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnAttrsChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnLockGranted: unsafe extern "system" fn(*mut core::ffi::c_void, TEXT_STORE_LOCK_FLAGS) -> windows_core::HRESULT,
    pub OnStartEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITextStoreAnchorSink_Impl: windows_core::IUnknownImpl {
    fn OnTextChange(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: Option<&IAnchor>, paend: Option<&IAnchor>) -> windows_core::Result<()>;
    fn OnSelectionChange(&self) -> windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OnAttrsChange(&self, pastart: Option<&IAnchor>, paend: Option<&IAnchor>, cattrs: u32, paattrs: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> windows_core::Result<()>;
}
impl ITextStoreAnchorSink_Vtbl {
    pub const fn new<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTextChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnTextChange(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend)).into()
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnSelectionChange(this).into()
        }
        unsafe extern "system" fn OnLayoutChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnLayoutChange(this, core::mem::transmute_copy(&lcode), core::mem::transmute_copy(&vcview)).into()
        }
        unsafe extern "system" fn OnStatusChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, cattrs: u32, paattrs: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnAttrsChange(this, windows_core::from_raw_borrowed(&pastart), windows_core::from_raw_borrowed(&paend), core::mem::transmute_copy(&cattrs), core::mem::transmute_copy(&paattrs)).into()
        }
        unsafe extern "system" fn OnLockGranted<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnLockGranted(this, core::mem::transmute_copy(&dwlockflags)).into()
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnStartEditTransaction(this).into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreAnchorSink_Impl::OnEndEditTransaction(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTextChange: OnTextChange::<Identity, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchorSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextStoreAnchorSink {}
windows_core::imp::define_interface!(ITextStoreSinkAnchorEx, ITextStoreSinkAnchorEx_Vtbl, 0x25642426_028d_4474_977b_111bb114fe3e);
impl core::ops::Deref for ITextStoreSinkAnchorEx {
    type Target = ITextStoreAnchorSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextStoreSinkAnchorEx, windows_core::IUnknown, ITextStoreAnchorSink);
impl ITextStoreSinkAnchorEx {
    pub unsafe fn OnDisconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDisconnect)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITextStoreSinkAnchorEx_Vtbl {
    pub base__: ITextStoreAnchorSink_Vtbl,
    pub OnDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITextStoreSinkAnchorEx_Impl: ITextStoreAnchorSink_Impl {
    fn OnDisconnect(&self) -> windows_core::Result<()>;
}
impl ITextStoreSinkAnchorEx_Vtbl {
    pub const fn new<Identity: ITextStoreSinkAnchorEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDisconnect<Identity: ITextStoreSinkAnchorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextStoreSinkAnchorEx_Impl::OnDisconnect(this).into()
        }
        Self { base__: ITextStoreAnchorSink_Vtbl::new::<Identity, OFFSET>(), OnDisconnect: OnDisconnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreSinkAnchorEx as windows_core::Interface>::IID || iid == &<ITextStoreAnchorSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextStoreSinkAnchorEx {}
windows_core::imp::define_interface!(ITfActiveLanguageProfileNotifySink, ITfActiveLanguageProfileNotifySink_Vtbl, 0xb246cb75_a93e_4652_bf8c_b3fe0cfd7e57);
windows_core::imp::interface_hierarchy!(ITfActiveLanguageProfileNotifySink, windows_core::IUnknown);
impl ITfActiveLanguageProfileNotifySink {
    pub unsafe fn OnActivated(&self, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, factivated: bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnActivated)(windows_core::Interface::as_raw(self), clsid, guidprofile, factivated.into()).ok()
    }
}
#[repr(C)]
pub struct ITfActiveLanguageProfileNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnActivated: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ITfActiveLanguageProfileNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnActivated(&self, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, factivated: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl ITfActiveLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: ITfActiveLanguageProfileNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnActivated<Identity: ITfActiveLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, factivated: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfActiveLanguageProfileNotifySink_Impl::OnActivated(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&factivated)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivated: OnActivated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfActiveLanguageProfileNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfActiveLanguageProfileNotifySink {}
windows_core::imp::define_interface!(ITfCandidateList, ITfCandidateList_Vtbl, 0xa3ad50fb_9bdb_49e3_a843_6c76520fbf5d);
windows_core::imp::interface_hierarchy!(ITfCandidateList, windows_core::IUnknown);
impl ITfCandidateList {
    pub unsafe fn EnumCandidates(&self) -> windows_core::Result<IEnumTfCandidates> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCandidates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCandidate(&self, nindex: u32) -> windows_core::Result<ITfCandidateString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCandidate)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCandidateNum(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCandidateNum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetResult(&self, nindex: u32, imcr: TfCandidateResult) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResult)(windows_core::Interface::as_raw(self), nindex, imcr).ok()
    }
}
#[repr(C)]
pub struct ITfCandidateList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumCandidates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCandidate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCandidateNum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetResult: unsafe extern "system" fn(*mut core::ffi::c_void, u32, TfCandidateResult) -> windows_core::HRESULT,
}
pub trait ITfCandidateList_Impl: windows_core::IUnknownImpl {
    fn EnumCandidates(&self) -> windows_core::Result<IEnumTfCandidates>;
    fn GetCandidate(&self, nindex: u32) -> windows_core::Result<ITfCandidateString>;
    fn GetCandidateNum(&self) -> windows_core::Result<u32>;
    fn SetResult(&self, nindex: u32, imcr: TfCandidateResult) -> windows_core::Result<()>;
}
impl ITfCandidateList_Vtbl {
    pub const fn new<Identity: ITfCandidateList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumCandidates<Identity: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateList_Impl::EnumCandidates(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidate<Identity: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppcand: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateList_Impl::GetCandidate(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    ppcand.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidateNum<Identity: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncnt: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateList_Impl::GetCandidateNum(this) {
                Ok(ok__) => {
                    pncnt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Identity: ITfCandidateList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, imcr: TfCandidateResult) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateList_Impl::SetResult(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&imcr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumCandidates: EnumCandidates::<Identity, OFFSET>,
            GetCandidate: GetCandidate::<Identity, OFFSET>,
            GetCandidateNum: GetCandidateNum::<Identity, OFFSET>,
            SetResult: SetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCandidateList {}
windows_core::imp::define_interface!(ITfCandidateListUIElement, ITfCandidateListUIElement_Vtbl, 0xea1ea138_19df_11d7_a6d2_00065b84435c);
impl core::ops::Deref for ITfCandidateListUIElement {
    type Target = ITfUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfCandidateListUIElement, windows_core::IUnknown, ITfUIElement);
impl ITfCandidateListUIElement {
    pub unsafe fn GetUpdatedFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUpdatedFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSelection(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), uindex, &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetPageIndex(&self, pindex: &mut [u32], pupagecnt: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPageIndex)(windows_core::Interface::as_raw(self), core::mem::transmute(pindex.as_ptr()), pindex.len().try_into().unwrap(), core::mem::transmute(pupagecnt)).ok()
    }
    pub unsafe fn SetPageIndex(&self, pindex: &[u32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPageIndex)(windows_core::Interface::as_raw(self), core::mem::transmute(pindex.as_ptr()), pindex.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetCurrentPage(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfCandidateListUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetUpdatedFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SetPageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
    pub GetCurrentPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ITfCandidateListUIElement_Impl: ITfUIElement_Impl {
    fn GetUpdatedFlags(&self) -> windows_core::Result<u32>;
    fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetSelection(&self) -> windows_core::Result<u32>;
    fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetPageIndex(&self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> windows_core::Result<()>;
    fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> windows_core::Result<()>;
    fn GetCurrentPage(&self) -> windows_core::Result<u32>;
}
impl ITfCandidateListUIElement_Vtbl {
    pub const fn new<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUpdatedFlags<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetUpdatedFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetDocumentMgr(this) {
                Ok(ok__) => {
                    ppdim.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetCount(this) {
                Ok(ok__) => {
                    pucount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puindex: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetSelection(this) {
                Ok(ok__) => {
                    puindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetString(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    pstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageIndex<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElement_Impl::GetPageIndex(this, core::mem::transmute_copy(&pindex), core::mem::transmute_copy(&usize), core::mem::transmute_copy(&pupagecnt)).into()
        }
        unsafe extern "system" fn SetPageIndex<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElement_Impl::SetPageIndex(this, core::mem::transmute_copy(&pindex), core::mem::transmute_copy(&upagecnt)).into()
        }
        unsafe extern "system" fn GetCurrentPage<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupage: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateListUIElement_Impl::GetCurrentPage(this) {
                Ok(ok__) => {
                    pupage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Identity, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetPageIndex: GetPageIndex::<Identity, OFFSET>,
            SetPageIndex: SetPageIndex::<Identity, OFFSET>,
            GetCurrentPage: GetCurrentPage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateListUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCandidateListUIElement {}
windows_core::imp::define_interface!(ITfCandidateListUIElementBehavior, ITfCandidateListUIElementBehavior_Vtbl, 0x85fad185_58ce_497a_9460_355366b64b9a);
impl core::ops::Deref for ITfCandidateListUIElementBehavior {
    type Target = ITfCandidateListUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfCandidateListUIElementBehavior, windows_core::IUnknown, ITfUIElement, ITfCandidateListUIElement);
impl ITfCandidateListUIElementBehavior {
    pub unsafe fn SetSelection(&self, nindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nindex).ok()
    }
    pub unsafe fn Finalize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finalize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfCandidateListUIElementBehavior_Vtbl {
    pub base__: ITfCandidateListUIElement_Vtbl,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCandidateListUIElementBehavior_Impl: ITfCandidateListUIElement_Impl {
    fn SetSelection(&self, nindex: u32) -> windows_core::Result<()>;
    fn Finalize(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
impl ITfCandidateListUIElementBehavior_Vtbl {
    pub const fn new<Identity: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSelection<Identity: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElementBehavior_Impl::SetSelection(this, core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn Finalize<Identity: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElementBehavior_Impl::Finalize(this).into()
        }
        unsafe extern "system" fn Abort<Identity: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCandidateListUIElementBehavior_Impl::Abort(this).into()
        }
        Self {
            base__: ITfCandidateListUIElement_Vtbl::new::<Identity, OFFSET>(),
            SetSelection: SetSelection::<Identity, OFFSET>,
            Finalize: Finalize::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateListUIElementBehavior as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID || iid == &<ITfCandidateListUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCandidateListUIElementBehavior {}
windows_core::imp::define_interface!(ITfCandidateString, ITfCandidateString_Vtbl, 0x581f317e_fd9d_443f_b972_ed00467c5d40);
windows_core::imp::interface_hierarchy!(ITfCandidateString, windows_core::IUnknown);
impl ITfCandidateString {
    pub unsafe fn GetString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetIndex(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfCandidateString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ITfCandidateString_Impl: windows_core::IUnknownImpl {
    fn GetString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIndex(&self) -> windows_core::Result<u32>;
}
impl ITfCandidateString_Vtbl {
    pub const fn new<Identity: ITfCandidateString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetString<Identity: ITfCandidateString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateString_Impl::GetString(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ITfCandidateString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnindex: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCandidateString_Impl::GetIndex(this) {
                Ok(ok__) => {
                    pnindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetString: GetString::<Identity, OFFSET>, GetIndex: GetIndex::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateString as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCandidateString {}
windows_core::imp::define_interface!(ITfCategoryMgr, ITfCategoryMgr_Vtbl, 0xc3acefb5_f69d_4905_938f_fcadcf4be830);
windows_core::imp::interface_hierarchy!(ITfCategoryMgr, windows_core::IUnknown);
impl ITfCategoryMgr {
    pub unsafe fn RegisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterCategory)(windows_core::Interface::as_raw(self), rclsid, rcatid, rguid).ok()
    }
    pub unsafe fn UnregisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterCategory)(windows_core::Interface::as_raw(self), rclsid, rcatid, rguid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCategoriesInItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCategoriesInItem)(windows_core::Interface::as_raw(self), rguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumItemsInCategory(&self, rcatid: *const windows_core::GUID) -> windows_core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumItemsInCategory)(windows_core::Interface::as_raw(self), rcatid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindClosestCategory(&self, rguid: *const windows_core::GUID, pcatid: *mut windows_core::GUID, ppcatidlist: &[*const windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindClosestCategory)(windows_core::Interface::as_raw(self), rguid, core::mem::transmute(pcatid), core::mem::transmute(ppcatidlist.as_ptr()), ppcatidlist.len().try_into().unwrap()).ok()
    }
    pub unsafe fn RegisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, pchdesc: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterGUIDDescription)(windows_core::Interface::as_raw(self), rclsid, rguid, core::mem::transmute(pchdesc.as_ptr()), pchdesc.len().try_into().unwrap()).ok()
    }
    pub unsafe fn UnregisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterGUIDDescription)(windows_core::Interface::as_raw(self), rclsid, rguid).ok()
    }
    pub unsafe fn GetGUIDDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGUIDDescription)(windows_core::Interface::as_raw(self), rguid, &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn RegisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, dw: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterGUIDDWORD)(windows_core::Interface::as_raw(self), rclsid, rguid, dw).ok()
    }
    pub unsafe fn UnregisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterGUIDDWORD)(windows_core::Interface::as_raw(self), rclsid, rguid).ok()
    }
    pub unsafe fn GetGUIDDWORD(&self, rguid: *const windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGUIDDWORD)(windows_core::Interface::as_raw(self), rguid, &mut result__).map(|| result__)
    }
    pub unsafe fn RegisterGUID(&self, rguid: *const windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterGUID)(windows_core::Interface::as_raw(self), rguid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetGUID(&self, guidatom: u32) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), guidatom, &mut result__).map(|| result__)
    }
    pub unsafe fn IsEqualTfGuidAtom(&self, guidatom: u32, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEqualTfGuidAtom)(windows_core::Interface::as_raw(self), guidatom, rguid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfCategoryMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub UnregisterCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCategoriesInItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCategoriesInItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumItemsInCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumItemsInCategory: usize,
    pub FindClosestCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::GUID, *const *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub RegisterGUIDDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub UnregisterGUIDDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetGUIDDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterGUIDDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub UnregisterGUIDDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetGUIDDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub RegisterGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsEqualTfGuidAtom: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfCategoryMgr_Impl: windows_core::IUnknownImpl {
    fn RegisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnregisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumCategoriesInItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn EnumItemsInCategory(&self, rcatid: *const windows_core::GUID) -> windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn FindClosestCategory(&self, rguid: *const windows_core::GUID, pcatid: *mut windows_core::GUID, ppcatidlist: *const *const windows_core::GUID, ulcount: u32) -> windows_core::Result<()>;
    fn RegisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, pchdesc: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
    fn UnregisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGUIDDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, dw: u32) -> windows_core::Result<()>;
    fn UnregisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGUIDDWORD(&self, rguid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn RegisterGUID(&self, rguid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetGUID(&self, guidatom: u32) -> windows_core::Result<windows_core::GUID>;
    fn IsEqualTfGuidAtom(&self, guidatom: u32, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfCategoryMgr_Vtbl {
    pub const fn new<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterCategory<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::RegisterCategory(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rcatid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn UnregisterCategory<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::UnregisterCategory(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rcatid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCategoriesInItem<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::EnumCategoriesInItem(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsInCategory<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcatid: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::EnumItemsInCategory(this, core::mem::transmute_copy(&rcatid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClosestCategory<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pcatid: *mut windows_core::GUID, ppcatidlist: *const *const windows_core::GUID, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::FindClosestCategory(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pcatid), core::mem::transmute_copy(&ppcatidlist), core::mem::transmute_copy(&ulcount)).into()
        }
        unsafe extern "system" fn RegisterGUIDDescription<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, pchdesc: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::RegisterGUIDDescription(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDescription<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::UnregisterGUIDDescription(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDescription<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pbstrdesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::GetGUIDDescription(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pbstrdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUIDDWORD<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, dw: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::RegisterGUIDDWORD(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn UnregisterGUIDDWORD<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCategoryMgr_Impl::UnregisterGUIDDWORD(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn GetGUIDDWORD<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pdw: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::GetGUIDDWORD(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pdw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterGUID<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pguidatom: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::RegisterGUID(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pguidatom.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidatom: u32, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::GetGUID(this, core::mem::transmute_copy(&guidatom)) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualTfGuidAtom<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidatom: u32, rguid: *const windows_core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCategoryMgr_Impl::IsEqualTfGuidAtom(this, core::mem::transmute_copy(&guidatom), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pfequal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterCategory: RegisterCategory::<Identity, OFFSET>,
            UnregisterCategory: UnregisterCategory::<Identity, OFFSET>,
            EnumCategoriesInItem: EnumCategoriesInItem::<Identity, OFFSET>,
            EnumItemsInCategory: EnumItemsInCategory::<Identity, OFFSET>,
            FindClosestCategory: FindClosestCategory::<Identity, OFFSET>,
            RegisterGUIDDescription: RegisterGUIDDescription::<Identity, OFFSET>,
            UnregisterGUIDDescription: UnregisterGUIDDescription::<Identity, OFFSET>,
            GetGUIDDescription: GetGUIDDescription::<Identity, OFFSET>,
            RegisterGUIDDWORD: RegisterGUIDDWORD::<Identity, OFFSET>,
            UnregisterGUIDDWORD: UnregisterGUIDDWORD::<Identity, OFFSET>,
            GetGUIDDWORD: GetGUIDDWORD::<Identity, OFFSET>,
            RegisterGUID: RegisterGUID::<Identity, OFFSET>,
            GetGUID: GetGUID::<Identity, OFFSET>,
            IsEqualTfGuidAtom: IsEqualTfGuidAtom::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCategoryMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfCategoryMgr {}
windows_core::imp::define_interface!(ITfCleanupContextDurationSink, ITfCleanupContextDurationSink_Vtbl, 0x45c35144_154e_4797_bed8_d33ae7bf8794);
windows_core::imp::interface_hierarchy!(ITfCleanupContextDurationSink, windows_core::IUnknown);
impl ITfCleanupContextDurationSink {
    pub unsafe fn OnStartCleanupContext(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStartCleanupContext)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnEndCleanupContext(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnEndCleanupContext)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfCleanupContextDurationSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStartCleanupContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndCleanupContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCleanupContextDurationSink_Impl: windows_core::IUnknownImpl {
    fn OnStartCleanupContext(&self) -> windows_core::Result<()>;
    fn OnEndCleanupContext(&self) -> windows_core::Result<()>;
}
impl ITfCleanupContextDurationSink_Vtbl {
    pub const fn new<Identity: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStartCleanupContext<Identity: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCleanupContextDurationSink_Impl::OnStartCleanupContext(this).into()
        }
        unsafe extern "system" fn OnEndCleanupContext<Identity: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCleanupContextDurationSink_Impl::OnEndCleanupContext(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartCleanupContext: OnStartCleanupContext::<Identity, OFFSET>,
            OnEndCleanupContext: OnEndCleanupContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCleanupContextDurationSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCleanupContextDurationSink {}
windows_core::imp::define_interface!(ITfCleanupContextSink, ITfCleanupContextSink_Vtbl, 0x01689689_7acb_4e9b_ab7c_7ea46b12b522);
windows_core::imp::interface_hierarchy!(ITfCleanupContextSink, windows_core::IUnknown);
impl ITfCleanupContextSink {
    pub unsafe fn OnCleanupContext<P1>(&self, ecwrite: u32, pic: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfContext>,
    {
        (windows_core::Interface::vtable(self).OnCleanupContext)(windows_core::Interface::as_raw(self), ecwrite, pic.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfCleanupContextSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCleanupContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCleanupContextSink_Impl: windows_core::IUnknownImpl {
    fn OnCleanupContext(&self, ecwrite: u32, pic: Option<&ITfContext>) -> windows_core::Result<()>;
}
impl ITfCleanupContextSink_Vtbl {
    pub const fn new<Identity: ITfCleanupContextSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCleanupContext<Identity: ITfCleanupContextSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCleanupContextSink_Impl::OnCleanupContext(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pic)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCleanupContext: OnCleanupContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCleanupContextSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCleanupContextSink {}
windows_core::imp::define_interface!(ITfClientId, ITfClientId_Vtbl, 0xd60a7b49_1b9f_4be2_b702_47e9dc05dec3);
windows_core::imp::interface_hierarchy!(ITfClientId, windows_core::IUnknown);
impl ITfClientId {
    pub unsafe fn GetClientId(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClientId)(windows_core::Interface::as_raw(self), rclsid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfClientId_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
}
pub trait ITfClientId_Impl: windows_core::IUnknownImpl {
    fn GetClientId(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<u32>;
}
impl ITfClientId_Vtbl {
    pub const fn new<Identity: ITfClientId_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientId<Identity: ITfClientId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ptid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfClientId_Impl::GetClientId(this, core::mem::transmute_copy(&rclsid)) {
                Ok(ok__) => {
                    ptid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClientId: GetClientId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfClientId as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfClientId {}
windows_core::imp::define_interface!(ITfCompartment, ITfCompartment_Vtbl, 0xbb08f7a9_607a_4384_8623_056892b64371);
windows_core::imp::interface_hierarchy!(ITfCompartment, windows_core::IUnknown);
impl ITfCompartment {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, tid: u32, pvarvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), tid, core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfCompartment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetValue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfCompartment_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, tid: u32, pvarvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetValue(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITfCompartment_Vtbl {
    pub const fn new<Identity: ITfCompartment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: ITfCompartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, pvarvalue: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCompartment_Impl::SetValue(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ITfCompartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompartment_Impl::GetValue(this) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetValue: SetValue::<Identity, OFFSET>, GetValue: GetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompartment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITfCompartment {}
windows_core::imp::define_interface!(ITfCompartmentEventSink, ITfCompartmentEventSink_Vtbl, 0x743abd5f_f26d_48df_8cc5_238492419b64);
windows_core::imp::interface_hierarchy!(ITfCompartmentEventSink, windows_core::IUnknown);
impl ITfCompartmentEventSink {
    pub unsafe fn OnChange(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnChange)(windows_core::Interface::as_raw(self), rguid).ok()
    }
}
#[repr(C)]
pub struct ITfCompartmentEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITfCompartmentEventSink_Impl: windows_core::IUnknownImpl {
    fn OnChange(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ITfCompartmentEventSink_Vtbl {
    pub const fn new<Identity: ITfCompartmentEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnChange<Identity: ITfCompartmentEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCompartmentEventSink_Impl::OnChange(this, core::mem::transmute_copy(&rguid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompartmentEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCompartmentEventSink {}
windows_core::imp::define_interface!(ITfCompartmentMgr, ITfCompartmentMgr_Vtbl, 0x7dcf57ac_18ad_438b_824d_979bffb74b7c);
windows_core::imp::interface_hierarchy!(ITfCompartmentMgr, windows_core::IUnknown);
impl ITfCompartmentMgr {
    pub unsafe fn GetCompartment(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfCompartment> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCompartment)(windows_core::Interface::as_raw(self), rguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClearCompartment(&self, tid: u32, rguid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearCompartment)(windows_core::Interface::as_raw(self), tid, rguid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCompartments(&self) -> windows_core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCompartments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfCompartmentMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCompartment: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearCompartment: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCompartments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCompartments: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfCompartmentMgr_Impl: windows_core::IUnknownImpl {
    fn GetCompartment(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfCompartment>;
    fn ClearCompartment(&self, tid: u32, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumCompartments(&self) -> windows_core::Result<super::super::System::Com::IEnumGUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfCompartmentMgr_Vtbl {
    pub const fn new<Identity: ITfCompartmentMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCompartment<Identity: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, ppcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompartmentMgr_Impl::GetCompartment(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    ppcomp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearCompartment<Identity: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCompartmentMgr_Impl::ClearCompartment(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn EnumCompartments<Identity: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompartmentMgr_Impl::EnumCompartments(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCompartment: GetCompartment::<Identity, OFFSET>,
            ClearCompartment: ClearCompartment::<Identity, OFFSET>,
            EnumCompartments: EnumCompartments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompartmentMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfCompartmentMgr {}
windows_core::imp::define_interface!(ITfComposition, ITfComposition_Vtbl, 0x20168d64_5a8f_4a5a_b7bd_cfa29f4d0fd9);
windows_core::imp::interface_hierarchy!(ITfComposition, windows_core::IUnknown);
impl ITfComposition {
    pub unsafe fn GetRange(&self) -> windows_core::Result<ITfRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShiftStart<P1>(&self, ecwrite: u32, pnewstart: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).ShiftStart)(windows_core::Interface::as_raw(self), ecwrite, pnewstart.param().abi()).ok()
    }
    pub unsafe fn ShiftEnd<P1>(&self, ecwrite: u32, pnewend: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).ShiftEnd)(windows_core::Interface::as_raw(self), ecwrite, pnewend.param().abi()).ok()
    }
    pub unsafe fn EndComposition(&self, ecwrite: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndComposition)(windows_core::Interface::as_raw(self), ecwrite).ok()
    }
}
#[repr(C)]
pub struct ITfComposition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShiftStart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShiftEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndComposition: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfComposition_Impl: windows_core::IUnknownImpl {
    fn GetRange(&self) -> windows_core::Result<ITfRange>;
    fn ShiftStart(&self, ecwrite: u32, pnewstart: Option<&ITfRange>) -> windows_core::Result<()>;
    fn ShiftEnd(&self, ecwrite: u32, pnewend: Option<&ITfRange>) -> windows_core::Result<()>;
    fn EndComposition(&self, ecwrite: u32) -> windows_core::Result<()>;
}
impl ITfComposition_Vtbl {
    pub const fn new<Identity: ITfComposition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRange<Identity: ITfComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfComposition_Impl::GetRange(this) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftStart<Identity: ITfComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pnewstart: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfComposition_Impl::ShiftStart(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pnewstart)).into()
        }
        unsafe extern "system" fn ShiftEnd<Identity: ITfComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pnewend: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfComposition_Impl::ShiftEnd(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pnewend)).into()
        }
        unsafe extern "system" fn EndComposition<Identity: ITfComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfComposition_Impl::EndComposition(this, core::mem::transmute_copy(&ecwrite)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRange: GetRange::<Identity, OFFSET>,
            ShiftStart: ShiftStart::<Identity, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, OFFSET>,
            EndComposition: EndComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfComposition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfComposition {}
windows_core::imp::define_interface!(ITfCompositionSink, ITfCompositionSink_Vtbl, 0xa781718c_579a_4b15_a280_32b8577acc5e);
windows_core::imp::interface_hierarchy!(ITfCompositionSink, windows_core::IUnknown);
impl ITfCompositionSink {
    pub unsafe fn OnCompositionTerminated<P1>(&self, ecwrite: u32, pcomposition: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfComposition>,
    {
        (windows_core::Interface::vtable(self).OnCompositionTerminated)(windows_core::Interface::as_raw(self), ecwrite, pcomposition.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfCompositionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCompositionTerminated: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCompositionSink_Impl: windows_core::IUnknownImpl {
    fn OnCompositionTerminated(&self, ecwrite: u32, pcomposition: Option<&ITfComposition>) -> windows_core::Result<()>;
}
impl ITfCompositionSink_Vtbl {
    pub const fn new<Identity: ITfCompositionSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCompositionTerminated<Identity: ITfCompositionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pcomposition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfCompositionSink_Impl::OnCompositionTerminated(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pcomposition)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCompositionTerminated: OnCompositionTerminated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompositionSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCompositionSink {}
windows_core::imp::define_interface!(ITfCompositionView, ITfCompositionView_Vtbl, 0xd7540241_f9a1_4364_befc_dbcd2c4395b7);
windows_core::imp::interface_hierarchy!(ITfCompositionView, windows_core::IUnknown);
impl ITfCompositionView {
    pub unsafe fn GetOwnerClsid(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwnerClsid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRange(&self) -> windows_core::Result<ITfRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfCompositionView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwnerClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCompositionView_Impl: windows_core::IUnknownImpl {
    fn GetOwnerClsid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetRange(&self) -> windows_core::Result<ITfRange>;
}
impl ITfCompositionView_Vtbl {
    pub const fn new<Identity: ITfCompositionView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwnerClsid<Identity: ITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompositionView_Impl::GetOwnerClsid(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Identity: ITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCompositionView_Impl::GetRange(this) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerClsid: GetOwnerClsid::<Identity, OFFSET>,
            GetRange: GetRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompositionView as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCompositionView {}
windows_core::imp::define_interface!(ITfConfigureSystemKeystrokeFeed, ITfConfigureSystemKeystrokeFeed_Vtbl, 0x0d2c969a_bc9c_437c_84ee_951c49b1a764);
windows_core::imp::interface_hierarchy!(ITfConfigureSystemKeystrokeFeed, windows_core::IUnknown);
impl ITfConfigureSystemKeystrokeFeed {
    pub unsafe fn DisableSystemKeystrokeFeed(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableSystemKeystrokeFeed)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableSystemKeystrokeFeed(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableSystemKeystrokeFeed)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfConfigureSystemKeystrokeFeed_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DisableSystemKeystrokeFeed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableSystemKeystrokeFeed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfConfigureSystemKeystrokeFeed_Impl: windows_core::IUnknownImpl {
    fn DisableSystemKeystrokeFeed(&self) -> windows_core::Result<()>;
    fn EnableSystemKeystrokeFeed(&self) -> windows_core::Result<()>;
}
impl ITfConfigureSystemKeystrokeFeed_Vtbl {
    pub const fn new<Identity: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisableSystemKeystrokeFeed<Identity: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfConfigureSystemKeystrokeFeed_Impl::DisableSystemKeystrokeFeed(this).into()
        }
        unsafe extern "system" fn EnableSystemKeystrokeFeed<Identity: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfConfigureSystemKeystrokeFeed_Impl::EnableSystemKeystrokeFeed(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DisableSystemKeystrokeFeed: DisableSystemKeystrokeFeed::<Identity, OFFSET>,
            EnableSystemKeystrokeFeed: EnableSystemKeystrokeFeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfConfigureSystemKeystrokeFeed as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfConfigureSystemKeystrokeFeed {}
windows_core::imp::define_interface!(ITfContext, ITfContext_Vtbl, 0xaa80e7fd_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfContext, windows_core::IUnknown);
impl ITfContext {
    pub unsafe fn RequestEditSession<P1>(&self, tid: u32, pes: P1, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS) -> windows_core::Result<windows_core::HRESULT>
    where
        P1: windows_core::Param<ITfEditSession>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestEditSession)(windows_core::Interface::as_raw(self), tid, pes.param().abi(), dwflags, &mut result__).map(|| result__)
    }
    pub unsafe fn InWriteSession(&self, tid: u32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InWriteSession)(windows_core::Interface::as_raw(self), tid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetSelection(&self, ec: u32, ulindex: u32, pselection: &mut [TF_SELECTION], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ec, ulindex, pselection.len().try_into().unwrap(), core::mem::transmute(pselection.as_ptr()), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn SetSelection(&self, ec: u32, pselection: &[TF_SELECTION]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), ec, pselection.len().try_into().unwrap(), core::mem::transmute(pselection.as_ptr())).ok()
    }
    pub unsafe fn GetStart(&self, ec: u32) -> windows_core::Result<ITfRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStart)(windows_core::Interface::as_raw(self), ec, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEnd(&self, ec: u32) -> windows_core::Result<ITfRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnd)(windows_core::Interface::as_raw(self), ec, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetActiveView(&self) -> windows_core::Result<ITfContextView> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumViews(&self) -> windows_core::Result<IEnumTfContextViews> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumViews)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TS_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfProperty> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), guidprop, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAppProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfReadOnlyProperty> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAppProperty)(windows_core::Interface::as_raw(self), guidprop, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TrackProperties(&self, prgprop: &[*const windows_core::GUID], prgappprop: &[*const windows_core::GUID]) -> windows_core::Result<ITfReadOnlyProperty> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TrackProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(prgprop.as_ptr()), prgprop.len().try_into().unwrap(), core::mem::transmute(prgappprop.as_ptr()), prgappprop.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumProperties(&self) -> windows_core::Result<IEnumTfProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRangeBackup<P1>(&self, ec: u32, prange: P1) -> windows_core::Result<ITfRangeBackup>
    where
        P1: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRangeBackup)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestEditSession: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, TF_CONTEXT_EDIT_CONTEXT_FLAGS, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub InWriteSession: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut TF_SELECTION, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const TF_SELECTION) -> windows_core::HRESULT,
    pub GetStart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrackProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const windows_core::GUID, u32, *const *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRangeBackup: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfContext_Impl: windows_core::IUnknownImpl {
    fn RequestEditSession(&self, tid: u32, pes: Option<&ITfEditSession>, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS) -> windows_core::Result<windows_core::HRESULT>;
    fn InWriteSession(&self, tid: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetSelection(&self, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> windows_core::Result<()>;
    fn GetStart(&self, ec: u32) -> windows_core::Result<ITfRange>;
    fn GetEnd(&self, ec: u32) -> windows_core::Result<ITfRange>;
    fn GetActiveView(&self) -> windows_core::Result<ITfContextView>;
    fn EnumViews(&self) -> windows_core::Result<IEnumTfContextViews>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn GetProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfProperty>;
    fn GetAppProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfReadOnlyProperty>;
    fn TrackProperties(&self, prgprop: *const *const windows_core::GUID, cprop: u32, prgappprop: *const *const windows_core::GUID, cappprop: u32) -> windows_core::Result<ITfReadOnlyProperty>;
    fn EnumProperties(&self) -> windows_core::Result<IEnumTfProperties>;
    fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn CreateRangeBackup(&self, ec: u32, prange: Option<&ITfRange>) -> windows_core::Result<ITfRangeBackup>;
}
impl ITfContext_Vtbl {
    pub const fn new<Identity: ITfContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestEditSession<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, pes: *mut core::ffi::c_void, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::RequestEditSession(this, core::mem::transmute_copy(&tid), windows_core::from_raw_borrowed(&pes), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    phrsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InWriteSession<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::InWriteSession(this, core::mem::transmute_copy(&tid)) {
                Ok(ok__) => {
                    pfwritesession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContext_Impl::GetSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn SetSelection<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContext_Impl::SetSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
        }
        unsafe extern "system" fn GetStart<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppstart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetStart(this, core::mem::transmute_copy(&ec)) {
                Ok(ok__) => {
                    ppstart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetEnd(this, core::mem::transmute_copy(&ec)) {
                Ok(ok__) => {
                    ppend.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetActiveView(this) {
                Ok(ok__) => {
                    ppview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::EnumViews(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetProperty(this, core::mem::transmute_copy(&guidprop)) {
                Ok(ok__) => {
                    ppprop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppProperty<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetAppProperty(this, core::mem::transmute_copy(&guidprop)) {
                Ok(ok__) => {
                    ppprop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackProperties<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prgprop: *const *const windows_core::GUID, cprop: u32, prgappprop: *const *const windows_core::GUID, cappprop: u32, ppproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::TrackProperties(this, core::mem::transmute_copy(&prgprop), core::mem::transmute_copy(&cprop), core::mem::transmute_copy(&prgappprop), core::mem::transmute_copy(&cappprop)) {
                Ok(ok__) => {
                    ppproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProperties<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::EnumProperties(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::GetDocumentMgr(this) {
                Ok(ok__) => {
                    ppdm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRangeBackup<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, ppbackup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContext_Impl::CreateRangeBackup(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    ppbackup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestEditSession: RequestEditSession::<Identity, OFFSET>,
            InWriteSession: InWriteSession::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetStart: GetStart::<Identity, OFFSET>,
            GetEnd: GetEnd::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            EnumViews: EnumViews::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetAppProperty: GetAppProperty::<Identity, OFFSET>,
            TrackProperties: TrackProperties::<Identity, OFFSET>,
            EnumProperties: EnumProperties::<Identity, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, OFFSET>,
            CreateRangeBackup: CreateRangeBackup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfContext {}
windows_core::imp::define_interface!(ITfContextComposition, ITfContextComposition_Vtbl, 0xd40c8aae_ac92_4fc7_9a11_0ee0e23aa39b);
windows_core::imp::interface_hierarchy!(ITfContextComposition, windows_core::IUnknown);
impl ITfContextComposition {
    pub unsafe fn StartComposition<P1, P2>(&self, ecwrite: u32, pcompositionrange: P1, psink: P2) -> windows_core::Result<ITfComposition>
    where
        P1: windows_core::Param<ITfRange>,
        P2: windows_core::Param<ITfCompositionSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartComposition)(windows_core::Interface::as_raw(self), ecwrite, pcompositionrange.param().abi(), psink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumCompositions(&self) -> windows_core::Result<IEnumITfCompositionView> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCompositions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindComposition<P1>(&self, ecread: u32, ptestrange: P1) -> windows_core::Result<IEnumITfCompositionView>
    where
        P1: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindComposition)(windows_core::Interface::as_raw(self), ecread, ptestrange.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TakeOwnership<P1, P2>(&self, ecwrite: u32, pcomposition: P1, psink: P2) -> windows_core::Result<ITfComposition>
    where
        P1: windows_core::Param<ITfCompositionView>,
        P2: windows_core::Param<ITfCompositionSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TakeOwnership)(windows_core::Interface::as_raw(self), ecwrite, pcomposition.param().abi(), psink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfContextComposition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartComposition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumCompositions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindComposition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfContextComposition_Impl: windows_core::IUnknownImpl {
    fn StartComposition(&self, ecwrite: u32, pcompositionrange: Option<&ITfRange>, psink: Option<&ITfCompositionSink>) -> windows_core::Result<ITfComposition>;
    fn EnumCompositions(&self) -> windows_core::Result<IEnumITfCompositionView>;
    fn FindComposition(&self, ecread: u32, ptestrange: Option<&ITfRange>) -> windows_core::Result<IEnumITfCompositionView>;
    fn TakeOwnership(&self, ecwrite: u32, pcomposition: Option<&ITfCompositionView>, psink: Option<&ITfCompositionSink>) -> windows_core::Result<ITfComposition>;
}
impl ITfContextComposition_Vtbl {
    pub const fn new<Identity: ITfContextComposition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartComposition<Identity: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pcompositionrange: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, ppcomposition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextComposition_Impl::StartComposition(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pcompositionrange), windows_core::from_raw_borrowed(&psink)) {
                Ok(ok__) => {
                    ppcomposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCompositions<Identity: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextComposition_Impl::EnumCompositions(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComposition<Identity: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecread: u32, ptestrange: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextComposition_Impl::FindComposition(this, core::mem::transmute_copy(&ecread), windows_core::from_raw_borrowed(&ptestrange)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: u32, pcomposition: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, ppcomposition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextComposition_Impl::TakeOwnership(this, core::mem::transmute_copy(&ecwrite), windows_core::from_raw_borrowed(&pcomposition), windows_core::from_raw_borrowed(&psink)) {
                Ok(ok__) => {
                    ppcomposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartComposition: StartComposition::<Identity, OFFSET>,
            EnumCompositions: EnumCompositions::<Identity, OFFSET>,
            FindComposition: FindComposition::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextComposition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfContextComposition {}
windows_core::imp::define_interface!(ITfContextKeyEventSink, ITfContextKeyEventSink_Vtbl, 0x0552ba5d_c835_4934_bf50_846aaa67432f);
windows_core::imp::interface_hierarchy!(ITfContextKeyEventSink, windows_core::IUnknown);
impl ITfContextKeyEventSink {
    pub unsafe fn OnKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnKeyDown)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn OnKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnKeyUp)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn OnTestKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnTestKeyDown)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn OnTestKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnTestKeyUp)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfContextKeyEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnTestKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnTestKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ITfContextKeyEventSink_Impl: windows_core::IUnknownImpl {
    fn OnKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl ITfContextKeyEventSink_Vtbl {
    pub const fn new<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnKeyDown<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextKeyEventSink_Impl::OnKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextKeyEventSink_Impl::OnKeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextKeyEventSink_Impl::OnTestKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextKeyEventSink_Impl::OnTestKeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnKeyDown: OnKeyDown::<Identity, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextKeyEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfContextKeyEventSink {}
windows_core::imp::define_interface!(ITfContextOwner, ITfContextOwner_Vtbl, 0xaa80e80c_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfContextOwner, windows_core::IUnknown);
impl ITfContextOwner {
    pub unsafe fn GetACPFromPoint(&self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetACPFromPoint)(windows_core::Interface::as_raw(self), ptscreen, dwflags, &mut result__).map(|| result__)
    }
    pub unsafe fn GetTextExt(&self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), acpstart, acpend, core::mem::transmute(prc), core::mem::transmute(pfclipped)).ok()
    }
    pub unsafe fn GetScreenExt(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TS_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetWnd(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAttribute(&self, rguidattribute: *const windows_core::GUID) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), rguidattribute, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfContextOwner_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetACPFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::POINT, u32, *mut i32) -> windows_core::HRESULT,
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut super::super::Foundation::RECT, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAttribute: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfContextOwner_Impl: windows_core::IUnknownImpl {
    fn GetACPFromPoint(&self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<i32>;
    fn GetTextExt(&self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn GetWnd(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn GetAttribute(&self, rguidattribute: *const windows_core::GUID) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITfContextOwner_Vtbl {
    pub const fn new<Identity: ITfContextOwner_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetACPFromPoint<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pacp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwner_Impl::GetTextExt(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetScreenExt(this) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdcs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetWnd(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidattribute: *const windows_core::GUID, pvarvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwner_Impl::GetAttribute(this, core::mem::transmute_copy(&rguidattribute)) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetACPFromPoint: GetACPFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
            GetAttribute: GetAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwner as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITfContextOwner {}
windows_core::imp::define_interface!(ITfContextOwnerCompositionServices, ITfContextOwnerCompositionServices_Vtbl, 0x86462810_593b_4916_9764_19c08e9ce110);
impl core::ops::Deref for ITfContextOwnerCompositionServices {
    type Target = ITfContextComposition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfContextOwnerCompositionServices, windows_core::IUnknown, ITfContextComposition);
impl ITfContextOwnerCompositionServices {
    pub unsafe fn TerminateComposition<P0>(&self, pcomposition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfCompositionView>,
    {
        (windows_core::Interface::vtable(self).TerminateComposition)(windows_core::Interface::as_raw(self), pcomposition.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfContextOwnerCompositionServices_Vtbl {
    pub base__: ITfContextComposition_Vtbl,
    pub TerminateComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfContextOwnerCompositionServices_Impl: ITfContextComposition_Impl {
    fn TerminateComposition(&self, pcomposition: Option<&ITfCompositionView>) -> windows_core::Result<()>;
}
impl ITfContextOwnerCompositionServices_Vtbl {
    pub const fn new<Identity: ITfContextOwnerCompositionServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TerminateComposition<Identity: ITfContextOwnerCompositionServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerCompositionServices_Impl::TerminateComposition(this, windows_core::from_raw_borrowed(&pcomposition)).into()
        }
        Self { base__: ITfContextComposition_Vtbl::new::<Identity, OFFSET>(), TerminateComposition: TerminateComposition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionServices as windows_core::Interface>::IID || iid == &<ITfContextComposition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfContextOwnerCompositionServices {}
windows_core::imp::define_interface!(ITfContextOwnerCompositionSink, ITfContextOwnerCompositionSink_Vtbl, 0x5f20aa40_b57a_4f34_96ab_3576f377cc79);
windows_core::imp::interface_hierarchy!(ITfContextOwnerCompositionSink, windows_core::IUnknown);
impl ITfContextOwnerCompositionSink {
    pub unsafe fn OnStartComposition<P0>(&self, pcomposition: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITfCompositionView>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnStartComposition)(windows_core::Interface::as_raw(self), pcomposition.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn OnUpdateComposition<P0, P1>(&self, pcomposition: P0, prangenew: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfCompositionView>,
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).OnUpdateComposition)(windows_core::Interface::as_raw(self), pcomposition.param().abi(), prangenew.param().abi()).ok()
    }
    pub unsafe fn OnEndComposition<P0>(&self, pcomposition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfCompositionView>,
    {
        (windows_core::Interface::vtable(self).OnEndComposition)(windows_core::Interface::as_raw(self), pcomposition.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfContextOwnerCompositionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStartComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnUpdateComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfContextOwnerCompositionSink_Impl: windows_core::IUnknownImpl {
    fn OnStartComposition(&self, pcomposition: Option<&ITfCompositionView>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnUpdateComposition(&self, pcomposition: Option<&ITfCompositionView>, prangenew: Option<&ITfRange>) -> windows_core::Result<()>;
    fn OnEndComposition(&self, pcomposition: Option<&ITfCompositionView>) -> windows_core::Result<()>;
}
impl ITfContextOwnerCompositionSink_Vtbl {
    pub const fn new<Identity: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStartComposition<Identity: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void, pfok: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwnerCompositionSink_Impl::OnStartComposition(this, windows_core::from_raw_borrowed(&pcomposition)) {
                Ok(ok__) => {
                    pfok.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUpdateComposition<Identity: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void, prangenew: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerCompositionSink_Impl::OnUpdateComposition(this, windows_core::from_raw_borrowed(&pcomposition), windows_core::from_raw_borrowed(&prangenew)).into()
        }
        unsafe extern "system" fn OnEndComposition<Identity: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerCompositionSink_Impl::OnEndComposition(this, windows_core::from_raw_borrowed(&pcomposition)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartComposition: OnStartComposition::<Identity, OFFSET>,
            OnUpdateComposition: OnUpdateComposition::<Identity, OFFSET>,
            OnEndComposition: OnEndComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfContextOwnerCompositionSink {}
windows_core::imp::define_interface!(ITfContextOwnerServices, ITfContextOwnerServices_Vtbl, 0xb23eb630_3e1c_11d3_a745_0050040ab407);
windows_core::imp::interface_hierarchy!(ITfContextOwnerServices, windows_core::IUnknown);
impl ITfContextOwnerServices {
    pub unsafe fn OnLayoutChange(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLayoutChange)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttributeChange(&self, rguidattribute: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAttributeChange)(windows_core::Interface::as_raw(self), rguidattribute).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0, P1, P3>(&self, pprop: P0, prange: P1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfProperty>,
        P1: windows_core::Param<ITfRange>,
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pprop.param().abi(), prange.param().abi(), core::mem::transmute(phdr), pstream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Unserialize<P0, P2, P3>(&self, pprop: P0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P2, ploader: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfProperty>,
        P2: windows_core::Param<super::super::System::Com::IStream>,
        P3: windows_core::Param<ITfPersistentPropertyLoaderACP>,
    {
        (windows_core::Interface::vtable(self).Unserialize)(windows_core::Interface::as_raw(self), pprop.param().abi(), phdr, pstream.param().abi(), ploader.param().abi()).ok()
    }
    pub unsafe fn ForceLoadProperty<P0>(&self, pprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfProperty>,
    {
        (windows_core::Interface::vtable(self).ForceLoadProperty)(windows_core::Interface::as_raw(self), pprop.param().abi()).ok()
    }
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRange)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfContextOwnerServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnAttributeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Unserialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Unserialize: usize,
    pub ForceLoadProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfContextOwnerServices_Impl: windows_core::IUnknownImpl {
    fn OnLayoutChange(&self) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OnAttributeChange(&self, rguidattribute: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Serialize(&self, pprop: Option<&ITfProperty>, prange: Option<&ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Unserialize(&self, pprop: Option<&ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Option<&super::super::System::Com::IStream>, ploader: Option<&ITfPersistentPropertyLoaderACP>) -> windows_core::Result<()>;
    fn ForceLoadProperty(&self, pprop: Option<&ITfProperty>) -> windows_core::Result<()>;
    fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfContextOwnerServices_Vtbl {
    pub const fn new<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLayoutChange<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::OnLayoutChange(this).into()
        }
        unsafe extern "system" fn OnStatusChange<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnAttributeChange<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidattribute: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::OnAttributeChange(this, core::mem::transmute_copy(&rguidattribute)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::Serialize(this, windows_core::from_raw_borrowed(&pprop), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&phdr), windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Unserialize<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void, ploader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::Unserialize(this, windows_core::from_raw_borrowed(&pprop), core::mem::transmute_copy(&phdr), windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&ploader)).into()
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextOwnerServices_Impl::ForceLoadProperty(this, windows_core::from_raw_borrowed(&pprop)).into()
        }
        unsafe extern "system" fn CreateRange<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextOwnerServices_Impl::CreateRange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLayoutChange: OnLayoutChange::<Identity, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            OnAttributeChange: OnAttributeChange::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
            Unserialize: Unserialize::<Identity, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, OFFSET>,
            CreateRange: CreateRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfContextOwnerServices {}
windows_core::imp::define_interface!(ITfContextView, ITfContextView_Vtbl, 0x2433bf8e_0f9b_435c_ba2c_180611978c30);
windows_core::imp::interface_hierarchy!(ITfContextView, windows_core::IUnknown);
impl ITfContextView {
    pub unsafe fn GetRangeFromPoint(&self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<ITfRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRangeFromPoint)(windows_core::Interface::as_raw(self), ec, ppt, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTextExt<P1>(&self, ec: u32, prange: P1, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), core::mem::transmute(prc), core::mem::transmute(pfclipped)).ok()
    }
    pub unsafe fn GetScreenExt(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetWnd(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfContextView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::POINT, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::Foundation::RECT, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait ITfContextView_Impl: windows_core::IUnknownImpl {
    fn GetRangeFromPoint(&self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> windows_core::Result<ITfRange>;
    fn GetTextExt(&self, ec: u32, prange: Option<&ITfRange>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(&self) -> windows_core::Result<super::super::Foundation::HWND>;
}
impl ITfContextView_Vtbl {
    pub const fn new<Identity: ITfContextView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRangeFromPoint<Identity: ITfContextView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextView_Impl::GetRangeFromPoint(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ppt), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITfContextView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfContextView_Impl::GetTextExt(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITfContextView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextView_Impl::GetScreenExt(this) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITfContextView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfContextView_Impl::GetWnd(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRangeFromPoint: GetRangeFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextView as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfContextView {}
windows_core::imp::define_interface!(ITfCreatePropertyStore, ITfCreatePropertyStore_Vtbl, 0x2463fbf0_b0af_11d2_afc5_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfCreatePropertyStore, windows_core::IUnknown);
impl ITfCreatePropertyStore {
    pub unsafe fn IsStoreSerializable<P1, P2>(&self, guidprop: *const windows_core::GUID, prange: P1, ppropstore: P2) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P1: windows_core::Param<ITfRange>,
        P2: windows_core::Param<ITfPropertyStore>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsStoreSerializable)(windows_core::Interface::as_raw(self), guidprop, prange.param().abi(), ppropstore.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyStore<P1, P3>(&self, guidprop: *const windows_core::GUID, prange: P1, cb: u32, pstream: P3) -> windows_core::Result<ITfPropertyStore>
    where
        P1: windows_core::Param<ITfRange>,
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePropertyStore)(windows_core::Interface::as_raw(self), guidprop, prange.param().abi(), cb, pstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfCreatePropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsStoreSerializable: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyStore: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfCreatePropertyStore_Impl: windows_core::IUnknownImpl {
    fn IsStoreSerializable(&self, guidprop: *const windows_core::GUID, prange: Option<&ITfRange>, ppropstore: Option<&ITfPropertyStore>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CreatePropertyStore(&self, guidprop: *const windows_core::GUID, prange: Option<&ITfRange>, cb: u32, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<ITfPropertyStore>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfCreatePropertyStore_Vtbl {
    pub const fn new<Identity: ITfCreatePropertyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsStoreSerializable<Identity: ITfCreatePropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, prange: *mut core::ffi::c_void, ppropstore: *mut core::ffi::c_void, pfserializable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCreatePropertyStore_Impl::IsStoreSerializable(this, core::mem::transmute_copy(&guidprop), windows_core::from_raw_borrowed(&prange), windows_core::from_raw_borrowed(&ppropstore)) {
                Ok(ok__) => {
                    pfserializable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyStore<Identity: ITfCreatePropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, prange: *mut core::ffi::c_void, cb: u32, pstream: *mut core::ffi::c_void, ppstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfCreatePropertyStore_Impl::CreatePropertyStore(this, core::mem::transmute_copy(&guidprop), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&cb), windows_core::from_raw_borrowed(&pstream)) {
                Ok(ok__) => {
                    ppstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStoreSerializable: IsStoreSerializable::<Identity, OFFSET>,
            CreatePropertyStore: CreatePropertyStore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCreatePropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfCreatePropertyStore {}
windows_core::imp::define_interface!(ITfDisplayAttributeInfo, ITfDisplayAttributeInfo_Vtbl, 0x70528852_2f26_4aea_8c96_215150578932);
windows_core::imp::interface_hierarchy!(ITfDisplayAttributeInfo, windows_core::IUnknown);
impl ITfDisplayAttributeInfo {
    pub unsafe fn GetGUID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetAttributeInfo(&self, pda: *mut TF_DISPLAYATTRIBUTE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributeInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pda)).ok()
    }
    pub unsafe fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAttributeInfo)(windows_core::Interface::as_raw(self), pda).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfDisplayAttributeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT,
    pub SetAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfDisplayAttributeInfo_Impl: windows_core::IUnknownImpl {
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAttributeInfo(&self, pda: *mut TF_DISPLAYATTRIBUTE) -> windows_core::Result<()>;
    fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl ITfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGUID<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeInfo_Impl::GetGUID(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeInfo_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstrdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeInfo_Impl::GetAttributeInfo(this, core::mem::transmute_copy(&pda)).into()
        }
        unsafe extern "system" fn SetAttributeInfo<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeInfo_Impl::SetAttributeInfo(this, core::mem::transmute_copy(&pda)).into()
        }
        unsafe extern "system" fn Reset<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeInfo_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGUID: GetGUID::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Identity, OFFSET>,
            SetAttributeInfo: SetAttributeInfo::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfDisplayAttributeInfo {}
windows_core::imp::define_interface!(ITfDisplayAttributeMgr, ITfDisplayAttributeMgr_Vtbl, 0x8ded7393_5db1_475c_9e71_a39111b0ff67);
windows_core::imp::interface_hierarchy!(ITfDisplayAttributeMgr, windows_core::IUnknown);
impl ITfDisplayAttributeMgr {
    pub unsafe fn OnUpdateInfo(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnUpdateInfo)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDisplayAttributeInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID, ppinfo: *mut Option<ITfDisplayAttributeInfo>, pclsidowner: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDisplayAttributeInfo)(windows_core::Interface::as_raw(self), guid, core::mem::transmute(ppinfo), core::mem::transmute(pclsidowner)).ok()
    }
}
#[repr(C)]
pub struct ITfDisplayAttributeMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUpdateInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDisplayAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITfDisplayAttributeMgr_Impl: windows_core::IUnknownImpl {
    fn OnUpdateInfo(&self) -> windows_core::Result<()>;
    fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID, ppinfo: *mut Option<ITfDisplayAttributeInfo>, pclsidowner: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl ITfDisplayAttributeMgr_Vtbl {
    pub const fn new<Identity: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdateInfo<Identity: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeMgr_Impl::OnUpdateInfo(this).into()
        }
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeMgr_Impl::EnumDisplayAttributeInfo(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, ppinfo: *mut *mut core::ffi::c_void, pclsidowner: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeMgr_Impl::GetDisplayAttributeInfo(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&ppinfo), core::mem::transmute_copy(&pclsidowner)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdateInfo: OnUpdateInfo::<Identity, OFFSET>,
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfDisplayAttributeMgr {}
windows_core::imp::define_interface!(ITfDisplayAttributeNotifySink, ITfDisplayAttributeNotifySink_Vtbl, 0xad56f402_e162_4f25_908f_7d577cf9bda9);
windows_core::imp::interface_hierarchy!(ITfDisplayAttributeNotifySink, windows_core::IUnknown);
impl ITfDisplayAttributeNotifySink {
    pub unsafe fn OnUpdateInfo(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnUpdateInfo)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfDisplayAttributeNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUpdateInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfDisplayAttributeNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnUpdateInfo(&self) -> windows_core::Result<()>;
}
impl ITfDisplayAttributeNotifySink_Vtbl {
    pub const fn new<Identity: ITfDisplayAttributeNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdateInfo<Identity: ITfDisplayAttributeNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDisplayAttributeNotifySink_Impl::OnUpdateInfo(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdateInfo: OnUpdateInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfDisplayAttributeNotifySink {}
windows_core::imp::define_interface!(ITfDisplayAttributeProvider, ITfDisplayAttributeProvider_Vtbl, 0xfee47777_163c_4769_996a_6e9c50ad8f54);
windows_core::imp::interface_hierarchy!(ITfDisplayAttributeProvider, windows_core::IUnknown);
impl ITfDisplayAttributeProvider {
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDisplayAttributeInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITfDisplayAttributeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayAttributeInfo)(windows_core::Interface::as_raw(self), guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfDisplayAttributeProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumDisplayAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfDisplayAttributeProvider_Impl: windows_core::IUnknownImpl {
    fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITfDisplayAttributeInfo>;
}
impl ITfDisplayAttributeProvider_Vtbl {
    pub const fn new<Identity: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeProvider_Impl::EnumDisplayAttributeInfo(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDisplayAttributeProvider_Impl::GetDisplayAttributeInfo(this, core::mem::transmute_copy(&guid)) {
                Ok(ok__) => {
                    ppinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfDisplayAttributeProvider {}
windows_core::imp::define_interface!(ITfDocumentMgr, ITfDocumentMgr_Vtbl, 0xaa80e7f4_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfDocumentMgr, windows_core::IUnknown);
impl ITfDocumentMgr {
    pub unsafe fn CreateContext<P2>(&self, tidowner: u32, dwflags: u32, punk: P2, ppic: *mut Option<ITfContext>, pectextstore: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateContext)(windows_core::Interface::as_raw(self), tidowner, dwflags, punk.param().abi(), core::mem::transmute(ppic), core::mem::transmute(pectextstore)).ok()
    }
    pub unsafe fn Push<P0>(&self, pic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfContext>,
    {
        (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), pic.param().abi()).ok()
    }
    pub unsafe fn Pop(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pop)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetTop(&self) -> windows_core::Result<ITfContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTop)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBase(&self) -> windows_core::Result<ITfContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBase)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumContexts(&self) -> windows_core::Result<IEnumTfContexts> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumContexts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfDocumentMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pop: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumContexts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfDocumentMgr_Impl: windows_core::IUnknownImpl {
    fn CreateContext(&self, tidowner: u32, dwflags: u32, punk: Option<&windows_core::IUnknown>, ppic: *mut Option<ITfContext>, pectextstore: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
    fn Pop(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetTop(&self) -> windows_core::Result<ITfContext>;
    fn GetBase(&self) -> windows_core::Result<ITfContext>;
    fn EnumContexts(&self) -> windows_core::Result<IEnumTfContexts>;
}
impl ITfDocumentMgr_Vtbl {
    pub const fn new<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateContext<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tidowner: u32, dwflags: u32, punk: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void, pectextstore: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDocumentMgr_Impl::CreateContext(this, core::mem::transmute_copy(&tidowner), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&ppic), core::mem::transmute_copy(&pectextstore)).into()
        }
        unsafe extern "system" fn Push<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDocumentMgr_Impl::Push(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        unsafe extern "system" fn Pop<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfDocumentMgr_Impl::Pop(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetTop<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDocumentMgr_Impl::GetTop(this) {
                Ok(ok__) => {
                    ppic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBase<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDocumentMgr_Impl::GetBase(this) {
                Ok(ok__) => {
                    ppic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumContexts<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfDocumentMgr_Impl::EnumContexts(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateContext: CreateContext::<Identity, OFFSET>,
            Push: Push::<Identity, OFFSET>,
            Pop: Pop::<Identity, OFFSET>,
            GetTop: GetTop::<Identity, OFFSET>,
            GetBase: GetBase::<Identity, OFFSET>,
            EnumContexts: EnumContexts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDocumentMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfDocumentMgr {}
windows_core::imp::define_interface!(ITfEditRecord, ITfEditRecord_Vtbl, 0x42d4d099_7c1a_4a89_b836_6c6f22160df0);
windows_core::imp::interface_hierarchy!(ITfEditRecord, windows_core::IUnknown);
impl ITfEditRecord {
    pub unsafe fn GetSelectionStatus(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelectionStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTextAndPropertyUpdates(&self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: &[*const windows_core::GUID]) -> windows_core::Result<IEnumTfRanges> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTextAndPropertyUpdates)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(prgproperties.as_ptr()), prgproperties.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfEditRecord_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetTextAndPropertyUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, *const *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfEditRecord_Impl: windows_core::IUnknownImpl {
    fn GetSelectionStatus(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetTextAndPropertyUpdates(&self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const windows_core::GUID, cproperties: u32) -> windows_core::Result<IEnumTfRanges>;
}
impl ITfEditRecord_Vtbl {
    pub const fn new<Identity: ITfEditRecord_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelectionStatus<Identity: ITfEditRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfchanged: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfEditRecord_Impl::GetSelectionStatus(this) {
                Ok(ok__) => {
                    pfchanged.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextAndPropertyUpdates<Identity: ITfEditRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const windows_core::GUID, cproperties: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfEditRecord_Impl::GetTextAndPropertyUpdates(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&prgproperties), core::mem::transmute_copy(&cproperties)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelectionStatus: GetSelectionStatus::<Identity, OFFSET>,
            GetTextAndPropertyUpdates: GetTextAndPropertyUpdates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfEditRecord as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfEditRecord {}
windows_core::imp::define_interface!(ITfEditSession, ITfEditSession_Vtbl, 0xaa80e803_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfEditSession, windows_core::IUnknown);
impl ITfEditSession {
    pub unsafe fn DoEditSession(&self, ec: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoEditSession)(windows_core::Interface::as_raw(self), ec).ok()
    }
}
#[repr(C)]
pub struct ITfEditSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoEditSession: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfEditSession_Impl: windows_core::IUnknownImpl {
    fn DoEditSession(&self, ec: u32) -> windows_core::Result<()>;
}
impl ITfEditSession_Vtbl {
    pub const fn new<Identity: ITfEditSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoEditSession<Identity: ITfEditSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfEditSession_Impl::DoEditSession(this, core::mem::transmute_copy(&ec)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoEditSession: DoEditSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfEditSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfEditSession {}
windows_core::imp::define_interface!(ITfEditTransactionSink, ITfEditTransactionSink_Vtbl, 0x708fbf70_b520_416b_b06c_2c41ab44f8ba);
windows_core::imp::interface_hierarchy!(ITfEditTransactionSink, windows_core::IUnknown);
impl ITfEditTransactionSink {
    pub unsafe fn OnStartEditTransaction<P0>(&self, pic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfContext>,
    {
        (windows_core::Interface::vtable(self).OnStartEditTransaction)(windows_core::Interface::as_raw(self), pic.param().abi()).ok()
    }
    pub unsafe fn OnEndEditTransaction<P0>(&self, pic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfContext>,
    {
        (windows_core::Interface::vtable(self).OnEndEditTransaction)(windows_core::Interface::as_raw(self), pic.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfEditTransactionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStartEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfEditTransactionSink_Impl: windows_core::IUnknownImpl {
    fn OnStartEditTransaction(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
}
impl ITfEditTransactionSink_Vtbl {
    pub const fn new<Identity: ITfEditTransactionSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStartEditTransaction<Identity: ITfEditTransactionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfEditTransactionSink_Impl::OnStartEditTransaction(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ITfEditTransactionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfEditTransactionSink_Impl::OnEndEditTransaction(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartEditTransaction: OnStartEditTransaction::<Identity, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfEditTransactionSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfEditTransactionSink {}
windows_core::imp::define_interface!(ITfFnAdviseText, ITfFnAdviseText_Vtbl, 0x3527268b_7d53_4dd9_92b7_7296ae461249);
impl core::ops::Deref for ITfFnAdviseText {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnAdviseText, windows_core::IUnknown, ITfFunction);
impl ITfFnAdviseText {
    pub unsafe fn OnTextUpdate<P0>(&self, prange: P0, pchtext: &[u16]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).OnTextUpdate)(windows_core::Interface::as_raw(self), prange.param().abi(), core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap()).ok()
    }
    pub unsafe fn OnLatticeUpdate<P0, P1>(&self, prange: P0, plattice: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfRange>,
        P1: windows_core::Param<ITfLMLattice>,
    {
        (windows_core::Interface::vtable(self).OnLatticeUpdate)(windows_core::Interface::as_raw(self), prange.param().abi(), plattice.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfFnAdviseText_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub OnTextUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub OnLatticeUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnAdviseText_Impl: ITfFunction_Impl {
    fn OnTextUpdate(&self, prange: Option<&ITfRange>, pchtext: &windows_core::PCWSTR, cch: i32) -> windows_core::Result<()>;
    fn OnLatticeUpdate(&self, prange: Option<&ITfRange>, plattice: Option<&ITfLMLattice>) -> windows_core::Result<()>;
}
impl ITfFnAdviseText_Vtbl {
    pub const fn new<Identity: ITfFnAdviseText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTextUpdate<Identity: ITfFnAdviseText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pchtext: windows_core::PCWSTR, cch: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnAdviseText_Impl::OnTextUpdate(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn OnLatticeUpdate<Identity: ITfFnAdviseText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, plattice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnAdviseText_Impl::OnLatticeUpdate(this, windows_core::from_raw_borrowed(&prange), windows_core::from_raw_borrowed(&plattice)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            OnTextUpdate: OnTextUpdate::<Identity, OFFSET>,
            OnLatticeUpdate: OnLatticeUpdate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnAdviseText as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnAdviseText {}
windows_core::imp::define_interface!(ITfFnBalloon, ITfFnBalloon_Vtbl, 0x3bab89e4_5fbe_45f4_a5bc_dca36ad225a8);
windows_core::imp::interface_hierarchy!(ITfFnBalloon, windows_core::IUnknown);
impl ITfFnBalloon {
    pub unsafe fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateBalloon)(windows_core::Interface::as_raw(self), style, core::mem::transmute(pch.as_ptr()), pch.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct ITfFnBalloon_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UpdateBalloon: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBBalloonStyle, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
pub trait ITfFnBalloon_Impl: windows_core::IUnknownImpl {
    fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
}
impl ITfFnBalloon_Vtbl {
    pub const fn new<Identity: ITfFnBalloon_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateBalloon<Identity: ITfFnBalloon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, style: TfLBBalloonStyle, pch: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnBalloon_Impl::UpdateBalloon(this, core::mem::transmute_copy(&style), core::mem::transmute(&pch), core::mem::transmute_copy(&cch)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), UpdateBalloon: UpdateBalloon::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnBalloon as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnBalloon {}
windows_core::imp::define_interface!(ITfFnConfigure, ITfFnConfigure_Vtbl, 0x88f567c6_1757_49f8_a1b2_89234c1eeff9);
impl core::ops::Deref for ITfFnConfigure {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnConfigure, windows_core::IUnknown, ITfFunction);
impl ITfFnConfigure {
    pub unsafe fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), hwndparent, langid, rguidprofile).ok()
    }
}
#[repr(C)]
pub struct ITfFnConfigure_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u16, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITfFnConfigure_Impl: ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ITfFnConfigure_Vtbl {
    pub const fn new<Identity: ITfFnConfigure_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: ITfFnConfigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnConfigure_Impl::Show(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&rguidprofile)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnConfigure as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnConfigure {}
windows_core::imp::define_interface!(ITfFnConfigureRegisterEudc, ITfFnConfigureRegisterEudc_Vtbl, 0xb5e26ff5_d7ad_4304_913f_21a2ed95a1b0);
impl core::ops::Deref for ITfFnConfigureRegisterEudc {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnConfigureRegisterEudc, windows_core::IUnknown, ITfFunction);
impl ITfFnConfigureRegisterEudc {
    pub unsafe fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: &windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), hwndparent, langid, rguidprofile, core::mem::transmute_copy(bstrregistered)).ok()
    }
}
#[repr(C)]
pub struct ITfFnConfigureRegisterEudc_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u16, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnConfigureRegisterEudc_Impl: ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl ITfFnConfigureRegisterEudc_Vtbl {
    pub const fn new<Identity: ITfFnConfigureRegisterEudc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: ITfFnConfigureRegisterEudc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnConfigureRegisterEudc_Impl::Show(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&rguidprofile), core::mem::transmute(&bstrregistered)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterEudc as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnConfigureRegisterEudc {}
windows_core::imp::define_interface!(ITfFnConfigureRegisterWord, ITfFnConfigureRegisterWord_Vtbl, 0xbb95808a_6d8f_4bca_8400_5390b586aedf);
impl core::ops::Deref for ITfFnConfigureRegisterWord {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnConfigureRegisterWord, windows_core::IUnknown, ITfFunction);
impl ITfFnConfigureRegisterWord {
    pub unsafe fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: &windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), hwndparent, langid, rguidprofile, core::mem::transmute_copy(bstrregistered)).ok()
    }
}
#[repr(C)]
pub struct ITfFnConfigureRegisterWord_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u16, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnConfigureRegisterWord_Impl: ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl ITfFnConfigureRegisterWord_Vtbl {
    pub const fn new<Identity: ITfFnConfigureRegisterWord_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: ITfFnConfigureRegisterWord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const windows_core::GUID, bstrregistered: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnConfigureRegisterWord_Impl::Show(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&rguidprofile), core::mem::transmute(&bstrregistered)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnConfigureRegisterWord as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnConfigureRegisterWord {}
windows_core::imp::define_interface!(ITfFnCustomSpeechCommand, ITfFnCustomSpeechCommand_Vtbl, 0xfca6c349_a12f_43a3_8dd6_5a5a4282577b);
impl core::ops::Deref for ITfFnCustomSpeechCommand {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnCustomSpeechCommand, windows_core::IUnknown, ITfFunction);
impl ITfFnCustomSpeechCommand {
    pub unsafe fn SetSpeechCommandProvider<P0>(&self, pspcmdprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetSpeechCommandProvider)(windows_core::Interface::as_raw(self), pspcmdprovider.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfFnCustomSpeechCommand_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub SetSpeechCommandProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnCustomSpeechCommand_Impl: ITfFunction_Impl {
    fn SetSpeechCommandProvider(&self, pspcmdprovider: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ITfFnCustomSpeechCommand_Vtbl {
    pub const fn new<Identity: ITfFnCustomSpeechCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSpeechCommandProvider<Identity: ITfFnCustomSpeechCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspcmdprovider: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnCustomSpeechCommand_Impl::SetSpeechCommandProvider(this, windows_core::from_raw_borrowed(&pspcmdprovider)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), SetSpeechCommandProvider: SetSpeechCommandProvider::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnCustomSpeechCommand as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnCustomSpeechCommand {}
windows_core::imp::define_interface!(ITfFnGetLinguisticAlternates, ITfFnGetLinguisticAlternates_Vtbl, 0xea163ce2_7a65_4506_82a3_c528215da64e);
impl core::ops::Deref for ITfFnGetLinguisticAlternates {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnGetLinguisticAlternates, windows_core::IUnknown, ITfFunction);
impl ITfFnGetLinguisticAlternates {
    pub unsafe fn GetAlternates<P0>(&self, prange: P0) -> windows_core::Result<ITfCandidateList>
    where
        P0: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAlternates)(windows_core::Interface::as_raw(self), prange.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfFnGetLinguisticAlternates_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub GetAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnGetLinguisticAlternates_Impl: ITfFunction_Impl {
    fn GetAlternates(&self, prange: Option<&ITfRange>) -> windows_core::Result<ITfCandidateList>;
}
impl ITfFnGetLinguisticAlternates_Vtbl {
    pub const fn new<Identity: ITfFnGetLinguisticAlternates_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAlternates<Identity: ITfFnGetLinguisticAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppcandidatelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnGetLinguisticAlternates_Impl::GetAlternates(this, windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    ppcandidatelist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), GetAlternates: GetAlternates::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnGetLinguisticAlternates as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnGetLinguisticAlternates {}
windows_core::imp::define_interface!(ITfFnGetPreferredTouchKeyboardLayout, ITfFnGetPreferredTouchKeyboardLayout_Vtbl, 0x5f309a41_590a_4acc_a97f_d8efff13fdfc);
impl core::ops::Deref for ITfFnGetPreferredTouchKeyboardLayout {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnGetPreferredTouchKeyboardLayout, windows_core::IUnknown, ITfFunction);
impl ITfFnGetPreferredTouchKeyboardLayout {
    pub unsafe fn GetLayout(&self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(ptkblayouttype), pwpreferredlayoutid).ok()
    }
}
#[repr(C)]
pub struct ITfFnGetPreferredTouchKeyboardLayout_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub GetLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TKBLayoutType, *const u16) -> windows_core::HRESULT,
}
pub trait ITfFnGetPreferredTouchKeyboardLayout_Impl: ITfFunction_Impl {
    fn GetLayout(&self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> windows_core::Result<()>;
}
impl ITfFnGetPreferredTouchKeyboardLayout_Vtbl {
    pub const fn new<Identity: ITfFnGetPreferredTouchKeyboardLayout_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLayout<Identity: ITfFnGetPreferredTouchKeyboardLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnGetPreferredTouchKeyboardLayout_Impl::GetLayout(this, core::mem::transmute_copy(&ptkblayouttype), core::mem::transmute_copy(&pwpreferredlayoutid)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), GetLayout: GetLayout::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnGetPreferredTouchKeyboardLayout as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnGetPreferredTouchKeyboardLayout {}
windows_core::imp::define_interface!(ITfFnGetSAPIObject, ITfFnGetSAPIObject_Vtbl, 0x5c0ab7ea_167d_4f59_bfb5_4693755e90ca);
impl core::ops::Deref for ITfFnGetSAPIObject {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnGetSAPIObject, windows_core::IUnknown, ITfFunction);
impl ITfFnGetSAPIObject {
    pub unsafe fn Get(&self, sobj: TfSapiObject) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), sobj, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfFnGetSAPIObject_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, TfSapiObject, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnGetSAPIObject_Impl: ITfFunction_Impl {
    fn Get(&self, sobj: TfSapiObject) -> windows_core::Result<windows_core::IUnknown>;
}
impl ITfFnGetSAPIObject_Vtbl {
    pub const fn new<Identity: ITfFnGetSAPIObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: ITfFnGetSAPIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sobj: TfSapiObject, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnGetSAPIObject_Impl::Get(this, core::mem::transmute_copy(&sobj)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnGetSAPIObject as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnGetSAPIObject {}
windows_core::imp::define_interface!(ITfFnLMInternal, ITfFnLMInternal_Vtbl, 0x04b825b1_ac9a_4f7b_b5ad_c7168f1ee445);
impl core::ops::Deref for ITfFnLMInternal {
    type Target = ITfFnLMProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnLMInternal, windows_core::IUnknown, ITfFunction, ITfFnLMProcessor);
impl ITfFnLMInternal {
    pub unsafe fn ProcessLattice<P0>(&self, prange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).ProcessLattice)(windows_core::Interface::as_raw(self), prange.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfFnLMInternal_Vtbl {
    pub base__: ITfFnLMProcessor_Vtbl,
    pub ProcessLattice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnLMInternal_Impl: ITfFnLMProcessor_Impl {
    fn ProcessLattice(&self, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
impl ITfFnLMInternal_Vtbl {
    pub const fn new<Identity: ITfFnLMInternal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessLattice<Identity: ITfFnLMInternal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMInternal_Impl::ProcessLattice(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        Self { base__: ITfFnLMProcessor_Vtbl::new::<Identity, OFFSET>(), ProcessLattice: ProcessLattice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnLMInternal as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID || iid == &<ITfFnLMProcessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnLMInternal {}
windows_core::imp::define_interface!(ITfFnLMProcessor, ITfFnLMProcessor_Vtbl, 0x7afbf8e7_ac4b_4082_b058_890899d3a010);
impl core::ops::Deref for ITfFnLMProcessor {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnLMProcessor, windows_core::IUnknown, ITfFunction);
impl ITfFnLMProcessor {
    pub unsafe fn QueryRange<P0>(&self, prange: P0, ppnewrange: *mut Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).QueryRange)(windows_core::Interface::as_raw(self), prange.param().abi(), core::mem::transmute(ppnewrange), core::mem::transmute(pfaccepted)).ok()
    }
    pub unsafe fn QueryLangID(&self, langid: u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryLangID)(windows_core::Interface::as_raw(self), langid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetReconversion<P0>(&self, prange: P0) -> windows_core::Result<ITfCandidateList>
    where
        P0: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReconversion)(windows_core::Interface::as_raw(self), prange.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reconvert<P0>(&self, prange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).Reconvert)(windows_core::Interface::as_raw(self), prange.param().abi()).ok()
    }
    pub unsafe fn QueryKey(&self, fup: bool, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryKey)(windows_core::Interface::as_raw(self), fup.into(), vkey, lparamkeydata, &mut result__).map(|| result__)
    }
    pub unsafe fn InvokeKey(&self, fup: bool, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InvokeKey)(windows_core::Interface::as_raw(self), fup.into(), vkey, lparamkeydata).ok()
    }
    pub unsafe fn InvokeFunc<P0>(&self, pic: P0, refguidfunc: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfContext>,
    {
        (windows_core::Interface::vtable(self).InvokeFunc)(windows_core::Interface::as_raw(self), pic.param().abi(), refguidfunc).ok()
    }
}
#[repr(C)]
pub struct ITfFnLMProcessor_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub QueryRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub QueryLangID: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetReconversion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reconvert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub InvokeKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub InvokeFunc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITfFnLMProcessor_Impl: ITfFunction_Impl {
    fn QueryRange(&self, prange: Option<&ITfRange>, ppnewrange: *mut Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn QueryLangID(&self, langid: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetReconversion(&self, prange: Option<&ITfRange>) -> windows_core::Result<ITfCandidateList>;
    fn Reconvert(&self, prange: Option<&ITfRange>) -> windows_core::Result<()>;
    fn QueryKey(&self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InvokeKey(&self, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn InvokeFunc(&self, pic: Option<&ITfContext>, refguidfunc: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ITfFnLMProcessor_Vtbl {
    pub const fn new<Identity: ITfFnLMProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryRange<Identity: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppnewrange: *mut *mut core::ffi::c_void, pfaccepted: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMProcessor_Impl::QueryRange(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&ppnewrange), core::mem::transmute_copy(&pfaccepted)).into()
        }
        unsafe extern "system" fn QueryLangID<Identity: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnLMProcessor_Impl::QueryLangID(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    pfaccepted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReconversion<Identity: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppcandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnLMProcessor_Impl::GetReconversion(this, windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    ppcandlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Identity: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMProcessor_Impl::Reconvert(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        unsafe extern "system" fn QueryKey<Identity: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnLMProcessor_Impl::QueryKey(this, core::mem::transmute_copy(&fup), core::mem::transmute_copy(&vkey), core::mem::transmute_copy(&lparamkeydata)) {
                Ok(ok__) => {
                    pfinterested.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeKey<Identity: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMProcessor_Impl::InvokeKey(this, core::mem::transmute_copy(&fup), core::mem::transmute_copy(&vkey), core::mem::transmute_copy(&lparamkeydata)).into()
        }
        unsafe extern "system" fn InvokeFunc<Identity: ITfFnLMProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, refguidfunc: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLMProcessor_Impl::InvokeFunc(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&refguidfunc)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            QueryRange: QueryRange::<Identity, OFFSET>,
            QueryLangID: QueryLangID::<Identity, OFFSET>,
            GetReconversion: GetReconversion::<Identity, OFFSET>,
            Reconvert: Reconvert::<Identity, OFFSET>,
            QueryKey: QueryKey::<Identity, OFFSET>,
            InvokeKey: InvokeKey::<Identity, OFFSET>,
            InvokeFunc: InvokeFunc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnLMProcessor as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnLMProcessor {}
windows_core::imp::define_interface!(ITfFnLangProfileUtil, ITfFnLangProfileUtil_Vtbl, 0xa87a8574_a6c1_4e15_99f0_3d3965f548eb);
impl core::ops::Deref for ITfFnLangProfileUtil {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnLangProfileUtil, windows_core::IUnknown, ITfFunction);
impl ITfFnLangProfileUtil {
    pub unsafe fn RegisterActiveProfiles(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterActiveProfiles)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsProfileAvailableForLang(&self, langid: u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsProfileAvailableForLang)(windows_core::Interface::as_raw(self), langid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfFnLangProfileUtil_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub RegisterActiveProfiles: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsProfileAvailableForLang: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ITfFnLangProfileUtil_Impl: ITfFunction_Impl {
    fn RegisterActiveProfiles(&self) -> windows_core::Result<()>;
    fn IsProfileAvailableForLang(&self, langid: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl ITfFnLangProfileUtil_Vtbl {
    pub const fn new<Identity: ITfFnLangProfileUtil_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterActiveProfiles<Identity: ITfFnLangProfileUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnLangProfileUtil_Impl::RegisterActiveProfiles(this).into()
        }
        unsafe extern "system" fn IsProfileAvailableForLang<Identity: ITfFnLangProfileUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnLangProfileUtil_Impl::IsProfileAvailableForLang(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    pfavailable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            RegisterActiveProfiles: RegisterActiveProfiles::<Identity, OFFSET>,
            IsProfileAvailableForLang: IsProfileAvailableForLang::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnLangProfileUtil as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnLangProfileUtil {}
windows_core::imp::define_interface!(ITfFnPlayBack, ITfFnPlayBack_Vtbl, 0xa3a416a4_0f64_11d3_b5b7_00c04fc324a1);
impl core::ops::Deref for ITfFnPlayBack {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnPlayBack, windows_core::IUnknown, ITfFunction);
impl ITfFnPlayBack {
    pub unsafe fn QueryRange<P0>(&self, prange: P0, ppnewrange: *mut Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).QueryRange)(windows_core::Interface::as_raw(self), prange.param().abi(), core::mem::transmute(ppnewrange), core::mem::transmute(pfplayable)).ok()
    }
    pub unsafe fn Play<P0>(&self, prange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self), prange.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfFnPlayBack_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub QueryRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnPlayBack_Impl: ITfFunction_Impl {
    fn QueryRange(&self, prange: Option<&ITfRange>, ppnewrange: *mut Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Play(&self, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
impl ITfFnPlayBack_Vtbl {
    pub const fn new<Identity: ITfFnPlayBack_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryRange<Identity: ITfFnPlayBack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppnewrange: *mut *mut core::ffi::c_void, pfplayable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnPlayBack_Impl::QueryRange(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&ppnewrange), core::mem::transmute_copy(&pfplayable)).into()
        }
        unsafe extern "system" fn Play<Identity: ITfFnPlayBack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnPlayBack_Impl::Play(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), QueryRange: QueryRange::<Identity, OFFSET>, Play: Play::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnPlayBack as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnPlayBack {}
windows_core::imp::define_interface!(ITfFnPropertyUIStatus, ITfFnPropertyUIStatus_Vtbl, 0x2338ac6e_2b9d_44c0_a75e_ee64f256b3bd);
impl core::ops::Deref for ITfFnPropertyUIStatus {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnPropertyUIStatus, windows_core::IUnknown, ITfFunction);
impl ITfFnPropertyUIStatus {
    pub unsafe fn GetStatus(&self, refguidprop: *const windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), refguidprop, &mut result__).map(|| result__)
    }
    pub unsafe fn SetStatus(&self, refguidprop: *const windows_core::GUID, dw: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), refguidprop, dw).ok()
    }
}
#[repr(C)]
pub struct ITfFnPropertyUIStatus_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
pub trait ITfFnPropertyUIStatus_Impl: ITfFunction_Impl {
    fn GetStatus(&self, refguidprop: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn SetStatus(&self, refguidprop: *const windows_core::GUID, dw: u32) -> windows_core::Result<()>;
}
impl ITfFnPropertyUIStatus_Vtbl {
    pub const fn new<Identity: ITfFnPropertyUIStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: ITfFnPropertyUIStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguidprop: *const windows_core::GUID, pdw: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnPropertyUIStatus_Impl::GetStatus(this, core::mem::transmute_copy(&refguidprop)) {
                Ok(ok__) => {
                    pdw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: ITfFnPropertyUIStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguidprop: *const windows_core::GUID, dw: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnPropertyUIStatus_Impl::SetStatus(this, core::mem::transmute_copy(&refguidprop), core::mem::transmute_copy(&dw)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), GetStatus: GetStatus::<Identity, OFFSET>, SetStatus: SetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnPropertyUIStatus as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnPropertyUIStatus {}
windows_core::imp::define_interface!(ITfFnReconversion, ITfFnReconversion_Vtbl, 0x4cea93c0_0a58_11d3_8df0_00105a2799b5);
impl core::ops::Deref for ITfFnReconversion {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnReconversion, windows_core::IUnknown, ITfFunction);
impl ITfFnReconversion {
    pub unsafe fn QueryRange<P0>(&self, prange: P0, ppnewrange: *mut Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).QueryRange)(windows_core::Interface::as_raw(self), prange.param().abi(), core::mem::transmute(ppnewrange), core::mem::transmute(pfconvertable)).ok()
    }
    pub unsafe fn GetReconversion<P0>(&self, prange: P0) -> windows_core::Result<ITfCandidateList>
    where
        P0: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReconversion)(windows_core::Interface::as_raw(self), prange.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reconvert<P0>(&self, prange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).Reconvert)(windows_core::Interface::as_raw(self), prange.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfFnReconversion_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub QueryRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetReconversion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reconvert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnReconversion_Impl: ITfFunction_Impl {
    fn QueryRange(&self, prange: Option<&ITfRange>, ppnewrange: *mut Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetReconversion(&self, prange: Option<&ITfRange>) -> windows_core::Result<ITfCandidateList>;
    fn Reconvert(&self, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
impl ITfFnReconversion_Vtbl {
    pub const fn new<Identity: ITfFnReconversion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryRange<Identity: ITfFnReconversion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppnewrange: *mut *mut core::ffi::c_void, pfconvertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnReconversion_Impl::QueryRange(this, windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&ppnewrange), core::mem::transmute_copy(&pfconvertable)).into()
        }
        unsafe extern "system" fn GetReconversion<Identity: ITfFnReconversion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, ppcandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnReconversion_Impl::GetReconversion(this, windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    ppcandlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconvert<Identity: ITfFnReconversion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnReconversion_Impl::Reconvert(this, windows_core::from_raw_borrowed(&prange)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            QueryRange: QueryRange::<Identity, OFFSET>,
            GetReconversion: GetReconversion::<Identity, OFFSET>,
            Reconvert: Reconvert::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnReconversion as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnReconversion {}
windows_core::imp::define_interface!(ITfFnSearchCandidateProvider, ITfFnSearchCandidateProvider_Vtbl, 0x87a2ad8f_f27b_4920_8501_67602280175d);
impl core::ops::Deref for ITfFnSearchCandidateProvider {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnSearchCandidateProvider, windows_core::IUnknown, ITfFunction);
impl ITfFnSearchCandidateProvider {
    pub unsafe fn GetSearchCandidates(&self, bstrquery: &windows_core::BSTR, bstrapplicationid: &windows_core::BSTR) -> windows_core::Result<ITfCandidateList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSearchCandidates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrquery), core::mem::transmute_copy(bstrapplicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetResult(&self, bstrquery: &windows_core::BSTR, bstrapplicationid: &windows_core::BSTR, bstrresult: &windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResult)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrquery), core::mem::transmute_copy(bstrapplicationid), core::mem::transmute_copy(bstrresult)).ok()
    }
}
#[repr(C)]
pub struct ITfFnSearchCandidateProvider_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub GetSearchCandidates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFnSearchCandidateProvider_Impl: ITfFunction_Impl {
    fn GetSearchCandidates(&self, bstrquery: &windows_core::BSTR, bstrapplicationid: &windows_core::BSTR) -> windows_core::Result<ITfCandidateList>;
    fn SetResult(&self, bstrquery: &windows_core::BSTR, bstrapplicationid: &windows_core::BSTR, bstrresult: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl ITfFnSearchCandidateProvider_Vtbl {
    pub const fn new<Identity: ITfFnSearchCandidateProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSearchCandidates<Identity: ITfFnSearchCandidateProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrquery: *mut core::ffi::c_void, bstrapplicationid: *mut core::ffi::c_void, pplist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFnSearchCandidateProvider_Impl::GetSearchCandidates(this, core::mem::transmute(&bstrquery), core::mem::transmute(&bstrapplicationid)) {
                Ok(ok__) => {
                    pplist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResult<Identity: ITfFnSearchCandidateProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrquery: *mut core::ffi::c_void, bstrapplicationid: *mut core::ffi::c_void, bstrresult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnSearchCandidateProvider_Impl::SetResult(this, core::mem::transmute(&bstrquery), core::mem::transmute(&bstrapplicationid), core::mem::transmute(&bstrresult)).into()
        }
        Self {
            base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(),
            GetSearchCandidates: GetSearchCandidates::<Identity, OFFSET>,
            SetResult: SetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnSearchCandidateProvider as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnSearchCandidateProvider {}
windows_core::imp::define_interface!(ITfFnShowHelp, ITfFnShowHelp_Vtbl, 0x5ab1d30c_094d_4c29_8ea5_0bf59be87bf3);
impl core::ops::Deref for ITfFnShowHelp {
    type Target = ITfFunction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfFnShowHelp, windows_core::IUnknown, ITfFunction);
impl ITfFnShowHelp {
    pub unsafe fn Show(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), hwndparent).ok()
    }
}
#[repr(C)]
pub struct ITfFnShowHelp_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait ITfFnShowHelp_Impl: ITfFunction_Impl {
    fn Show(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl ITfFnShowHelp_Vtbl {
    pub const fn new<Identity: ITfFnShowHelp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: ITfFnShowHelp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfFnShowHelp_Impl::Show(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        Self { base__: ITfFunction_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFnShowHelp as windows_core::Interface>::IID || iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFnShowHelp {}
windows_core::imp::define_interface!(ITfFunction, ITfFunction_Vtbl, 0xdb593490_098f_11d3_8df0_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfFunction, windows_core::IUnknown);
impl ITfFunction {
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfFunction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFunction_Impl: windows_core::IUnknownImpl {
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITfFunction_Vtbl {
    pub const fn new<Identity: ITfFunction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDisplayName<Identity: ITfFunction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFunction_Impl::GetDisplayName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDisplayName: GetDisplayName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFunction {}
windows_core::imp::define_interface!(ITfFunctionProvider, ITfFunctionProvider_Vtbl, 0x101d6610_0990_11d3_8df0_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfFunctionProvider, windows_core::IUnknown);
impl ITfFunctionProvider {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetFunction(&self, rguid: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFunction)(windows_core::Interface::as_raw(self), rguid, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfFunctionProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFunction: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFunctionProvider_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFunction(&self, rguid: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl ITfFunctionProvider_Vtbl {
    pub const fn new<Identity: ITfFunctionProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFunctionProvider_Impl::GetType(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFunctionProvider_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstrdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunction<Identity: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfFunctionProvider_Impl::GetFunction(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFunctionProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFunctionProvider {}
windows_core::imp::define_interface!(ITfInputProcessorProfileActivationSink, ITfInputProcessorProfileActivationSink_Vtbl, 0x71c6e74e_0f28_11d8_a82a_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfileActivationSink, windows_core::IUnknown);
impl ITfInputProcessorProfileActivationSink {
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn OnActivated(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, catid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnActivated)(windows_core::Interface::as_raw(self), dwprofiletype, langid, clsid, catid, guidprofile, hkl, dwflags).ok()
    }
}
#[repr(C)]
pub struct ITfInputProcessorProfileActivationSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub OnActivated: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, super::Input::KeyboardAndMouse::HKL, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    OnActivated: usize,
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait ITfInputProcessorProfileActivationSink_Impl: windows_core::IUnknownImpl {
    fn OnActivated(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, catid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl ITfInputProcessorProfileActivationSink_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfileActivationSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnActivated<Identity: ITfInputProcessorProfileActivationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, catid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileActivationSink_Impl::OnActivated(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&catid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivated: OnActivated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileActivationSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for ITfInputProcessorProfileActivationSink {}
windows_core::imp::define_interface!(ITfInputProcessorProfileMgr, ITfInputProcessorProfileMgr_Vtbl, 0x71c6e74c_0f28_11d8_a82a_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfileMgr, windows_core::IUnknown);
impl ITfInputProcessorProfileMgr {
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn ActivateProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ActivateProfile)(windows_core::Interface::as_raw(self), dwprofiletype, langid, clsid, guidprofile, hkl, dwflags).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn DeactivateProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeactivateProfile)(windows_core::Interface::as_raw(self), dwprofiletype, langid, clsid, guidprofile, hkl, dwflags).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProfile)(windows_core::Interface::as_raw(self), dwprofiletype, langid, clsid, guidprofile, hkl, core::mem::transmute(pprofile)).ok()
    }
    pub unsafe fn EnumProfiles(&self, langid: u16) -> windows_core::Result<IEnumTfInputProcessorProfiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumProfiles)(windows_core::Interface::as_raw(self), langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReleaseInputProcessor(&self, rclsid: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseInputProcessor)(windows_core::Interface::as_raw(self), rclsid, dwflags).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn RegisterProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: &[u16], pchiconfile: &[u16], uiconindex: u32, hklsubstitute: super::Input::KeyboardAndMouse::HKL, dwpreferredlayout: u32, benabledbydefault: bool, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, core::mem::transmute(pchdesc.as_ptr()), pchdesc.len().try_into().unwrap(), core::mem::transmute(pchiconfile.as_ptr()), pchiconfile.len().try_into().unwrap(), uiconindex, hklsubstitute, dwpreferredlayout, benabledbydefault.into(), dwflags).ok()
    }
    pub unsafe fn UnregisterProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, dwflags).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetActiveProfile(&self, catid: *const windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetActiveProfile)(windows_core::Interface::as_raw(self), catid, core::mem::transmute(pprofile)).ok()
    }
}
#[repr(C)]
pub struct ITfInputProcessorProfileMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub ActivateProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *const windows_core::GUID, *const windows_core::GUID, super::Input::KeyboardAndMouse::HKL, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    ActivateProfile: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub DeactivateProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *const windows_core::GUID, *const windows_core::GUID, super::Input::KeyboardAndMouse::HKL, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    DeactivateProfile: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u16, *const windows_core::GUID, *const windows_core::GUID, super::Input::KeyboardAndMouse::HKL, *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetProfile: usize,
    pub EnumProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseInputProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub RegisterProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, u32, super::Input::KeyboardAndMouse::HKL, u32, super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    RegisterProfile: usize,
    pub UnregisterProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetActiveProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetActiveProfile: usize,
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait ITfInputProcessorProfileMgr_Impl: windows_core::IUnknownImpl {
    fn ActivateProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::Result<()>;
    fn DeactivateProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::Result<()>;
    fn GetProfile(&self, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::Result<()>;
    fn EnumProfiles(&self, langid: u16) -> windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn ReleaseInputProcessor(&self, rclsid: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn RegisterProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: &windows_core::PCWSTR, cchdesc: u32, pchiconfile: &windows_core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: super::Input::KeyboardAndMouse::HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> windows_core::Result<()>;
    fn UnregisterProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn GetActiveProfile(&self, catid: *const windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl ITfInputProcessorProfileMgr_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::ActivateProfile(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeactivateProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::DeactivateProfile(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::GetProfile(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&pprofile)).into()
        }
        unsafe extern "system" fn EnumProfiles<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfileMgr_Impl::EnumProfiles(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseInputProcessor<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::ReleaseInputProcessor(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RegisterProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: windows_core::PCWSTR, cchdesc: u32, pchiconfile: windows_core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: super::Input::KeyboardAndMouse::HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::RegisterProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cchdesc), core::mem::transmute(&pchiconfile), core::mem::transmute_copy(&cchfile), core::mem::transmute_copy(&uiconindex), core::mem::transmute_copy(&hklsubstitute), core::mem::transmute_copy(&dwpreferredlayout), core::mem::transmute_copy(&benabledbydefault), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn UnregisterProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::UnregisterProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, catid: *const windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfileMgr_Impl::GetActiveProfile(this, core::mem::transmute_copy(&catid), core::mem::transmute_copy(&pprofile)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ActivateProfile: ActivateProfile::<Identity, OFFSET>,
            DeactivateProfile: DeactivateProfile::<Identity, OFFSET>,
            GetProfile: GetProfile::<Identity, OFFSET>,
            EnumProfiles: EnumProfiles::<Identity, OFFSET>,
            ReleaseInputProcessor: ReleaseInputProcessor::<Identity, OFFSET>,
            RegisterProfile: RegisterProfile::<Identity, OFFSET>,
            UnregisterProfile: UnregisterProfile::<Identity, OFFSET>,
            GetActiveProfile: GetActiveProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for ITfInputProcessorProfileMgr {}
windows_core::imp::define_interface!(ITfInputProcessorProfileSubstituteLayout, ITfInputProcessorProfileSubstituteLayout_Vtbl, 0x4fd67194_1002_4513_bff2_c0ddf6258552);
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfileSubstituteLayout, windows_core::IUnknown);
impl ITfInputProcessorProfileSubstituteLayout {
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetSubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<super::Input::KeyboardAndMouse::HKL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubstituteKeyboardLayout)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfInputProcessorProfileSubstituteLayout_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetSubstituteKeyboardLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, *mut super::Input::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetSubstituteKeyboardLayout: usize,
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait ITfInputProcessorProfileSubstituteLayout_Impl: windows_core::IUnknownImpl {
    fn GetSubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<super::Input::KeyboardAndMouse::HKL>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl ITfInputProcessorProfileSubstituteLayout_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfileSubstituteLayout_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSubstituteKeyboardLayout<Identity: ITfInputProcessorProfileSubstituteLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, phkl: *mut super::Input::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfileSubstituteLayout_Impl::GetSubstituteKeyboardLayout(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)) {
                Ok(ok__) => {
                    phkl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSubstituteKeyboardLayout: GetSubstituteKeyboardLayout::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileSubstituteLayout as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for ITfInputProcessorProfileSubstituteLayout {}
windows_core::imp::define_interface!(ITfInputProcessorProfiles, ITfInputProcessorProfiles_Vtbl, 0x1f02b6c5_7842_4ee6_8a0b_9a24183a95ca);
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfiles, windows_core::IUnknown);
impl ITfInputProcessorProfiles {
    pub unsafe fn Register(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), rclsid).ok()
    }
    pub unsafe fn Unregister(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unregister)(windows_core::Interface::as_raw(self), rclsid).ok()
    }
    pub unsafe fn AddLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: &[u16], pchiconfile: &[u16], uiconindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, core::mem::transmute(pchdesc.as_ptr()), pchdesc.len().try_into().unwrap(), core::mem::transmute(pchiconfile.as_ptr()), pchiconfile.len().try_into().unwrap(), uiconindex).ok()
    }
    pub unsafe fn RemoveLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumInputProcessorInfo(&self) -> windows_core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumInputProcessorInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const windows_core::GUID, pclsid: *mut windows_core::GUID, pguidprofile: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDefaultLanguageProfile)(windows_core::Interface::as_raw(self), langid, catid, core::mem::transmute(pclsid), core::mem::transmute(pguidprofile)).ok()
    }
    pub unsafe fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const windows_core::GUID, guidprofiles: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultLanguageProfile)(windows_core::Interface::as_raw(self), langid, rclsid, guidprofiles).ok()
    }
    pub unsafe fn ActivateLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofiles: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ActivateLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofiles).ok()
    }
    pub unsafe fn GetActiveLanguageProfile(&self, rclsid: *const windows_core::GUID, plangid: *mut u16, pguidprofile: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetActiveLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, core::mem::transmute(plangid), core::mem::transmute(pguidprofile)).ok()
    }
    pub unsafe fn GetLanguageProfileDescription(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLanguageProfileDescription)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetCurrentLanguage(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ChangeCurrentLanguage(&self, langid: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangeCurrentLanguage)(windows_core::Interface::as_raw(self), langid).ok()
    }
    pub unsafe fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLanguageList)(windows_core::Interface::as_raw(self), core::mem::transmute(pplangid), core::mem::transmute(pulcount)).ok()
    }
    pub unsafe fn EnumLanguageProfiles(&self, langid: u16) -> windows_core::Result<IEnumTfLanguageProfiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumLanguageProfiles)(windows_core::Interface::as_raw(self), langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnableLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, fenable.into()).ok()
    }
    pub unsafe fn IsEnabledLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEnabledLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).map(|| result__)
    }
    pub unsafe fn EnableLanguageProfileByDefault(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableLanguageProfileByDefault)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, fenable.into()).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn SubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SubstituteKeyboardLayout)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, hkl).ok()
    }
}
#[repr(C)]
pub struct ITfInputProcessorProfiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub AddLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub RemoveLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumInputProcessorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumInputProcessorInfo: usize,
    pub GetDefaultLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *const windows_core::GUID, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetDefaultLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub ActivateLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetActiveLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u16, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetLanguageProfileDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ChangeCurrentLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetLanguageList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16, *mut u32) -> windows_core::HRESULT,
    pub EnumLanguageProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsEnabledLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub EnableLanguageProfileByDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub SubstituteKeyboardLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, super::Input::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    SubstituteKeyboardLayout: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
pub trait ITfInputProcessorProfiles_Impl: windows_core::IUnknownImpl {
    fn Register(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Unregister(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn AddLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: &windows_core::PCWSTR, cchdesc: u32, pchiconfile: &windows_core::PCWSTR, cchfile: u32, uiconindex: u32) -> windows_core::Result<()>;
    fn RemoveLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumInputProcessorInfo(&self) -> windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const windows_core::GUID, pclsid: *mut windows_core::GUID, pguidprofile: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const windows_core::GUID, guidprofiles: *const windows_core::GUID) -> windows_core::Result<()>;
    fn ActivateLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofiles: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetActiveLanguageProfile(&self, rclsid: *const windows_core::GUID, plangid: *mut u16, pguidprofile: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetLanguageProfileDescription(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn GetCurrentLanguage(&self) -> windows_core::Result<u16>;
    fn ChangeCurrentLanguage(&self, langid: u16) -> windows_core::Result<()>;
    fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> windows_core::Result<()>;
    fn EnumLanguageProfiles(&self, langid: u16) -> windows_core::Result<IEnumTfLanguageProfiles>;
    fn EnableLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsEnabledLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn EnableLanguageProfileByDefault(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl ITfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Register<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::Register(this, core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn Unregister<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::Unregister(this, core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn AddLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchdesc: windows_core::PCWSTR, cchdesc: u32, pchiconfile: windows_core::PCWSTR, cchfile: u32, uiconindex: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::AddLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cchdesc), core::mem::transmute(&pchiconfile), core::mem::transmute_copy(&cchfile), core::mem::transmute_copy(&uiconindex)).into()
        }
        unsafe extern "system" fn RemoveLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::RemoveLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)).into()
        }
        unsafe extern "system" fn EnumInputProcessorInfo<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::EnumInputProcessorInfo(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, catid: *const windows_core::GUID, pclsid: *mut windows_core::GUID, pguidprofile: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::GetDefaultLanguageProfile(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&catid), core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn SetDefaultLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, rclsid: *const windows_core::GUID, guidprofiles: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::SetDefaultLanguageProfile(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn ActivateLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofiles: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::ActivateLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofiles)).into()
        }
        unsafe extern "system" fn GetActiveLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, plangid: *mut u16, pguidprofile: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::GetActiveLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&plangid), core::mem::transmute_copy(&pguidprofile)).into()
        }
        unsafe extern "system" fn GetLanguageProfileDescription<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pbstrprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::GetLanguageProfileDescription(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)) {
                Ok(ok__) => {
                    pbstrprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLanguage<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plangid: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::GetCurrentLanguage(this) {
                Ok(ok__) => {
                    plangid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCurrentLanguage<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::ChangeCurrentLanguage(this, core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn GetLanguageList<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::GetLanguageList(this, core::mem::transmute_copy(&pplangid), core::mem::transmute_copy(&pulcount)).into()
        }
        unsafe extern "system" fn EnumLanguageProfiles<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::EnumLanguageProfiles(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::EnableLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn IsEnabledLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputProcessorProfiles_Impl::IsEnabledLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)) {
                Ok(ok__) => {
                    pfenable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableLanguageProfileByDefault<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::EnableLanguageProfileByDefault(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SubstituteKeyboardLayout<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, hkl: super::Input::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfiles_Impl::SubstituteKeyboardLayout(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            Unregister: Unregister::<Identity, OFFSET>,
            AddLanguageProfile: AddLanguageProfile::<Identity, OFFSET>,
            RemoveLanguageProfile: RemoveLanguageProfile::<Identity, OFFSET>,
            EnumInputProcessorInfo: EnumInputProcessorInfo::<Identity, OFFSET>,
            GetDefaultLanguageProfile: GetDefaultLanguageProfile::<Identity, OFFSET>,
            SetDefaultLanguageProfile: SetDefaultLanguageProfile::<Identity, OFFSET>,
            ActivateLanguageProfile: ActivateLanguageProfile::<Identity, OFFSET>,
            GetActiveLanguageProfile: GetActiveLanguageProfile::<Identity, OFFSET>,
            GetLanguageProfileDescription: GetLanguageProfileDescription::<Identity, OFFSET>,
            GetCurrentLanguage: GetCurrentLanguage::<Identity, OFFSET>,
            ChangeCurrentLanguage: ChangeCurrentLanguage::<Identity, OFFSET>,
            GetLanguageList: GetLanguageList::<Identity, OFFSET>,
            EnumLanguageProfiles: EnumLanguageProfiles::<Identity, OFFSET>,
            EnableLanguageProfile: EnableLanguageProfile::<Identity, OFFSET>,
            IsEnabledLanguageProfile: IsEnabledLanguageProfile::<Identity, OFFSET>,
            EnableLanguageProfileByDefault: EnableLanguageProfileByDefault::<Identity, OFFSET>,
            SubstituteKeyboardLayout: SubstituteKeyboardLayout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfiles as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl windows_core::RuntimeName for ITfInputProcessorProfiles {}
windows_core::imp::define_interface!(ITfInputProcessorProfilesEx, ITfInputProcessorProfilesEx_Vtbl, 0x892f230f_fe00_4a41_a98e_fcd6de0d35ef);
impl core::ops::Deref for ITfInputProcessorProfilesEx {
    type Target = ITfInputProcessorProfiles;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfilesEx, windows_core::IUnknown, ITfInputProcessorProfiles);
impl ITfInputProcessorProfilesEx {
    pub unsafe fn SetLanguageProfileDisplayName(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchfile: &[u16], uresid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLanguageProfileDisplayName)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, core::mem::transmute(pchfile.as_ptr()), pchfile.len().try_into().unwrap(), uresid).ok()
    }
}
#[repr(C)]
pub struct ITfInputProcessorProfilesEx_Vtbl {
    pub base__: ITfInputProcessorProfiles_Vtbl,
    pub SetLanguageProfileDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, *const windows_core::GUID, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
pub trait ITfInputProcessorProfilesEx_Impl: ITfInputProcessorProfiles_Impl {
    fn SetLanguageProfileDisplayName(&self, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchfile: &windows_core::PCWSTR, cchfile: u32, uresid: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl ITfInputProcessorProfilesEx_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfilesEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLanguageProfileDisplayName<Identity: ITfInputProcessorProfilesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: u16, guidprofile: *const windows_core::GUID, pchfile: windows_core::PCWSTR, cchfile: u32, uresid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputProcessorProfilesEx_Impl::SetLanguageProfileDisplayName(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute(&pchfile), core::mem::transmute_copy(&cchfile), core::mem::transmute_copy(&uresid)).into()
        }
        Self {
            base__: ITfInputProcessorProfiles_Vtbl::new::<Identity, OFFSET>(),
            SetLanguageProfileDisplayName: SetLanguageProfileDisplayName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfilesEx as windows_core::Interface>::IID || iid == &<ITfInputProcessorProfiles as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl windows_core::RuntimeName for ITfInputProcessorProfilesEx {}
windows_core::imp::define_interface!(ITfInputScope, ITfInputScope_Vtbl, 0xfde1eaee_6924_4cdf_91e7_da38cff5559d);
windows_core::imp::interface_hierarchy!(ITfInputScope, windows_core::IUnknown);
impl ITfInputScope {
    pub unsafe fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputScopes)(windows_core::Interface::as_raw(self), core::mem::transmute(pprginputscopes), core::mem::transmute(pccount)).ok()
    }
    pub unsafe fn GetPhrase(&self, ppbstrphrases: *mut *mut windows_core::BSTR, pccount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPhrase)(windows_core::Interface::as_raw(self), core::mem::transmute(ppbstrphrases), core::mem::transmute(pccount)).ok()
    }
    pub unsafe fn GetRegularExpression(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegularExpression)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetSRGS(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSRGS)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetXML(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXML)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfInputScope_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputScopes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut InputScope, *mut u32) -> windows_core::HRESULT,
    pub GetPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetRegularExpression: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSRGS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfInputScope_Impl: windows_core::IUnknownImpl {
    fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> windows_core::Result<()>;
    fn GetPhrase(&self, ppbstrphrases: *mut *mut windows_core::BSTR, pccount: *mut u32) -> windows_core::Result<()>;
    fn GetRegularExpression(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSRGS(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetXML(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITfInputScope_Vtbl {
    pub const fn new<Identity: ITfInputScope_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputScopes<Identity: ITfInputScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputScope_Impl::GetInputScopes(this, core::mem::transmute_copy(&pprginputscopes), core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetPhrase<Identity: ITfInputScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbstrphrases: *mut *mut *mut core::ffi::c_void, pccount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfInputScope_Impl::GetPhrase(this, core::mem::transmute_copy(&ppbstrphrases), core::mem::transmute_copy(&pccount)).into()
        }
        unsafe extern "system" fn GetRegularExpression<Identity: ITfInputScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrregexp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputScope_Impl::GetRegularExpression(this) {
                Ok(ok__) => {
                    pbstrregexp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSRGS<Identity: ITfInputScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsrgs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputScope_Impl::GetSRGS(this) {
                Ok(ok__) => {
                    pbstrsrgs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXML<Identity: ITfInputScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputScope_Impl::GetXML(this) {
                Ok(ok__) => {
                    pbstrxml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputScopes: GetInputScopes::<Identity, OFFSET>,
            GetPhrase: GetPhrase::<Identity, OFFSET>,
            GetRegularExpression: GetRegularExpression::<Identity, OFFSET>,
            GetSRGS: GetSRGS::<Identity, OFFSET>,
            GetXML: GetXML::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputScope as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfInputScope {}
windows_core::imp::define_interface!(ITfInputScope2, ITfInputScope2_Vtbl, 0x5731eaa0_6bc2_4681_a532_92fbb74d7c41);
impl core::ops::Deref for ITfInputScope2 {
    type Target = ITfInputScope;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfInputScope2, windows_core::IUnknown, ITfInputScope);
impl ITfInputScope2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumWordList(&self) -> windows_core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumWordList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfInputScope2_Vtbl {
    pub base__: ITfInputScope_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumWordList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumWordList: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfInputScope2_Impl: ITfInputScope_Impl {
    fn EnumWordList(&self) -> windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfInputScope2_Vtbl {
    pub const fn new<Identity: ITfInputScope2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumWordList<Identity: ITfInputScope2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInputScope2_Impl::EnumWordList(this) {
                Ok(ok__) => {
                    ppenumstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfInputScope_Vtbl::new::<Identity, OFFSET>(), EnumWordList: EnumWordList::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputScope2 as windows_core::Interface>::IID || iid == &<ITfInputScope as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfInputScope2 {}
windows_core::imp::define_interface!(ITfInsertAtSelection, ITfInsertAtSelection_Vtbl, 0x55ce16ba_3014_41c1_9ceb_fade1446ac6c);
windows_core::imp::interface_hierarchy!(ITfInsertAtSelection, windows_core::IUnknown);
impl ITfInsertAtSelection {
    pub unsafe fn InsertTextAtSelection(&self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: &[u16]) -> windows_core::Result<ITfRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), ec, dwflags, core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<P2>(&self, ec: u32, dwflags: u32, pdataobject: P2) -> windows_core::Result<ITfRange>
    where
        P2: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), ec, dwflags, pdataobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfInsertAtSelection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, INSERT_TEXT_AT_SELECTION_FLAGS, windows_core::PCWSTR, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfInsertAtSelection_Impl: windows_core::IUnknownImpl {
    fn InsertTextAtSelection(&self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: &windows_core::PCWSTR, cch: i32) -> windows_core::Result<ITfRange>;
    fn InsertEmbeddedAtSelection(&self, ec: u32, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<ITfRange>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfInsertAtSelection_Vtbl {
    pub const fn new<Identity: ITfInsertAtSelection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITfInsertAtSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: windows_core::PCWSTR, cch: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInsertAtSelection_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITfInsertAtSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfInsertAtSelection_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject)) {
                Ok(ok__) => {
                    pprange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInsertAtSelection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfInsertAtSelection {}
windows_core::imp::define_interface!(ITfIntegratableCandidateListUIElement, ITfIntegratableCandidateListUIElement_Vtbl, 0xc7a6f54f_b180_416f_b2bf_7bf2e4683d7b);
windows_core::imp::interface_hierarchy!(ITfIntegratableCandidateListUIElement, windows_core::IUnknown);
impl ITfIntegratableCandidateListUIElement {
    pub unsafe fn SetIntegrationStyle(&self, guidintegrationstyle: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIntegrationStyle)(windows_core::Interface::as_raw(self), core::mem::transmute(guidintegrationstyle)).ok()
    }
    pub unsafe fn GetSelectionStyle(&self) -> windows_core::Result<TfIntegratableCandidateListSelectionStyle> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelectionStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OnKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnKeyDown)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn ShowCandidateNumbers(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowCandidateNumbers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FinalizeExactCompositionString(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FinalizeExactCompositionString)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfIntegratableCandidateListUIElement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIntegrationStyle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub GetSelectionStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TfIntegratableCandidateListSelectionStyle) -> windows_core::HRESULT,
    pub OnKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ShowCandidateNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FinalizeExactCompositionString: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfIntegratableCandidateListUIElement_Impl: windows_core::IUnknownImpl {
    fn SetIntegrationStyle(&self, guidintegrationstyle: &windows_core::GUID) -> windows_core::Result<()>;
    fn GetSelectionStyle(&self) -> windows_core::Result<TfIntegratableCandidateListSelectionStyle>;
    fn OnKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ShowCandidateNumbers(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn FinalizeExactCompositionString(&self) -> windows_core::Result<()>;
}
impl ITfIntegratableCandidateListUIElement_Vtbl {
    pub const fn new<Identity: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIntegrationStyle<Identity: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidintegrationstyle: windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfIntegratableCandidateListUIElement_Impl::SetIntegrationStyle(this, core::mem::transmute(&guidintegrationstyle)).into()
        }
        unsafe extern "system" fn GetSelectionStyle<Identity: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfIntegratableCandidateListUIElement_Impl::GetSelectionStyle(this) {
                Ok(ok__) => {
                    ptfselectionstyle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Identity: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfIntegratableCandidateListUIElement_Impl::OnKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowCandidateNumbers<Identity: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfshow: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfIntegratableCandidateListUIElement_Impl::ShowCandidateNumbers(this) {
                Ok(ok__) => {
                    pfshow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalizeExactCompositionString<Identity: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfIntegratableCandidateListUIElement_Impl::FinalizeExactCompositionString(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIntegrationStyle: SetIntegrationStyle::<Identity, OFFSET>,
            GetSelectionStyle: GetSelectionStyle::<Identity, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, OFFSET>,
            ShowCandidateNumbers: ShowCandidateNumbers::<Identity, OFFSET>,
            FinalizeExactCompositionString: FinalizeExactCompositionString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfIntegratableCandidateListUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfIntegratableCandidateListUIElement {}
windows_core::imp::define_interface!(ITfKeyEventSink, ITfKeyEventSink_Vtbl, 0xaa80e7f5_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfKeyEventSink, windows_core::IUnknown);
impl ITfKeyEventSink {
    pub unsafe fn OnSetFocus(&self, fforeground: bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnSetFocus)(windows_core::Interface::as_raw(self), fforeground.into()).ok()
    }
    pub unsafe fn OnTestKeyDown<P0>(&self, pic: P0, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnTestKeyDown)(windows_core::Interface::as_raw(self), pic.param().abi(), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn OnTestKeyUp<P0>(&self, pic: P0, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnTestKeyUp)(windows_core::Interface::as_raw(self), pic.param().abi(), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn OnKeyDown<P0>(&self, pic: P0, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnKeyDown)(windows_core::Interface::as_raw(self), pic.param().abi(), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn OnKeyUp<P0>(&self, pic: P0, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnKeyUp)(windows_core::Interface::as_raw(self), pic.param().abi(), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn OnPreservedKey<P0>(&self, pic: P0, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnPreservedKey)(windows_core::Interface::as_raw(self), pic.param().abi(), rguid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfKeyEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnTestKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnTestKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnPreservedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ITfKeyEventSink_Impl: windows_core::IUnknownImpl {
    fn OnSetFocus(&self, fforeground: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnTestKeyDown(&self, pic: Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(&self, pic: Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyDown(&self, pic: Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(&self, pic: Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnPreservedKey(&self, pic: Option<&ITfContext>, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl ITfKeyEventSink_Vtbl {
    pub const fn new<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetFocus<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeyEventSink_Impl::OnSetFocus(this, core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnTestKeyDown(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnTestKeyUp(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyDown<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnKeyDown(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyUp<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnKeyUp(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPreservedKey<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeyEventSink_Impl::OnPreservedKey(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, OFFSET>,
            OnPreservedKey: OnPreservedKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfKeyEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfKeyEventSink {}
windows_core::imp::define_interface!(ITfKeyTraceEventSink, ITfKeyTraceEventSink_Vtbl, 0x1cd4c13b_1c36_4191_a70a_7f3e611f367d);
windows_core::imp::interface_hierarchy!(ITfKeyTraceEventSink, windows_core::IUnknown);
impl ITfKeyTraceEventSink {
    pub unsafe fn OnKeyTraceDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnKeyTraceDown)(windows_core::Interface::as_raw(self), wparam, lparam).ok()
    }
    pub unsafe fn OnKeyTraceUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnKeyTraceUp)(windows_core::Interface::as_raw(self), wparam, lparam).ok()
    }
}
#[repr(C)]
pub struct ITfKeyTraceEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnKeyTraceDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub OnKeyTraceUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
}
pub trait ITfKeyTraceEventSink_Impl: windows_core::IUnknownImpl {
    fn OnKeyTraceDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn OnKeyTraceUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
}
impl ITfKeyTraceEventSink_Vtbl {
    pub const fn new<Identity: ITfKeyTraceEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnKeyTraceDown<Identity: ITfKeyTraceEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeyTraceEventSink_Impl::OnKeyTraceDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn OnKeyTraceUp<Identity: ITfKeyTraceEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeyTraceEventSink_Impl::OnKeyTraceUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnKeyTraceDown: OnKeyTraceDown::<Identity, OFFSET>,
            OnKeyTraceUp: OnKeyTraceUp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfKeyTraceEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfKeyTraceEventSink {}
windows_core::imp::define_interface!(ITfKeystrokeMgr, ITfKeystrokeMgr_Vtbl, 0xaa80e7f0_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfKeystrokeMgr, windows_core::IUnknown);
impl ITfKeystrokeMgr {
    pub unsafe fn AdviseKeyEventSink<P1>(&self, tid: u32, psink: P1, fforeground: bool) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfKeyEventSink>,
    {
        (windows_core::Interface::vtable(self).AdviseKeyEventSink)(windows_core::Interface::as_raw(self), tid, psink.param().abi(), fforeground.into()).ok()
    }
    pub unsafe fn UnadviseKeyEventSink(&self, tid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnadviseKeyEventSink)(windows_core::Interface::as_raw(self), tid).ok()
    }
    pub unsafe fn GetForeground(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetForeground)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TestKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TestKeyDown)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn TestKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TestKeyUp)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn KeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeyDown)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn KeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeyUp)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
    }
    pub unsafe fn GetPreservedKey<P0>(&self, pic: P0, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<ITfContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreservedKey)(windows_core::Interface::as_raw(self), pic.param().abi(), pprekey, &mut result__).map(|| result__)
    }
    pub unsafe fn IsPreservedKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPreservedKey)(windows_core::Interface::as_raw(self), rguid, pprekey, &mut result__).map(|| result__)
    }
    pub unsafe fn PreserveKey(&self, tid: u32, rguid: *const windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PreserveKey)(windows_core::Interface::as_raw(self), tid, rguid, prekey, core::mem::transmute(pchdesc.as_ptr()), pchdesc.len().try_into().unwrap()).ok()
    }
    pub unsafe fn UnpreserveKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnpreserveKey)(windows_core::Interface::as_raw(self), rguid, pprekey).ok()
    }
    pub unsafe fn SetPreservedKeyDescription(&self, rguid: *const windows_core::GUID, pchdesc: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPreservedKeyDescription)(windows_core::Interface::as_raw(self), rguid, core::mem::transmute(pchdesc.as_ptr()), pchdesc.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetPreservedKeyDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreservedKeyDescription)(windows_core::Interface::as_raw(self), rguid, &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn SimulatePreservedKey<P0>(&self, pic: P0, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SimulatePreservedKey)(windows_core::Interface::as_raw(self), pic.param().abi(), rguid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfKeystrokeMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseKeyEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub UnadviseKeyEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetForeground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TestKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub TestKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub KeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub KeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetPreservedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const TF_PRESERVEDKEY, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsPreservedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const TF_PRESERVEDKEY, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub PreserveKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const TF_PRESERVEDKEY, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub UnpreserveKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const TF_PRESERVEDKEY) -> windows_core::HRESULT,
    pub SetPreservedKeyDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetPreservedKeyDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SimulatePreservedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ITfKeystrokeMgr_Impl: windows_core::IUnknownImpl {
    fn AdviseKeyEventSink(&self, tid: u32, psink: Option<&ITfKeyEventSink>, fforeground: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UnadviseKeyEventSink(&self, tid: u32) -> windows_core::Result<()>;
    fn GetForeground(&self) -> windows_core::Result<windows_core::GUID>;
    fn TestKeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn TestKeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn KeyDown(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn KeyUp(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetPreservedKey(&self, pic: Option<&ITfContext>, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<windows_core::GUID>;
    fn IsPreservedKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn PreserveKey(&self, tid: u32, rguid: *const windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: &windows_core::PCWSTR, cchdesc: u32) -> windows_core::Result<()>;
    fn UnpreserveKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<()>;
    fn SetPreservedKeyDescription(&self, rguid: *const windows_core::GUID, pchdesc: &windows_core::PCWSTR, cchdesc: u32) -> windows_core::Result<()>;
    fn GetPreservedKeyDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn SimulatePreservedKey(&self, pic: Option<&ITfContext>, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl ITfKeystrokeMgr_Vtbl {
    pub const fn new<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseKeyEventSink<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, psink: *mut core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::AdviseKeyEventSink(this, core::mem::transmute_copy(&tid), windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&fforeground)).into()
        }
        unsafe extern "system" fn UnadviseKeyEventSink<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::UnadviseKeyEventSink(this, core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn GetForeground<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::GetForeground(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyDown<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::TestKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestKeyUp<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::TestKeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyDown<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::KeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUp<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::KeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreservedKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::GetPreservedKey(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&pprekey)) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPreservedKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::IsPreservedKey(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pprekey)) {
                Ok(ok__) => {
                    pfregistered.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreserveKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, rguid: *const windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: windows_core::PCWSTR, cchdesc: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::PreserveKey(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&prekey), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn UnpreserveKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::UnpreserveKey(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pprekey)).into()
        }
        unsafe extern "system" fn SetPreservedKeyDescription<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pchdesc: windows_core::PCWSTR, cchdesc: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfKeystrokeMgr_Impl::SetPreservedKeyDescription(this, core::mem::transmute_copy(&rguid), core::mem::transmute(&pchdesc), core::mem::transmute_copy(&cchdesc)).into()
        }
        unsafe extern "system" fn GetPreservedKeyDescription<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pbstrdesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::GetPreservedKeyDescription(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pbstrdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimulatePreservedKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfKeystrokeMgr_Impl::SimulatePreservedKey(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseKeyEventSink: AdviseKeyEventSink::<Identity, OFFSET>,
            UnadviseKeyEventSink: UnadviseKeyEventSink::<Identity, OFFSET>,
            GetForeground: GetForeground::<Identity, OFFSET>,
            TestKeyDown: TestKeyDown::<Identity, OFFSET>,
            TestKeyUp: TestKeyUp::<Identity, OFFSET>,
            KeyDown: KeyDown::<Identity, OFFSET>,
            KeyUp: KeyUp::<Identity, OFFSET>,
            GetPreservedKey: GetPreservedKey::<Identity, OFFSET>,
            IsPreservedKey: IsPreservedKey::<Identity, OFFSET>,
            PreserveKey: PreserveKey::<Identity, OFFSET>,
            UnpreserveKey: UnpreserveKey::<Identity, OFFSET>,
            SetPreservedKeyDescription: SetPreservedKeyDescription::<Identity, OFFSET>,
            GetPreservedKeyDescription: GetPreservedKeyDescription::<Identity, OFFSET>,
            SimulatePreservedKey: SimulatePreservedKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfKeystrokeMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfKeystrokeMgr {}
windows_core::imp::define_interface!(ITfLMLattice, ITfLMLattice_Vtbl, 0xd4236675_a5bf_4570_9d42_5d6d7b02d59b);
windows_core::imp::interface_hierarchy!(ITfLMLattice, windows_core::IUnknown);
impl ITfLMLattice {
    pub unsafe fn QueryType(&self, rguidtype: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryType)(windows_core::Interface::as_raw(self), rguidtype, &mut result__).map(|| result__)
    }
    pub unsafe fn EnumLatticeElements(&self, dwframestart: u32, rguidtype: *const windows_core::GUID) -> windows_core::Result<IEnumTfLatticeElements> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumLatticeElements)(windows_core::Interface::as_raw(self), dwframestart, rguidtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfLMLattice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub EnumLatticeElements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfLMLattice_Impl: windows_core::IUnknownImpl {
    fn QueryType(&self, rguidtype: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn EnumLatticeElements(&self, dwframestart: u32, rguidtype: *const windows_core::GUID) -> windows_core::Result<IEnumTfLatticeElements>;
}
impl ITfLMLattice_Vtbl {
    pub const fn new<Identity: ITfLMLattice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryType<Identity: ITfLMLattice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidtype: *const windows_core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLMLattice_Impl::QueryType(this, core::mem::transmute_copy(&rguidtype)) {
                Ok(ok__) => {
                    pfsupported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumLatticeElements<Identity: ITfLMLattice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwframestart: u32, rguidtype: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLMLattice_Impl::EnumLatticeElements(this, core::mem::transmute_copy(&dwframestart), core::mem::transmute_copy(&rguidtype)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryType: QueryType::<Identity, OFFSET>,
            EnumLatticeElements: EnumLatticeElements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLMLattice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfLMLattice {}
windows_core::imp::define_interface!(ITfLangBarEventSink, ITfLangBarEventSink_Vtbl, 0x18a4e900_e0ae_11d2_afdd_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfLangBarEventSink, windows_core::IUnknown);
impl ITfLangBarEventSink {
    pub unsafe fn OnSetFocus(&self, dwthreadid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnSetFocus)(windows_core::Interface::as_raw(self), dwthreadid).ok()
    }
    pub unsafe fn OnThreadTerminate(&self, dwthreadid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadTerminate)(windows_core::Interface::as_raw(self), dwthreadid).ok()
    }
    pub unsafe fn OnThreadItemChange(&self, dwthreadid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnThreadItemChange)(windows_core::Interface::as_raw(self), dwthreadid).ok()
    }
    pub unsafe fn OnModalInput(&self, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnModalInput)(windows_core::Interface::as_raw(self), dwthreadid, umsg, wparam, lparam).ok()
    }
    pub unsafe fn ShowFloating(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowFloating)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemFloatingRect)(windows_core::Interface::as_raw(self), dwthreadid, rguid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfLangBarEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnThreadTerminate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnThreadItemChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnModalInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub ShowFloating: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetItemFloatingRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
}
pub trait ITfLangBarEventSink_Impl: windows_core::IUnknownImpl {
    fn OnSetFocus(&self, dwthreadid: u32) -> windows_core::Result<()>;
    fn OnThreadTerminate(&self, dwthreadid: u32) -> windows_core::Result<()>;
    fn OnThreadItemChange(&self, dwthreadid: u32) -> windows_core::Result<()>;
    fn OnModalInput(&self, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn ShowFloating(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::RECT>;
}
impl ITfLangBarEventSink_Vtbl {
    pub const fn new<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetFocus<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::OnSetFocus(this, core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::OnThreadTerminate(this, core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnThreadItemChange<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::OnThreadItemChange(this, core::mem::transmute_copy(&dwthreadid)).into()
        }
        unsafe extern "system" fn OnModalInput<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::OnModalInput(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn ShowFloating<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarEventSink_Impl::ShowFloating(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, rguid: *const windows_core::GUID, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarEventSink_Impl::GetItemFloatingRect(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, OFFSET>,
            OnThreadItemChange: OnThreadItemChange::<Identity, OFFSET>,
            OnModalInput: OnModalInput::<Identity, OFFSET>,
            ShowFloating: ShowFloating::<Identity, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfLangBarEventSink {}
windows_core::imp::define_interface!(ITfLangBarItem, ITfLangBarItem_Vtbl, 0x73540d69_edeb_4ee9_96c9_23aa30b25916);
windows_core::imp::interface_hierarchy!(ITfLangBarItem, windows_core::IUnknown);
impl ITfLangBarItem {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pinfo)).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Show(&self, fshow: bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), fshow.into()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTooltipString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfLangBarItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TF_LANGBARITEMINFO) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetTooltipString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfLangBarItem_Impl: windows_core::IUnknownImpl {
    fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Show(&self, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetTooltipString(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITfLangBarItem_Vtbl {
    pub const fn new<Identity: ITfLangBarItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInfo<Identity: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItem_Impl::GetInfo(this, core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItem_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Identity: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItem_Impl::Show(this, core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn GetTooltipString<Identity: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtooltip: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItem_Impl::GetTooltipString(this) {
                Ok(ok__) => {
                    pbstrtooltip.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            GetTooltipString: GetTooltipString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfLangBarItem {}
windows_core::imp::define_interface!(ITfLangBarItemBalloon, ITfLangBarItemBalloon_Vtbl, 0x01c2d285_d3c7_4b7b_b5b5_d97411d0c283);
impl core::ops::Deref for ITfLangBarItemBalloon {
    type Target = ITfLangBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfLangBarItemBalloon, windows_core::IUnknown, ITfLangBarItem);
impl ITfLangBarItemBalloon {
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnClick)(windows_core::Interface::as_raw(self), click, core::mem::transmute(pt), prcarea).ok()
    }
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreferredSize)(windows_core::Interface::as_raw(self), pszdefault, &mut result__).map(|| result__)
    }
    pub unsafe fn GetBalloonInfo(&self) -> windows_core::Result<TF_LBBALLOONINFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBalloonInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfLangBarItemBalloon_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    pub OnClick: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBIClick, super::super::Foundation::POINT, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetPreferredSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SIZE, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
    pub GetBalloonInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TF_LBBALLOONINFO) -> windows_core::HRESULT,
}
pub trait ITfLangBarItemBalloon_Impl: ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> windows_core::Result<super::super::Foundation::SIZE>;
    fn GetBalloonInfo(&self) -> windows_core::Result<TF_LBBALLOONINFO>;
}
impl ITfLangBarItemBalloon_Vtbl {
    pub const fn new<Identity: ITfLangBarItemBalloon_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnClick<Identity: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBalloon_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBalloon_Impl::GetPreferredSize(this, core::mem::transmute_copy(&pszdefault)) {
                Ok(ok__) => {
                    psz.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBalloonInfo<Identity: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBalloon_Impl::GetBalloonInfo(this) {
                Ok(ok__) => {
                    pinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, OFFSET>,
            GetBalloonInfo: GetBalloonInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBalloon as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfLangBarItemBalloon {}
windows_core::imp::define_interface!(ITfLangBarItemBitmap, ITfLangBarItemBitmap_Vtbl, 0x73830352_d722_4179_ada5_f045c98df355);
impl core::ops::Deref for ITfLangBarItemBitmap {
    type Target = ITfLangBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfLangBarItemBitmap, windows_core::IUnknown, ITfLangBarItem);
impl ITfLangBarItemBitmap {
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnClick)(windows_core::Interface::as_raw(self), click, core::mem::transmute(pt), prcarea).ok()
    }
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreferredSize)(windows_core::Interface::as_raw(self), pszdefault, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bmwidth, bmheight, dwflags, core::mem::transmute(phbmp), core::mem::transmute(phbmpmask)).ok()
    }
}
#[repr(C)]
pub struct ITfLangBarItemBitmap_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    pub OnClick: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBIClick, super::super::Foundation::POINT, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetPreferredSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SIZE, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DrawBitmap: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait ITfLangBarItemBitmap_Impl: ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> windows_core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ITfLangBarItemBitmap_Vtbl {
    pub const fn new<Identity: ITfLangBarItemBitmap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnClick<Identity: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmap_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBitmap_Impl::GetPreferredSize(this, core::mem::transmute_copy(&pszdefault)) {
                Ok(ok__) => {
                    psz.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmap_Impl::DrawBitmap(this, core::mem::transmute_copy(&bmwidth), core::mem::transmute_copy(&bmheight), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&phbmp), core::mem::transmute_copy(&phbmpmask)).into()
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmap as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for ITfLangBarItemBitmap {}
windows_core::imp::define_interface!(ITfLangBarItemBitmapButton, ITfLangBarItemBitmapButton_Vtbl, 0xa26a0525_3fae_4fa0_89ee_88a964f9f1b5);
impl core::ops::Deref for ITfLangBarItemBitmapButton {
    type Target = ITfLangBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfLangBarItemBitmapButton, windows_core::IUnknown, ITfLangBarItem);
impl ITfLangBarItemBitmapButton {
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnClick)(windows_core::Interface::as_raw(self), click, core::mem::transmute(pt), prcarea).ok()
    }
    pub unsafe fn InitMenu<P0>(&self, pmenu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfMenu>,
    {
        (windows_core::Interface::vtable(self).InitMenu)(windows_core::Interface::as_raw(self), pmenu.param().abi()).ok()
    }
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMenuSelect)(windows_core::Interface::as_raw(self), wid).ok()
    }
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreferredSize)(windows_core::Interface::as_raw(self), pszdefault, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bmwidth, bmheight, dwflags, core::mem::transmute(phbmp), core::mem::transmute(phbmpmask)).ok()
    }
    pub unsafe fn GetText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfLangBarItemBitmapButton_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    pub OnClick: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBIClick, super::super::Foundation::POINT, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub InitMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnMenuSelect: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPreferredSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SIZE, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *mut super::super::Graphics::Gdi::HBITMAP, *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DrawBitmap: usize,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait ITfLangBarItemBitmapButton_Impl: ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn InitMenu(&self, pmenu: Option<&ITfMenu>) -> windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: u32) -> windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> windows_core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>;
    fn GetText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ITfLangBarItemBitmapButton_Vtbl {
    pub const fn new<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnClick<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmapButton_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmenu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmapButton_Impl::InitMenu(this, windows_core::from_raw_borrowed(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmapButton_Impl::OnMenuSelect(this, core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBitmapButton_Impl::GetPreferredSize(this, core::mem::transmute_copy(&pszdefault)) {
                Ok(ok__) => {
                    psz.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemBitmapButton_Impl::DrawBitmap(this, core::mem::transmute_copy(&bmwidth), core::mem::transmute_copy(&bmheight), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&phbmp), core::mem::transmute_copy(&phbmpmask)).into()
        }
        unsafe extern "system" fn GetText<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemBitmapButton_Impl::GetText(this) {
                Ok(ok__) => {
                    pbstrtext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            InitMenu: InitMenu::<Identity, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmapButton as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for ITfLangBarItemBitmapButton {}
windows_core::imp::define_interface!(ITfLangBarItemButton, ITfLangBarItemButton_Vtbl, 0x28c7f1d0_de25_11d2_afdd_00105a2799b5);
impl core::ops::Deref for ITfLangBarItemButton {
    type Target = ITfLangBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfLangBarItemButton, windows_core::IUnknown, ITfLangBarItem);
impl ITfLangBarItemButton {
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnClick)(windows_core::Interface::as_raw(self), click, core::mem::transmute(pt), prcarea).ok()
    }
    pub unsafe fn InitMenu<P0>(&self, pmenu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfMenu>,
    {
        (windows_core::Interface::vtable(self).InitMenu)(windows_core::Interface::as_raw(self), pmenu.param().abi()).ok()
    }
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMenuSelect)(windows_core::Interface::as_raw(self), wid).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self) -> windows_core::Result<super::WindowsAndMessaging::HICON> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIcon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfLangBarItemButton_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    pub OnClick: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBIClick, super::super::Foundation::POINT, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub InitMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnMenuSelect: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ITfLangBarItemButton_Impl: ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn InitMenu(&self, pmenu: Option<&ITfMenu>) -> windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: u32) -> windows_core::Result<()>;
    fn GetIcon(&self) -> windows_core::Result<super::WindowsAndMessaging::HICON>;
    fn GetText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ITfLangBarItemButton_Vtbl {
    pub const fn new<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnClick<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemButton_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
        }
        unsafe extern "system" fn InitMenu<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmenu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemButton_Impl::InitMenu(this, windows_core::from_raw_borrowed(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemButton_Impl::OnMenuSelect(this, core::mem::transmute_copy(&wid)).into()
        }
        unsafe extern "system" fn GetIcon<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phicon: *mut super::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemButton_Impl::GetIcon(this) {
                Ok(ok__) => {
                    phicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemButton_Impl::GetText(this) {
                Ok(ok__) => {
                    pbstrtext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            InitMenu: InitMenu::<Identity, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemButton as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for ITfLangBarItemButton {}
windows_core::imp::define_interface!(ITfLangBarItemMgr, ITfLangBarItemMgr_Vtbl, 0xba468c55_9956_4fb1_a59d_52a7dd7cc6aa);
windows_core::imp::interface_hierarchy!(ITfLangBarItemMgr, windows_core::IUnknown);
impl ITfLangBarItemMgr {
    pub unsafe fn EnumItems(&self) -> windows_core::Result<IEnumTfLangBarItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfLangBarItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), rguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddItem<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfLangBarItem>,
    {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn RemoveItem<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfLangBarItem>,
    {
        (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn AdviseItemSink<P0>(&self, punk: P0, pdwcookie: *mut u32, rguiditem: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfLangBarItemSink>,
    {
        (windows_core::Interface::vtable(self).AdviseItemSink)(windows_core::Interface::as_raw(self), punk.param().abi(), core::mem::transmute(pdwcookie), rguiditem).ok()
    }
    pub unsafe fn UnadviseItemSink(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnadviseItemSink)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemFloatingRect)(windows_core::Interface::as_raw(self), dwthreadid, rguid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetItemsStatus(&self, ulcount: u32, prgguid: *const windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemsStatus)(windows_core::Interface::as_raw(self), ulcount, prgguid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetItemNum(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemNum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetItems(&self, ulcount: u32, ppitem: *mut Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetItems)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(ppitem), core::mem::transmute(pinfo), core::mem::transmute(pdwstatus), core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn AdviseItemsSink(&self, ulcount: u32, ppunk: *const Option<ITfLangBarItemSink>, pguiditem: *const windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdviseItemsSink)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(ppunk), pguiditem, &mut result__).map(|| result__)
    }
    pub unsafe fn UnadviseItemsSink(&self, pdwcookie: &[u32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnadviseItemsSink)(windows_core::Interface::as_raw(self), pdwcookie.len().try_into().unwrap(), core::mem::transmute(pdwcookie.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ITfLangBarItemMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdviseItemSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub UnadviseItemSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetItemFloatingRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetItemsStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetItemNum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItems: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut TF_LANGBARITEMINFO, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub AdviseItemsSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub UnadviseItemsSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
}
pub trait ITfLangBarItemMgr_Impl: windows_core::IUnknownImpl {
    fn EnumItems(&self) -> windows_core::Result<IEnumTfLangBarItems>;
    fn GetItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfLangBarItem>;
    fn AddItem(&self, punk: Option<&ITfLangBarItem>) -> windows_core::Result<()>;
    fn RemoveItem(&self, punk: Option<&ITfLangBarItem>) -> windows_core::Result<()>;
    fn AdviseItemSink(&self, punk: Option<&ITfLangBarItemSink>, pdwcookie: *mut u32, rguiditem: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnadviseItemSink(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetItemsStatus(&self, ulcount: u32, prgguid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetItemNum(&self) -> windows_core::Result<u32>;
    fn GetItems(&self, ulcount: u32, ppitem: *mut Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn AdviseItemsSink(&self, ulcount: u32, ppunk: *const Option<ITfLangBarItemSink>, pguiditem: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn UnadviseItemsSink(&self, ulcount: u32, pdwcookie: *const u32) -> windows_core::Result<()>;
}
impl ITfLangBarItemMgr_Vtbl {
    pub const fn new<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumItems<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::EnumItems(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::GetItem(this, core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::AddItem(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::RemoveItem(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn AdviseItemSink<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, pdwcookie: *mut u32, rguiditem: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::AdviseItemSink(this, windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&pdwcookie), core::mem::transmute_copy(&rguiditem)).into()
        }
        unsafe extern "system" fn UnadviseItemSink<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::UnadviseItemSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, rguid: *const windows_core::GUID, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::GetItemFloatingRect(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&rguid)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsStatus<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, prgguid: *const windows_core::GUID, pdwstatus: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::GetItemsStatus(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&prgguid)) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemNum<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::GetItemNum(this) {
                Ok(ok__) => {
                    pulcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItems<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppitem: *mut *mut core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::GetItems(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppitem), core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&pdwstatus), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn AdviseItemsSink<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppunk: *const *mut core::ffi::c_void, pguiditem: *const windows_core::GUID, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarItemMgr_Impl::AdviseItemsSink(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppunk), core::mem::transmute_copy(&pguiditem)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseItemsSink<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pdwcookie: *const u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemMgr_Impl::UnadviseItemsSink(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pdwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumItems: EnumItems::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
            AdviseItemSink: AdviseItemSink::<Identity, OFFSET>,
            UnadviseItemSink: UnadviseItemSink::<Identity, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, OFFSET>,
            GetItemsStatus: GetItemsStatus::<Identity, OFFSET>,
            GetItemNum: GetItemNum::<Identity, OFFSET>,
            GetItems: GetItems::<Identity, OFFSET>,
            AdviseItemsSink: AdviseItemsSink::<Identity, OFFSET>,
            UnadviseItemsSink: UnadviseItemsSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfLangBarItemMgr {}
windows_core::imp::define_interface!(ITfLangBarItemSink, ITfLangBarItemSink_Vtbl, 0x57dbe1a0_de25_11d2_afdd_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfLangBarItemSink, windows_core::IUnknown);
impl ITfLangBarItemSink {
    pub unsafe fn OnUpdate(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnUpdate)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
}
#[repr(C)]
pub struct ITfLangBarItemSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfLangBarItemSink_Impl: windows_core::IUnknownImpl {
    fn OnUpdate(&self, dwflags: u32) -> windows_core::Result<()>;
}
impl ITfLangBarItemSink_Vtbl {
    pub const fn new<Identity: ITfLangBarItemSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdate<Identity: ITfLangBarItemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarItemSink_Impl::OnUpdate(this, core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdate: OnUpdate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfLangBarItemSink {}
windows_core::imp::define_interface!(ITfLangBarMgr, ITfLangBarMgr_Vtbl, 0x87955690_e627_11d2_8ddb_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfLangBarMgr, windows_core::IUnknown);
impl ITfLangBarMgr {
    pub unsafe fn AdviseEventSink<P0>(&self, psink: P0, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfLangBarEventSink>,
    {
        (windows_core::Interface::vtable(self).AdviseEventSink)(windows_core::Interface::as_raw(self), psink.param().abi(), hwnd, dwflags, pdwcookie).ok()
    }
    pub unsafe fn UnadviseEventSink(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnadviseEventSink)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn GetThreadMarshalInterface(&self, dwthreadid: u32, dwtype: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThreadMarshalInterface)(windows_core::Interface::as_raw(self), dwthreadid, dwtype, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetThreadLangBarItemMgr(&self, dwthreadid: u32, pplbi: *mut Option<ITfLangBarItemMgr>, pdwthreadid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetThreadLangBarItemMgr)(windows_core::Interface::as_raw(self), dwthreadid, core::mem::transmute(pplbi), core::mem::transmute(pdwthreadid)).ok()
    }
    pub unsafe fn GetInputProcessorProfiles(&self, dwthreadid: u32, ppaip: *mut Option<ITfInputProcessorProfiles>, pdwthreadid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputProcessorProfiles)(windows_core::Interface::as_raw(self), dwthreadid, core::mem::transmute(ppaip), core::mem::transmute(pdwthreadid)).ok()
    }
    pub unsafe fn RestoreLastFocus(&self, pdwthreadid: *mut u32, fprev: bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestoreLastFocus)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwthreadid), fprev.into()).ok()
    }
    pub unsafe fn SetModalInput<P0>(&self, psink: P0, dwthreadid: u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfLangBarEventSink>,
    {
        (windows_core::Interface::vtable(self).SetModalInput)(windows_core::Interface::as_raw(self), psink.param().abi(), dwthreadid, dwflags).ok()
    }
    pub unsafe fn ShowFloating(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowFloating)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetShowFloatingStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetShowFloatingStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfLangBarMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND, u32, *const u32) -> windows_core::HRESULT,
    pub UnadviseEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetThreadMarshalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetThreadLangBarItemMgr: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetInputProcessorProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RestoreLastFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetModalInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ShowFloating: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetShowFloatingStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ITfLangBarMgr_Impl: windows_core::IUnknownImpl {
    fn AdviseEventSink(&self, psink: Option<&ITfLangBarEventSink>, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> windows_core::Result<()>;
    fn UnadviseEventSink(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn GetThreadMarshalInterface(&self, dwthreadid: u32, dwtype: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetThreadLangBarItemMgr(&self, dwthreadid: u32, pplbi: *mut Option<ITfLangBarItemMgr>, pdwthreadid: *mut u32) -> windows_core::Result<()>;
    fn GetInputProcessorProfiles(&self, dwthreadid: u32, ppaip: *mut Option<ITfInputProcessorProfiles>, pdwthreadid: *mut u32) -> windows_core::Result<()>;
    fn RestoreLastFocus(&self, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetModalInput(&self, psink: Option<&ITfLangBarEventSink>, dwthreadid: u32, dwflags: u32) -> windows_core::Result<()>;
    fn ShowFloating(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetShowFloatingStatus(&self) -> windows_core::Result<u32>;
}
impl ITfLangBarMgr_Vtbl {
    pub const fn new<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseEventSink<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::AdviseEventSink(this, windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn UnadviseEventSink<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::UnadviseEventSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetThreadMarshalInterface<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, dwtype: u32, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarMgr_Impl::GetThreadMarshalInterface(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadLangBarItemMgr<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, pplbi: *mut *mut core::ffi::c_void, pdwthreadid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::GetThreadLangBarItemMgr(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&pplbi), core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn GetInputProcessorProfiles<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: u32, ppaip: *mut *mut core::ffi::c_void, pdwthreadid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::GetInputProcessorProfiles(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&ppaip), core::mem::transmute_copy(&pdwthreadid)).into()
        }
        unsafe extern "system" fn RestoreLastFocus<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::RestoreLastFocus(this, core::mem::transmute_copy(&pdwthreadid), core::mem::transmute_copy(&fprev)).into()
        }
        unsafe extern "system" fn SetModalInput<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, dwthreadid: u32, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::SetModalInput(this, windows_core::from_raw_borrowed(&psink), core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ShowFloating<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLangBarMgr_Impl::ShowFloating(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetShowFloatingStatus<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLangBarMgr_Impl::GetShowFloatingStatus(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseEventSink: AdviseEventSink::<Identity, OFFSET>,
            UnadviseEventSink: UnadviseEventSink::<Identity, OFFSET>,
            GetThreadMarshalInterface: GetThreadMarshalInterface::<Identity, OFFSET>,
            GetThreadLangBarItemMgr: GetThreadLangBarItemMgr::<Identity, OFFSET>,
            GetInputProcessorProfiles: GetInputProcessorProfiles::<Identity, OFFSET>,
            RestoreLastFocus: RestoreLastFocus::<Identity, OFFSET>,
            SetModalInput: SetModalInput::<Identity, OFFSET>,
            ShowFloating: ShowFloating::<Identity, OFFSET>,
            GetShowFloatingStatus: GetShowFloatingStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfLangBarMgr {}
windows_core::imp::define_interface!(ITfLanguageProfileNotifySink, ITfLanguageProfileNotifySink_Vtbl, 0x43c9fe15_f494_4c17_9de2_b8a4ac350aa8);
windows_core::imp::interface_hierarchy!(ITfLanguageProfileNotifySink, windows_core::IUnknown);
impl ITfLanguageProfileNotifySink {
    pub unsafe fn OnLanguageChange(&self, langid: u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnLanguageChange)(windows_core::Interface::as_raw(self), langid, &mut result__).map(|| result__)
    }
    pub unsafe fn OnLanguageChanged(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLanguageChanged)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfLanguageProfileNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLanguageChange: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnLanguageChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfLanguageProfileNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnLanguageChange(&self, langid: u16) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn OnLanguageChanged(&self) -> windows_core::Result<()>;
}
impl ITfLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLanguageChange<Identity: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfLanguageProfileNotifySink_Impl::OnLanguageChange(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    pfaccept.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLanguageChanged<Identity: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfLanguageProfileNotifySink_Impl::OnLanguageChanged(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLanguageChange: OnLanguageChange::<Identity, OFFSET>,
            OnLanguageChanged: OnLanguageChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLanguageProfileNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfLanguageProfileNotifySink {}
windows_core::imp::define_interface!(ITfMSAAControl, ITfMSAAControl_Vtbl, 0xb5f8fb3b_393f_4f7c_84cb_504924c2705a);
windows_core::imp::interface_hierarchy!(ITfMSAAControl, windows_core::IUnknown);
impl ITfMSAAControl {
    pub unsafe fn SystemEnableMSAA(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SystemEnableMSAA)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SystemDisableMSAA(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SystemDisableMSAA)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfMSAAControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SystemEnableMSAA: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemDisableMSAA: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfMSAAControl_Impl: windows_core::IUnknownImpl {
    fn SystemEnableMSAA(&self) -> windows_core::Result<()>;
    fn SystemDisableMSAA(&self) -> windows_core::Result<()>;
}
impl ITfMSAAControl_Vtbl {
    pub const fn new<Identity: ITfMSAAControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SystemEnableMSAA<Identity: ITfMSAAControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMSAAControl_Impl::SystemEnableMSAA(this).into()
        }
        unsafe extern "system" fn SystemDisableMSAA<Identity: ITfMSAAControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMSAAControl_Impl::SystemDisableMSAA(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SystemEnableMSAA: SystemEnableMSAA::<Identity, OFFSET>,
            SystemDisableMSAA: SystemDisableMSAA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMSAAControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfMSAAControl {}
windows_core::imp::define_interface!(ITfMenu, ITfMenu_Vtbl, 0x6f8a98e4_aaa0_4f15_8c5b_07e0df0a3dd8);
windows_core::imp::interface_hierarchy!(ITfMenu, windows_core::IUnknown);
impl ITfMenu {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddMenuItem(&self, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: &[u16], ppmenu: *mut Option<ITfMenu>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddMenuItem)(windows_core::Interface::as_raw(self), uid, dwflags, hbmp, hbmpmask, core::mem::transmute(pch.as_ptr()), pch.len().try_into().unwrap(), core::mem::transmute(ppmenu)).ok()
    }
}
#[repr(C)]
pub struct ITfMenu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::super::Graphics::Gdi::HBITMAP, super::super::Graphics::Gdi::HBITMAP, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddMenuItem: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait ITfMenu_Impl: windows_core::IUnknownImpl {
    fn AddMenuItem(&self, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: &windows_core::PCWSTR, cch: u32, ppmenu: *mut Option<ITfMenu>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ITfMenu_Vtbl {
    pub const fn new<Identity: ITfMenu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddMenuItem<Identity: ITfMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: windows_core::PCWSTR, cch: u32, ppmenu: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMenu_Impl::AddMenuItem(this, core::mem::transmute_copy(&uid), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&hbmp), core::mem::transmute_copy(&hbmpmask), core::mem::transmute(&pch), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&ppmenu)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddMenuItem: AddMenuItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMenu as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for ITfMenu {}
windows_core::imp::define_interface!(ITfMessagePump, ITfMessagePump_Vtbl, 0x8f1b8ad8_0b6b_4874_90c5_bd76011e8f7c);
windows_core::imp::interface_hierarchy!(ITfMessagePump, windows_core::IUnknown);
impl ITfMessagePump {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn PeekMessageA(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PeekMessageA)(windows_core::Interface::as_raw(self), core::mem::transmute(pmsg), hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg, core::mem::transmute(pfresult)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetMessageA(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMessageA)(windows_core::Interface::as_raw(self), core::mem::transmute(pmsg), hwnd, wmsgfiltermin, wmsgfiltermax, core::mem::transmute(pfresult)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn PeekMessageW(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PeekMessageW)(windows_core::Interface::as_raw(self), core::mem::transmute(pmsg), hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg, core::mem::transmute(pfresult)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetMessageW(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMessageW)(windows_core::Interface::as_raw(self), core::mem::transmute(pmsg), hwnd, wmsgfiltermin, wmsgfiltermax, core::mem::transmute(pfresult)).ok()
    }
}
#[repr(C)]
pub struct ITfMessagePump_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub PeekMessageA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowsAndMessaging::MSG, super::super::Foundation::HWND, u32, u32, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    PeekMessageA: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetMessageA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowsAndMessaging::MSG, super::super::Foundation::HWND, u32, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetMessageA: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub PeekMessageW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowsAndMessaging::MSG, super::super::Foundation::HWND, u32, u32, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    PeekMessageW: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetMessageW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowsAndMessaging::MSG, super::super::Foundation::HWND, u32, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetMessageW: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ITfMessagePump_Impl: windows_core::IUnknownImpl {
    fn PeekMessageA(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetMessageA(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn PeekMessageW(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetMessageW(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ITfMessagePump_Vtbl {
    pub const fn new<Identity: ITfMessagePump_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PeekMessageA<Identity: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMessagePump_Impl::PeekMessageA(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&wremovemsg), core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageA<Identity: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMessagePump_Impl::GetMessageA(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn PeekMessageW<Identity: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMessagePump_Impl::PeekMessageW(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&wremovemsg), core::mem::transmute_copy(&pfresult)).into()
        }
        unsafe extern "system" fn GetMessageW<Identity: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMessagePump_Impl::GetMessageW(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&pfresult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PeekMessageA: PeekMessageA::<Identity, OFFSET>,
            GetMessageA: GetMessageA::<Identity, OFFSET>,
            PeekMessageW: PeekMessageW::<Identity, OFFSET>,
            GetMessageW: GetMessageW::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMessagePump as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for ITfMessagePump {}
windows_core::imp::define_interface!(ITfMouseSink, ITfMouseSink_Vtbl, 0xa1adaaa2_3a24_449d_ac96_5183e7f5c217);
windows_core::imp::interface_hierarchy!(ITfMouseSink, windows_core::IUnknown);
impl ITfMouseSink {
    pub unsafe fn OnMouseEvent(&self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnMouseEvent)(windows_core::Interface::as_raw(self), uedge, uquadrant, dwbtnstatus, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfMouseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMouseEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ITfMouseSink_Impl: windows_core::IUnknownImpl {
    fn OnMouseEvent(&self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl ITfMouseSink_Vtbl {
    pub const fn new<Identity: ITfMouseSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMouseEvent<Identity: ITfMouseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfMouseSink_Impl::OnMouseEvent(this, core::mem::transmute_copy(&uedge), core::mem::transmute_copy(&uquadrant), core::mem::transmute_copy(&dwbtnstatus)) {
                Ok(ok__) => {
                    pfeaten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnMouseEvent: OnMouseEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMouseSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfMouseSink {}
windows_core::imp::define_interface!(ITfMouseTracker, ITfMouseTracker_Vtbl, 0x09d146cd_a544_4132_925b_7afa8ef322d0);
windows_core::imp::interface_hierarchy!(ITfMouseTracker, windows_core::IUnknown);
impl ITfMouseTracker {
    pub unsafe fn AdviseMouseSink<P0, P1>(&self, range: P0, psink: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ITfRange>,
        P1: windows_core::Param<ITfMouseSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdviseMouseSink)(windows_core::Interface::as_raw(self), range.param().abi(), psink.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnadviseMouseSink)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
}
#[repr(C)]
pub struct ITfMouseTracker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseMouseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnadviseMouseSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfMouseTracker_Impl: windows_core::IUnknownImpl {
    fn AdviseMouseSink(&self, range: Option<&ITfRange>, psink: Option<&ITfMouseSink>) -> windows_core::Result<u32>;
    fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl ITfMouseTracker_Vtbl {
    pub const fn new<Identity: ITfMouseTracker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseMouseSink<Identity: ITfMouseTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfMouseTracker_Impl::AdviseMouseSink(this, windows_core::from_raw_borrowed(&range), windows_core::from_raw_borrowed(&psink)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: ITfMouseTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMouseTracker_Impl::UnadviseMouseSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Identity, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMouseTracker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfMouseTracker {}
windows_core::imp::define_interface!(ITfMouseTrackerACP, ITfMouseTrackerACP_Vtbl, 0x3bdd78e2_c16e_47fd_b883_ce6facc1a208);
windows_core::imp::interface_hierarchy!(ITfMouseTrackerACP, windows_core::IUnknown);
impl ITfMouseTrackerACP {
    pub unsafe fn AdviseMouseSink<P0, P1>(&self, range: P0, psink: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ITfRangeACP>,
        P1: windows_core::Param<ITfMouseSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdviseMouseSink)(windows_core::Interface::as_raw(self), range.param().abi(), psink.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnadviseMouseSink)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
}
#[repr(C)]
pub struct ITfMouseTrackerACP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseMouseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnadviseMouseSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfMouseTrackerACP_Impl: windows_core::IUnknownImpl {
    fn AdviseMouseSink(&self, range: Option<&ITfRangeACP>, psink: Option<&ITfMouseSink>) -> windows_core::Result<u32>;
    fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl ITfMouseTrackerACP_Vtbl {
    pub const fn new<Identity: ITfMouseTrackerACP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseMouseSink<Identity: ITfMouseTrackerACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfMouseTrackerACP_Impl::AdviseMouseSink(this, windows_core::from_raw_borrowed(&range), windows_core::from_raw_borrowed(&psink)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: ITfMouseTrackerACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfMouseTrackerACP_Impl::UnadviseMouseSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Identity, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMouseTrackerACP as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfMouseTrackerACP {}
windows_core::imp::define_interface!(ITfPersistentPropertyLoaderACP, ITfPersistentPropertyLoaderACP_Vtbl, 0x4ef89150_0807_11d3_8df0_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfPersistentPropertyLoaderACP, windows_core::IUnknown);
impl ITfPersistentPropertyLoaderACP {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadProperty(&self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadProperty)(windows_core::Interface::as_raw(self), phdr, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfPersistentPropertyLoaderACP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadProperty: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfPersistentPropertyLoaderACP_Impl: windows_core::IUnknownImpl {
    fn LoadProperty(&self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfPersistentPropertyLoaderACP_Vtbl {
    pub const fn new<Identity: ITfPersistentPropertyLoaderACP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadProperty<Identity: ITfPersistentPropertyLoaderACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPersistentPropertyLoaderACP_Impl::LoadProperty(this, core::mem::transmute_copy(&phdr)) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadProperty: LoadProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfPersistentPropertyLoaderACP as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfPersistentPropertyLoaderACP {}
windows_core::imp::define_interface!(ITfPreservedKeyNotifySink, ITfPreservedKeyNotifySink_Vtbl, 0x6f77c993_d2b1_446e_853e_5912efc8a286);
windows_core::imp::interface_hierarchy!(ITfPreservedKeyNotifySink, windows_core::IUnknown);
impl ITfPreservedKeyNotifySink {
    pub unsafe fn OnUpdated(&self, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnUpdated)(windows_core::Interface::as_raw(self), pprekey).ok()
    }
}
#[repr(C)]
pub struct ITfPreservedKeyNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *const TF_PRESERVEDKEY) -> windows_core::HRESULT,
}
pub trait ITfPreservedKeyNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnUpdated(&self, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<()>;
}
impl ITfPreservedKeyNotifySink_Vtbl {
    pub const fn new<Identity: ITfPreservedKeyNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdated<Identity: ITfPreservedKeyNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfPreservedKeyNotifySink_Impl::OnUpdated(this, core::mem::transmute_copy(&pprekey)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdated: OnUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfPreservedKeyNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfPreservedKeyNotifySink {}
windows_core::imp::define_interface!(ITfProperty, ITfProperty_Vtbl, 0xe2449660_9542_11d2_bf46_00105a2799b5);
impl core::ops::Deref for ITfProperty {
    type Target = ITfReadOnlyProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfProperty, windows_core::IUnknown, ITfReadOnlyProperty);
impl ITfProperty {
    pub unsafe fn FindRange<P1>(&self, ec: u32, prange: P1, pprange: *mut Option<ITfRange>, apos: TfAnchor) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).FindRange)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), core::mem::transmute(pprange), apos).ok()
    }
    pub unsafe fn SetValueStore<P1, P2>(&self, ec: u32, prange: P1, ppropstore: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
        P2: windows_core::Param<ITfPropertyStore>,
    {
        (windows_core::Interface::vtable(self).SetValueStore)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), ppropstore.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue<P1>(&self, ec: u32, prange: P1, pvarvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn Clear<P1>(&self, ec: u32, prange: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), ec, prange.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfProperty_Vtbl {
    pub base__: ITfReadOnlyProperty_Vtbl,
    pub FindRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, TfAnchor) -> windows_core::HRESULT,
    pub SetValueStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfProperty_Impl: ITfReadOnlyProperty_Impl {
    fn FindRange(&self, ec: u32, prange: Option<&ITfRange>, pprange: *mut Option<ITfRange>, apos: TfAnchor) -> windows_core::Result<()>;
    fn SetValueStore(&self, ec: u32, prange: Option<&ITfRange>, ppropstore: Option<&ITfPropertyStore>) -> windows_core::Result<()>;
    fn SetValue(&self, ec: u32, prange: Option<&ITfRange>, pvarvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self, ec: u32, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITfProperty_Vtbl {
    pub const fn new<Identity: ITfProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindRange<Identity: ITfProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void, apos: TfAnchor) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfProperty_Impl::FindRange(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&pprange), core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn SetValueStore<Identity: ITfProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, ppropstore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfProperty_Impl::SetValueStore(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), windows_core::from_raw_borrowed(&ppropstore)).into()
        }
        unsafe extern "system" fn SetValue<Identity: ITfProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, pvarvalue: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfProperty_Impl::SetValue(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn Clear<Identity: ITfProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfProperty_Impl::Clear(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange)).into()
        }
        Self {
            base__: ITfReadOnlyProperty_Vtbl::new::<Identity, OFFSET>(),
            FindRange: FindRange::<Identity, OFFSET>,
            SetValueStore: SetValueStore::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfProperty as windows_core::Interface>::IID || iid == &<ITfReadOnlyProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITfProperty {}
windows_core::imp::define_interface!(ITfPropertyStore, ITfPropertyStore_Vtbl, 0x6834b120_88cb_11d2_bf45_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfPropertyStore, windows_core::IUnknown);
impl ITfPropertyStore {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataType(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetData(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn OnTextUpdated<P1>(&self, dwflags: u32, prangenew: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P1: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnTextUpdated)(windows_core::Interface::as_raw(self), dwflags, prangenew.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Shrink<P0>(&self, prangenew: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Shrink)(windows_core::Interface::as_raw(self), prangenew.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Divide<P0, P1>(&self, prangethis: P0, prangenew: P1) -> windows_core::Result<ITfPropertyStore>
    where
        P0: windows_core::Param<ITfRange>,
        P1: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Divide)(windows_core::Interface::as_raw(self), prangethis.param().abi(), prangenew.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ITfPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyRangeCreator(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyRangeCreator)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0>(&self, pstream: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pstream.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfPropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetData: usize,
    pub OnTextUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Shrink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Divide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyRangeCreator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfPropertyStore_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDataType(&self) -> windows_core::Result<u32>;
    fn GetData(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn OnTextUpdated(&self, dwflags: u32, prangenew: Option<&ITfRange>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Shrink(&self, prangenew: Option<&ITfRange>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Divide(&self, prangethis: Option<&ITfRange>, prangenew: Option<&ITfRange>) -> windows_core::Result<ITfPropertyStore>;
    fn Clone(&self) -> windows_core::Result<ITfPropertyStore>;
    fn GetPropertyRangeCreator(&self) -> windows_core::Result<windows_core::GUID>;
    fn Serialize(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITfPropertyStore_Vtbl {
    pub const fn new<Identity: ITfPropertyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::GetType(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwreserved: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::GetDataType(this) {
                Ok(ok__) => {
                    pdwreserved.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::GetData(this) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTextUpdated<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, prangenew: *mut core::ffi::c_void, pfaccept: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::OnTextUpdated(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&prangenew)) {
                Ok(ok__) => {
                    pfaccept.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangenew: *mut core::ffi::c_void, pffree: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::Shrink(this, windows_core::from_raw_borrowed(&prangenew)) {
                Ok(ok__) => {
                    pffree.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Divide<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangethis: *mut core::ffi::c_void, prangenew: *mut core::ffi::c_void, pppropstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::Divide(this, windows_core::from_raw_borrowed(&prangethis), windows_core::from_raw_borrowed(&prangenew)) {
                Ok(ok__) => {
                    pppropstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::Clone(this) {
                Ok(ok__) => {
                    ppropstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyRangeCreator<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::GetPropertyRangeCreator(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfPropertyStore_Impl::Serialize(this, windows_core::from_raw_borrowed(&pstream)) {
                Ok(ok__) => {
                    pcb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetDataType: GetDataType::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            OnTextUpdated: OnTextUpdated::<Identity, OFFSET>,
            Shrink: Shrink::<Identity, OFFSET>,
            Divide: Divide::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetPropertyRangeCreator: GetPropertyRangeCreator::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITfPropertyStore {}
windows_core::imp::define_interface!(ITfQueryEmbedded, ITfQueryEmbedded_Vtbl, 0x0fab9bdb_d250_4169_84e5_6be118fdd7a8);
windows_core::imp::interface_hierarchy!(ITfQueryEmbedded, windows_core::IUnknown);
impl ITfQueryEmbedded {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfQueryEmbedded_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::super::System::Com::FORMATETC, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryInsertEmbedded: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfQueryEmbedded_Impl: windows_core::IUnknownImpl {
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfQueryEmbedded_Vtbl {
    pub const fn new<Identity: ITfQueryEmbedded_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITfQueryEmbedded_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfQueryEmbedded_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                Ok(ok__) => {
                    pfinsertable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfQueryEmbedded as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfQueryEmbedded {}
windows_core::imp::define_interface!(ITfRange, ITfRange_Vtbl, 0xaa80e7ff_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfRange, windows_core::IUnknown);
impl ITfRange {
    pub unsafe fn GetText(&self, ec: u32, dwflags: u32, pchtext: &mut [u16], pcch: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), ec, dwflags, core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap(), core::mem::transmute(pcch)).ok()
    }
    pub unsafe fn SetText(&self, ec: u32, dwflags: u32, pchtext: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), ec, dwflags, core::mem::transmute(pchtext.as_ptr()), pchtext.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, ec: u32) -> windows_core::Result<super::super::System::Com::IDataObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), ec, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEmbedded(&self, ec: u32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), ec, rguidservice, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P2>(&self, ec: u32, dwflags: u32, pdataobject: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), ec, dwflags, pdataobject.param().abi()).ok()
    }
    pub unsafe fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShiftStart)(windows_core::Interface::as_raw(self), ec, cchreq, core::mem::transmute(pcch), core::mem::transmute(phalt)).ok()
    }
    pub unsafe fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShiftEnd)(windows_core::Interface::as_raw(self), ec, cchreq, core::mem::transmute(pcch), core::mem::transmute(phalt)).ok()
    }
    pub unsafe fn ShiftStartToRange<P1>(&self, ec: u32, prange: P1, apos: TfAnchor) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).ShiftStartToRange)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), apos).ok()
    }
    pub unsafe fn ShiftEndToRange<P1>(&self, ec: u32, prange: P1, apos: TfAnchor) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).ShiftEndToRange)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), apos).ok()
    }
    pub unsafe fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShiftStartRegion)(windows_core::Interface::as_raw(self), ec, dir, &mut result__).map(|| result__)
    }
    pub unsafe fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShiftEndRegion)(windows_core::Interface::as_raw(self), ec, dir, &mut result__).map(|| result__)
    }
    pub unsafe fn IsEmpty(&self, ec: u32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEmpty)(windows_core::Interface::as_raw(self), ec, &mut result__).map(|| result__)
    }
    pub unsafe fn Collapse(&self, ec: u32, apos: TfAnchor) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Collapse)(windows_core::Interface::as_raw(self), ec, apos).ok()
    }
    pub unsafe fn IsEqualStart<P1>(&self, ec: u32, pwith: P1, apos: TfAnchor) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P1: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEqualStart)(windows_core::Interface::as_raw(self), ec, pwith.param().abi(), apos, &mut result__).map(|| result__)
    }
    pub unsafe fn IsEqualEnd<P1>(&self, ec: u32, pwith: P1, apos: TfAnchor) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P1: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEqualEnd)(windows_core::Interface::as_raw(self), ec, pwith.param().abi(), apos, &mut result__).map(|| result__)
    }
    pub unsafe fn CompareStart<P1>(&self, ec: u32, pwith: P1, apos: TfAnchor) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompareStart)(windows_core::Interface::as_raw(self), ec, pwith.param().abi(), apos, &mut result__).map(|| result__)
    }
    pub unsafe fn CompareEnd<P1>(&self, ec: u32, pwith: P1, apos: TfAnchor) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompareEnd)(windows_core::Interface::as_raw(self), ec, pwith.param().abi(), apos, &mut result__).map(|| result__)
    }
    pub unsafe fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdjustForInsert)(windows_core::Interface::as_raw(self), ec, cchinsert, &mut result__).map(|| result__)
    }
    pub unsafe fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGravity)(windows_core::Interface::as_raw(self), core::mem::transmute(pgstart), core::mem::transmute(pgend)).ok()
    }
    pub unsafe fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGravity)(windows_core::Interface::as_raw(self), ec, gstart, gend).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ITfRange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<ITfContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfRange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub ShiftStart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *mut i32, *const TF_HALTCOND) -> windows_core::HRESULT,
    pub ShiftEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *mut i32, *const TF_HALTCOND) -> windows_core::HRESULT,
    pub ShiftStartToRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, TfAnchor) -> windows_core::HRESULT,
    pub ShiftEndToRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, TfAnchor) -> windows_core::HRESULT,
    pub ShiftStartRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, TfShiftDir, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ShiftEndRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, TfShiftDir, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void, u32, TfAnchor) -> windows_core::HRESULT,
    pub IsEqualStart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, TfAnchor, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsEqualEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, TfAnchor, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CompareStart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, TfAnchor, *mut i32) -> windows_core::HRESULT,
    pub CompareEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, TfAnchor, *mut i32) -> windows_core::HRESULT,
    pub AdjustForInsert: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TfGravity, *mut TfGravity) -> windows_core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, u32, TfGravity, TfGravity) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfRange_Impl: windows_core::IUnknownImpl {
    fn GetText(&self, ec: u32, dwflags: u32, pchtext: windows_core::PWSTR, cchmax: u32, pcch: *mut u32) -> windows_core::Result<()>;
    fn SetText(&self, ec: u32, dwflags: u32, pchtext: &windows_core::PCWSTR, cch: i32) -> windows_core::Result<()>;
    fn GetFormattedText(&self, ec: u32) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(&self, ec: u32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn InsertEmbedded(&self, ec: u32, dwflags: u32, pdataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<()>;
    fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::Result<()>;
    fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::Result<()>;
    fn ShiftStartToRange(&self, ec: u32, prange: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<()>;
    fn ShiftEndToRange(&self, ec: u32, prange: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<()>;
    fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEmpty(&self, ec: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Collapse(&self, ec: u32, apos: TfAnchor) -> windows_core::Result<()>;
    fn IsEqualStart(&self, ec: u32, pwith: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEqualEnd(&self, ec: u32, pwith: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareStart(&self, ec: u32, pwith: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<i32>;
    fn CompareEnd(&self, ec: u32, pwith: Option<&ITfRange>, apos: TfAnchor) -> windows_core::Result<i32>;
    fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> windows_core::Result<()>;
    fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ITfRange>;
    fn GetContext(&self) -> windows_core::Result<ITfContext>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfRange_Vtbl {
    pub const fn new<Identity: ITfRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetText<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: u32, pchtext: windows_core::PWSTR, cchmax: u32, pcch: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::GetText(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cchmax), core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetText<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: u32, pchtext: windows_core::PCWSTR, cch: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::SetText(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pchtext), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::GetFormattedText(this, core::mem::transmute_copy(&ec)) {
                Ok(ok__) => {
                    ppdataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::GetEmbedded(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::InsertEmbedded(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn ShiftStart<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::ShiftStart(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftEnd<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::ShiftEnd(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&phalt)).into()
        }
        unsafe extern "system" fn ShiftStartToRange<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, apos: TfAnchor) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::ShiftStartToRange(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftEndToRange<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, apos: TfAnchor) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::ShiftEndToRange(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange), core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn ShiftStartRegion<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::ShiftStartRegion(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dir)) {
                Ok(ok__) => {
                    pfnoregion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShiftEndRegion<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::ShiftEndRegion(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dir)) {
                Ok(ok__) => {
                    pfnoregion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEmpty<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::IsEmpty(this, core::mem::transmute_copy(&ec)) {
                Ok(ok__) => {
                    pfempty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, apos: TfAnchor) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::Collapse(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&apos)).into()
        }
        unsafe extern "system" fn IsEqualStart<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pwith: *mut core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::IsEqualStart(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&pwith), core::mem::transmute_copy(&apos)) {
                Ok(ok__) => {
                    pfequal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqualEnd<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pwith: *mut core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::IsEqualEnd(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&pwith), core::mem::transmute_copy(&apos)) {
                Ok(ok__) => {
                    pfequal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareStart<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pwith: *mut core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::CompareStart(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&pwith), core::mem::transmute_copy(&apos)) {
                Ok(ok__) => {
                    plresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEnd<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, pwith: *mut core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::CompareEnd(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&pwith), core::mem::transmute_copy(&apos)) {
                Ok(ok__) => {
                    plresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjustForInsert<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::AdjustForInsert(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&cchinsert)) {
                Ok(ok__) => {
                    pfinsertok.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGravity<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::GetGravity(this, core::mem::transmute_copy(&pgstart), core::mem::transmute_copy(&pgend)).into()
        }
        unsafe extern "system" fn SetGravity<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRange_Impl::SetGravity(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&gstart), core::mem::transmute_copy(&gend)).into()
        }
        unsafe extern "system" fn Clone<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::Clone(this) {
                Ok(ok__) => {
                    ppclone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfRange_Impl::GetContext(this) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            ShiftStart: ShiftStart::<Identity, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, OFFSET>,
            ShiftStartToRange: ShiftStartToRange::<Identity, OFFSET>,
            ShiftEndToRange: ShiftEndToRange::<Identity, OFFSET>,
            ShiftStartRegion: ShiftStartRegion::<Identity, OFFSET>,
            ShiftEndRegion: ShiftEndRegion::<Identity, OFFSET>,
            IsEmpty: IsEmpty::<Identity, OFFSET>,
            Collapse: Collapse::<Identity, OFFSET>,
            IsEqualStart: IsEqualStart::<Identity, OFFSET>,
            IsEqualEnd: IsEqualEnd::<Identity, OFFSET>,
            CompareStart: CompareStart::<Identity, OFFSET>,
            CompareEnd: CompareEnd::<Identity, OFFSET>,
            AdjustForInsert: AdjustForInsert::<Identity, OFFSET>,
            GetGravity: GetGravity::<Identity, OFFSET>,
            SetGravity: SetGravity::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfRange as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfRange {}
windows_core::imp::define_interface!(ITfRangeACP, ITfRangeACP_Vtbl, 0x057a6296_029b_4154_b79a_0d461d4ea94c);
impl core::ops::Deref for ITfRangeACP {
    type Target = ITfRange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfRangeACP, windows_core::IUnknown, ITfRange);
impl ITfRangeACP {
    pub unsafe fn GetExtent(&self, pacpanchor: *mut i32, pcch: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetExtent)(windows_core::Interface::as_raw(self), core::mem::transmute(pacpanchor), core::mem::transmute(pcch)).ok()
    }
    pub unsafe fn SetExtent(&self, acpanchor: i32, cch: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExtent)(windows_core::Interface::as_raw(self), acpanchor, cch).ok()
    }
}
#[repr(C)]
pub struct ITfRangeACP_Vtbl {
    pub base__: ITfRange_Vtbl,
    pub GetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITfRangeACP_Impl: ITfRange_Impl {
    fn GetExtent(&self, pacpanchor: *mut i32, pcch: *mut i32) -> windows_core::Result<()>;
    fn SetExtent(&self, acpanchor: i32, cch: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITfRangeACP_Vtbl {
    pub const fn new<Identity: ITfRangeACP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExtent<Identity: ITfRangeACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRangeACP_Impl::GetExtent(this, core::mem::transmute_copy(&pacpanchor), core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn SetExtent<Identity: ITfRangeACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpanchor: i32, cch: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRangeACP_Impl::SetExtent(this, core::mem::transmute_copy(&acpanchor), core::mem::transmute_copy(&cch)).into()
        }
        Self { base__: ITfRange_Vtbl::new::<Identity, OFFSET>(), GetExtent: GetExtent::<Identity, OFFSET>, SetExtent: SetExtent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfRangeACP as windows_core::Interface>::IID || iid == &<ITfRange as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITfRangeACP {}
windows_core::imp::define_interface!(ITfRangeBackup, ITfRangeBackup_Vtbl, 0x463a506d_6992_49d2_9b88_93d55e70bb16);
windows_core::imp::interface_hierarchy!(ITfRangeBackup, windows_core::IUnknown);
impl ITfRangeBackup {
    pub unsafe fn Restore<P1>(&self, ec: u32, prange: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self), ec, prange.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfRangeBackup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfRangeBackup_Impl: windows_core::IUnknownImpl {
    fn Restore(&self, ec: u32, prange: Option<&ITfRange>) -> windows_core::Result<()>;
}
impl ITfRangeBackup_Vtbl {
    pub const fn new<Identity: ITfRangeBackup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Restore<Identity: ITfRangeBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfRangeBackup_Impl::Restore(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Restore: Restore::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfRangeBackup as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfRangeBackup {}
windows_core::imp::define_interface!(ITfReadOnlyProperty, ITfReadOnlyProperty_Vtbl, 0x17d49a3d_f8b8_4b2f_b254_52319dd64c53);
windows_core::imp::interface_hierarchy!(ITfReadOnlyProperty, windows_core::IUnknown);
impl ITfReadOnlyProperty {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumRanges<P2>(&self, ec: u32, ppenum: *mut Option<IEnumTfRanges>, ptargetrange: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ITfRange>,
    {
        (windows_core::Interface::vtable(self).EnumRanges)(windows_core::Interface::as_raw(self), ec, core::mem::transmute(ppenum), ptargetrange.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue<P1>(&self, ec: u32, prange: P1) -> windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P1: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<ITfContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfReadOnlyProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub EnumRanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetValue: usize,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfReadOnlyProperty_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn EnumRanges(&self, ec: u32, ppenum: *mut Option<IEnumTfRanges>, ptargetrange: Option<&ITfRange>) -> windows_core::Result<()>;
    fn GetValue(&self, ec: u32, prange: Option<&ITfRange>) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetContext(&self) -> windows_core::Result<ITfContext>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITfReadOnlyProperty_Vtbl {
    pub const fn new<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadOnlyProperty_Impl::GetType(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRanges<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, ppenum: *mut *mut core::ffi::c_void, ptargetrange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfReadOnlyProperty_Impl::EnumRanges(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ppenum), windows_core::from_raw_borrowed(&ptargetrange)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: u32, prange: *mut core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadOnlyProperty_Impl::GetValue(this, core::mem::transmute_copy(&ec), windows_core::from_raw_borrowed(&prange)) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadOnlyProperty_Impl::GetContext(this) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            EnumRanges: EnumRanges::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReadOnlyProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITfReadOnlyProperty {}
windows_core::imp::define_interface!(ITfReadingInformationUIElement, ITfReadingInformationUIElement_Vtbl, 0xea1ea139_19df_11d7_a6d2_00065b84435c);
impl core::ops::Deref for ITfReadingInformationUIElement {
    type Target = ITfUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfReadingInformationUIElement, windows_core::IUnknown, ITfUIElement);
impl ITfReadingInformationUIElement {
    pub unsafe fn GetUpdatedFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUpdatedFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<ITfContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetMaxReadingStringLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxReadingStringLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetErrorIndex(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsVerticalOrderPreferred(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsVerticalOrderPreferred)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfReadingInformationUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetUpdatedFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMaxReadingStringLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetErrorIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsVerticalOrderPreferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ITfReadingInformationUIElement_Impl: ITfUIElement_Impl {
    fn GetUpdatedFlags(&self) -> windows_core::Result<u32>;
    fn GetContext(&self) -> windows_core::Result<ITfContext>;
    fn GetString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetMaxReadingStringLength(&self) -> windows_core::Result<u32>;
    fn GetErrorIndex(&self) -> windows_core::Result<u32>;
    fn IsVerticalOrderPreferred(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl ITfReadingInformationUIElement_Vtbl {
    pub const fn new<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUpdatedFlags<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetUpdatedFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetContext(this) {
                Ok(ok__) => {
                    ppic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetString(this) {
                Ok(ok__) => {
                    pstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxReadingStringLength<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchmax: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetMaxReadingStringLength(this) {
                Ok(ok__) => {
                    pcchmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorIndex<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorindex: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::GetErrorIndex(this) {
                Ok(ok__) => {
                    perrorindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVerticalOrderPreferred<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvertical: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReadingInformationUIElement_Impl::IsVerticalOrderPreferred(this) {
                Ok(ok__) => {
                    pfvertical.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetMaxReadingStringLength: GetMaxReadingStringLength::<Identity, OFFSET>,
            GetErrorIndex: GetErrorIndex::<Identity, OFFSET>,
            IsVerticalOrderPreferred: IsVerticalOrderPreferred::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReadingInformationUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfReadingInformationUIElement {}
windows_core::imp::define_interface!(ITfReverseConversion, ITfReverseConversion_Vtbl, 0xa415e162_157d_417d_8a8c_0ab26c7d2781);
windows_core::imp::interface_hierarchy!(ITfReverseConversion, windows_core::IUnknown);
impl ITfReverseConversion {
    pub unsafe fn DoReverseConversion<P0>(&self, lpstr: P0) -> windows_core::Result<ITfReverseConversionList>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoReverseConversion)(windows_core::Interface::as_raw(self), lpstr.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfReverseConversion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoReverseConversion: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfReverseConversion_Impl: windows_core::IUnknownImpl {
    fn DoReverseConversion(&self, lpstr: &windows_core::PCWSTR) -> windows_core::Result<ITfReverseConversionList>;
}
impl ITfReverseConversion_Vtbl {
    pub const fn new<Identity: ITfReverseConversion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoReverseConversion<Identity: ITfReverseConversion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpstr: windows_core::PCWSTR, pplist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReverseConversion_Impl::DoReverseConversion(this, core::mem::transmute(&lpstr)) {
                Ok(ok__) => {
                    pplist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoReverseConversion: DoReverseConversion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReverseConversion as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfReverseConversion {}
windows_core::imp::define_interface!(ITfReverseConversionList, ITfReverseConversionList_Vtbl, 0x151d69f0_86f4_4674_b721_56911e797f47);
windows_core::imp::interface_hierarchy!(ITfReverseConversionList, windows_core::IUnknown);
impl ITfReverseConversionList {
    pub unsafe fn GetLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), uindex, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfReverseConversionList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfReverseConversionList_Impl: windows_core::IUnknownImpl {
    fn GetLength(&self) -> windows_core::Result<u32>;
    fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
}
impl ITfReverseConversionList_Vtbl {
    pub const fn new<Identity: ITfReverseConversionList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLength<Identity: ITfReverseConversionList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puindex: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReverseConversionList_Impl::GetLength(this) {
                Ok(ok__) => {
                    puindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ITfReverseConversionList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReverseConversionList_Impl::GetString(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReverseConversionList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfReverseConversionList {}
windows_core::imp::define_interface!(ITfReverseConversionMgr, ITfReverseConversionMgr_Vtbl, 0xb643c236_c493_41b6_abb3_692412775cc4);
windows_core::imp::interface_hierarchy!(ITfReverseConversionMgr, windows_core::IUnknown);
impl ITfReverseConversionMgr {
    pub unsafe fn GetReverseConversion(&self, langid: u16, guidprofile: *const windows_core::GUID, dwflag: u32) -> windows_core::Result<ITfReverseConversion> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReverseConversion)(windows_core::Interface::as_raw(self), langid, guidprofile, dwflag, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfReverseConversionMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetReverseConversion: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfReverseConversionMgr_Impl: windows_core::IUnknownImpl {
    fn GetReverseConversion(&self, langid: u16, guidprofile: *const windows_core::GUID, dwflag: u32) -> windows_core::Result<ITfReverseConversion>;
}
impl ITfReverseConversionMgr_Vtbl {
    pub const fn new<Identity: ITfReverseConversionMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetReverseConversion<Identity: ITfReverseConversionMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, guidprofile: *const windows_core::GUID, dwflag: u32, ppreverseconversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfReverseConversionMgr_Impl::GetReverseConversion(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&dwflag)) {
                Ok(ok__) => {
                    ppreverseconversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetReverseConversion: GetReverseConversion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReverseConversionMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfReverseConversionMgr {}
windows_core::imp::define_interface!(ITfSource, ITfSource_Vtbl, 0x4ea48a35_60ae_446f_8fd6_e6a8d82459f7);
windows_core::imp::interface_hierarchy!(ITfSource, windows_core::IUnknown);
impl ITfSource {
    pub unsafe fn AdviseSink<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdviseSink)(windows_core::Interface::as_raw(self), riid, punk.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn UnadviseSink(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnadviseSink)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
}
#[repr(C)]
pub struct ITfSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfSource_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn UnadviseSink(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl ITfSource_Vtbl {
    pub const fn new<Identity: ITfSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITfSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfSource_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnadviseSink<Identity: ITfSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSource_Impl::UnadviseSink(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfSource {}
windows_core::imp::define_interface!(ITfSourceSingle, ITfSourceSingle_Vtbl, 0x73131f9c_56a9_49dd_b0ee_d046633f7528);
windows_core::imp::interface_hierarchy!(ITfSourceSingle, windows_core::IUnknown);
impl ITfSourceSingle {
    pub unsafe fn AdviseSingleSink<P2>(&self, tid: u32, riid: *const windows_core::GUID, punk: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AdviseSingleSink)(windows_core::Interface::as_raw(self), tid, riid, punk.param().abi()).ok()
    }
    pub unsafe fn UnadviseSingleSink(&self, tid: u32, riid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnadviseSingleSink)(windows_core::Interface::as_raw(self), tid, riid).ok()
    }
}
#[repr(C)]
pub struct ITfSourceSingle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSingleSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnadviseSingleSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITfSourceSingle_Impl: windows_core::IUnknownImpl {
    fn AdviseSingleSink(&self, tid: u32, riid: *const windows_core::GUID, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UnadviseSingleSink(&self, tid: u32, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ITfSourceSingle_Vtbl {
    pub const fn new<Identity: ITfSourceSingle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSingleSink<Identity: ITfSourceSingle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSourceSingle_Impl::AdviseSingleSink(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn UnadviseSingleSink<Identity: ITfSourceSingle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSourceSingle_Impl::UnadviseSingleSink(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSingleSink: AdviseSingleSink::<Identity, OFFSET>,
            UnadviseSingleSink: UnadviseSingleSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSourceSingle as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfSourceSingle {}
windows_core::imp::define_interface!(ITfSpeechUIServer, ITfSpeechUIServer_Vtbl, 0x90e9a944_9244_489f_a78f_de67afc013a7);
windows_core::imp::interface_hierarchy!(ITfSpeechUIServer, windows_core::IUnknown);
impl ITfSpeechUIServer {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ShowUI(&self, fshow: bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowUI)(windows_core::Interface::as_raw(self), fshow.into()).ok()
    }
    pub unsafe fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateBalloon)(windows_core::Interface::as_raw(self), style, core::mem::transmute(pch.as_ptr()), pch.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct ITfSpeechUIServer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowUI: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub UpdateBalloon: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBBalloonStyle, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
pub trait ITfSpeechUIServer_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn ShowUI(&self, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
}
impl ITfSpeechUIServer_Vtbl {
    pub const fn new<Identity: ITfSpeechUIServer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: ITfSpeechUIServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSpeechUIServer_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn ShowUI<Identity: ITfSpeechUIServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSpeechUIServer_Impl::ShowUI(this, core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn UpdateBalloon<Identity: ITfSpeechUIServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, style: TfLBBalloonStyle, pch: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSpeechUIServer_Impl::UpdateBalloon(this, core::mem::transmute_copy(&style), core::mem::transmute(&pch), core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            ShowUI: ShowUI::<Identity, OFFSET>,
            UpdateBalloon: UpdateBalloon::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSpeechUIServer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfSpeechUIServer {}
windows_core::imp::define_interface!(ITfStatusSink, ITfStatusSink_Vtbl, 0x6b7d8d73_b267_4f69_b32e_1ca321ce4f45);
windows_core::imp::interface_hierarchy!(ITfStatusSink, windows_core::IUnknown);
impl ITfStatusSink {
    pub unsafe fn OnStatusChange<P0>(&self, pic: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfContext>,
    {
        (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), pic.param().abi(), dwflags).ok()
    }
}
#[repr(C)]
pub struct ITfStatusSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfStatusSink_Impl: windows_core::IUnknownImpl {
    fn OnStatusChange(&self, pic: Option<&ITfContext>, dwflags: u32) -> windows_core::Result<()>;
}
impl ITfStatusSink_Vtbl {
    pub const fn new<Identity: ITfStatusSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStatusChange<Identity: ITfStatusSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfStatusSink_Impl::OnStatusChange(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnStatusChange: OnStatusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfStatusSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfStatusSink {}
windows_core::imp::define_interface!(ITfSystemDeviceTypeLangBarItem, ITfSystemDeviceTypeLangBarItem_Vtbl, 0x45672eb9_9059_46a2_838d_4530355f6a77);
windows_core::imp::interface_hierarchy!(ITfSystemDeviceTypeLangBarItem, windows_core::IUnknown);
impl ITfSystemDeviceTypeLangBarItem {
    pub unsafe fn SetIconMode(&self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIconMode)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetIconMode(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIconMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfSystemDeviceTypeLangBarItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIconMode: unsafe extern "system" fn(*mut core::ffi::c_void, LANG_BAR_ITEM_ICON_MODE_FLAGS) -> windows_core::HRESULT,
    pub GetIconMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ITfSystemDeviceTypeLangBarItem_Impl: windows_core::IUnknownImpl {
    fn SetIconMode(&self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> windows_core::Result<()>;
    fn GetIconMode(&self) -> windows_core::Result<u32>;
}
impl ITfSystemDeviceTypeLangBarItem_Vtbl {
    pub const fn new<Identity: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIconMode<Identity: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemDeviceTypeLangBarItem_Impl::SetIconMode(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetIconMode<Identity: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfSystemDeviceTypeLangBarItem_Impl::GetIconMode(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIconMode: SetIconMode::<Identity, OFFSET>,
            GetIconMode: GetIconMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemDeviceTypeLangBarItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfSystemDeviceTypeLangBarItem {}
windows_core::imp::define_interface!(ITfSystemLangBarItem, ITfSystemLangBarItem_Vtbl, 0x1e13e9ec_6b33_4d4a_b5eb_8a92f029f356);
windows_core::imp::interface_hierarchy!(ITfSystemLangBarItem, windows_core::IUnknown);
impl ITfSystemLangBarItem {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetIcon(&self, hicon: super::WindowsAndMessaging::HICON) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIcon)(windows_core::Interface::as_raw(self), hicon).ok()
    }
    pub unsafe fn SetTooltipString(&self, pchtooltip: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTooltipString)(windows_core::Interface::as_raw(self), core::mem::transmute(pchtooltip.as_ptr()), pchtooltip.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct ITfSystemLangBarItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetIcon: usize,
    pub SetTooltipString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ITfSystemLangBarItem_Impl: windows_core::IUnknownImpl {
    fn SetIcon(&self, hicon: super::WindowsAndMessaging::HICON) -> windows_core::Result<()>;
    fn SetTooltipString(&self, pchtooltip: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ITfSystemLangBarItem_Vtbl {
    pub const fn new<Identity: ITfSystemLangBarItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIcon<Identity: ITfSystemLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: super::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItem_Impl::SetIcon(this, core::mem::transmute_copy(&hicon)).into()
        }
        unsafe extern "system" fn SetTooltipString<Identity: ITfSystemLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchtooltip: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItem_Impl::SetTooltipString(this, core::mem::transmute(&pchtooltip), core::mem::transmute_copy(&cch)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIcon: SetIcon::<Identity, OFFSET>,
            SetTooltipString: SetTooltipString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for ITfSystemLangBarItem {}
windows_core::imp::define_interface!(ITfSystemLangBarItemSink, ITfSystemLangBarItemSink_Vtbl, 0x1449d9ab_13cf_4687_aa3e_8d8b18574396);
windows_core::imp::interface_hierarchy!(ITfSystemLangBarItemSink, windows_core::IUnknown);
impl ITfSystemLangBarItemSink {
    pub unsafe fn InitMenu<P0>(&self, pmenu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfMenu>,
    {
        (windows_core::Interface::vtable(self).InitMenu)(windows_core::Interface::as_raw(self), pmenu.param().abi()).ok()
    }
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMenuSelect)(windows_core::Interface::as_raw(self), wid).ok()
    }
}
#[repr(C)]
pub struct ITfSystemLangBarItemSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnMenuSelect: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfSystemLangBarItemSink_Impl: windows_core::IUnknownImpl {
    fn InitMenu(&self, pmenu: Option<&ITfMenu>) -> windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: u32) -> windows_core::Result<()>;
}
impl ITfSystemLangBarItemSink_Vtbl {
    pub const fn new<Identity: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitMenu<Identity: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmenu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItemSink_Impl::InitMenu(this, windows_core::from_raw_borrowed(&pmenu)).into()
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItemSink_Impl::OnMenuSelect(this, core::mem::transmute_copy(&wid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitMenu: InitMenu::<Identity, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfSystemLangBarItemSink {}
windows_core::imp::define_interface!(ITfSystemLangBarItemText, ITfSystemLangBarItemText_Vtbl, 0x5c4ce0e5_ba49_4b52_ac6b_3b397b4f701f);
windows_core::imp::interface_hierarchy!(ITfSystemLangBarItemText, windows_core::IUnknown);
impl ITfSystemLangBarItemText {
    pub unsafe fn SetItemText(&self, pch: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetItemText)(windows_core::Interface::as_raw(self), core::mem::transmute(pch.as_ptr()), pch.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetItemText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfSystemLangBarItemText_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetItemText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetItemText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfSystemLangBarItemText_Impl: windows_core::IUnknownImpl {
    fn SetItemText(&self, pch: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
    fn GetItemText(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITfSystemLangBarItemText_Vtbl {
    pub const fn new<Identity: ITfSystemLangBarItemText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetItemText<Identity: ITfSystemLangBarItemText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pch: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfSystemLangBarItemText_Impl::SetItemText(this, core::mem::transmute(&pch), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn GetItemText<Identity: ITfSystemLangBarItemText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfSystemLangBarItemText_Impl::GetItemText(this) {
                Ok(ok__) => {
                    pbstrtext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetItemText: SetItemText::<Identity, OFFSET>,
            GetItemText: GetItemText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemText as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfSystemLangBarItemText {}
windows_core::imp::define_interface!(ITfTextEditSink, ITfTextEditSink_Vtbl, 0x8127d409_ccd3_4683_967a_b43d5b482bf7);
windows_core::imp::interface_hierarchy!(ITfTextEditSink, windows_core::IUnknown);
impl ITfTextEditSink {
    pub unsafe fn OnEndEdit<P0, P2>(&self, pic: P0, ecreadonly: u32, peditrecord: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfContext>,
        P2: windows_core::Param<ITfEditRecord>,
    {
        (windows_core::Interface::vtable(self).OnEndEdit)(windows_core::Interface::as_raw(self), pic.param().abi(), ecreadonly, peditrecord.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfTextEditSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnEndEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfTextEditSink_Impl: windows_core::IUnknownImpl {
    fn OnEndEdit(&self, pic: Option<&ITfContext>, ecreadonly: u32, peditrecord: Option<&ITfEditRecord>) -> windows_core::Result<()>;
}
impl ITfTextEditSink_Vtbl {
    pub const fn new<Identity: ITfTextEditSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEndEdit<Identity: ITfTextEditSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, ecreadonly: u32, peditrecord: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextEditSink_Impl::OnEndEdit(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&ecreadonly), windows_core::from_raw_borrowed(&peditrecord)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEndEdit: OnEndEdit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextEditSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTextEditSink {}
windows_core::imp::define_interface!(ITfTextInputProcessor, ITfTextInputProcessor_Vtbl, 0xaa80e7f7_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfTextInputProcessor, windows_core::IUnknown);
impl ITfTextInputProcessor {
    pub unsafe fn Activate<P0>(&self, ptim: P0, tid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfThreadMgr>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), ptim.param().abi(), tid).ok()
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfTextInputProcessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfTextInputProcessor_Impl: windows_core::IUnknownImpl {
    fn Activate(&self, ptim: Option<&ITfThreadMgr>, tid: u32) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<()>;
}
impl ITfTextInputProcessor_Vtbl {
    pub const fn new<Identity: ITfTextInputProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: ITfTextInputProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptim: *mut core::ffi::c_void, tid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextInputProcessor_Impl::Activate(this, windows_core::from_raw_borrowed(&ptim), core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ITfTextInputProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextInputProcessor_Impl::Deactivate(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextInputProcessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTextInputProcessor {}
windows_core::imp::define_interface!(ITfTextInputProcessorEx, ITfTextInputProcessorEx_Vtbl, 0x6e4e2102_f9cd_433d_b496_303ce03a6507);
impl core::ops::Deref for ITfTextInputProcessorEx {
    type Target = ITfTextInputProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfTextInputProcessorEx, windows_core::IUnknown, ITfTextInputProcessor);
impl ITfTextInputProcessorEx {
    pub unsafe fn ActivateEx<P0>(&self, ptim: P0, tid: u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfThreadMgr>,
    {
        (windows_core::Interface::vtable(self).ActivateEx)(windows_core::Interface::as_raw(self), ptim.param().abi(), tid, dwflags).ok()
    }
}
#[repr(C)]
pub struct ITfTextInputProcessorEx_Vtbl {
    pub base__: ITfTextInputProcessor_Vtbl,
    pub ActivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait ITfTextInputProcessorEx_Impl: ITfTextInputProcessor_Impl {
    fn ActivateEx(&self, ptim: Option<&ITfThreadMgr>, tid: u32, dwflags: u32) -> windows_core::Result<()>;
}
impl ITfTextInputProcessorEx_Vtbl {
    pub const fn new<Identity: ITfTextInputProcessorEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateEx<Identity: ITfTextInputProcessorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptim: *mut core::ffi::c_void, tid: u32, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextInputProcessorEx_Impl::ActivateEx(this, windows_core::from_raw_borrowed(&ptim), core::mem::transmute_copy(&tid), core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: ITfTextInputProcessor_Vtbl::new::<Identity, OFFSET>(), ActivateEx: ActivateEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextInputProcessorEx as windows_core::Interface>::IID || iid == &<ITfTextInputProcessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTextInputProcessorEx {}
windows_core::imp::define_interface!(ITfTextLayoutSink, ITfTextLayoutSink_Vtbl, 0x2af2d06a_dd5b_4927_a0b4_54f19c91fade);
windows_core::imp::interface_hierarchy!(ITfTextLayoutSink, windows_core::IUnknown);
impl ITfTextLayoutSink {
    pub unsafe fn OnLayoutChange<P0, P2>(&self, pic: P0, lcode: TfLayoutCode, pview: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfContext>,
        P2: windows_core::Param<ITfContextView>,
    {
        (windows_core::Interface::vtable(self).OnLayoutChange)(windows_core::Interface::as_raw(self), pic.param().abi(), lcode, pview.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfTextLayoutSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TfLayoutCode, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfTextLayoutSink_Impl: windows_core::IUnknownImpl {
    fn OnLayoutChange(&self, pic: Option<&ITfContext>, lcode: TfLayoutCode, pview: Option<&ITfContextView>) -> windows_core::Result<()>;
}
impl ITfTextLayoutSink_Vtbl {
    pub const fn new<Identity: ITfTextLayoutSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLayoutChange<Identity: ITfTextLayoutSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, lcode: TfLayoutCode, pview: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfTextLayoutSink_Impl::OnLayoutChange(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&lcode), windows_core::from_raw_borrowed(&pview)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnLayoutChange: OnLayoutChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextLayoutSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTextLayoutSink {}
windows_core::imp::define_interface!(ITfThreadFocusSink, ITfThreadFocusSink_Vtbl, 0xc0f1db0c_3a20_405c_a303_96b6010a885f);
windows_core::imp::interface_hierarchy!(ITfThreadFocusSink, windows_core::IUnknown);
impl ITfThreadFocusSink {
    pub unsafe fn OnSetThreadFocus(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnSetThreadFocus)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnKillThreadFocus(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnKillThreadFocus)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfThreadFocusSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSetThreadFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnKillThreadFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfThreadFocusSink_Impl: windows_core::IUnknownImpl {
    fn OnSetThreadFocus(&self) -> windows_core::Result<()>;
    fn OnKillThreadFocus(&self) -> windows_core::Result<()>;
}
impl ITfThreadFocusSink_Vtbl {
    pub const fn new<Identity: ITfThreadFocusSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetThreadFocus<Identity: ITfThreadFocusSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadFocusSink_Impl::OnSetThreadFocus(this).into()
        }
        unsafe extern "system" fn OnKillThreadFocus<Identity: ITfThreadFocusSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadFocusSink_Impl::OnKillThreadFocus(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetThreadFocus: OnSetThreadFocus::<Identity, OFFSET>,
            OnKillThreadFocus: OnKillThreadFocus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadFocusSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfThreadFocusSink {}
windows_core::imp::define_interface!(ITfThreadMgr, ITfThreadMgr_Vtbl, 0xaa80e801_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfThreadMgr, windows_core::IUnknown);
impl ITfThreadMgr {
    pub unsafe fn Activate(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDocumentMgrs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFocus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFocus<P0>(&self, pdimfocus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfDocumentMgr>,
    {
        (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self), pdimfocus.param().abi()).ok()
    }
    pub unsafe fn AssociateFocus<P1>(&self, hwnd: super::super::Foundation::HWND, pdimnew: P1) -> windows_core::Result<ITfDocumentMgr>
    where
        P1: windows_core::Param<ITfDocumentMgr>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AssociateFocus)(windows_core::Interface::as_raw(self), hwnd, pdimnew.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsThreadFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsThreadFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFunctionProvider)(windows_core::Interface::as_raw(self), clsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFunctionProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlobalCompartment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfThreadMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDocumentMgrs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AssociateFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsThreadFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFunctionProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumFunctionProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGlobalCompartment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfThreadMgr_Impl: windows_core::IUnknownImpl {
    fn Activate(&self) -> windows_core::Result<u32>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(&self, pdimfocus: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn AssociateFocus(&self, hwnd: super::super::Foundation::HWND, pdimnew: Option<&ITfDocumentMgr>) -> windows_core::Result<ITfDocumentMgr>;
    fn IsThreadFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr>;
}
impl ITfThreadMgr_Vtbl {
    pub const fn new<Identity: ITfThreadMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::Activate(this) {
                Ok(ok__) => {
                    ptid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr_Impl::Deactivate(this).into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::CreateDocumentMgr(this) {
                Ok(ok__) => {
                    ppdim.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::EnumDocumentMgrs(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdimfocus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::GetFocus(this) {
                Ok(ok__) => {
                    ppdimfocus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdimfocus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr_Impl::SetFocus(this, windows_core::from_raw_borrowed(&pdimfocus)).into()
        }
        unsafe extern "system" fn AssociateFocus<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: *mut core::ffi::c_void, ppdimprev: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::AssociateFocus(this, core::mem::transmute_copy(&hwnd), windows_core::from_raw_borrowed(&pdimnew)) {
                Ok(ok__) => {
                    ppdimprev.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreadFocus<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::IsThreadFocus(this) {
                Ok(ok__) => {
                    pfthreadfocus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, ppfuncprov: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::GetFunctionProvider(this, core::mem::transmute_copy(&clsid)) {
                Ok(ok__) => {
                    ppfuncprov.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::EnumFunctionProviders(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcompmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr_Impl::GetGlobalCompartment(this) {
                Ok(ok__) => {
                    ppcompmgr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, OFFSET>,
            GetFocus: GetFocus::<Identity, OFFSET>,
            SetFocus: SetFocus::<Identity, OFFSET>,
            AssociateFocus: AssociateFocus::<Identity, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfThreadMgr {}
windows_core::imp::define_interface!(ITfThreadMgr2, ITfThreadMgr2_Vtbl, 0x0ab198ef_6477_4ee8_8812_6780edb82d5e);
windows_core::imp::interface_hierarchy!(ITfThreadMgr2, windows_core::IUnknown);
impl ITfThreadMgr2 {
    pub unsafe fn Activate(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDocumentMgrs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFocus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFocus<P0>(&self, pdimfocus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfDocumentMgr>,
    {
        (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self), pdimfocus.param().abi()).ok()
    }
    pub unsafe fn IsThreadFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsThreadFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFunctionProvider)(windows_core::Interface::as_raw(self), clsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFunctionProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlobalCompartment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ActivateEx)(windows_core::Interface::as_raw(self), core::mem::transmute(ptid), dwflags).ok()
    }
    pub unsafe fn GetActiveFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SuspendKeystrokeHandling(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuspendKeystrokeHandling)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeKeystrokeHandling(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResumeKeystrokeHandling)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITfThreadMgr2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDocumentMgrs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsThreadFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFunctionProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumFunctionProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGlobalCompartment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub GetActiveFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SuspendKeystrokeHandling: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeKeystrokeHandling: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfThreadMgr2_Impl: windows_core::IUnknownImpl {
    fn Activate(&self) -> windows_core::Result<u32>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(&self, pdimfocus: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn IsThreadFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr>;
    fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetActiveFlags(&self) -> windows_core::Result<u32>;
    fn SuspendKeystrokeHandling(&self) -> windows_core::Result<()>;
    fn ResumeKeystrokeHandling(&self) -> windows_core::Result<()>;
}
impl ITfThreadMgr2_Vtbl {
    pub const fn new<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::Activate(this) {
                Ok(ok__) => {
                    ptid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::Deactivate(this).into()
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::CreateDocumentMgr(this) {
                Ok(ok__) => {
                    ppdim.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::EnumDocumentMgrs(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdimfocus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::GetFocus(this) {
                Ok(ok__) => {
                    ppdimfocus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdimfocus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::SetFocus(this, windows_core::from_raw_borrowed(&pdimfocus)).into()
        }
        unsafe extern "system" fn IsThreadFocus<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::IsThreadFocus(this) {
                Ok(ok__) => {
                    pfthreadfocus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, ppfuncprov: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::GetFunctionProvider(this, core::mem::transmute_copy(&clsid)) {
                Ok(ok__) => {
                    ppfuncprov.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::EnumFunctionProviders(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcompmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::GetGlobalCompartment(this) {
                Ok(ok__) => {
                    ppcompmgr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateEx<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::ActivateEx(this, core::mem::transmute_copy(&ptid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgr2_Impl::GetActiveFlags(this) {
                Ok(ok__) => {
                    lpdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendKeystrokeHandling<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::SuspendKeystrokeHandling(this).into()
        }
        unsafe extern "system" fn ResumeKeystrokeHandling<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgr2_Impl::ResumeKeystrokeHandling(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, OFFSET>,
            GetFocus: GetFocus::<Identity, OFFSET>,
            SetFocus: SetFocus::<Identity, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, OFFSET>,
            ActivateEx: ActivateEx::<Identity, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, OFFSET>,
            SuspendKeystrokeHandling: SuspendKeystrokeHandling::<Identity, OFFSET>,
            ResumeKeystrokeHandling: ResumeKeystrokeHandling::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgr2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfThreadMgr2 {}
windows_core::imp::define_interface!(ITfThreadMgrEventSink, ITfThreadMgrEventSink_Vtbl, 0xaa80e80e_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfThreadMgrEventSink, windows_core::IUnknown);
impl ITfThreadMgrEventSink {
    pub unsafe fn OnInitDocumentMgr<P0>(&self, pdim: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfDocumentMgr>,
    {
        (windows_core::Interface::vtable(self).OnInitDocumentMgr)(windows_core::Interface::as_raw(self), pdim.param().abi()).ok()
    }
    pub unsafe fn OnUninitDocumentMgr<P0>(&self, pdim: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfDocumentMgr>,
    {
        (windows_core::Interface::vtable(self).OnUninitDocumentMgr)(windows_core::Interface::as_raw(self), pdim.param().abi()).ok()
    }
    pub unsafe fn OnSetFocus<P0, P1>(&self, pdimfocus: P0, pdimprevfocus: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfDocumentMgr>,
        P1: windows_core::Param<ITfDocumentMgr>,
    {
        (windows_core::Interface::vtable(self).OnSetFocus)(windows_core::Interface::as_raw(self), pdimfocus.param().abi(), pdimprevfocus.param().abi()).ok()
    }
    pub unsafe fn OnPushContext<P0>(&self, pic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfContext>,
    {
        (windows_core::Interface::vtable(self).OnPushContext)(windows_core::Interface::as_raw(self), pic.param().abi()).ok()
    }
    pub unsafe fn OnPopContext<P0>(&self, pic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfContext>,
    {
        (windows_core::Interface::vtable(self).OnPopContext)(windows_core::Interface::as_raw(self), pic.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITfThreadMgrEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnInitDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnUninitDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPushContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPopContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfThreadMgrEventSink_Impl: windows_core::IUnknownImpl {
    fn OnInitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn OnUninitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn OnSetFocus(&self, pdimfocus: Option<&ITfDocumentMgr>, pdimprevfocus: Option<&ITfDocumentMgr>) -> windows_core::Result<()>;
    fn OnPushContext(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
    fn OnPopContext(&self, pic: Option<&ITfContext>) -> windows_core::Result<()>;
}
impl ITfThreadMgrEventSink_Vtbl {
    pub const fn new<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInitDocumentMgr<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdim: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnInitDocumentMgr(this, windows_core::from_raw_borrowed(&pdim)).into()
        }
        unsafe extern "system" fn OnUninitDocumentMgr<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdim: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnUninitDocumentMgr(this, windows_core::from_raw_borrowed(&pdim)).into()
        }
        unsafe extern "system" fn OnSetFocus<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdimfocus: *mut core::ffi::c_void, pdimprevfocus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnSetFocus(this, windows_core::from_raw_borrowed(&pdimfocus), windows_core::from_raw_borrowed(&pdimprevfocus)).into()
        }
        unsafe extern "system" fn OnPushContext<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnPushContext(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        unsafe extern "system" fn OnPopContext<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEventSink_Impl::OnPopContext(this, windows_core::from_raw_borrowed(&pic)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInitDocumentMgr: OnInitDocumentMgr::<Identity, OFFSET>,
            OnUninitDocumentMgr: OnUninitDocumentMgr::<Identity, OFFSET>,
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnPushContext: OnPushContext::<Identity, OFFSET>,
            OnPopContext: OnPopContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgrEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfThreadMgrEventSink {}
windows_core::imp::define_interface!(ITfThreadMgrEx, ITfThreadMgrEx_Vtbl, 0x3e90ade3_7594_4cb0_bb58_69628f5f458c);
impl core::ops::Deref for ITfThreadMgrEx {
    type Target = ITfThreadMgr;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfThreadMgrEx, windows_core::IUnknown, ITfThreadMgr);
impl ITfThreadMgrEx {
    pub unsafe fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ActivateEx)(windows_core::Interface::as_raw(self), core::mem::transmute(ptid), dwflags).ok()
    }
    pub unsafe fn GetActiveFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActiveFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfThreadMgrEx_Vtbl {
    pub base__: ITfThreadMgr_Vtbl,
    pub ActivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub GetActiveFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ITfThreadMgrEx_Impl: ITfThreadMgr_Impl {
    fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetActiveFlags(&self) -> windows_core::Result<u32>;
}
impl ITfThreadMgrEx_Vtbl {
    pub const fn new<Identity: ITfThreadMgrEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateEx<Identity: ITfThreadMgrEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfThreadMgrEx_Impl::ActivateEx(this, core::mem::transmute_copy(&ptid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetActiveFlags<Identity: ITfThreadMgrEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfThreadMgrEx_Impl::GetActiveFlags(this) {
                Ok(ok__) => {
                    lpdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ITfThreadMgr_Vtbl::new::<Identity, OFFSET>(),
            ActivateEx: ActivateEx::<Identity, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgrEx as windows_core::Interface>::IID || iid == &<ITfThreadMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfThreadMgrEx {}
windows_core::imp::define_interface!(ITfToolTipUIElement, ITfToolTipUIElement_Vtbl, 0x52b18b5c_555d_46b2_b00a_fa680144fbdb);
impl core::ops::Deref for ITfToolTipUIElement {
    type Target = ITfUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfToolTipUIElement, windows_core::IUnknown, ITfUIElement);
impl ITfToolTipUIElement {
    pub unsafe fn GetString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct ITfToolTipUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfToolTipUIElement_Impl: ITfUIElement_Impl {
    fn GetString(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITfToolTipUIElement_Vtbl {
    pub const fn new<Identity: ITfToolTipUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetString<Identity: ITfToolTipUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfToolTipUIElement_Impl::GetString(this) {
                Ok(ok__) => {
                    pstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(), GetString: GetString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfToolTipUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfToolTipUIElement {}
windows_core::imp::define_interface!(ITfTransitoryExtensionSink, ITfTransitoryExtensionSink_Vtbl, 0xa615096f_1c57_4813_8a15_55ee6e5a839c);
windows_core::imp::interface_hierarchy!(ITfTransitoryExtensionSink, windows_core::IUnknown);
impl ITfTransitoryExtensionSink {
    pub unsafe fn OnTransitoryExtensionUpdated<P0, P2, P3>(&self, pic: P0, ecreadonly: u32, presultrange: P2, pcompositionrange: P3) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
        P2: windows_core::Param<ITfRange>,
        P3: windows_core::Param<ITfRange>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnTransitoryExtensionUpdated)(windows_core::Interface::as_raw(self), pic.param().abi(), ecreadonly, presultrange.param().abi(), pcompositionrange.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfTransitoryExtensionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTransitoryExtensionUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ITfTransitoryExtensionSink_Impl: windows_core::IUnknownImpl {
    fn OnTransitoryExtensionUpdated(&self, pic: Option<&ITfContext>, ecreadonly: u32, presultrange: Option<&ITfRange>, pcompositionrange: Option<&ITfRange>) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl ITfTransitoryExtensionSink_Vtbl {
    pub const fn new<Identity: ITfTransitoryExtensionSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTransitoryExtensionUpdated<Identity: ITfTransitoryExtensionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, ecreadonly: u32, presultrange: *mut core::ffi::c_void, pcompositionrange: *mut core::ffi::c_void, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfTransitoryExtensionSink_Impl::OnTransitoryExtensionUpdated(this, windows_core::from_raw_borrowed(&pic), core::mem::transmute_copy(&ecreadonly), windows_core::from_raw_borrowed(&presultrange), windows_core::from_raw_borrowed(&pcompositionrange)) {
                Ok(ok__) => {
                    pfdeleteresultrange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTransitoryExtensionUpdated: OnTransitoryExtensionUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTransitoryExtensionSink {}
windows_core::imp::define_interface!(ITfTransitoryExtensionUIElement, ITfTransitoryExtensionUIElement_Vtbl, 0x858f956a_972f_42a2_a2f2_0321e1abe209);
impl core::ops::Deref for ITfTransitoryExtensionUIElement {
    type Target = ITfUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfTransitoryExtensionUIElement, windows_core::IUnknown, ITfUIElement);
impl ITfTransitoryExtensionUIElement {
    pub unsafe fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfTransitoryExtensionUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfTransitoryExtensionUIElement_Impl: ITfUIElement_Impl {
    fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
}
impl ITfTransitoryExtensionUIElement_Vtbl {
    pub const fn new<Identity: ITfTransitoryExtensionUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentMgr<Identity: ITfTransitoryExtensionUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfTransitoryExtensionUIElement_Impl::GetDocumentMgr(this) {
                Ok(ok__) => {
                    ppdim.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(), GetDocumentMgr: GetDocumentMgr::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTransitoryExtensionUIElement {}
windows_core::imp::define_interface!(ITfUIElement, ITfUIElement_Vtbl, 0xea1ea137_19df_11d7_a6d2_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfUIElement, windows_core::IUnknown);
impl ITfUIElement {
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetGUID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Show(&self, bshow: bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), bshow.into()).ok()
    }
    pub unsafe fn IsShown(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsShown)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITfUIElement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsShown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ITfUIElement_Impl: windows_core::IUnknownImpl {
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn Show(&self, bshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsShown(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl ITfUIElement_Vtbl {
    pub const fn new<Identity: ITfUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDescription<Identity: ITfUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElement_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGUID<Identity: ITfUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElement_Impl::GetGUID(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Identity: ITfUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElement_Impl::Show(this, core::mem::transmute_copy(&bshow)).into()
        }
        unsafe extern "system" fn IsShown<Identity: ITfUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElement_Impl::IsShown(this) {
                Ok(ok__) => {
                    pbshow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetGUID: GetGUID::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            IsShown: IsShown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfUIElement {}
windows_core::imp::define_interface!(ITfUIElementMgr, ITfUIElementMgr_Vtbl, 0xea1ea135_19df_11d7_a6d2_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfUIElementMgr, windows_core::IUnknown);
impl ITfUIElementMgr {
    pub unsafe fn BeginUIElement<P0>(&self, pelement: P0, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITfUIElement>,
    {
        (windows_core::Interface::vtable(self).BeginUIElement)(windows_core::Interface::as_raw(self), pelement.param().abi(), core::mem::transmute(pbshow), core::mem::transmute(pdwuielementid)).ok()
    }
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateUIElement)(windows_core::Interface::as_raw(self), dwuielementid).ok()
    }
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndUIElement)(windows_core::Interface::as_raw(self), dwuielementid).ok()
    }
    pub unsafe fn GetUIElement(&self, dwuielementid: u32) -> windows_core::Result<ITfUIElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUIElement)(windows_core::Interface::as_raw(self), dwuielementid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumUIElements(&self) -> windows_core::Result<IEnumTfUIElements> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumUIElements)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITfUIElementMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut u32) -> windows_core::HRESULT,
    pub UpdateUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EndUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumUIElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfUIElementMgr_Impl: windows_core::IUnknownImpl {
    fn BeginUIElement(&self, pelement: Option<&ITfUIElement>, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> windows_core::Result<()>;
    fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
    fn EndUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
    fn GetUIElement(&self, dwuielementid: u32) -> windows_core::Result<ITfUIElement>;
    fn EnumUIElements(&self) -> windows_core::Result<IEnumTfUIElements>;
}
impl ITfUIElementMgr_Vtbl {
    pub const fn new<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginUIElement<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementMgr_Impl::BeginUIElement(this, windows_core::from_raw_borrowed(&pelement), core::mem::transmute_copy(&pbshow), core::mem::transmute_copy(&pdwuielementid)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementMgr_Impl::UpdateUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementMgr_Impl::EndUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn GetUIElement<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32, ppelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElementMgr_Impl::GetUIElement(this, core::mem::transmute_copy(&dwuielementid)) {
                Ok(ok__) => {
                    ppelement.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumUIElements<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITfUIElementMgr_Impl::EnumUIElements(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginUIElement: BeginUIElement::<Identity, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, OFFSET>,
            EndUIElement: EndUIElement::<Identity, OFFSET>,
            GetUIElement: GetUIElement::<Identity, OFFSET>,
            EnumUIElements: EnumUIElements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfUIElementMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfUIElementMgr {}
windows_core::imp::define_interface!(ITfUIElementSink, ITfUIElementSink_Vtbl, 0xea1ea136_19df_11d7_a6d2_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfUIElementSink, windows_core::IUnknown);
impl ITfUIElementSink {
    pub unsafe fn BeginUIElement(&self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginUIElement)(windows_core::Interface::as_raw(self), dwuielementid, core::mem::transmute(pbshow)).ok()
    }
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateUIElement)(windows_core::Interface::as_raw(self), dwuielementid).ok()
    }
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndUIElement)(windows_core::Interface::as_raw(self), dwuielementid).ok()
    }
}
#[repr(C)]
pub struct ITfUIElementSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub UpdateUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EndUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfUIElementSink_Impl: windows_core::IUnknownImpl {
    fn BeginUIElement(&self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
    fn EndUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
}
impl ITfUIElementSink_Vtbl {
    pub const fn new<Identity: ITfUIElementSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginUIElement<Identity: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementSink_Impl::BeginUIElement(this, core::mem::transmute_copy(&dwuielementid), core::mem::transmute_copy(&pbshow)).into()
        }
        unsafe extern "system" fn UpdateUIElement<Identity: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementSink_Impl::UpdateUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
        }
        unsafe extern "system" fn EndUIElement<Identity: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITfUIElementSink_Impl::EndUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginUIElement: BeginUIElement::<Identity, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, OFFSET>,
            EndUIElement: EndUIElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfUIElementSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfUIElementSink {}
windows_core::imp::define_interface!(IUIManagerEventSink, IUIManagerEventSink_Vtbl, 0xcd91d690_a7e8_4265_9b38_8bb3bbaba7de);
windows_core::imp::interface_hierarchy!(IUIManagerEventSink, windows_core::IUnknown);
impl IUIManagerEventSink {
    pub unsafe fn OnWindowOpening(&self, prcbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnWindowOpening)(windows_core::Interface::as_raw(self), prcbounds).ok()
    }
    pub unsafe fn OnWindowOpened(&self, prcbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnWindowOpened)(windows_core::Interface::as_raw(self), prcbounds).ok()
    }
    pub unsafe fn OnWindowUpdating(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnWindowUpdating)(windows_core::Interface::as_raw(self), prcupdatedbounds).ok()
    }
    pub unsafe fn OnWindowUpdated(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnWindowUpdated)(windows_core::Interface::as_raw(self), prcupdatedbounds).ok()
    }
    pub unsafe fn OnWindowClosing(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnWindowClosing)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnWindowClosed(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnWindowClosed)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIManagerEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnWindowOpening: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub OnWindowOpened: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub OnWindowUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub OnWindowUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub OnWindowClosing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnWindowClosed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIManagerEventSink_Impl: windows_core::IUnknownImpl {
    fn OnWindowOpening(&self, prcbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnWindowOpened(&self, prcbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnWindowUpdating(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnWindowUpdated(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnWindowClosing(&self) -> windows_core::Result<()>;
    fn OnWindowClosed(&self) -> windows_core::Result<()>;
}
impl IUIManagerEventSink_Vtbl {
    pub const fn new<Identity: IUIManagerEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnWindowOpening<Identity: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowOpening(this, core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowOpened<Identity: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowOpened(this, core::mem::transmute_copy(&prcbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdating<Identity: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowUpdating(this, core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowUpdated<Identity: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowUpdated(this, core::mem::transmute_copy(&prcupdatedbounds)).into()
        }
        unsafe extern "system" fn OnWindowClosing<Identity: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowClosing(this).into()
        }
        unsafe extern "system" fn OnWindowClosed<Identity: IUIManagerEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIManagerEventSink_Impl::OnWindowClosed(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnWindowOpening: OnWindowOpening::<Identity, OFFSET>,
            OnWindowOpened: OnWindowOpened::<Identity, OFFSET>,
            OnWindowUpdating: OnWindowUpdating::<Identity, OFFSET>,
            OnWindowUpdated: OnWindowUpdated::<Identity, OFFSET>,
            OnWindowClosing: OnWindowClosing::<Identity, OFFSET>,
            OnWindowClosed: OnWindowClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIManagerEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIManagerEventSink {}
windows_core::imp::define_interface!(IVersionInfo, IVersionInfo_Vtbl, 0x401518ec_db00_4611_9b29_2a0e4b9afa85);
windows_core::imp::interface_hierarchy!(IVersionInfo, windows_core::IUnknown);
impl IVersionInfo {
    pub unsafe fn GetSubcomponentCount(&self, ulsub: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubcomponentCount)(windows_core::Interface::as_raw(self), ulsub, &mut result__).map(|| result__)
    }
    pub unsafe fn GetImplementationID(&self, ulsub: u32) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImplementationID)(windows_core::Interface::as_raw(self), ulsub, &mut result__).map(|| result__)
    }
    pub unsafe fn GetBuildVersion(&self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBuildVersion)(windows_core::Interface::as_raw(self), ulsub, core::mem::transmute(pdwmajor), core::mem::transmute(pdwminor)).ok()
    }
    pub unsafe fn GetComponentDescription(&self, ulsub: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetComponentDescription)(windows_core::Interface::as_raw(self), ulsub, &mut result__).map(|| core::mem::transmute(result__))
    }
    pub unsafe fn GetInstanceDescription(&self, ulsub: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInstanceDescription)(windows_core::Interface::as_raw(self), ulsub, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(C)]
pub struct IVersionInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSubcomponentCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetImplementationID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetBuildVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetComponentDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInstanceDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVersionInfo_Impl: windows_core::IUnknownImpl {
    fn GetSubcomponentCount(&self, ulsub: u32) -> windows_core::Result<u32>;
    fn GetImplementationID(&self, ulsub: u32) -> windows_core::Result<windows_core::GUID>;
    fn GetBuildVersion(&self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::Result<()>;
    fn GetComponentDescription(&self, ulsub: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetInstanceDescription(&self, ulsub: u32) -> windows_core::Result<windows_core::BSTR>;
}
impl IVersionInfo_Vtbl {
    pub const fn new<Identity: IVersionInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSubcomponentCount<Identity: IVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, ulcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVersionInfo_Impl::GetSubcomponentCount(this, core::mem::transmute_copy(&ulsub)) {
                Ok(ok__) => {
                    ulcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplementationID<Identity: IVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, implid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVersionInfo_Impl::GetImplementationID(this, core::mem::transmute_copy(&ulsub)) {
                Ok(ok__) => {
                    implid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuildVersion<Identity: IVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVersionInfo_Impl::GetBuildVersion(this, core::mem::transmute_copy(&ulsub), core::mem::transmute_copy(&pdwmajor), core::mem::transmute_copy(&pdwminor)).into()
        }
        unsafe extern "system" fn GetComponentDescription<Identity: IVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, pimplstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVersionInfo_Impl::GetComponentDescription(this, core::mem::transmute_copy(&ulsub)) {
                Ok(ok__) => {
                    pimplstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceDescription<Identity: IVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsub: u32, pimplstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVersionInfo_Impl::GetInstanceDescription(this, core::mem::transmute_copy(&ulsub)) {
                Ok(ok__) => {
                    pimplstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSubcomponentCount: GetSubcomponentCount::<Identity, OFFSET>,
            GetImplementationID: GetImplementationID::<Identity, OFFSET>,
            GetBuildVersion: GetBuildVersion::<Identity, OFFSET>,
            GetComponentDescription: GetComponentDescription::<Identity, OFFSET>,
            GetInstanceDescription: GetInstanceDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVersionInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVersionInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputScope(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LANG_BAR_ITEM_ICON_MODE_FLAGS(pub u32);
pub const LIBID_MSAATEXTLib: windows_core::GUID = windows_core::GUID::from_u128(0x150e2d7a_dac1_4582_947d_2a8fd78b82cd);
pub const MSAAControl: windows_core::GUID = windows_core::GUID::from_u128(0x08cd963f_7a3e_4f5c_9bd8_d692bb043c5b);
pub const STYLE_ACTIVE_SELECTION: TfIntegratableCandidateListSelectionStyle = TfIntegratableCandidateListSelectionStyle(0i32);
pub const STYLE_IMPLIED_SELECTION: TfIntegratableCandidateListSelectionStyle = TfIntegratableCandidateListSelectionStyle(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEXT_STORE_CHANGE_FLAGS(pub u32);
impl TEXT_STORE_CHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TEXT_STORE_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TEXT_STORE_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEXT_STORE_LOCK_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEXT_STORE_TEXT_CHANGE_FLAGS(pub u32);
impl TEXT_STORE_TEXT_CHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TF_AE_END: TfActiveSelEnd = TfActiveSelEnd(2i32);
pub const TF_AE_NONE: TfActiveSelEnd = TfActiveSelEnd(0i32);
pub const TF_AE_START: TfActiveSelEnd = TfActiveSelEnd(1i32);
pub const TF_ANCHOR_END: TfAnchor = TfAnchor(1i32);
pub const TF_ANCHOR_START: TfAnchor = TfAnchor(0i32);
pub const TF_ATTR_CONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(2i32);
pub const TF_ATTR_FIXEDCONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(5i32);
pub const TF_ATTR_INPUT: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(0i32);
pub const TF_ATTR_INPUT_ERROR: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(4i32);
pub const TF_ATTR_OTHER: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(-1i32);
pub const TF_ATTR_TARGET_CONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(1i32);
pub const TF_ATTR_TARGET_NOTCONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(3i32);
pub const TF_CHAR_EMBEDDED: u32 = 65532u32;
pub const TF_CLUIE_COUNT: u32 = 2u32;
pub const TF_CLUIE_CURRENTPAGE: u32 = 32u32;
pub const TF_CLUIE_DOCUMENTMGR: u32 = 1u32;
pub const TF_CLUIE_PAGEINDEX: u32 = 16u32;
pub const TF_CLUIE_SELECTION: u32 = 4u32;
pub const TF_CLUIE_STRING: u32 = 8u32;
pub const TF_COMMANDING_ENABLED: u32 = 4u32;
pub const TF_COMMANDING_ON: u32 = 8u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TF_CONTEXT_EDIT_CONTEXT_FLAGS(pub u32);
impl TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TF_CONVERSIONMODE_ALPHANUMERIC: u32 = 0u32;
pub const TF_CONVERSIONMODE_CHARCODE: u32 = 32u32;
pub const TF_CONVERSIONMODE_EUDC: u32 = 512u32;
pub const TF_CONVERSIONMODE_FIXED: u32 = 2048u32;
pub const TF_CONVERSIONMODE_FULLSHAPE: u32 = 8u32;
pub const TF_CONVERSIONMODE_KATAKANA: u32 = 2u32;
pub const TF_CONVERSIONMODE_NATIVE: u32 = 1u32;
pub const TF_CONVERSIONMODE_NOCONVERSION: u32 = 256u32;
pub const TF_CONVERSIONMODE_ROMAN: u32 = 16u32;
pub const TF_CONVERSIONMODE_SOFTKEYBOARD: u32 = 128u32;
pub const TF_CONVERSIONMODE_SYMBOL: u32 = 1024u32;
pub const TF_CT_COLORREF: TF_DA_COLORTYPE = TF_DA_COLORTYPE(2i32);
pub const TF_CT_NONE: TF_DA_COLORTYPE = TF_DA_COLORTYPE(0i32);
pub const TF_CT_SYSCOLOR: TF_DA_COLORTYPE = TF_DA_COLORTYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TF_DA_ATTR_INFO(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TF_DA_COLOR {
    pub r#type: TF_DA_COLORTYPE,
    pub Anonymous: TF_DA_COLOR_0,
}
impl Default for TF_DA_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TF_DA_COLOR_0 {
    pub nIndex: i32,
    pub cr: super::super::Foundation::COLORREF,
}
impl Default for TF_DA_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TF_DA_COLORTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TF_DA_LINESTYLE(pub i32);
pub const TF_DEFAULT_SELECTION: u32 = 4294967295u32;
pub const TF_DICTATION_ENABLED: u32 = 2u32;
pub const TF_DICTATION_ON: u32 = 1u32;
pub const TF_DISABLE_BALLOON: u32 = 2u32;
pub const TF_DISABLE_COMMANDING: u32 = 4u32;
pub const TF_DISABLE_DICTATION: u32 = 2u32;
pub const TF_DISABLE_SPEECH: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TF_DISPLAYATTRIBUTE {
    pub crText: TF_DA_COLOR,
    pub crBk: TF_DA_COLOR,
    pub lsStyle: TF_DA_LINESTYLE,
    pub fBoldLine: super::super::Foundation::BOOL,
    pub crLine: TF_DA_COLOR,
    pub bAttr: TF_DA_ATTR_INFO,
}
impl Default for TF_DISPLAYATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_DTLBI_NONE: LANG_BAR_ITEM_ICON_MODE_FLAGS = LANG_BAR_ITEM_ICON_MODE_FLAGS(0u32);
pub const TF_DTLBI_USEPROFILEICON: LANG_BAR_ITEM_ICON_MODE_FLAGS = LANG_BAR_ITEM_ICON_MODE_FLAGS(1u32);
pub const TF_ENABLE_PROCESS_ATOM: windows_core::PCWSTR = windows_core::w!("_CTF_ENABLE_PROCESS_ATOM_");
pub const TF_ES_ASYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(8u32);
pub const TF_ES_ASYNCDONTCARE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(0u32);
pub const TF_ES_READ: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(2u32);
pub const TF_ES_READWRITE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(6u32);
pub const TF_ES_SYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(1u32);
pub const TF_E_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80040506_u32 as _);
pub const TF_E_COMPOSITION_REJECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040508_u32 as _);
pub const TF_E_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040504_u32 as _);
pub const TF_E_EMPTYCONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x80040509_u32 as _);
pub const TF_E_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8004020A_u32 as _);
pub const TF_E_INVALIDPOINT: windows_core::HRESULT = windows_core::HRESULT(0x80040207_u32 as _);
pub const TF_E_INVALIDPOS: windows_core::HRESULT = windows_core::HRESULT(0x80040200_u32 as _);
pub const TF_E_INVALIDVIEW: windows_core::HRESULT = windows_core::HRESULT(0x80040505_u32 as _);
pub const TF_E_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80040500_u32 as _);
pub const TF_E_NOCONVERSION: windows_core::HRESULT = windows_core::HRESULT(0x80040600_u32 as _);
pub const TF_E_NOINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80040204_u32 as _);
pub const TF_E_NOLAYOUT: windows_core::HRESULT = windows_core::HRESULT(0x80040206_u32 as _);
pub const TF_E_NOLOCK: windows_core::HRESULT = windows_core::HRESULT(0x80040201_u32 as _);
pub const TF_E_NOOBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80040202_u32 as _);
pub const TF_E_NOPROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80040503_u32 as _);
pub const TF_E_NOSELECTION: windows_core::HRESULT = windows_core::HRESULT(0x80040205_u32 as _);
pub const TF_E_NOSERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80040203_u32 as _);
pub const TF_E_NOTOWNEDRANGE: windows_core::HRESULT = windows_core::HRESULT(0x80040502_u32 as _);
pub const TF_E_RANGE_NOT_COVERED: windows_core::HRESULT = windows_core::HRESULT(0x80040507_u32 as _);
pub const TF_E_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x80040209_u32 as _);
pub const TF_E_STACKFULL: windows_core::HRESULT = windows_core::HRESULT(0x80040501_u32 as _);
pub const TF_E_SYNCHRONOUS: windows_core::HRESULT = windows_core::HRESULT(0x80040208_u32 as _);
pub const TF_FLOATINGLANGBAR_WNDTITLE: windows_core::PCWSTR = windows_core::w!("TF_FloatingLangBar_WndTitle");
pub const TF_FLOATINGLANGBAR_WNDTITLEA: windows_core::PCSTR = windows_core::s!("TF_FloatingLangBar_WndTitle");
pub const TF_FLOATINGLANGBAR_WNDTITLEW: windows_core::PCWSTR = windows_core::w!("TF_FloatingLangBar_WndTitle");
pub const TF_GRAVITY_BACKWARD: TfGravity = TfGravity(0i32);
pub const TF_GRAVITY_FORWARD: TfGravity = TfGravity(1i32);
pub const TF_GTP_INCL_TEXT: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = GET_TEXT_AND_PROPERTY_UPDATES_FLAGS(1u32);
pub const TF_GTP_NONE: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = GET_TEXT_AND_PROPERTY_UPDATES_FLAGS(0u32);
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct TF_HALTCOND {
    pub pHaltRange: core::mem::ManuallyDrop<Option<ITfRange>>,
    pub aHaltPos: TfAnchor,
    pub dwFlags: u32,
}
impl Default for TF_HALTCOND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_HF_OBJECT: u32 = 1u32;
pub const TF_IAS_NOQUERY: INSERT_TEXT_AT_SELECTION_FLAGS = INSERT_TEXT_AT_SELECTION_FLAGS(1u32);
pub const TF_IAS_NO_DEFAULT_COMPOSITION: INSERT_TEXT_AT_SELECTION_FLAGS = INSERT_TEXT_AT_SELECTION_FLAGS(2147483648u32);
pub const TF_IAS_QUERYONLY: INSERT_TEXT_AT_SELECTION_FLAGS = INSERT_TEXT_AT_SELECTION_FLAGS(2u32);
pub const TF_IE_CORRECTION: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TF_INPUTPROCESSORPROFILE {
    pub dwProfileType: u32,
    pub langid: u16,
    pub clsid: windows_core::GUID,
    pub guidProfile: windows_core::GUID,
    pub catid: windows_core::GUID,
    pub hklSubstitute: super::Input::KeyboardAndMouse::HKL,
    pub dwCaps: u32,
    pub hkl: super::Input::KeyboardAndMouse::HKL,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl Default for TF_INPUTPROCESSORPROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_INVALID_COOKIE: u32 = 4294967295u32;
pub const TF_INVALID_EDIT_COOKIE: u32 = 0u32;
pub const TF_IPPMF_DISABLEPROFILE: u32 = 2u32;
pub const TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE: u32 = 4u32;
pub const TF_IPPMF_ENABLEPROFILE: u32 = 1u32;
pub const TF_IPPMF_FORPROCESS: u32 = 268435456u32;
pub const TF_IPPMF_FORSESSION: u32 = 536870912u32;
pub const TF_IPPMF_FORSYSTEMALL: u32 = 1073741824u32;
pub const TF_IPP_CAPS_COMLESSSUPPORT: u32 = 8u32;
pub const TF_IPP_CAPS_DISABLEONTRANSITORY: u32 = 1u32;
pub const TF_IPP_CAPS_IMMERSIVESUPPORT: u32 = 65536u32;
pub const TF_IPP_CAPS_SECUREMODESUPPORT: u32 = 2u32;
pub const TF_IPP_CAPS_SYSTRAYSUPPORT: u32 = 131072u32;
pub const TF_IPP_CAPS_UIELEMENTENABLED: u32 = 4u32;
pub const TF_IPP_CAPS_WOW16SUPPORT: u32 = 16u32;
pub const TF_IPP_FLAG_ACTIVE: u32 = 1u32;
pub const TF_IPP_FLAG_ENABLED: u32 = 2u32;
pub const TF_IPP_FLAG_SUBSTITUTEDBYINPUTPROCESSOR: u32 = 4u32;
pub const TF_IPSINK_FLAG_ACTIVE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TF_LANGBARITEMINFO {
    pub clsidService: windows_core::GUID,
    pub guidItem: windows_core::GUID,
    pub dwStyle: u32,
    pub ulSort: u32,
    pub szDescription: [u16; 32],
}
impl Default for TF_LANGBARITEMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TF_LANGUAGEPROFILE {
    pub clsid: windows_core::GUID,
    pub langid: u16,
    pub catid: windows_core::GUID,
    pub fActive: super::super::Foundation::BOOL,
    pub guidProfile: windows_core::GUID,
}
impl Default for TF_LANGUAGEPROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct TF_LBBALLOONINFO {
    pub style: TfLBBalloonStyle,
    pub bstrText: core::mem::ManuallyDrop<windows_core::BSTR>,
}
impl Default for TF_LBBALLOONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_LBI_BALLOON: u32 = 16u32;
pub const TF_LBI_BITMAP: u32 = 8u32;
pub const TF_LBI_BMPF_VERTICAL: u32 = 1u32;
pub const TF_LBI_CLK_LEFT: TfLBIClick = TfLBIClick(2i32);
pub const TF_LBI_CLK_RIGHT: TfLBIClick = TfLBIClick(1i32);
pub const TF_LBI_CUSTOMUI: u32 = 32u32;
pub const TF_LBI_DESC_MAXLEN: u32 = 32u32;
pub const TF_LBI_ICON: u32 = 1u32;
pub const TF_LBI_STATUS: u32 = 65536u32;
pub const TF_LBI_STATUS_BTN_TOGGLED: u32 = 65536u32;
pub const TF_LBI_STATUS_DISABLED: u32 = 2u32;
pub const TF_LBI_STATUS_HIDDEN: u32 = 1u32;
pub const TF_LBI_STYLE_BTN_BUTTON: u32 = 65536u32;
pub const TF_LBI_STYLE_BTN_MENU: u32 = 131072u32;
pub const TF_LBI_STYLE_BTN_TOGGLE: u32 = 262144u32;
pub const TF_LBI_STYLE_HIDDENBYDEFAULT: u32 = 16u32;
pub const TF_LBI_STYLE_HIDDENSTATUSCONTROL: u32 = 1u32;
pub const TF_LBI_STYLE_HIDEONNOOTHERITEMS: u32 = 4u32;
pub const TF_LBI_STYLE_SHOWNINTRAY: u32 = 2u32;
pub const TF_LBI_STYLE_SHOWNINTRAYONLY: u32 = 8u32;
pub const TF_LBI_STYLE_TEXTCOLORICON: u32 = 32u32;
pub const TF_LBI_TEXT: u32 = 2u32;
pub const TF_LBI_TOOLTIP: u32 = 4u32;
pub const TF_LBMENUF_CHECKED: u32 = 1u32;
pub const TF_LBMENUF_GRAYED: u32 = 16u32;
pub const TF_LBMENUF_RADIOCHECKED: u32 = 8u32;
pub const TF_LBMENUF_SEPARATOR: u32 = 4u32;
pub const TF_LBMENUF_SUBMENU: u32 = 2u32;
pub const TF_LB_BALLOON_MISS: TfLBBalloonStyle = TfLBBalloonStyle(2i32);
pub const TF_LB_BALLOON_RECO: TfLBBalloonStyle = TfLBBalloonStyle(0i32);
pub const TF_LB_BALLOON_SHOW: TfLBBalloonStyle = TfLBBalloonStyle(1i32);
pub const TF_LC_CHANGE: TfLayoutCode = TfLayoutCode(1i32);
pub const TF_LC_CREATE: TfLayoutCode = TfLayoutCode(0i32);
pub const TF_LC_DESTROY: TfLayoutCode = TfLayoutCode(2i32);
#[repr(C)]
pub struct TF_LMLATTELEMENT {
    pub dwFrameStart: u32,
    pub dwFrameLen: u32,
    pub dwFlags: u32,
    pub Anonymous: TF_LMLATTELEMENT_0,
    pub bstrText: core::mem::ManuallyDrop<windows_core::BSTR>,
}
impl Clone for TF_LMLATTELEMENT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for TF_LMLATTELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TF_LMLATTELEMENT_0 {
    pub iCost: i32,
}
impl Default for TF_LMLATTELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_LS_DASH: TF_DA_LINESTYLE = TF_DA_LINESTYLE(3i32);
pub const TF_LS_DOT: TF_DA_LINESTYLE = TF_DA_LINESTYLE(2i32);
pub const TF_LS_NONE: TF_DA_LINESTYLE = TF_DA_LINESTYLE(0i32);
pub const TF_LS_SOLID: TF_DA_LINESTYLE = TF_DA_LINESTYLE(1i32);
pub const TF_LS_SQUIGGLE: TF_DA_LINESTYLE = TF_DA_LINESTYLE(4i32);
pub const TF_MENUREADY: u32 = 1u32;
pub const TF_MOD_ALT: u32 = 1u32;
pub const TF_MOD_CONTROL: u32 = 2u32;
pub const TF_MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
pub const TF_MOD_LALT: u32 = 64u32;
pub const TF_MOD_LCONTROL: u32 = 128u32;
pub const TF_MOD_LSHIFT: u32 = 256u32;
pub const TF_MOD_ON_KEYUP: u32 = 512u32;
pub const TF_MOD_RALT: u32 = 8u32;
pub const TF_MOD_RCONTROL: u32 = 16u32;
pub const TF_MOD_RSHIFT: u32 = 32u32;
pub const TF_MOD_SHIFT: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TF_PERSISTENT_PROPERTY_HEADER_ACP {
    pub guidType: windows_core::GUID,
    pub ichStart: i32,
    pub cch: i32,
    pub cb: u32,
    pub dwPrivate: u32,
    pub clsidTIP: windows_core::GUID,
}
impl Default for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_POPF_ALL: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TF_PRESERVEDKEY {
    pub uVKey: u32,
    pub uModifiers: u32,
}
impl Default for TF_PRESERVEDKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_PROCESS_ATOM: windows_core::PCWSTR = windows_core::w!("_CTF_PROCESS_ATOM_");
pub const TF_PROFILETYPE_INPUTPROCESSOR: u32 = 1u32;
pub const TF_PROFILETYPE_KEYBOARDLAYOUT: u32 = 2u32;
pub const TF_PROFILE_ARRAY: windows_core::GUID = windows_core::GUID::from_u128(0xd38eff65_aa46_4fd5_91a7_67845fb02f5b);
pub const TF_PROFILE_CANTONESE: windows_core::GUID = windows_core::GUID::from_u128(0x0aec109c_7e96_11d4_b2ef_0080c882687e);
pub const TF_PROFILE_CHANGJIE: windows_core::GUID = windows_core::GUID::from_u128(0x4bdf9f03_c7d3_11d4_b2ab_0080c882687e);
pub const TF_PROFILE_DAYI: windows_core::GUID = windows_core::GUID::from_u128(0x037b2c25_480c_4d7f_b027_d6ca6b69788a);
pub const TF_PROFILE_NEWCHANGJIE: windows_core::GUID = windows_core::GUID::from_u128(0xf3ba907a_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_NEWPHONETIC: windows_core::GUID = windows_core::GUID::from_u128(0xb2f9c502_1742_11d4_9790_0080c882687e);
pub const TF_PROFILE_NEWQUICK: windows_core::GUID = windows_core::GUID::from_u128(0x0b883ba0_c1c7_11d4_87f9_0080c882687e);
pub const TF_PROFILE_PHONETIC: windows_core::GUID = windows_core::GUID::from_u128(0x761309de_317a_11d4_9b5d_0080c882687e);
pub const TF_PROFILE_PINYIN: windows_core::GUID = windows_core::GUID::from_u128(0xf3ba9077_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_QUICK: windows_core::GUID = windows_core::GUID::from_u128(0x6024b45f_5c54_11d4_b921_0080c882687e);
pub const TF_PROFILE_SIMPLEFAST: windows_core::GUID = windows_core::GUID::from_u128(0xfa550b04_5ad7_411f_a5ac_ca038ec515d7);
pub const TF_PROFILE_TIGRINYA: windows_core::GUID = windows_core::GUID::from_u128(0x3cab88b7_cc3e_46a6_9765_b772ad7761ff);
pub const TF_PROFILE_WUBI: windows_core::GUID = windows_core::GUID::from_u128(0x82590c13_f4dd_44f4_ba1d_8667246fdf8e);
pub const TF_PROFILE_YI: windows_core::GUID = windows_core::GUID::from_u128(0x409c8376_007b_4357_ae8e_26316ee3fb0d);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub struct TF_PROPERTYVAL {
    pub guidId: windows_core::GUID,
    pub varValue: super::super::System::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Clone for TF_PROPERTYVAL {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TF_PROPERTYVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_PROPUI_STATUS_SAVETOFILE: u32 = 1u32;
pub const TF_RCM_COMLESS: u32 = 1u32;
pub const TF_RCM_HINT_COLLISION: u32 = 8u32;
pub const TF_RCM_HINT_READING_LENGTH: u32 = 4u32;
pub const TF_RCM_VKEY: u32 = 2u32;
pub const TF_RIP_FLAG_FREEUNUSEDLIBRARIES: u32 = 1u32;
pub const TF_RIUIE_CONTEXT: u32 = 1u32;
pub const TF_RIUIE_ERRORINDEX: u32 = 8u32;
pub const TF_RIUIE_MAXREADINGSTRINGLENGTH: u32 = 4u32;
pub const TF_RIUIE_STRING: u32 = 2u32;
pub const TF_RIUIE_VERTICALORDER: u32 = 16u32;
pub const TF_RP_HIDDENINSETTINGUI: u32 = 2u32;
pub const TF_RP_LOCALPROCESS: u32 = 4u32;
pub const TF_RP_LOCALTHREAD: u32 = 8u32;
pub const TF_RP_SUBITEMINSETTINGUI: u32 = 16u32;
pub const TF_SD_BACKWARD: TfShiftDir = TfShiftDir(0i32);
pub const TF_SD_FORWARD: TfShiftDir = TfShiftDir(1i32);
pub const TF_SD_LOADING: u32 = 2u32;
pub const TF_SD_READONLY: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct TF_SELECTION {
    pub range: core::mem::ManuallyDrop<Option<ITfRange>>,
    pub style: TF_SELECTIONSTYLE,
}
impl Default for TF_SELECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TF_SELECTIONSTYLE {
    pub ase: TfActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
impl Default for TF_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_SENTENCEMODE_AUTOMATIC: u32 = 4u32;
pub const TF_SENTENCEMODE_CONVERSATION: u32 = 16u32;
pub const TF_SENTENCEMODE_NONE: u32 = 0u32;
pub const TF_SENTENCEMODE_PHRASEPREDICT: u32 = 8u32;
pub const TF_SENTENCEMODE_PLAURALCLAUSE: u32 = 1u32;
pub const TF_SENTENCEMODE_SINGLECONVERT: u32 = 2u32;
pub const TF_SFT_DESKBAND: u32 = 2048u32;
pub const TF_SFT_DOCK: u32 = 2u32;
pub const TF_SFT_EXTRAICONSONMINIMIZED: u32 = 512u32;
pub const TF_SFT_HIDDEN: u32 = 8u32;
pub const TF_SFT_HIGHTRANSPARENCY: u32 = 64u32;
pub const TF_SFT_LABELS: u32 = 128u32;
pub const TF_SFT_LOWTRANSPARENCY: u32 = 32u32;
pub const TF_SFT_MINIMIZED: u32 = 4u32;
pub const TF_SFT_NOEXTRAICONSONMINIMIZED: u32 = 1024u32;
pub const TF_SFT_NOLABELS: u32 = 256u32;
pub const TF_SFT_NOTRANSPARENCY: u32 = 16u32;
pub const TF_SFT_SHOWNORMAL: u32 = 1u32;
pub const TF_SHOW_BALLOON: u32 = 1u32;
pub const TF_SPEECHUI_SHOWN: u32 = 16u32;
pub const TF_SS_DISJOINTSEL: u32 = 1u32;
pub const TF_SS_REGIONS: u32 = 2u32;
pub const TF_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
pub const TF_SS_TKBPREDICTIONENABLE: u32 = 32u32;
pub const TF_SS_TRANSITORY: u32 = 4u32;
pub const TF_ST_CORRECTION: u32 = 1u32;
pub const TF_S_ASYNC: windows_core::HRESULT = windows_core::HRESULT(0x40300_u32 as _);
pub const TF_TF_IGNOREEND: u32 = 2u32;
pub const TF_TF_MOVESTART: u32 = 1u32;
pub const TF_TMAE_COMLESS: u32 = 8u32;
pub const TF_TMAE_CONSOLE: u32 = 64u32;
pub const TF_TMAE_NOACTIVATEKEYBOARDLAYOUT: u32 = 32u32;
pub const TF_TMAE_NOACTIVATETIP: u32 = 1u32;
pub const TF_TMAE_SECUREMODE: u32 = 2u32;
pub const TF_TMAE_UIELEMENTENABLEDONLY: u32 = 4u32;
pub const TF_TMAE_WOW16: u32 = 16u32;
pub const TF_TMF_ACTIVATED: u32 = 2147483648u32;
pub const TF_TMF_COMLESS: u32 = 8u32;
pub const TF_TMF_CONSOLE: u32 = 64u32;
pub const TF_TMF_IMMERSIVEMODE: u32 = 1073741824u32;
pub const TF_TMF_NOACTIVATETIP: u32 = 1u32;
pub const TF_TMF_SECUREMODE: u32 = 2u32;
pub const TF_TMF_UIELEMENTENABLEDONLY: u32 = 4u32;
pub const TF_TMF_WOW16: u32 = 16u32;
pub const TF_TRANSITORYEXTENSION_ATSELECTION: u32 = 2u32;
pub const TF_TRANSITORYEXTENSION_FLOATING: u32 = 1u32;
pub const TF_TRANSITORYEXTENSION_NONE: u32 = 0u32;
pub const TF_TU_CORRECTION: u32 = 1u32;
pub const TF_URP_ALLPROFILES: u32 = 2u32;
pub const TF_URP_LOCALPROCESS: u32 = 4u32;
pub const TF_URP_LOCALTHREAD: u32 = 8u32;
pub const TF_US_HIDETIPUI: u32 = 1u32;
pub const TKBLT_CLASSIC: TKBLayoutType = TKBLayoutType(1i32);
pub const TKBLT_OPTIMIZED: TKBLayoutType = TKBLayoutType(2i32);
pub const TKBLT_UNDEFINED: TKBLayoutType = TKBLayoutType(0i32);
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_CHANGJIE: u32 = 61506u32;
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_DAYI: u32 = 61507u32;
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
pub const TKBL_OPT_JAPANESE_ABC: u32 = 1041u32;
pub const TKBL_OPT_KOREAN_HANGUL_2_BULSIK: u32 = 1042u32;
pub const TKBL_OPT_SIMPLIFIED_CHINESE_PINYIN: u32 = 2052u32;
pub const TKBL_OPT_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
pub const TKBL_UNDEFINED: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TKBLayoutType(pub i32);
pub const TKB_ALTERNATES_AUTOCORRECTION_APPLIED: u32 = 4u32;
pub const TKB_ALTERNATES_FOR_AUTOCORRECTION: u32 = 2u32;
pub const TKB_ALTERNATES_FOR_PREDICTION: u32 = 3u32;
pub const TKB_ALTERNATES_STANDARD: u32 = 1u32;
pub const TSATTRID_App: windows_core::GUID = windows_core::GUID::from_u128(0xa80f77df_4237_40e5_849c_b5fa51c13ac7);
pub const TSATTRID_App_IncorrectGrammar: windows_core::GUID = windows_core::GUID::from_u128(0xbd54e398_ad03_4b74_b6b3_5edb19996388);
pub const TSATTRID_App_IncorrectSpelling: windows_core::GUID = windows_core::GUID::from_u128(0xf42de43c_ef12_430d_944c_9a08970a25d2);
pub const TSATTRID_Font: windows_core::GUID = windows_core::GUID::from_u128(0x573ea825_749b_4f8a_9cfd_21c3605ca828);
pub const TSATTRID_Font_FaceName: windows_core::GUID = windows_core::GUID::from_u128(0xb536aeb6_053b_4eb8_b65a_50da1e81e72e);
pub const TSATTRID_Font_SizePts: windows_core::GUID = windows_core::GUID::from_u128(0xc8493302_a5e9_456d_af04_8005e4130f03);
pub const TSATTRID_Font_Style: windows_core::GUID = windows_core::GUID::from_u128(0x68b2a77f_6b0e_4f28_8177_571c2f3a42b1);
pub const TSATTRID_Font_Style_Animation: windows_core::GUID = windows_core::GUID::from_u128(0xdcf73d22_e029_47b7_bb36_f263a3d004cc);
pub const TSATTRID_Font_Style_Animation_BlinkingBackground: windows_core::GUID = windows_core::GUID::from_u128(0x86e5b104_0104_4b10_b585_00f2527522b5);
pub const TSATTRID_Font_Style_Animation_LasVegasLights: windows_core::GUID = windows_core::GUID::from_u128(0xf40423d5_0f87_4f8f_bada_e6d60c25e152);
pub const TSATTRID_Font_Style_Animation_MarchingBlackAnts: windows_core::GUID = windows_core::GUID::from_u128(0x7644e067_f186_4902_bfc6_ec815aa20e9d);
pub const TSATTRID_Font_Style_Animation_MarchingRedAnts: windows_core::GUID = windows_core::GUID::from_u128(0x78368dad_50fb_4c6f_840b_d486bb6cf781);
pub const TSATTRID_Font_Style_Animation_Shimmer: windows_core::GUID = windows_core::GUID::from_u128(0x2ce31b58_5293_4c36_8809_bf8bb51a27b3);
pub const TSATTRID_Font_Style_Animation_SparkleText: windows_core::GUID = windows_core::GUID::from_u128(0x533aad20_962c_4e9f_8c09_b42ea4749711);
pub const TSATTRID_Font_Style_Animation_WipeDown: windows_core::GUID = windows_core::GUID::from_u128(0x5872e874_367b_4803_b160_c90ff62569d0);
pub const TSATTRID_Font_Style_Animation_WipeRight: windows_core::GUID = windows_core::GUID::from_u128(0xb855cbe3_3d2c_4600_b1e9_e1c9ce02f842);
pub const TSATTRID_Font_Style_BackgroundColor: windows_core::GUID = windows_core::GUID::from_u128(0xb50eaa4e_3091_4468_81db_d79ea190c7c7);
pub const TSATTRID_Font_Style_Blink: windows_core::GUID = windows_core::GUID::from_u128(0xbfb2c036_7acf_4532_b720_b416dd7765a8);
pub const TSATTRID_Font_Style_Bold: windows_core::GUID = windows_core::GUID::from_u128(0x48813a43_8a20_4940_8e58_97823f7b268a);
pub const TSATTRID_Font_Style_Capitalize: windows_core::GUID = windows_core::GUID::from_u128(0x7d85a3ba_b4fd_43b3_befc_6b985c843141);
pub const TSATTRID_Font_Style_Color: windows_core::GUID = windows_core::GUID::from_u128(0x857a7a37_b8af_4e9a_81b4_acf700c8411b);
pub const TSATTRID_Font_Style_Emboss: windows_core::GUID = windows_core::GUID::from_u128(0xbd8ed742_349e_4e37_82fb_437979cb53a7);
pub const TSATTRID_Font_Style_Engrave: windows_core::GUID = windows_core::GUID::from_u128(0x9c3371de_8332_4897_be5d_89233223179a);
pub const TSATTRID_Font_Style_Height: windows_core::GUID = windows_core::GUID::from_u128(0x7e937477_12e6_458b_926a_1fa44ee8f391);
pub const TSATTRID_Font_Style_Hidden: windows_core::GUID = windows_core::GUID::from_u128(0xb1e28770_881c_475f_863f_887a647b1090);
pub const TSATTRID_Font_Style_Italic: windows_core::GUID = windows_core::GUID::from_u128(0x8740682a_a765_48e1_acfc_d22222b2f810);
pub const TSATTRID_Font_Style_Kerning: windows_core::GUID = windows_core::GUID::from_u128(0xcc26e1b4_2f9a_47c8_8bff_bf1eb7cce0dd);
pub const TSATTRID_Font_Style_Lowercase: windows_core::GUID = windows_core::GUID::from_u128(0x76d8ccb5_ca7b_4498_8ee9_d5c4f6f74c60);
pub const TSATTRID_Font_Style_Outlined: windows_core::GUID = windows_core::GUID::from_u128(0x10e6db31_db0d_4ac6_a7f5_9c9cff6f2ab4);
pub const TSATTRID_Font_Style_Overline: windows_core::GUID = windows_core::GUID::from_u128(0xe3989f4a_992b_4301_8ce1_a5b7c6d1f3c8);
pub const TSATTRID_Font_Style_Overline_Double: windows_core::GUID = windows_core::GUID::from_u128(0xdc46063a_e115_46e3_bcd8_ca6772aa95b4);
pub const TSATTRID_Font_Style_Overline_Single: windows_core::GUID = windows_core::GUID::from_u128(0x8440d94c_51ce_47b2_8d4c_15751e5f721b);
pub const TSATTRID_Font_Style_Position: windows_core::GUID = windows_core::GUID::from_u128(0x15cd26ab_f2fb_4062_b5a6_9a49e1a5cc0b);
pub const TSATTRID_Font_Style_Protected: windows_core::GUID = windows_core::GUID::from_u128(0x1c557cb2_14cf_4554_a574_ecb2f7e7efd4);
pub const TSATTRID_Font_Style_Shadow: windows_core::GUID = windows_core::GUID::from_u128(0x5f686d2f_c6cd_4c56_8a1a_994a4b9766be);
pub const TSATTRID_Font_Style_SmallCaps: windows_core::GUID = windows_core::GUID::from_u128(0xfacb6bc6_9100_4cc6_b969_11eea45a86b4);
pub const TSATTRID_Font_Style_Spacing: windows_core::GUID = windows_core::GUID::from_u128(0x98c1200d_8f06_409a_8e49_6a554bf7c153);
pub const TSATTRID_Font_Style_Strikethrough: windows_core::GUID = windows_core::GUID::from_u128(0x0c562193_2d08_4668_9601_ced41309d7af);
pub const TSATTRID_Font_Style_Strikethrough_Double: windows_core::GUID = windows_core::GUID::from_u128(0x62489b31_a3e7_4f94_ac43_ebaf8fcc7a9f);
pub const TSATTRID_Font_Style_Strikethrough_Single: windows_core::GUID = windows_core::GUID::from_u128(0x75d736b6_3c8f_4b97_ab78_1877cb990d31);
pub const TSATTRID_Font_Style_Subscript: windows_core::GUID = windows_core::GUID::from_u128(0x5774fb84_389b_43bc_a74b_1568347cf0f4);
pub const TSATTRID_Font_Style_Superscript: windows_core::GUID = windows_core::GUID::from_u128(0x2ea4993c_563c_49aa_9372_0bef09a9255b);
pub const TSATTRID_Font_Style_Underline: windows_core::GUID = windows_core::GUID::from_u128(0xc3c9c9f3_7902_444b_9a7b_48e70f4b50f7);
pub const TSATTRID_Font_Style_Underline_Double: windows_core::GUID = windows_core::GUID::from_u128(0x74d24aa6_1db3_4c69_a176_31120e7586d5);
pub const TSATTRID_Font_Style_Underline_Single: windows_core::GUID = windows_core::GUID::from_u128(0x1b6720e5_0f73_4951_a6b3_6f19e43c9461);
pub const TSATTRID_Font_Style_Uppercase: windows_core::GUID = windows_core::GUID::from_u128(0x33a300e8_e340_4937_b697_8f234045cd9a);
pub const TSATTRID_Font_Style_Weight: windows_core::GUID = windows_core::GUID::from_u128(0x12f3189c_8bb0_461b_b1fa_eaf907047fe0);
pub const TSATTRID_List: windows_core::GUID = windows_core::GUID::from_u128(0x436d673b_26f1_4aee_9e65_8f83a4ed4884);
pub const TSATTRID_List_LevelIndel: windows_core::GUID = windows_core::GUID::from_u128(0x7f7cc899_311f_487b_ad5d_e2a459e12d42);
pub const TSATTRID_List_Type: windows_core::GUID = windows_core::GUID::from_u128(0xae3e665e_4bce_49e3_a0fe_2db47d3a17ae);
pub const TSATTRID_List_Type_Arabic: windows_core::GUID = windows_core::GUID::from_u128(0x1338c5d6_98a3_4fa3_9bd1_7a60eef8e9e0);
pub const TSATTRID_List_Type_Bullet: windows_core::GUID = windows_core::GUID::from_u128(0xbccd77c5_4c4d_4ce2_b102_559f3b2bfcea);
pub const TSATTRID_List_Type_LowerLetter: windows_core::GUID = windows_core::GUID::from_u128(0x96372285_f3cf_491e_a925_3832347fd237);
pub const TSATTRID_List_Type_LowerRoman: windows_core::GUID = windows_core::GUID::from_u128(0x90466262_3980_4b8e_9368_918bd1218a41);
pub const TSATTRID_List_Type_UpperLetter: windows_core::GUID = windows_core::GUID::from_u128(0x7987b7cd_ce52_428b_9b95_a357f6f10c45);
pub const TSATTRID_List_Type_UpperRoman: windows_core::GUID = windows_core::GUID::from_u128(0x0f6ab552_4a80_467f_b2f1_127e2aa3ba9e);
pub const TSATTRID_OTHERS: windows_core::GUID = windows_core::GUID::from_u128(0xb3c32af9_57d0_46a9_bca8_dac238a13057);
pub const TSATTRID_Text: windows_core::GUID = windows_core::GUID::from_u128(0x7edb8e68_81f9_449d_a15a_87a8388faac0);
pub const TSATTRID_Text_Alignment: windows_core::GUID = windows_core::GUID::from_u128(0x139941e6_1767_456d_938e_35ba568b5cd4);
pub const TSATTRID_Text_Alignment_Center: windows_core::GUID = windows_core::GUID::from_u128(0xa4a95c16_53bf_4d55_8b87_4bdd8d4275fc);
pub const TSATTRID_Text_Alignment_Justify: windows_core::GUID = windows_core::GUID::from_u128(0xed350740_a0f7_42d3_8ea8_f81b6488faf0);
pub const TSATTRID_Text_Alignment_Left: windows_core::GUID = windows_core::GUID::from_u128(0x16ae95d3_6361_43a2_8495_d00f397f1693);
pub const TSATTRID_Text_Alignment_Right: windows_core::GUID = windows_core::GUID::from_u128(0xb36f0f98_1b9e_4360_8616_03fb08a78456);
pub const TSATTRID_Text_EmbeddedObject: windows_core::GUID = windows_core::GUID::from_u128(0x7edb8e68_81f9_449d_a15a_87a8388faac0);
pub const TSATTRID_Text_Hyphenation: windows_core::GUID = windows_core::GUID::from_u128(0xdadf4525_618e_49eb_b1a8_3b68bd7648e3);
pub const TSATTRID_Text_Language: windows_core::GUID = windows_core::GUID::from_u128(0xd8c04ef1_5753_4c25_8887_85443fe5f819);
pub const TSATTRID_Text_Link: windows_core::GUID = windows_core::GUID::from_u128(0x47cd9051_3722_4cd8_b7c8_4e17ca1759f5);
pub const TSATTRID_Text_Orientation: windows_core::GUID = windows_core::GUID::from_u128(0x6bab707f_8785_4c39_8b52_96f878303ffb);
pub const TSATTRID_Text_Para: windows_core::GUID = windows_core::GUID::from_u128(0x5edc5822_99dc_4dd6_aec3_b62baa5b2e7c);
pub const TSATTRID_Text_Para_FirstLineIndent: windows_core::GUID = windows_core::GUID::from_u128(0x07c97a13_7472_4dd8_90a9_91e3d7e4f29c);
pub const TSATTRID_Text_Para_LeftIndent: windows_core::GUID = windows_core::GUID::from_u128(0xfb2848e9_7471_41c9_b6b3_8a1450e01897);
pub const TSATTRID_Text_Para_LineSpacing: windows_core::GUID = windows_core::GUID::from_u128(0x699b380d_7f8c_46d6_a73b_dfe3d1538df3);
pub const TSATTRID_Text_Para_LineSpacing_AtLeast: windows_core::GUID = windows_core::GUID::from_u128(0xadfedf31_2d44_4434_a5ff_7f4c4990a905);
pub const TSATTRID_Text_Para_LineSpacing_Double: windows_core::GUID = windows_core::GUID::from_u128(0x82fb1805_a6c4_4231_ac12_6260af2aba28);
pub const TSATTRID_Text_Para_LineSpacing_Exactly: windows_core::GUID = windows_core::GUID::from_u128(0x3d45ad40_23de_48d7_a6b3_765420c620cc);
pub const TSATTRID_Text_Para_LineSpacing_Multiple: windows_core::GUID = windows_core::GUID::from_u128(0x910f1e3c_d6d0_4f65_8a3c_42b4b31868c5);
pub const TSATTRID_Text_Para_LineSpacing_OnePtFive: windows_core::GUID = windows_core::GUID::from_u128(0x0428a021_0397_4b57_9a17_0795994cd3c5);
pub const TSATTRID_Text_Para_LineSpacing_Single: windows_core::GUID = windows_core::GUID::from_u128(0xed350740_a0f7_42d3_8ea8_f81b6488faf0);
pub const TSATTRID_Text_Para_RightIndent: windows_core::GUID = windows_core::GUID::from_u128(0x2c7f26f9_a5e2_48da_b98a_520cb16513bf);
pub const TSATTRID_Text_Para_SpaceAfter: windows_core::GUID = windows_core::GUID::from_u128(0x7b0a3f55_22dc_425f_a411_93da1d8f9baa);
pub const TSATTRID_Text_Para_SpaceBefore: windows_core::GUID = windows_core::GUID::from_u128(0x8df98589_194a_4601_b251_9865a3e906dd);
pub const TSATTRID_Text_ReadOnly: windows_core::GUID = windows_core::GUID::from_u128(0x85836617_de32_4afd_a50f_a2db110e6e4d);
pub const TSATTRID_Text_RightToLeft: windows_core::GUID = windows_core::GUID::from_u128(0xca666e71_1b08_453d_bfdd_28e08c8aaf7a);
pub const TSATTRID_Text_VerticalWriting: windows_core::GUID = windows_core::GUID::from_u128(0x6bba8195_046f_4ea9_b311_97fd66c4274b);
pub const TS_AE_END: TsActiveSelEnd = TsActiveSelEnd(2i32);
pub const TS_AE_NONE: TsActiveSelEnd = TsActiveSelEnd(0i32);
pub const TS_AE_START: TsActiveSelEnd = TsActiveSelEnd(1i32);
pub const TS_AS_ATTR_CHANGE: u32 = 8u32;
pub const TS_AS_LAYOUT_CHANGE: u32 = 4u32;
pub const TS_AS_SEL_CHANGE: u32 = 2u32;
pub const TS_AS_STATUS_CHANGE: u32 = 16u32;
pub const TS_AS_TEXT_CHANGE: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub struct TS_ATTRVAL {
    pub idAttr: windows_core::GUID,
    pub dwOverlapId: u32,
    pub varValue: super::super::System::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Clone for TS_ATTRVAL {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_ATTR_FIND_BACKWARDS: u32 = 1u32;
pub const TS_ATTR_FIND_HIDDEN: u32 = 32u32;
pub const TS_ATTR_FIND_UPDATESTART: u32 = 4u32;
pub const TS_ATTR_FIND_WANT_END: u32 = 16u32;
pub const TS_ATTR_FIND_WANT_OFFSET: u32 = 2u32;
pub const TS_ATTR_FIND_WANT_VALUE: u32 = 8u32;
pub const TS_CHAR_EMBEDDED: u32 = 65532u32;
pub const TS_CHAR_REGION: u32 = 0u32;
pub const TS_CHAR_REPLACEMENT: u32 = 65533u32;
pub const TS_CH_FOLLOWING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = ANCHOR_CHANGE_HISTORY_FLAGS(2u32);
pub const TS_CH_PRECEDING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = ANCHOR_CHANGE_HISTORY_FLAGS(1u32);
pub const TS_DEFAULT_SELECTION: u32 = 4294967295u32;
pub const TS_E_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8004020A_u32 as _);
pub const TS_E_INVALIDPOINT: windows_core::HRESULT = windows_core::HRESULT(0x80040207_u32 as _);
pub const TS_E_INVALIDPOS: windows_core::HRESULT = windows_core::HRESULT(0x80040200_u32 as _);
pub const TS_E_NOINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80040204_u32 as _);
pub const TS_E_NOLAYOUT: windows_core::HRESULT = windows_core::HRESULT(0x80040206_u32 as _);
pub const TS_E_NOLOCK: windows_core::HRESULT = windows_core::HRESULT(0x80040201_u32 as _);
pub const TS_E_NOOBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80040202_u32 as _);
pub const TS_E_NOSELECTION: windows_core::HRESULT = windows_core::HRESULT(0x80040205_u32 as _);
pub const TS_E_NOSERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80040203_u32 as _);
pub const TS_E_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x80040209_u32 as _);
pub const TS_E_SYNCHRONOUS: windows_core::HRESULT = windows_core::HRESULT(0x80040208_u32 as _);
pub const TS_GEA_HIDDEN: u32 = 1u32;
pub const TS_GR_BACKWARD: TsGravity = TsGravity(0i32);
pub const TS_GR_FORWARD: TsGravity = TsGravity(1i32);
pub const TS_GTA_HIDDEN: u32 = 1u32;
pub const TS_IAS_NOQUERY: u32 = 1u32;
pub const TS_IAS_QUERYONLY: u32 = 2u32;
pub const TS_IE_COMPOSITION: u32 = 2u32;
pub const TS_IE_CORRECTION: u32 = 1u32;
pub const TS_LC_CHANGE: TsLayoutCode = TsLayoutCode(1i32);
pub const TS_LC_CREATE: TsLayoutCode = TsLayoutCode(0i32);
pub const TS_LC_DESTROY: TsLayoutCode = TsLayoutCode(2i32);
pub const TS_LF_READ: TEXT_STORE_LOCK_FLAGS = TEXT_STORE_LOCK_FLAGS(2u32);
pub const TS_LF_READWRITE: TEXT_STORE_LOCK_FLAGS = TEXT_STORE_LOCK_FLAGS(6u32);
pub const TS_LF_SYNC: u32 = 1u32;
pub const TS_RT_HIDDEN: TsRunType = TsRunType(1i32);
pub const TS_RT_OPAQUE: TsRunType = TsRunType(2i32);
pub const TS_RT_PLAIN: TsRunType = TsRunType(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TS_RUNINFO {
    pub uCount: u32,
    pub r#type: TsRunType,
}
impl Default for TS_RUNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_SD_BACKWARD: TsShiftDir = TsShiftDir(0i32);
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED: u32 = 128u32;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE: u32 = 256u32;
pub const TS_SD_FORWARD: TsShiftDir = TsShiftDir(1i32);
pub const TS_SD_INPUTPANEMANUALDISPLAYENABLE: u32 = 64u32;
pub const TS_SD_LOADING: u32 = 2u32;
pub const TS_SD_READONLY: u32 = 1u32;
pub const TS_SD_RESERVED: u32 = 4u32;
pub const TS_SD_TKBAUTOCORRECTENABLE: u32 = 8u32;
pub const TS_SD_TKBPREDICTIONENABLE: u32 = 16u32;
pub const TS_SD_UIINTEGRATIONENABLE: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TS_SELECTIONSTYLE {
    pub ase: TsActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
impl Default for TS_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TS_SELECTION_ACP {
    pub acpStart: i32,
    pub acpEnd: i32,
    pub style: TS_SELECTIONSTYLE,
}
impl Default for TS_SELECTION_ACP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct TS_SELECTION_ANCHOR {
    pub paStart: core::mem::ManuallyDrop<Option<IAnchor>>,
    pub paEnd: core::mem::ManuallyDrop<Option<IAnchor>>,
    pub style: TS_SELECTIONSTYLE,
}
impl Default for TS_SELECTION_ANCHOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_SHIFT_COUNT_HIDDEN: u32 = 1u32;
pub const TS_SHIFT_COUNT_ONLY: u32 = 8u32;
pub const TS_SHIFT_HALT_HIDDEN: u32 = 2u32;
pub const TS_SHIFT_HALT_VISIBLE: u32 = 4u32;
pub const TS_SS_DISJOINTSEL: u32 = 1u32;
pub const TS_SS_NOHIDDENTEXT: u32 = 8u32;
pub const TS_SS_REGIONS: u32 = 2u32;
pub const TS_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
pub const TS_SS_TKBPREDICTIONENABLE: u32 = 32u32;
pub const TS_SS_TRANSITORY: u32 = 4u32;
pub const TS_SS_UWPCONTROL: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TS_STATUS {
    pub dwDynamicFlags: u32,
    pub dwStaticFlags: u32,
}
impl Default for TS_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_STRF_END: u32 = 2u32;
pub const TS_STRF_MID: u32 = 1u32;
pub const TS_STRF_START: u32 = 0u32;
pub const TS_ST_CORRECTION: TEXT_STORE_TEXT_CHANGE_FLAGS = TEXT_STORE_TEXT_CHANGE_FLAGS(1u32);
pub const TS_ST_NONE: TEXT_STORE_TEXT_CHANGE_FLAGS = TEXT_STORE_TEXT_CHANGE_FLAGS(0u32);
pub const TS_S_ASYNC: windows_core::HRESULT = windows_core::HRESULT(0x40300_u32 as _);
pub const TS_TC_CORRECTION: TEXT_STORE_CHANGE_FLAGS = TEXT_STORE_CHANGE_FLAGS(1u32);
pub const TS_TC_NONE: TEXT_STORE_CHANGE_FLAGS = TEXT_STORE_CHANGE_FLAGS(0u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TS_TEXTCHANGE {
    pub acpStart: i32,
    pub acpOldEnd: i32,
    pub acpNewEnd: i32,
}
impl Default for TS_TEXTCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_VCOOKIE_NUL: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfActiveSelEnd(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfAnchor(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfCandidateResult(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfGravity(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfIntegratableCandidateListSelectionStyle(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfLBBalloonStyle(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfLBIClick(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfLayoutCode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfSapiObject(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TfShiftDir(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TsActiveSelEnd(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TsGravity(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TsLayoutCode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TsRunType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TsShiftDir(pub i32);
