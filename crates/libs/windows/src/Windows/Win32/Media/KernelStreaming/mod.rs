#[inline]
pub unsafe fn KsCreateAllocator(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING, allocatorhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("ksuser.dll" "system" fn KsCreateAllocator(connectionhandle : super::super::Foundation:: HANDLE, allocatorframing : *const KSALLOCATOR_FRAMING, allocatorhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    KsCreateAllocator(connectionhandle, allocatorframing, core::mem::transmute(allocatorhandle))
}
#[inline]
pub unsafe fn KsCreateAllocator2(connectionhandle: super::super::Foundation::HANDLE, allocatorframing: *const KSALLOCATOR_FRAMING) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("ksuser.dll" "system" fn KsCreateAllocator2(connectionhandle : super::super::Foundation:: HANDLE, allocatorframing : *const KSALLOCATOR_FRAMING, allocatorhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    KsCreateAllocator2(connectionhandle, allocatorframing, &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn KsCreateClock(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE, clockhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("ksuser.dll" "system" fn KsCreateClock(connectionhandle : super::super::Foundation:: HANDLE, clockcreate : *const KSCLOCK_CREATE, clockhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    KsCreateClock(connectionhandle, clockcreate, core::mem::transmute(clockhandle))
}
#[inline]
pub unsafe fn KsCreateClock2(connectionhandle: super::super::Foundation::HANDLE, clockcreate: *const KSCLOCK_CREATE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("ksuser.dll" "system" fn KsCreateClock2(connectionhandle : super::super::Foundation:: HANDLE, clockcreate : *const KSCLOCK_CREATE, clockhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    KsCreateClock2(connectionhandle, clockcreate, &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn KsCreatePin(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32, connectionhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("ksuser.dll" "system" fn KsCreatePin(filterhandle : super::super::Foundation:: HANDLE, connect : *const KSPIN_CONNECT, desiredaccess : u32, connectionhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    KsCreatePin(filterhandle, connect, desiredaccess, core::mem::transmute(connectionhandle))
}
#[inline]
pub unsafe fn KsCreatePin2(filterhandle: super::super::Foundation::HANDLE, connect: *const KSPIN_CONNECT, desiredaccess: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("ksuser.dll" "system" fn KsCreatePin2(filterhandle : super::super::Foundation:: HANDLE, connect : *const KSPIN_CONNECT, desiredaccess : u32, connectionhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    KsCreatePin2(filterhandle, connect, desiredaccess, &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn KsCreateTopologyNode(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32, nodehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("ksuser.dll" "system" fn KsCreateTopologyNode(parenthandle : super::super::Foundation:: HANDLE, nodecreate : *const KSNODE_CREATE, desiredaccess : u32, nodehandle : *mut super::super::Foundation:: HANDLE) -> u32);
    KsCreateTopologyNode(parenthandle, nodecreate, desiredaccess, core::mem::transmute(nodehandle))
}
#[inline]
pub unsafe fn KsCreateTopologyNode2(parenthandle: super::super::Foundation::HANDLE, nodecreate: *const KSNODE_CREATE, desiredaccess: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("ksuser.dll" "system" fn KsCreateTopologyNode2(parenthandle : super::super::Foundation:: HANDLE, nodecreate : *const KSNODE_CREATE, desiredaccess : u32, nodehandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    KsCreateTopologyNode2(parenthandle, nodecreate, desiredaccess, &mut result__).map(|| core::mem::transmute(result__))
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
#[inline]
pub unsafe fn KsGetMediaType(position: i32, ammediatype: *mut super::MediaFoundation::AM_MEDIA_TYPE, filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32) -> windows_core::Result<()> {
    windows_targets::link!("ksproxy.ax" "system" fn KsGetMediaType(position : i32, ammediatype : *mut super::MediaFoundation:: AM_MEDIA_TYPE, filterhandle : super::super::Foundation:: HANDLE, pinfactoryid : u32) -> windows_core::HRESULT);
    KsGetMediaType(position, core::mem::transmute(ammediatype), filterhandle, pinfactoryid).ok()
}
#[inline]
pub unsafe fn KsGetMediaTypeCount(filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32) -> windows_core::Result<u32> {
    windows_targets::link!("ksproxy.ax" "system" fn KsGetMediaTypeCount(filterhandle : super::super::Foundation:: HANDLE, pinfactoryid : u32, mediatypecount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    KsGetMediaTypeCount(filterhandle, pinfactoryid, &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn KsGetMultiplePinFactoryItems(filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32, propertyid: u32, items: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("ksproxy.ax" "system" fn KsGetMultiplePinFactoryItems(filterhandle : super::super::Foundation:: HANDLE, pinfactoryid : u32, propertyid : u32, items : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    KsGetMultiplePinFactoryItems(filterhandle, pinfactoryid, propertyid, core::mem::transmute(items)).ok()
}
#[inline]
pub unsafe fn KsOpenDefaultDevice(category: *const windows_core::GUID, access: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("ksproxy.ax" "system" fn KsOpenDefaultDevice(category : *const windows_core::GUID, access : u32, devicehandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    KsOpenDefaultDevice(category, access, &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn KsResolveRequiredAttributes(datarange: *const KSDATAFORMAT, attributes: Option<*const KSMULTIPLE_ITEM>) -> windows_core::Result<()> {
    windows_targets::link!("ksproxy.ax" "system" fn KsResolveRequiredAttributes(datarange : *const KSDATAFORMAT, attributes : *const KSMULTIPLE_ITEM) -> windows_core::HRESULT);
    KsResolveRequiredAttributes(datarange, core::mem::transmute(attributes.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn KsSynchronousDeviceControl(handle: super::super::Foundation::HANDLE, iocontrol: u32, inbuffer: Option<*const core::ffi::c_void>, inlength: u32, outbuffer: Option<*mut core::ffi::c_void>, outlength: u32, bytesreturned: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("ksproxy.ax" "system" fn KsSynchronousDeviceControl(handle : super::super::Foundation:: HANDLE, iocontrol : u32, inbuffer : *const core::ffi::c_void, inlength : u32, outbuffer : *mut core::ffi::c_void, outlength : u32, bytesreturned : *mut u32) -> windows_core::HRESULT);
    KsSynchronousDeviceControl(handle, iocontrol, core::mem::transmute(inbuffer.unwrap_or(core::mem::zeroed())), inlength, core::mem::transmute(outbuffer.unwrap_or(core::mem::zeroed())), outlength, core::mem::transmute(bytesreturned.unwrap_or(core::mem::zeroed()))).ok()
}
pub const AEC_MODE_FULL_DUPLEX: u32 = 2u32;
pub const AEC_MODE_HALF_DUPLEX: u32 = 1u32;
pub const AEC_MODE_PASS_THROUGH: u32 = 0u32;
pub const AEC_STATUS_FD_CURRENTLY_CONVERGED: u32 = 8u32;
pub const AEC_STATUS_FD_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1u32;
pub const AEC_STATUS_FD_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2u32;
pub const AEC_STATUS_FD_HISTORY_UNINITIALIZED: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct ALLOCATOR_PROPERTIES_EX {
    pub cBuffers: i32,
    pub cbBuffer: i32,
    pub cbAlign: i32,
    pub cbPrefix: i32,
    pub MemoryType: windows_core::GUID,
    pub BusType: windows_core::GUID,
    pub State: PIPE_STATE,
    pub Input: PIPE_TERMINATION,
    pub Output: PIPE_TERMINATION,
    pub Strategy: u32,
    pub Flags: u32,
    pub Weight: u32,
    pub LogicalMemoryType: KS_LogicalMemoryType,
    pub AllocatorPlace: PIPE_ALLOCATOR_PLACE,
    pub Dimensions: PIPE_DIMENSIONS,
    pub PhysicalRange: KS_FRAMING_RANGE,
    pub PrevSegment: core::mem::ManuallyDrop<Option<IKsAllocatorEx>>,
    pub CountNextSegments: u32,
    pub NextSegments: *mut Option<IKsAllocatorEx>,
    pub InsideFactors: u32,
    pub NumberPins: u32,
}
impl Default for ALLOCATOR_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const APO_CLASS_UUID: windows_core::GUID = windows_core::GUID::from_u128(0x5989fce8_9cd0_467d_8a6a_5419e31529d4);
pub const AUDIOENDPOINT_CLASS_UUID: windows_core::GUID = windows_core::GUID::from_u128(0xc166523c_fe0c_4a94_a586_f1a80cfbbf3e);
pub const AUDIOMODULE_MAX_DATA_SIZE: u32 = 64000u32;
pub const AUDIOMODULE_MAX_NAME_CCH_SIZE: u32 = 128u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIOPOSTURE_ORIENTATION(pub i32);
pub const AUDIOPOSTURE_ORIENTATION_NOTROTATED: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(0i32);
pub const AUDIOPOSTURE_ORIENTATION_ROTATED180DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(2i32);
pub const AUDIOPOSTURE_ORIENTATION_ROTATED270DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(3i32);
pub const AUDIOPOSTURE_ORIENTATION_ROTATED90DEGREESCOUNTERCLOCKWISE: AUDIOPOSTURE_ORIENTATION = AUDIOPOSTURE_ORIENTATION(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    pub ResourceGroupAcquired: super::super::Foundation::BOOL,
    pub ResourceGroupName: [u16; 256],
}
impl Default for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIO_CURVE_TYPE(pub i32);
pub const AUDIO_CURVE_TYPE_NONE: AUDIO_CURVE_TYPE = AUDIO_CURVE_TYPE(0i32);
pub const AUDIO_CURVE_TYPE_WINDOWS_FADE: AUDIO_CURVE_TYPE = AUDIO_CURVE_TYPE(1i32);
pub const AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adbe_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_AUTOMATIC_GAIN_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc0_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_BASS_BOOST: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc5_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_BASS_MANAGEMENT: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adca_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_BEAMFORMING: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc1_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_CONSTANT_TONE_REMOVAL: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc2_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_DEEP_NOISE_SUPPRESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64add0_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_DYNAMIC_RANGE_COMPRESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adce_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_ENVIRONMENTAL_EFFECTS: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adcb_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_EQUALIZER: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc3_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_FAR_FIELD_BEAMFORMING: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adcf_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_LOUDNESS_EQUALIZER: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc4_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_NOISE_SUPPRESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adbf_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_ROOM_CORRECTION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc9_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_SPEAKER_COMPENSATION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adcd_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_SPEAKER_FILL: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc8_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_SPEAKER_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adcc_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_VIRTUAL_HEADPHONES: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc7_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_EFFECT_TYPE_VIRTUAL_SURROUND: windows_core::GUID = windows_core::GUID::from_u128(0x6f64adc6_8211_11e2_8c70_2c27d7f001fa);
pub const AUDIO_SIGNALPROCESSINGMODE_COMMUNICATIONS: windows_core::GUID = windows_core::GUID::from_u128(0x98951333_b9cd_48b1_a0a3_ff40682d73f7);
pub const AUDIO_SIGNALPROCESSINGMODE_DEFAULT: windows_core::GUID = windows_core::GUID::from_u128(0xc18e2f7e_933d_4965_b7d1_1eef228d2af3);
pub const AUDIO_SIGNALPROCESSINGMODE_FAR_FIELD_SPEECH: windows_core::GUID = windows_core::GUID::from_u128(0x28941cba_3be6_4a78_9a76_30fd91559b64);
pub const AUDIO_SIGNALPROCESSINGMODE_MEDIA: windows_core::GUID = windows_core::GUID::from_u128(0x4780004e_7133_41d8_8c74_660dadd2c0ee);
pub const AUDIO_SIGNALPROCESSINGMODE_MOVIE: windows_core::GUID = windows_core::GUID::from_u128(0xb26feb0d_ec94_477c_9494_d1ab8e753f6e);
pub const AUDIO_SIGNALPROCESSINGMODE_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x9cf2a70b_f377_403b_bd6b_360863e0355c);
pub const AUDIO_SIGNALPROCESSINGMODE_RAW: windows_core::GUID = windows_core::GUID::from_u128(0x9e90ea20_b493_4fd1_a1a8_7e1361a956cf);
pub const AUDIO_SIGNALPROCESSINGMODE_SPEECH: windows_core::GUID = windows_core::GUID::from_u128(0xfc1cfc9b_b9d6_4cfa_b5e0_4bb2166878b2);
pub const AllocatorStrategy_DontCare: u32 = 0u32;
pub const AllocatorStrategy_MaximizeSpeed: u32 = 8u32;
pub const AllocatorStrategy_MinimizeFrameSize: u32 = 2u32;
pub const AllocatorStrategy_MinimizeNumberOfAllocators: u32 = 4u32;
pub const AllocatorStrategy_MinimizeNumberOfFrames: u32 = 1u32;
pub const BLUETOOTHLE_MIDI_SERVICE_UUID: windows_core::GUID = windows_core::GUID::from_u128(0x03b80e5a_ede8_4b33_a751_6ce34ec4c700);
pub const BLUETOOTH_MIDI_DATAIO_CHARACTERISTIC: windows_core::GUID = windows_core::GUID::from_u128(0x7772e5db_3868_4112_a1a9_f2669d106bf3);
pub const BUS_INTERFACE_REFERENCE_VERSION: u32 = 256u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAPTURE_MEMORY_ALLOCATION_FLAGS(pub i32);
pub const CASCADE_FORM: KSDS3D_HRTF_FILTER_METHOD = KSDS3D_HRTF_FILTER_METHOD(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CC_BYTE_PAIR {
    pub Decoded: [u8; 2],
    pub Reserved: u16,
}
impl Default for CC_BYTE_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CC_HW_FIELD {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub fieldFlags: u32,
    pub PictureNumber: i64,
    pub Lines: [CC_BYTE_PAIR; 12],
}
impl Default for CC_HW_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CC_MAX_HW_DECODE_LINES: u32 = 12u32;
pub const CLSID_KsIBasicAudioInterfaceHandler: windows_core::GUID = windows_core::GUID::from_u128(0xb9f8ac3e_0f71_11d2_b72c_00c04fb6bd3d);
pub const CLSID_Proxy: windows_core::GUID = windows_core::GUID::from_u128(0x17cca71b_ecd7_11d0_b908_00a0c9223196);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONSTRICTOR_OPTION(pub i32);
pub const CONSTRICTOR_OPTION_DISABLE: CONSTRICTOR_OPTION = CONSTRICTOR_OPTION(0i32);
pub const CONSTRICTOR_OPTION_MUTE: CONSTRICTOR_OPTION = CONSTRICTOR_OPTION(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVCAPS {
    pub CanRecord: i32,
    pub CanRecordStrobe: i32,
    pub HasAudio: i32,
    pub HasVideo: i32,
    pub UsesFiles: i32,
    pub CanSave: i32,
    pub DeviceType: i32,
    pub TCRead: i32,
    pub TCWrite: i32,
    pub CTLRead: i32,
    pub IndexRead: i32,
    pub Preroll: i32,
    pub Postroll: i32,
    pub SyncAcc: i32,
    pub NormRate: i32,
    pub CanPreview: i32,
    pub CanMonitorSrc: i32,
    pub CanTest: i32,
    pub VideoIn: i32,
    pub AudioIn: i32,
    pub Calibrate: i32,
    pub SeekType: i32,
    pub SimulatedHardware: i32,
}
impl Default for DEVCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVPKEY_KsAudio_Controller_DeviceInterface_Path: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x13e004d6_b066_43bd_913b_a415cd13da87), pid: 3 };
pub const DEVPKEY_KsAudio_PacketSize_Constraints: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x13e004d6_b066_43bd_913b_a415cd13da87), pid: 2 };
pub const DEVPKEY_KsAudio_PacketSize_Constraints2: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x9404f781_7191_409b_8b0b_80bf6ec229ae), pid: 2 };
pub const DIRECT_FORM: KSDS3D_HRTF_FILTER_METHOD = KSDS3D_HRTF_FILTER_METHOD(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DS3DVECTOR {
    pub Anonymous1: DS3DVECTOR_0,
    pub Anonymous2: DS3DVECTOR_1,
    pub Anonymous3: DS3DVECTOR_2,
}
impl Default for DS3DVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DS3DVECTOR_0 {
    pub x: f32,
    pub dvX: f32,
}
impl Default for DS3DVECTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DS3DVECTOR_1 {
    pub y: f32,
    pub dvY: f32,
}
impl Default for DS3DVECTOR_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DS3DVECTOR_2 {
    pub z: f32,
    pub dvZ: f32,
}
impl Default for DS3DVECTOR_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DS3D_HRTF_VERSION_1: KSDS3D_HRTF_FILTER_VERSION = KSDS3D_HRTF_FILTER_VERSION(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDeviceControlUseType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EPcxConnectionType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EPcxGenLocation(pub i32);
pub const EPcxGenLocation_enum_count: EPcxGenLocation = EPcxGenLocation(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EPcxGeoLocation(pub i32);
pub const EPcxGeoLocation_enum_count: EPcxGeoLocation = EPcxGeoLocation(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EPxcPortConnection(pub i32);
pub const EVENTSETID_CROSSBAR: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0641_28e4_11d0_a18c_00a0c9118956);
pub const EVENTSETID_TUNER: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0606_28e4_11d0_a18c_00a0c9118956);
pub const EVENTSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: windows_core::GUID = windows_core::GUID::from_u128(0x2fdffc5d_c732_4ba6_b5df_6b4d7fc88b8b);
pub const EVENTSETID_VIDEODECODER: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0621_28e4_11d0_a18c_00a0c9118956);
pub const FLOAT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = KSDS3D_HRTF_COEFF_FORMAT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FRAMING_CACHE_OPS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FRAMING_PROP(pub i32);
pub const FULL_FILTER: KSDS3D_HRTF_FILTER_QUALITY = KSDS3D_HRTF_FILTER_QUALITY(0i32);
pub const FramingProp_Ex: FRAMING_PROP = FRAMING_PROP(3i32);
pub const FramingProp_None: FRAMING_PROP = FRAMING_PROP(1i32);
pub const FramingProp_Old: FRAMING_PROP = FRAMING_PROP(2i32);
pub const FramingProp_Uninitialized: FRAMING_PROP = FRAMING_PROP(0i32);
pub const Framing_Cache_ReadLast: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(1i32);
pub const Framing_Cache_ReadOrig: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(2i32);
pub const Framing_Cache_Update: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(0i32);
pub const Framing_Cache_Write: FRAMING_CACHE_OPS = FRAMING_CACHE_OPS(3i32);
pub const GUID_NULL: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
windows_core::imp::define_interface!(IKsAggregateControl, IKsAggregateControl_Vtbl, 0x7f40eac0_3947_11d2_874e_00a0c9223196);
windows_core::imp::interface_hierarchy!(IKsAggregateControl, windows_core::IUnknown);
impl IKsAggregateControl {
    pub unsafe fn KsAddAggregate(&self, aggregateclass: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsAddAggregate)(windows_core::Interface::as_raw(self), aggregateclass).ok()
    }
    pub unsafe fn KsRemoveAggregate(&self, aggregateclass: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsRemoveAggregate)(windows_core::Interface::as_raw(self), aggregateclass).ok()
    }
}
#[repr(C)]
pub struct IKsAggregateControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsAddAggregate: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub KsRemoveAggregate: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IKsAggregateControl_Impl: windows_core::IUnknownImpl {
    fn KsAddAggregate(&self, aggregateclass: *const windows_core::GUID) -> windows_core::Result<()>;
    fn KsRemoveAggregate(&self, aggregateclass: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IKsAggregateControl_Vtbl {
    pub const fn new<Identity: IKsAggregateControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsAddAggregate<Identity: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aggregateclass: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAggregateControl_Impl::KsAddAggregate(this, core::mem::transmute_copy(&aggregateclass)).into()
        }
        unsafe extern "system" fn KsRemoveAggregate<Identity: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aggregateclass: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAggregateControl_Impl::KsRemoveAggregate(this, core::mem::transmute_copy(&aggregateclass)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsAddAggregate: KsAddAggregate::<Identity, OFFSET>,
            KsRemoveAggregate: KsRemoveAggregate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsAggregateControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsAggregateControl {}
windows_core::imp::define_interface!(IKsAllocator, IKsAllocator_Vtbl, 0x8da64899_c0d9_11d0_8413_0000f822fe8a);
windows_core::imp::interface_hierarchy!(IKsAllocator, windows_core::IUnknown);
impl IKsAllocator {
    pub unsafe fn KsGetAllocatorHandle(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).KsGetAllocatorHandle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn KsGetAllocatorMode(&self) -> KSALLOCATORMODE {
        (windows_core::Interface::vtable(self).KsGetAllocatorMode)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn KsGetAllocatorStatus(&self, allocatorstatus: *mut KSSTREAMALLOCATOR_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsGetAllocatorStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(allocatorstatus)).ok()
    }
    pub unsafe fn KsSetAllocatorMode(&self, mode: KSALLOCATORMODE) {
        (windows_core::Interface::vtable(self).KsSetAllocatorMode)(windows_core::Interface::as_raw(self), mode)
    }
}
#[repr(C)]
pub struct IKsAllocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsGetAllocatorHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub KsGetAllocatorMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> KSALLOCATORMODE,
    pub KsGetAllocatorStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KSSTREAMALLOCATOR_STATUS) -> windows_core::HRESULT,
    pub KsSetAllocatorMode: unsafe extern "system" fn(*mut core::ffi::c_void, KSALLOCATORMODE),
}
pub trait IKsAllocator_Impl: windows_core::IUnknownImpl {
    fn KsGetAllocatorHandle(&self) -> super::super::Foundation::HANDLE;
    fn KsGetAllocatorMode(&self) -> KSALLOCATORMODE;
    fn KsGetAllocatorStatus(&self, allocatorstatus: *mut KSSTREAMALLOCATOR_STATUS) -> windows_core::Result<()>;
    fn KsSetAllocatorMode(&self, mode: KSALLOCATORMODE);
}
impl IKsAllocator_Vtbl {
    pub const fn new<Identity: IKsAllocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsGetAllocatorHandle<Identity: IKsAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAllocator_Impl::KsGetAllocatorHandle(this)
        }
        unsafe extern "system" fn KsGetAllocatorMode<Identity: IKsAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> KSALLOCATORMODE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAllocator_Impl::KsGetAllocatorMode(this)
        }
        unsafe extern "system" fn KsGetAllocatorStatus<Identity: IKsAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocatorstatus: *mut KSSTREAMALLOCATOR_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAllocator_Impl::KsGetAllocatorStatus(this, core::mem::transmute_copy(&allocatorstatus)).into()
        }
        unsafe extern "system" fn KsSetAllocatorMode<Identity: IKsAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: KSALLOCATORMODE) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAllocator_Impl::KsSetAllocatorMode(this, core::mem::transmute_copy(&mode))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsGetAllocatorHandle: KsGetAllocatorHandle::<Identity, OFFSET>,
            KsGetAllocatorMode: KsGetAllocatorMode::<Identity, OFFSET>,
            KsGetAllocatorStatus: KsGetAllocatorStatus::<Identity, OFFSET>,
            KsSetAllocatorMode: KsSetAllocatorMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsAllocator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsAllocator {}
windows_core::imp::define_interface!(IKsAllocatorEx, IKsAllocatorEx_Vtbl, 0x091bb63a_603f_11d1_b067_00a0c9062802);
impl core::ops::Deref for IKsAllocatorEx {
    type Target = IKsAllocator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IKsAllocatorEx, windows_core::IUnknown, IKsAllocator);
impl IKsAllocatorEx {
    pub unsafe fn KsGetProperties(&self) -> *mut ALLOCATOR_PROPERTIES_EX {
        (windows_core::Interface::vtable(self).KsGetProperties)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn KsSetProperties(&self, param0: *const ALLOCATOR_PROPERTIES_EX) {
        (windows_core::Interface::vtable(self).KsSetProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(param0))
    }
    pub unsafe fn KsSetAllocatorHandle(&self, allocatorhandle: super::super::Foundation::HANDLE) {
        (windows_core::Interface::vtable(self).KsSetAllocatorHandle)(windows_core::Interface::as_raw(self), allocatorhandle)
    }
    pub unsafe fn KsCreateAllocatorAndGetHandle<P0>(&self, kspin: P0) -> super::super::Foundation::HANDLE
    where
        P0: windows_core::Param<IKsPin>,
    {
        (windows_core::Interface::vtable(self).KsCreateAllocatorAndGetHandle)(windows_core::Interface::as_raw(self), kspin.param().abi())
    }
}
#[repr(C)]
pub struct IKsAllocatorEx_Vtbl {
    pub base__: IKsAllocator_Vtbl,
    pub KsGetProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut ALLOCATOR_PROPERTIES_EX,
    pub KsSetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const ALLOCATOR_PROPERTIES_EX),
    pub KsSetAllocatorHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE),
    pub KsCreateAllocatorAndGetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
}
pub trait IKsAllocatorEx_Impl: IKsAllocator_Impl {
    fn KsGetProperties(&self) -> *mut ALLOCATOR_PROPERTIES_EX;
    fn KsSetProperties(&self, param0: *const ALLOCATOR_PROPERTIES_EX);
    fn KsSetAllocatorHandle(&self, allocatorhandle: super::super::Foundation::HANDLE);
    fn KsCreateAllocatorAndGetHandle(&self, kspin: Option<&IKsPin>) -> super::super::Foundation::HANDLE;
}
impl IKsAllocatorEx_Vtbl {
    pub const fn new<Identity: IKsAllocatorEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsGetProperties<Identity: IKsAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut ALLOCATOR_PROPERTIES_EX {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAllocatorEx_Impl::KsGetProperties(this)
        }
        unsafe extern "system" fn KsSetProperties<Identity: IKsAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const ALLOCATOR_PROPERTIES_EX) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAllocatorEx_Impl::KsSetProperties(this, core::mem::transmute_copy(&param0))
        }
        unsafe extern "system" fn KsSetAllocatorHandle<Identity: IKsAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocatorhandle: super::super::Foundation::HANDLE) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAllocatorEx_Impl::KsSetAllocatorHandle(this, core::mem::transmute_copy(&allocatorhandle))
        }
        unsafe extern "system" fn KsCreateAllocatorAndGetHandle<Identity: IKsAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kspin: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsAllocatorEx_Impl::KsCreateAllocatorAndGetHandle(this, windows_core::from_raw_borrowed(&kspin))
        }
        Self {
            base__: IKsAllocator_Vtbl::new::<Identity, OFFSET>(),
            KsGetProperties: KsGetProperties::<Identity, OFFSET>,
            KsSetProperties: KsSetProperties::<Identity, OFFSET>,
            KsSetAllocatorHandle: KsSetAllocatorHandle::<Identity, OFFSET>,
            KsCreateAllocatorAndGetHandle: KsCreateAllocatorAndGetHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsAllocatorEx as windows_core::Interface>::IID || iid == &<IKsAllocator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsAllocatorEx {}
windows_core::imp::define_interface!(IKsClockPropertySet, IKsClockPropertySet_Vtbl, 0x5c5cbd84_e755_11d0_ac18_00a0c9223196);
windows_core::imp::interface_hierarchy!(IKsClockPropertySet, windows_core::IUnknown);
impl IKsClockPropertySet {
    pub unsafe fn KsGetTime(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsGetTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn KsSetTime(&self, time: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsSetTime)(windows_core::Interface::as_raw(self), time).ok()
    }
    pub unsafe fn KsGetPhysicalTime(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsGetPhysicalTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn KsSetPhysicalTime(&self, time: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsSetPhysicalTime)(windows_core::Interface::as_raw(self), time).ok()
    }
    pub unsafe fn KsGetCorrelatedTime(&self) -> windows_core::Result<KSCORRELATED_TIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsGetCorrelatedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn KsSetCorrelatedTime(&self, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsSetCorrelatedTime)(windows_core::Interface::as_raw(self), correlatedtime).ok()
    }
    pub unsafe fn KsGetCorrelatedPhysicalTime(&self) -> windows_core::Result<KSCORRELATED_TIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsGetCorrelatedPhysicalTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn KsSetCorrelatedPhysicalTime(&self, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsSetCorrelatedPhysicalTime)(windows_core::Interface::as_raw(self), correlatedtime).ok()
    }
    pub unsafe fn KsGetResolution(&self) -> windows_core::Result<KSRESOLUTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsGetResolution)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn KsGetState(&self) -> windows_core::Result<KSSTATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsGetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IKsClockPropertySet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsGetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub KsSetTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub KsGetPhysicalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub KsSetPhysicalTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub KsGetCorrelatedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KSCORRELATED_TIME) -> windows_core::HRESULT,
    pub KsSetCorrelatedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const KSCORRELATED_TIME) -> windows_core::HRESULT,
    pub KsGetCorrelatedPhysicalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KSCORRELATED_TIME) -> windows_core::HRESULT,
    pub KsSetCorrelatedPhysicalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const KSCORRELATED_TIME) -> windows_core::HRESULT,
    pub KsGetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KSRESOLUTION) -> windows_core::HRESULT,
    pub KsGetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KSSTATE) -> windows_core::HRESULT,
}
pub trait IKsClockPropertySet_Impl: windows_core::IUnknownImpl {
    fn KsGetTime(&self) -> windows_core::Result<i64>;
    fn KsSetTime(&self, time: i64) -> windows_core::Result<()>;
    fn KsGetPhysicalTime(&self) -> windows_core::Result<i64>;
    fn KsSetPhysicalTime(&self, time: i64) -> windows_core::Result<()>;
    fn KsGetCorrelatedTime(&self) -> windows_core::Result<KSCORRELATED_TIME>;
    fn KsSetCorrelatedTime(&self, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::Result<()>;
    fn KsGetCorrelatedPhysicalTime(&self) -> windows_core::Result<KSCORRELATED_TIME>;
    fn KsSetCorrelatedPhysicalTime(&self, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::Result<()>;
    fn KsGetResolution(&self) -> windows_core::Result<KSRESOLUTION>;
    fn KsGetState(&self) -> windows_core::Result<KSSTATE>;
}
impl IKsClockPropertySet_Vtbl {
    pub const fn new<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsGetTime<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: *mut i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsClockPropertySet_Impl::KsGetTime(this) {
                Ok(ok__) => {
                    time.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetTime<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsClockPropertySet_Impl::KsSetTime(this, core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn KsGetPhysicalTime<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: *mut i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsClockPropertySet_Impl::KsGetPhysicalTime(this) {
                Ok(ok__) => {
                    time.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetPhysicalTime<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsClockPropertySet_Impl::KsSetPhysicalTime(this, core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn KsGetCorrelatedTime<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, correlatedtime: *mut KSCORRELATED_TIME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsClockPropertySet_Impl::KsGetCorrelatedTime(this) {
                Ok(ok__) => {
                    correlatedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetCorrelatedTime<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsClockPropertySet_Impl::KsSetCorrelatedTime(this, core::mem::transmute_copy(&correlatedtime)).into()
        }
        unsafe extern "system" fn KsGetCorrelatedPhysicalTime<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, correlatedtime: *mut KSCORRELATED_TIME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsClockPropertySet_Impl::KsGetCorrelatedPhysicalTime(this) {
                Ok(ok__) => {
                    correlatedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetCorrelatedPhysicalTime<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsClockPropertySet_Impl::KsSetCorrelatedPhysicalTime(this, core::mem::transmute_copy(&correlatedtime)).into()
        }
        unsafe extern "system" fn KsGetResolution<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolution: *mut KSRESOLUTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsClockPropertySet_Impl::KsGetResolution(this) {
                Ok(ok__) => {
                    resolution.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsGetState<Identity: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut KSSTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsClockPropertySet_Impl::KsGetState(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsGetTime: KsGetTime::<Identity, OFFSET>,
            KsSetTime: KsSetTime::<Identity, OFFSET>,
            KsGetPhysicalTime: KsGetPhysicalTime::<Identity, OFFSET>,
            KsSetPhysicalTime: KsSetPhysicalTime::<Identity, OFFSET>,
            KsGetCorrelatedTime: KsGetCorrelatedTime::<Identity, OFFSET>,
            KsSetCorrelatedTime: KsSetCorrelatedTime::<Identity, OFFSET>,
            KsGetCorrelatedPhysicalTime: KsGetCorrelatedPhysicalTime::<Identity, OFFSET>,
            KsSetCorrelatedPhysicalTime: KsSetCorrelatedPhysicalTime::<Identity, OFFSET>,
            KsGetResolution: KsGetResolution::<Identity, OFFSET>,
            KsGetState: KsGetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsClockPropertySet as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsClockPropertySet {}
windows_core::imp::define_interface!(IKsControl, IKsControl_Vtbl, 0x28f54685_06fd_11d2_b27a_00a0c9223196);
windows_core::imp::interface_hierarchy!(IKsControl, windows_core::IUnknown);
impl IKsControl {
    pub unsafe fn KsProperty(&self, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsProperty)(windows_core::Interface::as_raw(self), property, propertylength, core::mem::transmute(propertydata), datalength, core::mem::transmute(bytesreturned)).ok()
    }
    pub unsafe fn KsMethod(&self, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsMethod)(windows_core::Interface::as_raw(self), method, methodlength, core::mem::transmute(methoddata), datalength, core::mem::transmute(bytesreturned)).ok()
    }
    pub unsafe fn KsEvent(&self, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsEvent)(windows_core::Interface::as_raw(self), event, eventlength, core::mem::transmute(eventdata), datalength, core::mem::transmute(bytesreturned)).ok()
    }
}
#[repr(C)]
pub struct IKsControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const KSIDENTIFIER, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub KsMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *const KSIDENTIFIER, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub KsEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const KSIDENTIFIER, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IKsControl_Impl: windows_core::IUnknownImpl {
    fn KsProperty(&self, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
    fn KsMethod(&self, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
    fn KsEvent(&self, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
}
impl IKsControl_Vtbl {
    pub const fn new<Identity: IKsControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsProperty<Identity: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsControl_Impl::KsProperty(this, core::mem::transmute_copy(&property), core::mem::transmute_copy(&propertylength), core::mem::transmute_copy(&propertydata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsMethod<Identity: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsControl_Impl::KsMethod(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&methodlength), core::mem::transmute_copy(&methoddata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsEvent<Identity: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsControl_Impl::KsEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&eventlength), core::mem::transmute_copy(&eventdata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsProperty: KsProperty::<Identity, OFFSET>,
            KsMethod: KsMethod::<Identity, OFFSET>,
            KsEvent: KsEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsControl {}
windows_core::imp::define_interface!(IKsDataTypeCompletion, IKsDataTypeCompletion_Vtbl, 0x827d1a0e_0f73_11d2_b27a_00a0c9223196);
windows_core::imp::interface_hierarchy!(IKsDataTypeCompletion, windows_core::IUnknown);
impl IKsDataTypeCompletion {
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn KsCompleteMediaType(&self, filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32, ammediatype: *mut super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsCompleteMediaType)(windows_core::Interface::as_raw(self), filterhandle, pinfactoryid, core::mem::transmute(ammediatype)).ok()
    }
}
#[repr(C)]
pub struct IKsDataTypeCompletion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub KsCompleteMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, u32, *mut super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    KsCompleteMediaType: usize,
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IKsDataTypeCompletion_Impl: windows_core::IUnknownImpl {
    fn KsCompleteMediaType(&self, filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32, ammediatype: *mut super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IKsDataTypeCompletion_Vtbl {
    pub const fn new<Identity: IKsDataTypeCompletion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsCompleteMediaType<Identity: IKsDataTypeCompletion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32, ammediatype: *mut super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsDataTypeCompletion_Impl::KsCompleteMediaType(this, core::mem::transmute_copy(&filterhandle), core::mem::transmute_copy(&pinfactoryid), core::mem::transmute_copy(&ammediatype)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), KsCompleteMediaType: KsCompleteMediaType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsDataTypeCompletion as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::RuntimeName for IKsDataTypeCompletion {}
windows_core::imp::define_interface!(IKsDataTypeHandler, IKsDataTypeHandler_Vtbl, 0x5ffbaa02_49a3_11d0_9f36_00aa00a216a1);
windows_core::imp::interface_hierarchy!(IKsDataTypeHandler, windows_core::IUnknown);
impl IKsDataTypeHandler {
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn KsCompleteIoOperation<P0>(&self, sample: P0, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION, cancelled: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectShow::IMediaSample>,
    {
        (windows_core::Interface::vtable(self).KsCompleteIoOperation)(windows_core::Interface::as_raw(self), sample.param().abi(), core::mem::transmute(streamheader), iooperation, cancelled.into()).ok()
    }
    pub unsafe fn KsIsMediaTypeInRanges(&self, dataranges: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsIsMediaTypeInRanges)(windows_core::Interface::as_raw(self), dataranges).ok()
    }
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn KsPrepareIoOperation<P0>(&self, sample: P0, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectShow::IMediaSample>,
    {
        (windows_core::Interface::vtable(self).KsPrepareIoOperation)(windows_core::Interface::as_raw(self), sample.param().abi(), core::mem::transmute(streamheader), iooperation).ok()
    }
    pub unsafe fn KsQueryExtendedSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsQueryExtendedSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn KsSetMediaType(&self, ammediatype: *const super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsSetMediaType)(windows_core::Interface::as_raw(self), core::mem::transmute(ammediatype)).ok()
    }
}
#[repr(C)]
pub struct IKsDataTypeHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub KsCompleteIoOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, KSIOOPERATION, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    KsCompleteIoOperation: usize,
    pub KsIsMediaTypeInRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub KsPrepareIoOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, KSIOOPERATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    KsPrepareIoOperation: usize,
    pub KsQueryExtendedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub KsSetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    KsSetMediaType: usize,
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_Media_MediaFoundation"))]
pub trait IKsDataTypeHandler_Impl: windows_core::IUnknownImpl {
    fn KsCompleteIoOperation(&self, sample: Option<&super::DirectShow::IMediaSample>, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION, cancelled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn KsIsMediaTypeInRanges(&self, dataranges: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn KsPrepareIoOperation(&self, sample: Option<&super::DirectShow::IMediaSample>, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION) -> windows_core::Result<()>;
    fn KsQueryExtendedSize(&self) -> windows_core::Result<u32>;
    fn KsSetMediaType(&self, ammediatype: *const super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_Media_MediaFoundation"))]
impl IKsDataTypeHandler_Vtbl {
    pub const fn new<Identity: IKsDataTypeHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsCompleteIoOperation<Identity: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sample: *mut core::ffi::c_void, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION, cancelled: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsDataTypeHandler_Impl::KsCompleteIoOperation(this, windows_core::from_raw_borrowed(&sample), core::mem::transmute_copy(&streamheader), core::mem::transmute_copy(&iooperation), core::mem::transmute_copy(&cancelled)).into()
        }
        unsafe extern "system" fn KsIsMediaTypeInRanges<Identity: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataranges: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsDataTypeHandler_Impl::KsIsMediaTypeInRanges(this, core::mem::transmute_copy(&dataranges)).into()
        }
        unsafe extern "system" fn KsPrepareIoOperation<Identity: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sample: *mut core::ffi::c_void, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsDataTypeHandler_Impl::KsPrepareIoOperation(this, windows_core::from_raw_borrowed(&sample), core::mem::transmute_copy(&streamheader), core::mem::transmute_copy(&iooperation)).into()
        }
        unsafe extern "system" fn KsQueryExtendedSize<Identity: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendedsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsDataTypeHandler_Impl::KsQueryExtendedSize(this) {
                Ok(ok__) => {
                    extendedsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetMediaType<Identity: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ammediatype: *const super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsDataTypeHandler_Impl::KsSetMediaType(this, core::mem::transmute_copy(&ammediatype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsCompleteIoOperation: KsCompleteIoOperation::<Identity, OFFSET>,
            KsIsMediaTypeInRanges: KsIsMediaTypeInRanges::<Identity, OFFSET>,
            KsPrepareIoOperation: KsPrepareIoOperation::<Identity, OFFSET>,
            KsQueryExtendedSize: KsQueryExtendedSize::<Identity, OFFSET>,
            KsSetMediaType: KsSetMediaType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsDataTypeHandler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IKsDataTypeHandler {}
windows_core::imp::define_interface!(IKsFormatSupport, IKsFormatSupport_Vtbl, 0x3cb4a69d_bb6f_4d2b_95b7_452d2c155db5);
windows_core::imp::interface_hierarchy!(IKsFormatSupport, windows_core::IUnknown);
impl IKsFormatSupport {
    pub unsafe fn IsFormatSupported(&self, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsFormatSupported)(windows_core::Interface::as_raw(self), core::mem::transmute(pksformat), cbformat, core::mem::transmute(pbsupported)).ok()
    }
    pub unsafe fn GetDevicePreferredFormat(&self) -> windows_core::Result<*mut KSDATAFORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDevicePreferredFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IKsFormatSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KSDATAFORMAT, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetDevicePreferredFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut KSDATAFORMAT) -> windows_core::HRESULT,
}
pub trait IKsFormatSupport_Impl: windows_core::IUnknownImpl {
    fn IsFormatSupported(&self, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetDevicePreferredFormat(&self) -> windows_core::Result<*mut KSDATAFORMAT>;
}
impl IKsFormatSupport_Vtbl {
    pub const fn new<Identity: IKsFormatSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsFormatSupported<Identity: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsFormatSupport_Impl::IsFormatSupported(this, core::mem::transmute_copy(&pksformat), core::mem::transmute_copy(&cbformat), core::mem::transmute_copy(&pbsupported)).into()
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Identity: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppksformat: *mut *mut KSDATAFORMAT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsFormatSupport_Impl::GetDevicePreferredFormat(this) {
                Ok(ok__) => {
                    ppksformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsFormatSupported: IsFormatSupported::<Identity, OFFSET>,
            GetDevicePreferredFormat: GetDevicePreferredFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsFormatSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsFormatSupport {}
windows_core::imp::define_interface!(IKsInterfaceHandler, IKsInterfaceHandler_Vtbl, 0xd3abc7e0_9a61_11d0_a40d_00a0c9223196);
windows_core::imp::interface_hierarchy!(IKsInterfaceHandler, windows_core::IUnknown);
impl IKsInterfaceHandler {
    pub unsafe fn KsSetPin<P0>(&self, kspin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IKsPin>,
    {
        (windows_core::Interface::vtable(self).KsSetPin)(windows_core::Interface::as_raw(self), kspin.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn KsProcessMediaSamples<P0>(&self, ksdatatypehandler: P0, samplelist: *const Option<super::DirectShow::IMediaSample>, samplecount: *mut i32, iooperation: KSIOOPERATION, streamsegment: *mut *mut KSSTREAM_SEGMENT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IKsDataTypeHandler>,
    {
        (windows_core::Interface::vtable(self).KsProcessMediaSamples)(windows_core::Interface::as_raw(self), ksdatatypehandler.param().abi(), core::mem::transmute(samplelist), core::mem::transmute(samplecount), iooperation, core::mem::transmute(streamsegment)).ok()
    }
    pub unsafe fn KsCompleteIo(&self, streamsegment: *mut KSSTREAM_SEGMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsCompleteIo)(windows_core::Interface::as_raw(self), core::mem::transmute(streamsegment)).ok()
    }
}
#[repr(C)]
pub struct IKsInterfaceHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsSetPin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub KsProcessMediaSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *mut core::ffi::c_void, *mut i32, KSIOOPERATION, *mut *mut KSSTREAM_SEGMENT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    KsProcessMediaSamples: usize,
    pub KsCompleteIo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KSSTREAM_SEGMENT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IKsInterfaceHandler_Impl: windows_core::IUnknownImpl {
    fn KsSetPin(&self, kspin: Option<&IKsPin>) -> windows_core::Result<()>;
    fn KsProcessMediaSamples(&self, ksdatatypehandler: Option<&IKsDataTypeHandler>, samplelist: *const Option<super::DirectShow::IMediaSample>, samplecount: *mut i32, iooperation: KSIOOPERATION, streamsegment: *mut *mut KSSTREAM_SEGMENT) -> windows_core::Result<()>;
    fn KsCompleteIo(&self, streamsegment: *mut KSSTREAM_SEGMENT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IKsInterfaceHandler_Vtbl {
    pub const fn new<Identity: IKsInterfaceHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsSetPin<Identity: IKsInterfaceHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kspin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsInterfaceHandler_Impl::KsSetPin(this, windows_core::from_raw_borrowed(&kspin)).into()
        }
        unsafe extern "system" fn KsProcessMediaSamples<Identity: IKsInterfaceHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ksdatatypehandler: *mut core::ffi::c_void, samplelist: *const *mut core::ffi::c_void, samplecount: *mut i32, iooperation: KSIOOPERATION, streamsegment: *mut *mut KSSTREAM_SEGMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsInterfaceHandler_Impl::KsProcessMediaSamples(this, windows_core::from_raw_borrowed(&ksdatatypehandler), core::mem::transmute_copy(&samplelist), core::mem::transmute_copy(&samplecount), core::mem::transmute_copy(&iooperation), core::mem::transmute_copy(&streamsegment)).into()
        }
        unsafe extern "system" fn KsCompleteIo<Identity: IKsInterfaceHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamsegment: *mut KSSTREAM_SEGMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsInterfaceHandler_Impl::KsCompleteIo(this, core::mem::transmute_copy(&streamsegment)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsSetPin: KsSetPin::<Identity, OFFSET>,
            KsProcessMediaSamples: KsProcessMediaSamples::<Identity, OFFSET>,
            KsCompleteIo: KsCompleteIo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsInterfaceHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl windows_core::RuntimeName for IKsInterfaceHandler {}
windows_core::imp::define_interface!(IKsJackContainerId, IKsJackContainerId_Vtbl, 0xc99af463_d629_4ec4_8c00_e54d68154248);
windows_core::imp::interface_hierarchy!(IKsJackContainerId, windows_core::IUnknown);
impl IKsJackContainerId {
    pub unsafe fn GetJackContainerId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetJackContainerId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IKsJackContainerId_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackContainerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IKsJackContainerId_Impl: windows_core::IUnknownImpl {
    fn GetJackContainerId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IKsJackContainerId_Vtbl {
    pub const fn new<Identity: IKsJackContainerId_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackContainerId<Identity: IKsJackContainerId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjackcontainerid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsJackContainerId_Impl::GetJackContainerId(this) {
                Ok(ok__) => {
                    pjackcontainerid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetJackContainerId: GetJackContainerId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackContainerId as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsJackContainerId {}
windows_core::imp::define_interface!(IKsJackDescription, IKsJackDescription_Vtbl, 0x4509f757_2d46_4637_8e62_ce7db944f57b);
windows_core::imp::interface_hierarchy!(IKsJackDescription, windows_core::IUnknown);
impl IKsJackDescription {
    pub unsafe fn GetJackCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetJackCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetJackDescription(&self, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetJackDescription)(windows_core::Interface::as_raw(self), njack, core::mem::transmute(pdescription)).ok()
    }
}
#[repr(C)]
pub struct IKsJackDescription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetJackDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut KSJACK_DESCRIPTION) -> windows_core::HRESULT,
}
pub trait IKsJackDescription_Impl: windows_core::IUnknownImpl {
    fn GetJackCount(&self) -> windows_core::Result<u32>;
    fn GetJackDescription(&self, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> windows_core::Result<()>;
}
impl IKsJackDescription_Vtbl {
    pub const fn new<Identity: IKsJackDescription_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackCount<Identity: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsJackDescription_Impl::GetJackCount(this) {
                Ok(ok__) => {
                    pcjacks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription<Identity: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsJackDescription_Impl::GetJackDescription(this, core::mem::transmute_copy(&njack), core::mem::transmute_copy(&pdescription)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, OFFSET>,
            GetJackDescription: GetJackDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackDescription as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsJackDescription {}
windows_core::imp::define_interface!(IKsJackDescription2, IKsJackDescription2_Vtbl, 0x478f3a9b_e0c9_4827_9228_6f5505ffe76a);
windows_core::imp::interface_hierarchy!(IKsJackDescription2, windows_core::IUnknown);
impl IKsJackDescription2 {
    pub unsafe fn GetJackCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetJackCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetJackDescription2(&self, njack: u32) -> windows_core::Result<KSJACK_DESCRIPTION2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetJackDescription2)(windows_core::Interface::as_raw(self), njack, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IKsJackDescription2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetJackDescription2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut KSJACK_DESCRIPTION2) -> windows_core::HRESULT,
}
pub trait IKsJackDescription2_Impl: windows_core::IUnknownImpl {
    fn GetJackCount(&self) -> windows_core::Result<u32>;
    fn GetJackDescription2(&self, njack: u32) -> windows_core::Result<KSJACK_DESCRIPTION2>;
}
impl IKsJackDescription2_Vtbl {
    pub const fn new<Identity: IKsJackDescription2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackCount<Identity: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsJackDescription2_Impl::GetJackCount(this) {
                Ok(ok__) => {
                    pcjacks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription2<Identity: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsJackDescription2_Impl::GetJackDescription2(this, core::mem::transmute_copy(&njack)) {
                Ok(ok__) => {
                    pdescription2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, OFFSET>,
            GetJackDescription2: GetJackDescription2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackDescription2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsJackDescription2 {}
windows_core::imp::define_interface!(IKsJackDescription3, IKsJackDescription3_Vtbl, 0xe3f6778b_6660_4cc8_a291_ecc4192d9967);
windows_core::imp::interface_hierarchy!(IKsJackDescription3, windows_core::IUnknown);
impl IKsJackDescription3 {
    pub unsafe fn GetJackCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetJackCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetJackDescription3(&self, njack: u32) -> windows_core::Result<KSJACK_DESCRIPTION3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetJackDescription3)(windows_core::Interface::as_raw(self), njack, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IKsJackDescription3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetJackDescription3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut KSJACK_DESCRIPTION3) -> windows_core::HRESULT,
}
pub trait IKsJackDescription3_Impl: windows_core::IUnknownImpl {
    fn GetJackCount(&self) -> windows_core::Result<u32>;
    fn GetJackDescription3(&self, njack: u32) -> windows_core::Result<KSJACK_DESCRIPTION3>;
}
impl IKsJackDescription3_Vtbl {
    pub const fn new<Identity: IKsJackDescription3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackCount<Identity: IKsJackDescription3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsJackDescription3_Impl::GetJackCount(this) {
                Ok(ok__) => {
                    pcjacks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription3<Identity: IKsJackDescription3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: u32, pdescription3: *mut KSJACK_DESCRIPTION3) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsJackDescription3_Impl::GetJackDescription3(this, core::mem::transmute_copy(&njack)) {
                Ok(ok__) => {
                    pdescription3.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, OFFSET>,
            GetJackDescription3: GetJackDescription3::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackDescription3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsJackDescription3 {}
windows_core::imp::define_interface!(IKsJackSinkInformation, IKsJackSinkInformation_Vtbl, 0xd9bd72ed_290f_4581_9ff3_61027a8fe532);
windows_core::imp::interface_hierarchy!(IKsJackSinkInformation, windows_core::IUnknown);
impl IKsJackSinkInformation {
    pub unsafe fn GetJackSinkInformation(&self, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetJackSinkInformation)(windows_core::Interface::as_raw(self), core::mem::transmute(pjacksinkinformation)).ok()
    }
}
#[repr(C)]
pub struct IKsJackSinkInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackSinkInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KSJACK_SINK_INFORMATION) -> windows_core::HRESULT,
}
pub trait IKsJackSinkInformation_Impl: windows_core::IUnknownImpl {
    fn GetJackSinkInformation(&self, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> windows_core::Result<()>;
}
impl IKsJackSinkInformation_Vtbl {
    pub const fn new<Identity: IKsJackSinkInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackSinkInformation<Identity: IKsJackSinkInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsJackSinkInformation_Impl::GetJackSinkInformation(this, core::mem::transmute_copy(&pjacksinkinformation)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetJackSinkInformation: GetJackSinkInformation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackSinkInformation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsJackSinkInformation {}
windows_core::imp::define_interface!(IKsNodeControl, IKsNodeControl_Vtbl, 0x11737c14_24a7_4bb5_81a0_0d003813b0c4);
windows_core::imp::interface_hierarchy!(IKsNodeControl, windows_core::IUnknown);
impl IKsNodeControl {
    pub unsafe fn SetNodeId(&self, dwnodeid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNodeId)(windows_core::Interface::as_raw(self), dwnodeid).ok()
    }
    pub unsafe fn SetKsControl(&self, pkscontrol: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetKsControl)(windows_core::Interface::as_raw(self), pkscontrol).ok()
    }
}
#[repr(C)]
pub struct IKsNodeControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetNodeId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetKsControl: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IKsNodeControl_Impl: windows_core::IUnknownImpl {
    fn SetNodeId(&self, dwnodeid: u32) -> windows_core::Result<()>;
    fn SetKsControl(&self, pkscontrol: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IKsNodeControl_Vtbl {
    pub const fn new<Identity: IKsNodeControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNodeId<Identity: IKsNodeControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnodeid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsNodeControl_Impl::SetNodeId(this, core::mem::transmute_copy(&dwnodeid)).into()
        }
        unsafe extern "system" fn SetKsControl<Identity: IKsNodeControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkscontrol: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsNodeControl_Impl::SetKsControl(this, core::mem::transmute_copy(&pkscontrol)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetNodeId: SetNodeId::<Identity, OFFSET>,
            SetKsControl: SetKsControl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsNodeControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsNodeControl {}
windows_core::imp::define_interface!(IKsNotifyEvent, IKsNotifyEvent_Vtbl, 0x412bd695_f84b_46c1_ac73_54196dbc8fa7);
windows_core::imp::interface_hierarchy!(IKsNotifyEvent, windows_core::IUnknown);
impl IKsNotifyEvent {
    pub unsafe fn KsNotifyEvent(&self, event: u32, lparam1: usize, lparam2: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsNotifyEvent)(windows_core::Interface::as_raw(self), event, lparam1, lparam2).ok()
    }
}
#[repr(C)]
pub struct IKsNotifyEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsNotifyEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, usize) -> windows_core::HRESULT,
}
pub trait IKsNotifyEvent_Impl: windows_core::IUnknownImpl {
    fn KsNotifyEvent(&self, event: u32, lparam1: usize, lparam2: usize) -> windows_core::Result<()>;
}
impl IKsNotifyEvent_Vtbl {
    pub const fn new<Identity: IKsNotifyEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsNotifyEvent<Identity: IKsNotifyEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: u32, lparam1: usize, lparam2: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsNotifyEvent_Impl::KsNotifyEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&lparam1), core::mem::transmute_copy(&lparam2)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), KsNotifyEvent: KsNotifyEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsNotifyEvent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsNotifyEvent {}
windows_core::imp::define_interface!(IKsObject, IKsObject_Vtbl, 0x423c13a2_2070_11d0_9ef7_00aa00a216a1);
windows_core::imp::interface_hierarchy!(IKsObject, windows_core::IUnknown);
impl IKsObject {
    pub unsafe fn KsGetObjectHandle(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).KsGetObjectHandle)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IKsObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsGetObjectHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
}
pub trait IKsObject_Impl: windows_core::IUnknownImpl {
    fn KsGetObjectHandle(&self) -> super::super::Foundation::HANDLE;
}
impl IKsObject_Vtbl {
    pub const fn new<Identity: IKsObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsGetObjectHandle<Identity: IKsObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsObject_Impl::KsGetObjectHandle(this)
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), KsGetObjectHandle: KsGetObjectHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsObject {}
windows_core::imp::define_interface!(IKsPin, IKsPin_Vtbl, 0xb61178d1_a2d9_11cf_9e53_00aa00a216a1);
windows_core::imp::interface_hierarchy!(IKsPin, windows_core::IUnknown);
impl IKsPin {
    pub unsafe fn KsQueryMediums(&self) -> windows_core::Result<*mut KSMULTIPLE_ITEM> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsQueryMediums)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn KsQueryInterfaces(&self) -> windows_core::Result<*mut KSMULTIPLE_ITEM> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsQueryInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn KsCreateSinkPinHandle(&self, interface: *const KSIDENTIFIER, medium: *const KSIDENTIFIER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsCreateSinkPinHandle)(windows_core::Interface::as_raw(self), interface, medium).ok()
    }
    pub unsafe fn KsGetCurrentCommunication(&self, communication: Option<*mut KSPIN_COMMUNICATION>, interface: Option<*mut KSIDENTIFIER>, medium: Option<*mut KSIDENTIFIER>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsGetCurrentCommunication)(windows_core::Interface::as_raw(self), core::mem::transmute(communication.unwrap_or(core::mem::zeroed())), core::mem::transmute(interface.unwrap_or(core::mem::zeroed())), core::mem::transmute(medium.unwrap_or(core::mem::zeroed()))).ok()
    }
    pub unsafe fn KsPropagateAcquire(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsPropagateAcquire)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn KsDeliver<P0>(&self, sample: P0, flags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectShow::IMediaSample>,
    {
        (windows_core::Interface::vtable(self).KsDeliver)(windows_core::Interface::as_raw(self), sample.param().abi(), flags).ok()
    }
    pub unsafe fn KsMediaSamplesCompleted(&self, streamsegment: *const KSSTREAM_SEGMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsMediaSamplesCompleted)(windows_core::Interface::as_raw(self), core::mem::transmute(streamsegment)).ok()
    }
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn KsPeekAllocator(&self, operation: KSPEEKOPERATION) -> Option<super::DirectShow::IMemAllocator> {
        (windows_core::Interface::vtable(self).KsPeekAllocator)(windows_core::Interface::as_raw(self), operation)
    }
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn KsReceiveAllocator<P0>(&self, memallocator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectShow::IMemAllocator>,
    {
        (windows_core::Interface::vtable(self).KsReceiveAllocator)(windows_core::Interface::as_raw(self), memallocator.param().abi()).ok()
    }
    pub unsafe fn KsRenegotiateAllocator(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsRenegotiateAllocator)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn KsIncrementPendingIoCount(&self) -> i32 {
        (windows_core::Interface::vtable(self).KsIncrementPendingIoCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn KsDecrementPendingIoCount(&self) -> i32 {
        (windows_core::Interface::vtable(self).KsDecrementPendingIoCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn KsQualityNotify(&self, proportion: u32, timedelta: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsQualityNotify)(windows_core::Interface::as_raw(self), proportion, timedelta).ok()
    }
}
#[repr(C)]
pub struct IKsPin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsQueryMediums: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut KSMULTIPLE_ITEM) -> windows_core::HRESULT,
    pub KsQueryInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut KSMULTIPLE_ITEM) -> windows_core::HRESULT,
    pub KsCreateSinkPinHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const KSIDENTIFIER, *const KSIDENTIFIER) -> windows_core::HRESULT,
    pub KsGetCurrentCommunication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KSPIN_COMMUNICATION, *mut KSIDENTIFIER, *mut KSIDENTIFIER) -> windows_core::HRESULT,
    pub KsPropagateAcquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub KsDeliver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    KsDeliver: usize,
    pub KsMediaSamplesCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *const KSSTREAM_SEGMENT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub KsPeekAllocator: unsafe extern "system" fn(*mut core::ffi::c_void, KSPEEKOPERATION) -> Option<super::DirectShow::IMemAllocator>,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    KsPeekAllocator: usize,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub KsReceiveAllocator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    KsReceiveAllocator: usize,
    pub KsRenegotiateAllocator: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KsIncrementPendingIoCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    pub KsDecrementPendingIoCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    pub KsQualityNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IKsPin_Impl: windows_core::IUnknownImpl {
    fn KsQueryMediums(&self) -> windows_core::Result<*mut KSMULTIPLE_ITEM>;
    fn KsQueryInterfaces(&self) -> windows_core::Result<*mut KSMULTIPLE_ITEM>;
    fn KsCreateSinkPinHandle(&self, interface: *const KSIDENTIFIER, medium: *const KSIDENTIFIER) -> windows_core::Result<()>;
    fn KsGetCurrentCommunication(&self, communication: *mut KSPIN_COMMUNICATION, interface: *mut KSIDENTIFIER, medium: *mut KSIDENTIFIER) -> windows_core::Result<()>;
    fn KsPropagateAcquire(&self) -> windows_core::Result<()>;
    fn KsDeliver(&self, sample: Option<&super::DirectShow::IMediaSample>, flags: u32) -> windows_core::Result<()>;
    fn KsMediaSamplesCompleted(&self, streamsegment: *const KSSTREAM_SEGMENT) -> windows_core::Result<()>;
    fn KsPeekAllocator(&self, operation: KSPEEKOPERATION) -> Option<super::DirectShow::IMemAllocator>;
    fn KsReceiveAllocator(&self, memallocator: Option<&super::DirectShow::IMemAllocator>) -> windows_core::Result<()>;
    fn KsRenegotiateAllocator(&self) -> windows_core::Result<()>;
    fn KsIncrementPendingIoCount(&self) -> i32;
    fn KsDecrementPendingIoCount(&self) -> i32;
    fn KsQualityNotify(&self, proportion: u32, timedelta: i64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IKsPin_Vtbl {
    pub const fn new<Identity: IKsPin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsQueryMediums<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediumlist: *mut *mut KSMULTIPLE_ITEM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsPin_Impl::KsQueryMediums(this) {
                Ok(ok__) => {
                    mediumlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsQueryInterfaces<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacelist: *mut *mut KSMULTIPLE_ITEM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsPin_Impl::KsQueryInterfaces(this) {
                Ok(ok__) => {
                    interfacelist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsCreateSinkPinHandle<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interface: *const KSIDENTIFIER, medium: *const KSIDENTIFIER) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsCreateSinkPinHandle(this, core::mem::transmute_copy(&interface), core::mem::transmute_copy(&medium)).into()
        }
        unsafe extern "system" fn KsGetCurrentCommunication<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, communication: *mut KSPIN_COMMUNICATION, interface: *mut KSIDENTIFIER, medium: *mut KSIDENTIFIER) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsGetCurrentCommunication(this, core::mem::transmute_copy(&communication), core::mem::transmute_copy(&interface), core::mem::transmute_copy(&medium)).into()
        }
        unsafe extern "system" fn KsPropagateAcquire<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsPropagateAcquire(this).into()
        }
        unsafe extern "system" fn KsDeliver<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sample: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsDeliver(this, windows_core::from_raw_borrowed(&sample), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn KsMediaSamplesCompleted<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamsegment: *const KSSTREAM_SEGMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsMediaSamplesCompleted(this, core::mem::transmute_copy(&streamsegment)).into()
        }
        unsafe extern "system" fn KsPeekAllocator<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: KSPEEKOPERATION) -> Option<super::DirectShow::IMemAllocator> {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsPeekAllocator(this, core::mem::transmute_copy(&operation))
        }
        unsafe extern "system" fn KsReceiveAllocator<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsReceiveAllocator(this, windows_core::from_raw_borrowed(&memallocator)).into()
        }
        unsafe extern "system" fn KsRenegotiateAllocator<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsRenegotiateAllocator(this).into()
        }
        unsafe extern "system" fn KsIncrementPendingIoCount<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsIncrementPendingIoCount(this)
        }
        unsafe extern "system" fn KsDecrementPendingIoCount<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsDecrementPendingIoCount(this)
        }
        unsafe extern "system" fn KsQualityNotify<Identity: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proportion: u32, timedelta: i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPin_Impl::KsQualityNotify(this, core::mem::transmute_copy(&proportion), core::mem::transmute_copy(&timedelta)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsQueryMediums: KsQueryMediums::<Identity, OFFSET>,
            KsQueryInterfaces: KsQueryInterfaces::<Identity, OFFSET>,
            KsCreateSinkPinHandle: KsCreateSinkPinHandle::<Identity, OFFSET>,
            KsGetCurrentCommunication: KsGetCurrentCommunication::<Identity, OFFSET>,
            KsPropagateAcquire: KsPropagateAcquire::<Identity, OFFSET>,
            KsDeliver: KsDeliver::<Identity, OFFSET>,
            KsMediaSamplesCompleted: KsMediaSamplesCompleted::<Identity, OFFSET>,
            KsPeekAllocator: KsPeekAllocator::<Identity, OFFSET>,
            KsReceiveAllocator: KsReceiveAllocator::<Identity, OFFSET>,
            KsRenegotiateAllocator: KsRenegotiateAllocator::<Identity, OFFSET>,
            KsIncrementPendingIoCount: KsIncrementPendingIoCount::<Identity, OFFSET>,
            KsDecrementPendingIoCount: KsDecrementPendingIoCount::<Identity, OFFSET>,
            KsQualityNotify: KsQualityNotify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl windows_core::RuntimeName for IKsPin {}
windows_core::imp::define_interface!(IKsPinEx, IKsPinEx_Vtbl, 0x7bb38260_d19c_11d2_b38a_00a0c95ec22e);
impl core::ops::Deref for IKsPinEx {
    type Target = IKsPin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IKsPinEx, windows_core::IUnknown, IKsPin);
impl IKsPinEx {
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn KsNotifyError<P0>(&self, sample: P0, hr: windows_core::HRESULT)
    where
        P0: windows_core::Param<super::DirectShow::IMediaSample>,
    {
        (windows_core::Interface::vtable(self).KsNotifyError)(windows_core::Interface::as_raw(self), sample.param().abi(), hr)
    }
}
#[repr(C)]
pub struct IKsPinEx_Vtbl {
    pub base__: IKsPin_Vtbl,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub KsNotifyError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT),
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    KsNotifyError: usize,
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IKsPinEx_Impl: IKsPin_Impl {
    fn KsNotifyError(&self, sample: Option<&super::DirectShow::IMediaSample>, hr: windows_core::HRESULT);
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IKsPinEx_Vtbl {
    pub const fn new<Identity: IKsPinEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsNotifyError<Identity: IKsPinEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sample: *mut core::ffi::c_void, hr: windows_core::HRESULT) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinEx_Impl::KsNotifyError(this, windows_core::from_raw_borrowed(&sample), core::mem::transmute_copy(&hr))
        }
        Self { base__: IKsPin_Vtbl::new::<Identity, OFFSET>(), KsNotifyError: KsNotifyError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPinEx as windows_core::Interface>::IID || iid == &<IKsPin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl windows_core::RuntimeName for IKsPinEx {}
windows_core::imp::define_interface!(IKsPinFactory, IKsPinFactory_Vtbl, 0xcd5ebe6b_8b6e_11d1_8ae0_00a0c9223196);
windows_core::imp::interface_hierarchy!(IKsPinFactory, windows_core::IUnknown);
impl IKsPinFactory {
    pub unsafe fn KsPinFactory(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsPinFactory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IKsPinFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsPinFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IKsPinFactory_Impl: windows_core::IUnknownImpl {
    fn KsPinFactory(&self) -> windows_core::Result<u32>;
}
impl IKsPinFactory_Vtbl {
    pub const fn new<Identity: IKsPinFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsPinFactory<Identity: IKsPinFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfactory: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsPinFactory_Impl::KsPinFactory(this) {
                Ok(ok__) => {
                    pinfactory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), KsPinFactory: KsPinFactory::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPinFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsPinFactory {}
windows_core::imp::define_interface!(IKsPinPipe, IKsPinPipe_Vtbl, 0xe539cd90_a8b4_11d1_8189_00a0c9062802);
windows_core::imp::interface_hierarchy!(IKsPinPipe, windows_core::IUnknown);
impl IKsPinPipe {
    pub unsafe fn KsGetPinFramingCache(&self, framingex: *mut *mut KSALLOCATOR_FRAMING_EX, framingprop: *mut FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsGetPinFramingCache)(windows_core::Interface::as_raw(self), core::mem::transmute(framingex), core::mem::transmute(framingprop), option).ok()
    }
    pub unsafe fn KsSetPinFramingCache(&self, framingex: *const KSALLOCATOR_FRAMING_EX, framingprop: *const FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsSetPinFramingCache)(windows_core::Interface::as_raw(self), framingex, framingprop, option).ok()
    }
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn KsGetConnectedPin(&self) -> Option<super::DirectShow::IPin> {
        (windows_core::Interface::vtable(self).KsGetConnectedPin)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn KsGetPipe(&self, operation: KSPEEKOPERATION) -> Option<IKsAllocatorEx> {
        (windows_core::Interface::vtable(self).KsGetPipe)(windows_core::Interface::as_raw(self), operation)
    }
    pub unsafe fn KsSetPipe<P0>(&self, ksallocator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IKsAllocatorEx>,
    {
        (windows_core::Interface::vtable(self).KsSetPipe)(windows_core::Interface::as_raw(self), ksallocator.param().abi()).ok()
    }
    pub unsafe fn KsGetPipeAllocatorFlag(&self) -> u32 {
        (windows_core::Interface::vtable(self).KsGetPipeAllocatorFlag)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn KsSetPipeAllocatorFlag(&self, flag: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsSetPipeAllocatorFlag)(windows_core::Interface::as_raw(self), flag).ok()
    }
    pub unsafe fn KsGetPinBusCache(&self) -> windows_core::GUID {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KsGetPinBusCache)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn KsSetPinBusCache(&self, bus: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KsSetPinBusCache)(windows_core::Interface::as_raw(self), core::mem::transmute(bus)).ok()
    }
    pub unsafe fn KsGetPinName(&self) -> windows_core::PWSTR {
        (windows_core::Interface::vtable(self).KsGetPinName)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn KsGetFilterName(&self) -> windows_core::PWSTR {
        (windows_core::Interface::vtable(self).KsGetFilterName)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IKsPinPipe_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KsGetPinFramingCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut KSALLOCATOR_FRAMING_EX, *mut FRAMING_PROP, FRAMING_CACHE_OPS) -> windows_core::HRESULT,
    pub KsSetPinFramingCache: unsafe extern "system" fn(*mut core::ffi::c_void, *const KSALLOCATOR_FRAMING_EX, *const FRAMING_PROP, FRAMING_CACHE_OPS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub KsGetConnectedPin: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<super::DirectShow::IPin>,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    KsGetConnectedPin: usize,
    pub KsGetPipe: unsafe extern "system" fn(*mut core::ffi::c_void, KSPEEKOPERATION) -> Option<IKsAllocatorEx>,
    pub KsSetPipe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KsGetPipeAllocatorFlag: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub KsSetPipeAllocatorFlag: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub KsGetPinBusCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID),
    pub KsSetPinBusCache: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub KsGetPinName: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::PWSTR,
    pub KsGetFilterName: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::PWSTR,
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IKsPinPipe_Impl: windows_core::IUnknownImpl {
    fn KsGetPinFramingCache(&self, framingex: *mut *mut KSALLOCATOR_FRAMING_EX, framingprop: *mut FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::Result<()>;
    fn KsSetPinFramingCache(&self, framingex: *const KSALLOCATOR_FRAMING_EX, framingprop: *const FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::Result<()>;
    fn KsGetConnectedPin(&self) -> Option<super::DirectShow::IPin>;
    fn KsGetPipe(&self, operation: KSPEEKOPERATION) -> Option<IKsAllocatorEx>;
    fn KsSetPipe(&self, ksallocator: Option<&IKsAllocatorEx>) -> windows_core::Result<()>;
    fn KsGetPipeAllocatorFlag(&self) -> u32;
    fn KsSetPipeAllocatorFlag(&self, flag: u32) -> windows_core::Result<()>;
    fn KsGetPinBusCache(&self) -> windows_core::GUID;
    fn KsSetPinBusCache(&self, bus: &windows_core::GUID) -> windows_core::Result<()>;
    fn KsGetPinName(&self) -> windows_core::PWSTR;
    fn KsGetFilterName(&self) -> windows_core::PWSTR;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IKsPinPipe_Vtbl {
    pub const fn new<Identity: IKsPinPipe_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsGetPinFramingCache<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framingex: *mut *mut KSALLOCATOR_FRAMING_EX, framingprop: *mut FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsGetPinFramingCache(this, core::mem::transmute_copy(&framingex), core::mem::transmute_copy(&framingprop), core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn KsSetPinFramingCache<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framingex: *const KSALLOCATOR_FRAMING_EX, framingprop: *const FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsSetPinFramingCache(this, core::mem::transmute_copy(&framingex), core::mem::transmute_copy(&framingprop), core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn KsGetConnectedPin<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<super::DirectShow::IPin> {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsGetConnectedPin(this)
        }
        unsafe extern "system" fn KsGetPipe<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: KSPEEKOPERATION) -> Option<IKsAllocatorEx> {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsGetPipe(this, core::mem::transmute_copy(&operation))
        }
        unsafe extern "system" fn KsSetPipe<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ksallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsSetPipe(this, windows_core::from_raw_borrowed(&ksallocator)).into()
        }
        unsafe extern "system" fn KsGetPipeAllocatorFlag<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsGetPipeAllocatorFlag(this)
        }
        unsafe extern "system" fn KsSetPipeAllocatorFlag<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flag: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsSetPipeAllocatorFlag(this, core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn KsGetPinBusCache<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = IKsPinPipe_Impl::KsGetPinBusCache(this)
        }
        unsafe extern "system" fn KsSetPinBusCache<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bus: windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsSetPinBusCache(this, core::mem::transmute(&bus)).into()
        }
        unsafe extern "system" fn KsGetPinName<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PWSTR {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsGetPinName(this)
        }
        unsafe extern "system" fn KsGetFilterName<Identity: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PWSTR {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPinPipe_Impl::KsGetFilterName(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsGetPinFramingCache: KsGetPinFramingCache::<Identity, OFFSET>,
            KsSetPinFramingCache: KsSetPinFramingCache::<Identity, OFFSET>,
            KsGetConnectedPin: KsGetConnectedPin::<Identity, OFFSET>,
            KsGetPipe: KsGetPipe::<Identity, OFFSET>,
            KsSetPipe: KsSetPipe::<Identity, OFFSET>,
            KsGetPipeAllocatorFlag: KsGetPipeAllocatorFlag::<Identity, OFFSET>,
            KsSetPipeAllocatorFlag: KsSetPipeAllocatorFlag::<Identity, OFFSET>,
            KsGetPinBusCache: KsGetPinBusCache::<Identity, OFFSET>,
            KsSetPinBusCache: KsSetPinBusCache::<Identity, OFFSET>,
            KsGetPinName: KsGetPinName::<Identity, OFFSET>,
            KsGetFilterName: KsGetFilterName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPinPipe as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl windows_core::RuntimeName for IKsPinPipe {}
windows_core::imp::define_interface!(IKsPropertySet, IKsPropertySet_Vtbl, 0x31efac30_515c_11d0_a9aa_00aa0061be93);
windows_core::imp::interface_hierarchy!(IKsPropertySet, windows_core::IUnknown);
impl IKsPropertySet {
    pub unsafe fn Set(&self, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *const core::ffi::c_void, cbpropdata: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), guidpropset, dwpropid, pinstancedata, cbinstancedata, ppropdata, cbpropdata).ok()
    }
    pub unsafe fn Get(&self, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *mut core::ffi::c_void, cbpropdata: u32, pcbreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), guidpropset, dwpropid, pinstancedata, cbinstancedata, core::mem::transmute(ppropdata), cbpropdata, core::mem::transmute(pcbreturned)).ok()
    }
    pub unsafe fn QuerySupported(&self, guidpropset: *const windows_core::GUID, dwpropid: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuerySupported)(windows_core::Interface::as_raw(self), guidpropset, dwpropid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IKsPropertySet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub QuerySupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IKsPropertySet_Impl: windows_core::IUnknownImpl {
    fn Set(&self, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *const core::ffi::c_void, cbpropdata: u32) -> windows_core::Result<()>;
    fn Get(&self, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *mut core::ffi::c_void, cbpropdata: u32, pcbreturned: *mut u32) -> windows_core::Result<()>;
    fn QuerySupported(&self, guidpropset: *const windows_core::GUID, dwpropid: u32) -> windows_core::Result<u32>;
}
impl IKsPropertySet_Vtbl {
    pub const fn new<Identity: IKsPropertySet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *const core::ffi::c_void, cbpropdata: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPropertySet_Impl::Set(this, core::mem::transmute_copy(&guidpropset), core::mem::transmute_copy(&dwpropid), core::mem::transmute_copy(&pinstancedata), core::mem::transmute_copy(&cbinstancedata), core::mem::transmute_copy(&ppropdata), core::mem::transmute_copy(&cbpropdata)).into()
        }
        unsafe extern "system" fn Get<Identity: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *mut core::ffi::c_void, cbpropdata: u32, pcbreturned: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsPropertySet_Impl::Get(this, core::mem::transmute_copy(&guidpropset), core::mem::transmute_copy(&dwpropid), core::mem::transmute_copy(&pinstancedata), core::mem::transmute_copy(&cbinstancedata), core::mem::transmute_copy(&ppropdata), core::mem::transmute_copy(&cbpropdata), core::mem::transmute_copy(&pcbreturned)).into()
        }
        unsafe extern "system" fn QuerySupported<Identity: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpropset: *const windows_core::GUID, dwpropid: u32, ptypesupport: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsPropertySet_Impl::QuerySupported(this, core::mem::transmute_copy(&guidpropset), core::mem::transmute_copy(&dwpropid)) {
                Ok(ok__) => {
                    ptypesupport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            QuerySupported: QuerySupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPropertySet as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsPropertySet {}
windows_core::imp::define_interface!(IKsQualityForwarder, IKsQualityForwarder_Vtbl, 0x97ebaacb_95bd_11d0_a3ea_00a0c9223196);
impl core::ops::Deref for IKsQualityForwarder {
    type Target = IKsObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IKsQualityForwarder, windows_core::IUnknown, IKsObject);
impl IKsQualityForwarder {
    pub unsafe fn KsFlushClient<P0>(&self, pin: P0)
    where
        P0: windows_core::Param<IKsPin>,
    {
        (windows_core::Interface::vtable(self).KsFlushClient)(windows_core::Interface::as_raw(self), pin.param().abi())
    }
}
#[repr(C)]
pub struct IKsQualityForwarder_Vtbl {
    pub base__: IKsObject_Vtbl,
    pub KsFlushClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
pub trait IKsQualityForwarder_Impl: IKsObject_Impl {
    fn KsFlushClient(&self, pin: Option<&IKsPin>);
}
impl IKsQualityForwarder_Vtbl {
    pub const fn new<Identity: IKsQualityForwarder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsFlushClient<Identity: IKsQualityForwarder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsQualityForwarder_Impl::KsFlushClient(this, windows_core::from_raw_borrowed(&pin))
        }
        Self { base__: IKsObject_Vtbl::new::<Identity, OFFSET>(), KsFlushClient: KsFlushClient::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsQualityForwarder as windows_core::Interface>::IID || iid == &<IKsObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsQualityForwarder {}
windows_core::imp::define_interface!(IKsTopology, IKsTopology_Vtbl, 0x28f54683_06fd_11d2_b27a_00a0c9223196);
windows_core::imp::interface_hierarchy!(IKsTopology, windows_core::IUnknown);
impl IKsTopology {
    pub unsafe fn CreateNodeInstance<P3>(&self, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: P3, interfaceid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateNodeInstance)(windows_core::Interface::as_raw(self), nodeid, flags, desiredaccess, unkouter.param().abi(), interfaceid, core::mem::transmute(interface)).ok()
    }
}
#[repr(C)]
pub struct IKsTopology_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateNodeInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IKsTopology_Impl: windows_core::IUnknownImpl {
    fn CreateNodeInstance(&self, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: Option<&windows_core::IUnknown>, interfaceid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IKsTopology_Vtbl {
    pub const fn new<Identity: IKsTopology_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateNodeInstance<Identity: IKsTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: *mut core::ffi::c_void, interfaceid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsTopology_Impl::CreateNodeInstance(this, core::mem::transmute_copy(&nodeid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&desiredaccess), windows_core::from_raw_borrowed(&unkouter), core::mem::transmute_copy(&interfaceid), core::mem::transmute_copy(&interface)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateNodeInstance: CreateNodeInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsTopology as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsTopology {}
windows_core::imp::define_interface!(IKsTopologyInfo, IKsTopologyInfo_Vtbl, 0x720d4ac0_7533_11d0_a5d6_28db04c10000);
windows_core::imp::interface_hierarchy!(IKsTopologyInfo, windows_core::IUnknown);
impl IKsTopologyInfo {
    pub unsafe fn NumCategories(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumCategories)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_Category(&self, dwindex: u32) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Category)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
    }
    pub unsafe fn NumConnections(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumConnections)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_ConnectionInfo(&self, dwindex: u32) -> windows_core::Result<KSTOPOLOGY_CONNECTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ConnectionInfo)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
    }
    pub unsafe fn get_NodeName(&self, dwnodeid: u32, pwchnodename: Option<windows_core::PWSTR>, dwbufsize: u32, pdwnamelen: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).get_NodeName)(windows_core::Interface::as_raw(self), dwnodeid, core::mem::transmute(pwchnodename.unwrap_or(core::mem::zeroed())), dwbufsize, core::mem::transmute(pdwnamelen)).ok()
    }
    pub unsafe fn NumNodes(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumNodes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_NodeType(&self, dwnodeid: u32) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_NodeType)(windows_core::Interface::as_raw(self), dwnodeid, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateNodeInstance(&self, dwnodeid: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateNodeInstance)(windows_core::Interface::as_raw(self), dwnodeid, iid, core::mem::transmute(ppvobject)).ok()
    }
}
#[repr(C)]
pub struct IKsTopologyInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NumCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub get_Category: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub NumConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub get_ConnectionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut KSTOPOLOGY_CONNECTION) -> windows_core::HRESULT,
    pub get_NodeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub NumNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub get_NodeType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CreateNodeInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IKsTopologyInfo_Impl: windows_core::IUnknownImpl {
    fn NumCategories(&self) -> windows_core::Result<u32>;
    fn get_Category(&self, dwindex: u32) -> windows_core::Result<windows_core::GUID>;
    fn NumConnections(&self) -> windows_core::Result<u32>;
    fn get_ConnectionInfo(&self, dwindex: u32) -> windows_core::Result<KSTOPOLOGY_CONNECTION>;
    fn get_NodeName(&self, dwnodeid: u32, pwchnodename: windows_core::PWSTR, dwbufsize: u32, pdwnamelen: *mut u32) -> windows_core::Result<()>;
    fn NumNodes(&self) -> windows_core::Result<u32>;
    fn get_NodeType(&self, dwnodeid: u32) -> windows_core::Result<windows_core::GUID>;
    fn CreateNodeInstance(&self, dwnodeid: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IKsTopologyInfo_Vtbl {
    pub const fn new<Identity: IKsTopologyInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NumCategories<Identity: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwnumcategories: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsTopologyInfo_Impl::NumCategories(this) {
                Ok(ok__) => {
                    pdwnumcategories.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Category<Identity: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pcategory: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsTopologyInfo_Impl::get_Category(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    pcategory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumConnections<Identity: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwnumconnections: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsTopologyInfo_Impl::NumConnections(this) {
                Ok(ok__) => {
                    pdwnumconnections.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ConnectionInfo<Identity: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pconnectioninfo: *mut KSTOPOLOGY_CONNECTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsTopologyInfo_Impl::get_ConnectionInfo(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    pconnectioninfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_NodeName<Identity: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnodeid: u32, pwchnodename: windows_core::PWSTR, dwbufsize: u32, pdwnamelen: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsTopologyInfo_Impl::get_NodeName(this, core::mem::transmute_copy(&dwnodeid), core::mem::transmute_copy(&pwchnodename), core::mem::transmute_copy(&dwbufsize), core::mem::transmute_copy(&pdwnamelen)).into()
        }
        unsafe extern "system" fn NumNodes<Identity: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwnumnodes: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsTopologyInfo_Impl::NumNodes(this) {
                Ok(ok__) => {
                    pdwnumnodes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_NodeType<Identity: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnodeid: u32, pnodetype: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKsTopologyInfo_Impl::get_NodeType(this, core::mem::transmute_copy(&dwnodeid)) {
                Ok(ok__) => {
                    pnodetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNodeInstance<Identity: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnodeid: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKsTopologyInfo_Impl::CreateNodeInstance(this, core::mem::transmute_copy(&dwnodeid), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NumCategories: NumCategories::<Identity, OFFSET>,
            get_Category: get_Category::<Identity, OFFSET>,
            NumConnections: NumConnections::<Identity, OFFSET>,
            get_ConnectionInfo: get_ConnectionInfo::<Identity, OFFSET>,
            get_NodeName: get_NodeName::<Identity, OFFSET>,
            NumNodes: NumNodes::<Identity, OFFSET>,
            get_NodeType: get_NodeType::<Identity, OFFSET>,
            CreateNodeInstance: CreateNodeInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsTopologyInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsTopologyInfo {}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    pub Size: u32,
    pub PrimaryChannelCount: u32,
    pub PrimaryChannelStartPosition: u32,
    pub PrimaryChannelMask: u32,
    pub InterleavedChannelCount: u32,
    pub InterleavedChannelStartPosition: u32,
    pub InterleavedChannelMask: u32,
}
impl Default for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IOCTL_KS_DISABLE_EVENT: u32 = 3080203u32;
pub const IOCTL_KS_ENABLE_EVENT: u32 = 3080199u32;
pub const IOCTL_KS_HANDSHAKE: u32 = 3080223u32;
pub const IOCTL_KS_METHOD: u32 = 3080207u32;
pub const IOCTL_KS_PROPERTY: u32 = 3080195u32;
pub const IOCTL_KS_READ_STREAM: u32 = 3096599u32;
pub const IOCTL_KS_RESET_STATE: u32 = 3080219u32;
pub const IOCTL_KS_WRITE_STREAM: u32 = 3112979u32;
pub const JACKDESC2_DYNAMIC_FORMAT_CHANGE_CAPABILITY: u32 = 2u32;
pub const JACKDESC2_PRESENCE_DETECT_CAPABILITY: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAC3_ALTERNATE_AUDIO {
    pub fStereo: super::super::Foundation::BOOL,
    pub DualMode: u32,
}
impl Default for KSAC3_ALTERNATE_AUDIO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSAC3_ALTERNATE_AUDIO_1: u32 = 1u32;
pub const KSAC3_ALTERNATE_AUDIO_2: u32 = 2u32;
pub const KSAC3_ALTERNATE_AUDIO_BOTH: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAC3_BIT_STREAM_MODE {
    pub BitStreamMode: i32,
}
impl Default for KSAC3_BIT_STREAM_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAC3_DIALOGUE_LEVEL {
    pub DialogueLevel: u32,
}
impl Default for KSAC3_DIALOGUE_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAC3_DOWNMIX {
    pub fDownMix: super::super::Foundation::BOOL,
    pub fDolbySurround: super::super::Foundation::BOOL,
}
impl Default for KSAC3_DOWNMIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAC3_ERROR_CONCEALMENT {
    pub fRepeatPreviousBlock: super::super::Foundation::BOOL,
    pub fErrorInCurrentBlock: super::super::Foundation::BOOL,
}
impl Default for KSAC3_ERROR_CONCEALMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAC3_ROOM_TYPE {
    pub fLargeRoom: super::super::Foundation::BOOL,
}
impl Default for KSAC3_ROOM_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSAC3_SERVICE_COMMENTARY: u32 = 5u32;
pub const KSAC3_SERVICE_DIALOG_ONLY: u32 = 4u32;
pub const KSAC3_SERVICE_EMERGENCY_FLASH: u32 = 6u32;
pub const KSAC3_SERVICE_HEARING_IMPAIRED: u32 = 3u32;
pub const KSAC3_SERVICE_MAIN_AUDIO: u32 = 0u32;
pub const KSAC3_SERVICE_NO_DIALOG: u32 = 1u32;
pub const KSAC3_SERVICE_VISUALLY_IMPAIRED: u32 = 2u32;
pub const KSAC3_SERVICE_VOICE_OVER: u32 = 7u32;
pub const KSALGORITHMINSTANCE_SYSTEM_ACOUSTIC_ECHO_CANCEL: windows_core::GUID = windows_core::GUID::from_u128(0x1c22c56d_9879_4f5b_a389_27996ddc2810);
pub const KSALGORITHMINSTANCE_SYSTEM_AGC: windows_core::GUID = windows_core::GUID::from_u128(0x950e55b9_877c_4c67_be08_e47b5611130a);
pub const KSALGORITHMINSTANCE_SYSTEM_MICROPHONE_ARRAY_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0xb6f5a0a0_9e61_4f8c_91e3_76cf0f3c471f);
pub const KSALGORITHMINSTANCE_SYSTEM_NOISE_SUPPRESS: windows_core::GUID = windows_core::GUID::from_u128(0x5ab0882e_7274_4516_877d_4eee99ba4fd0);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSALLOCATORMODE(pub i32);
pub const KSALLOCATOR_FLAG_2D_BUFFER_REQUIRED: u32 = 32768u32;
pub const KSALLOCATOR_FLAG_ALLOCATOR_EXISTS: u32 = 2048u32;
pub const KSALLOCATOR_FLAG_ATTENTION_STEPPING: u32 = 8192u32;
pub const KSALLOCATOR_FLAG_CAN_ALLOCATE: u32 = 64u32;
pub const KSALLOCATOR_FLAG_CYCLE: u32 = 1024u32;
pub const KSALLOCATOR_FLAG_DEVICE_SPECIFIC: u32 = 32u32;
pub const KSALLOCATOR_FLAG_ENABLE_CACHED_MDL: u32 = 16384u32;
pub const KSALLOCATOR_FLAG_INDEPENDENT_RANGES: u32 = 4096u32;
pub const KSALLOCATOR_FLAG_INSIST_ON_FRAMESIZE_RATIO: u32 = 128u32;
pub const KSALLOCATOR_FLAG_MULTIPLE_OUTPUT: u32 = 512u32;
pub const KSALLOCATOR_FLAG_NO_FRAME_INTEGRITY: u32 = 256u32;
pub const KSALLOCATOR_FLAG_PARTIAL_READ_SUPPORT: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSALLOCATOR_FRAMING {
    pub Anonymous1: KSALLOCATOR_FRAMING_0,
    pub PoolType: u32,
    pub Frames: u32,
    pub FrameSize: u32,
    pub Anonymous2: KSALLOCATOR_FRAMING_1,
    pub Reserved: u32,
}
impl Default for KSALLOCATOR_FRAMING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSALLOCATOR_FRAMING_0 {
    pub OptionsFlags: u32,
    pub RequirementsFlags: u32,
}
impl Default for KSALLOCATOR_FRAMING_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSALLOCATOR_FRAMING_1 {
    pub FileAlignment: u32,
    pub FramePitch: i32,
}
impl Default for KSALLOCATOR_FRAMING_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSALLOCATOR_FRAMING_EX {
    pub CountItems: u32,
    pub PinFlags: u32,
    pub OutputCompression: KS_COMPRESSION,
    pub PinWeight: u32,
    pub FramingItem: [KS_FRAMING_ITEM; 1],
}
impl Default for KSALLOCATOR_FRAMING_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSALLOCATOR_OPTIONF_COMPATIBLE: u32 = 1u32;
pub const KSALLOCATOR_OPTIONF_SYSTEM_MEMORY: u32 = 2u32;
pub const KSALLOCATOR_OPTIONF_VALID: u32 = 3u32;
pub const KSALLOCATOR_REQUIREMENTF_FRAME_INTEGRITY: u32 = 4u32;
pub const KSALLOCATOR_REQUIREMENTF_INPLACE_MODIFIER: u32 = 1u32;
pub const KSALLOCATOR_REQUIREMENTF_MUST_ALLOCATE: u32 = 8u32;
pub const KSALLOCATOR_REQUIREMENTF_PREFERENCES_ONLY: u32 = 2147483648u32;
pub const KSALLOCATOR_REQUIREMENTF_SYSTEM_MEMORY: u32 = 2u32;
pub const KSALLOCATOR_REQUIREMENTF_SYSTEM_MEMORY_CUSTOM_ALLOCATION: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSATTRIBUTE {
    pub Size: u32,
    pub Flags: u32,
    pub Attribute: windows_core::GUID,
}
impl Default for KSATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSATTRIBUTEID_AUDIOSIGNALPROCESSING_MODE: windows_core::GUID = windows_core::GUID::from_u128(0xe1f89eb5_5f46_419b_967b_ff6770b98401);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    pub AttributeHeader: KSATTRIBUTE,
    pub SignalProcessingMode: windows_core::GUID,
}
impl Default for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSATTRIBUTE_REQUIRED: u32 = 1u32;
pub const KSAUDDECOUTMODE_PCM_51: u32 = 2u32;
pub const KSAUDDECOUTMODE_SPDIFF: u32 = 4u32;
pub const KSAUDDECOUTMODE_STEREO_ANALOG: u32 = 1u32;
pub const KSAUDFNAME_3D_CENTER: windows_core::GUID = windows_core::GUID::from_u128(0x9f0670b4_991f_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_3D_DEPTH: windows_core::GUID = windows_core::GUID::from_u128(0x63ff5747_991f_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_3D_STEREO: windows_core::GUID = windows_core::GUID::from_u128(0x185fede2_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_ALTERNATE_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0x2bc31d6b_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_AUX: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfe_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_AUX_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfd_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_AUX_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfc_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_BASS: windows_core::GUID = windows_core::GUID::from_u128(0x185fede0_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfb_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf3_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedea_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_CD_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fede9_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_IN: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf9_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf4_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedec_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_LINE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedeb_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MASTER_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fede4_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MASTER_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fede3_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MICROPHONE_BOOST: windows_core::GUID = windows_core::GUID::from_u128(0x2bc31d6a_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MIC_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf5_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIC_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedee_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIC_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185feded_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf8_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf2_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fede8_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDI_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fede7_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_MIDRANGE: windows_core::GUID = windows_core::GUID::from_u128(0xa2cbe478_ae84_49a1_8b72_4ad09b78ed34);
pub const KSAUDFNAME_MONO_MIX: windows_core::GUID = windows_core::GUID::from_u128(0x00dff078_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_MIX_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x2bc31d69_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_MIX_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x22b0eafe_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_OUT: windows_core::GUID = windows_core::GUID::from_u128(0xf9b41dc3_96e2_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_OUT_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x1ad247ec_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_MONO_OUT_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x1ad247eb_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_PC_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0x185fedff_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_PC_SPEAKER_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf1_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_PC_SPEAKER_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf0_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_PEAKMETER: windows_core::GUID = windows_core::GUID::from_u128(0x57e24340_fc5b_4612_a562_72b11a29dfae);
pub const KSAUDFNAME_RECORDING_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x185fedfa_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_RECORDING_SOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x185fedef_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_STEREO_MIX: windows_core::GUID = windows_core::GUID::from_u128(0x00dff077_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_STEREO_MIX_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x22b0eafd_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_STEREO_MIX_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x1ad247ed_96e3_11d2_ac4c_00c04f8efb68);
pub const KSAUDFNAME_TREBLE: windows_core::GUID = windows_core::GUID::from_u128(0x185fede1_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x915daec4_a434_11d2_ac52_00c04f8efb68);
pub const KSAUDFNAME_VIDEO_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x9b46e709_992a_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_VIDEO_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x9b46e708_992a_11d2_ac4d_00c04f8efb68);
pub const KSAUDFNAME_VOLUME_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf7_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_IN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fedf6_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x185fede6_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_OUT_MIX: windows_core::GUID = windows_core::GUID::from_u128(0x185fee00_9905_11d1_95a9_00c04fb925d3);
pub const KSAUDFNAME_WAVE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x185fede5_9905_11d1_95a9_00c04fb925d3);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    pub MinBufferBytes: u32,
    pub MaxBufferBytes: u32,
}
impl Default for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIOENGINE_DESCRIPTOR {
    pub nHostPinId: u32,
    pub nOffloadPinId: u32,
    pub nLoopbackPinId: u32,
}
impl Default for KSAUDIOENGINE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIOENGINE_DEVICECONTROLS {
    pub Volume: EDeviceControlUseType,
    pub Mute: EDeviceControlUseType,
    pub PeakMeter: EDeviceControlUseType,
}
impl Default for KSAUDIOENGINE_DEVICECONTROLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIOENGINE_VOLUMELEVEL {
    pub TargetVolume: i32,
    pub CurveType: AUDIO_CURVE_TYPE,
    pub CurveDuration: u64,
}
impl Default for KSAUDIOENGINE_VOLUMELEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIOMODULE_DESCRIPTOR {
    pub ClassId: windows_core::GUID,
    pub InstanceId: u32,
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub Name: [u16; 128],
}
impl Default for KSAUDIOMODULE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIOMODULE_NOTIFICATION {
    pub Anonymous: KSAUDIOMODULE_NOTIFICATION_0,
}
impl Default for KSAUDIOMODULE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSAUDIOMODULE_NOTIFICATION_0 {
    pub ProviderId: KSAUDIOMODULE_NOTIFICATION_0_0,
    pub Alignment: i64,
}
impl Default for KSAUDIOMODULE_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIOMODULE_NOTIFICATION_0_0 {
    pub DeviceId: windows_core::GUID,
    pub ClassId: windows_core::GUID,
    pub InstanceId: u32,
    pub Reserved: u32,
}
impl Default for KSAUDIOMODULE_NOTIFICATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIOMODULE_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub ClassId: windows_core::GUID,
    pub InstanceId: u32,
}
impl Default for KSAUDIOMODULE_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_CHANNEL_CONFIG {
    pub ActiveSpeakerPositions: i32,
}
impl Default for KSAUDIO_CHANNEL_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_COPY_PROTECTION {
    pub fCopyrighted: super::super::Foundation::BOOL,
    pub fOriginal: super::super::Foundation::BOOL,
}
impl Default for KSAUDIO_COPY_PROTECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSAUDIO_CPU_RESOURCES_HOST_CPU: u32 = 2147483647u32;
pub const KSAUDIO_CPU_RESOURCES_NOT_HOST_CPU: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_DYNAMIC_RANGE {
    pub QuietCompression: u32,
    pub LoudCompression: u32,
}
impl Default for KSAUDIO_DYNAMIC_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_MICROPHONE_COORDINATES {
    pub usType: u16,
    pub wXCoord: i16,
    pub wYCoord: i16,
    pub wZCoord: i16,
    pub wVerticalAngle: i16,
    pub wHorizontalAngle: i16,
}
impl Default for KSAUDIO_MICROPHONE_COORDINATES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_MIC_ARRAY_GEOMETRY {
    pub usVersion: u16,
    pub usMicArrayType: u16,
    pub wVerticalAngleBegin: i16,
    pub wVerticalAngleEnd: i16,
    pub wHorizontalAngleBegin: i16,
    pub wHorizontalAngleEnd: i16,
    pub usFrequencyBandLo: u16,
    pub usFrequencyBandHi: u16,
    pub usNumberOfMicrophones: u16,
    pub KsMicCoord: [KSAUDIO_MICROPHONE_COORDINATES; 1],
}
impl Default for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIO_MIXCAP_TABLE {
    pub InputChannels: u32,
    pub OutputChannels: u32,
    pub Capabilities: [KSAUDIO_MIX_CAPS; 1],
}
impl Default for KSAUDIO_MIXCAP_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_MIXLEVEL {
    pub Mute: super::super::Foundation::BOOL,
    pub Level: i32,
}
impl Default for KSAUDIO_MIXLEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSAUDIO_MIX_CAPS {
    pub Mute: super::super::Foundation::BOOL,
    pub Minimum: i32,
    pub Maximum: i32,
    pub Anonymous: KSAUDIO_MIX_CAPS_0,
}
impl Default for KSAUDIO_MIX_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSAUDIO_MIX_CAPS_0 {
    pub Reset: i32,
    pub Resolution: i32,
}
impl Default for KSAUDIO_MIX_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS {
    pub MinPacketPeriodInHns: u32,
    pub PacketSizeFileAlignment: u32,
    pub Reserved: u32,
    pub NumProcessingModeConstraints: u32,
    pub ProcessingModeConstraints: [KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT; 1],
}
impl Default for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    pub MinPacketPeriodInHns: u32,
    pub PacketSizeFileAlignment: u32,
    pub MaxPacketSizeInBytes: u32,
    pub NumProcessingModeConstraints: u32,
    pub ProcessingModeConstraints: [KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT; 1],
}
impl Default for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT {
    pub ProcessingMode: windows_core::GUID,
    pub SamplesPerProcessingPacket: u32,
    pub ProcessingPacketDurationInHns: u32,
}
impl Default for KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_POSITION {
    pub PlayOffset: u64,
    pub WriteOffset: u64,
}
impl Default for KSAUDIO_POSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_POSITIONEX {
    pub TimerFrequency: i64,
    pub TimeStamp1: i64,
    pub Position: KSAUDIO_POSITION,
    pub TimeStamp2: i64,
}
impl Default for KSAUDIO_POSITIONEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSAUDIO_PRESENTATION_POSITION {
    pub u64PositionInBlocks: u64,
    pub u64QPCPosition: u64,
}
impl Default for KSAUDIO_PRESENTATION_POSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSAUDIO_QUALITY_ADVANCED: u32 = 3u32;
pub const KSAUDIO_QUALITY_BASIC: u32 = 2u32;
pub const KSAUDIO_QUALITY_PC: u32 = 1u32;
pub const KSAUDIO_QUALITY_WORST: u32 = 0u32;
pub const KSAUDIO_SPEAKER_DIRECTOUT: u32 = 0u32;
pub const KSAUDIO_SPEAKER_GROUND_FRONT_CENTER: u32 = 4u32;
pub const KSAUDIO_SPEAKER_GROUND_FRONT_LEFT: u32 = 1u32;
pub const KSAUDIO_SPEAKER_GROUND_FRONT_RIGHT: u32 = 2u32;
pub const KSAUDIO_SPEAKER_GROUND_REAR_LEFT: u32 = 16u32;
pub const KSAUDIO_SPEAKER_GROUND_REAR_RIGHT: u32 = 32u32;
pub const KSAUDIO_SPEAKER_MONO: u32 = 4u32;
pub const KSAUDIO_SPEAKER_SUPER_WOOFER: u32 = 8u32;
pub const KSAUDIO_SPEAKER_TOP_MIDDLE: u32 = 2048u32;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_HEADPHONE: i32 = -1i32;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MAX: u32 = 180u32;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_MIN: u32 = 5u32;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_NARROW: u32 = 10u32;
pub const KSAUDIO_STEREO_SPEAKER_GEOMETRY_WIDE: u32 = 20u32;
pub const KSCAMERAPROFILE_BalancedVideoAndPhoto: windows_core::GUID = windows_core::GUID::from_u128(0x6b52b017_42c7_4a21_bfe3_23f009149887);
pub const KSCAMERAPROFILE_CompressedCamera: windows_core::GUID = windows_core::GUID::from_u128(0x0e34cdc1_27ad_437f_abde_02b629f37b44);
pub const KSCAMERAPROFILE_FLAGS_FACEDETECTION: u64 = 8u64;
pub const KSCAMERAPROFILE_FLAGS_PHOTOHDR: u64 = 4u64;
pub const KSCAMERAPROFILE_FLAGS_PREVIEW_RES_MUSTMATCH: u64 = 32u64;
pub const KSCAMERAPROFILE_FLAGS_VARIABLEPHOTOSEQUENCE: u64 = 16u64;
pub const KSCAMERAPROFILE_FLAGS_VIDEOHDR: u64 = 2u64;
pub const KSCAMERAPROFILE_FLAGS_VIDEOSTABLIZATION: u64 = 1u64;
pub const KSCAMERAPROFILE_FaceAuth_Mode: windows_core::GUID = windows_core::GUID::from_u128(0x81361b22_700b_4546_a2d4_c52e907bfc27);
pub const KSCAMERAPROFILE_HDRWithWCGPhoto: windows_core::GUID = windows_core::GUID::from_u128(0x9bf6f1ff_b555_4625_b326_a46def318fb7);
pub const KSCAMERAPROFILE_HDRWithWCGVideo: windows_core::GUID = windows_core::GUID::from_u128(0x4b27c336_4924_4989_b994_fdaf1dc7cd85);
pub const KSCAMERAPROFILE_HighFrameRate: windows_core::GUID = windows_core::GUID::from_u128(0x566e6113_8c35_48e7_b89f_d23fdc1219dc);
pub const KSCAMERAPROFILE_HighQualityPhoto: windows_core::GUID = windows_core::GUID::from_u128(0x32440725_961b_4ca3_b5b2_854e719d9e1b);
pub const KSCAMERAPROFILE_Legacy: windows_core::GUID = windows_core::GUID::from_u128(0xb4894d81_62b7_4eec_8740_80658c4a9d3e);
pub const KSCAMERAPROFILE_PhotoSequence: windows_core::GUID = windows_core::GUID::from_u128(0x02399d9d_4ee8_49ba_bc07_5ff156531413);
pub const KSCAMERAPROFILE_VariablePhotoSequence: windows_core::GUID = windows_core::GUID::from_u128(0x9ff2cb56_e75a_49b1_a928_9985d5946f87);
pub const KSCAMERAPROFILE_VideoConferencing: windows_core::GUID = windows_core::GUID::from_u128(0xc5444a88_e1bf_4597_b2dd_9e1ead864bb8);
pub const KSCAMERAPROFILE_VideoHDR8: windows_core::GUID = windows_core::GUID::from_u128(0xd4f3f4ec_bdff_4314_b1d4_008e281f74e7);
pub const KSCAMERAPROFILE_VideoRecording: windows_core::GUID = windows_core::GUID::from_u128(0xa0e517e8_8f8c_4f6f_9a57_46fc2f647ec0);
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_AUTO: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_FNF: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_HDR: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_ADVANCEDPHOTO_ULTRALOWLIGHT: u64 = 8u64;
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_BLUR: u64 = 1u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    pub Resolution: super::super::Foundation::SIZE,
    pub MaxFrameRate: KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0,
    pub MaskResolution: super::super::Foundation::SIZE,
    pub SubType: windows_core::GUID,
}
impl Default for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    pub Numerator: i32,
    pub Denominator: i32,
}
impl Default for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_MASK: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_SHALLOWFOCUS: u64 = 4u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    pub PitchAngle: i32,
    pub YawAngle: i32,
    pub Flag: u32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_CAPS_ASYNCCONTROL: u64 = 9223372036854775808u64;
pub const KSCAMERA_EXTENDEDPROP_CAPS_CANCELLABLE: u64 = 4611686018427387904u64;
pub const KSCAMERA_EXTENDEDPROP_CAPS_MASK: u64 = 18374686479671623680u64;
pub const KSCAMERA_EXTENDEDPROP_CAPS_RESERVED: u64 = 18374686479671623680u64;
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_AUTOFACEFRAMING: u64 = 1u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    pub ResolutionX: i32,
    pub ResolutionY: i32,
    pub PorchTop: i32,
    pub PorchLeft: i32,
    pub PorchBottom: i32,
    pub PorchRight: i32,
    pub NonUpscalingWindowSize: i32,
    pub MinWindowSize: i32,
    pub MaxWindowSize: i32,
    pub Reserved: i32,
}
impl Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    pub Size: u32,
    pub Count: u32,
}
impl Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_MANUAL: u64 = 0u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    pub OriginX: i32,
    pub OriginY: i32,
    pub WindowSize: i32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Value: i32,
    pub Reserved: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_FULLSTEP: u64 = 16u64;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_HALFSTEP: u64 = 8u64;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_QUARTERSTEP: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_SIXTHSTEP: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_EVCOMP_THIRDSTEP: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_ON: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_EYEGAZECORRECTION_STARE: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_ALTERNATIVE_FRAME_ILLUMINATION: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_BACKGROUND_SUBTRACTION: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_DISABLED: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_BLINK: u64 = 8u64;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_ON: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PHOTO: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_PREVIEW: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_SMILE: u64 = 16u64;
pub const KSCAMERA_EXTENDEDPROP_FACEDETECTION_VIDEO: u64 = 2u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    pub NormalizedFocalLengthX: u32,
    pub NormalizedFocalLengthY: u32,
    pub Flag: u32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_FILTERSCOPE: u32 = 4294967295u32;
pub const KSCAMERA_EXTENDEDPROP_FLAG_CANCELOPERATION: u64 = 9223372036854775808u64;
pub const KSCAMERA_EXTENDEDPROP_FLAG_MASK: u64 = 18374686479671623680u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_AUTO: u64 = 256u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ASSISTANT_ON: u64 = 128u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_AUTO_ADJUSTABLEPOWER: u64 = 8u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_MULTIFLASHSUPPORTED: u64 = 64u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_ON_ADJUSTABLEPOWER: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_REDEYEREDUCTION: u64 = 16u64;
pub const KSCAMERA_EXTENDEDPROP_FLASH_SINGLEFLASH: u64 = 32u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUSPRIORITY_ON: u64 = 1u64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_FOCUSSTATE(pub i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FAILED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(4i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_FOCUSED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(3i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_LOST: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(1i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_SEARCHING: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(2i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUSSTATE_UNINITIALIZED: KSCAMERA_EXTENDEDPROP_FOCUSSTATE = KSCAMERA_EXTENDEDPROP_FOCUSSTATE(0i32);
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUS: u64 = 256u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_CONTINUOUSLOCK: u64 = 512u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_HYPERFOCAL: u64 = 33554432u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_INFINITY: u64 = 16777216u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DISTANCE_NEAREST: u64 = 67108864u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_DRIVERFALLBACK_OFF: u64 = 2048u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_FULLRANGE: u64 = 262144u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_HYPERFOCAL: u64 = 1048576u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_INFINITY: u64 = 524288u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_MACRO: u64 = 65536u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_RANGE_NORMAL: u64 = 131072u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_REGIONBASED: u64 = 4096u64;
pub const KSCAMERA_EXTENDEDPROP_FOCUS_UNLOCK: u64 = 1024u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_HEADER {
    pub Version: u32,
    pub PinId: u32,
    pub Size: u32,
    pub Result: u32,
    pub Flags: u64,
    pub Capability: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_HISTOGRAM_ON: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALTERNATING_FRAME_ILLUMINATION: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_ALWAYS_ON: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_IRTORCHMODE_OFF: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_100: u64 = 8u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_12800: u64 = 1024u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_1600: u64 = 128u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_200: u64 = 16u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_25600: u64 = 2048u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_3200: u64 = 256u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_400: u64 = 32u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_50: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_6400: u64 = 512u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_80: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_800: u64 = 64u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_AUTO: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_ISO_MANUAL: u64 = 36028797018963968u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_METADATAINFO {
    pub BufferAlignment: i32,
    pub MaxMetadataBufferSize: u32,
}
impl Default for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_METADATA_ALIGNMENTREQUIRED: u64 = 256u64;
pub const KSCAMERA_EXTENDEDPROP_METADATA_MEMORYTYPE_MASK: u64 = 255u64;
pub const KSCAMERA_EXTENDEDPROP_METADATA_SYSTEMMEMORY: u64 = 1u64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_MetadataAlignment(pub i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_1024: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(10i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_128: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(7i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_16: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(4i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_2048: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(11i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_256: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(8i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_32: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(5i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_4096: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(12i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_512: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(9i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_64: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(6i32);
pub const KSCAMERA_EXTENDEDPROP_MetadataAlignment_8192: KSCAMERA_EXTENDEDPROP_MetadataAlignment = KSCAMERA_EXTENDEDPROP_MetadataAlignment(13i32);
pub const KSCAMERA_EXTENDEDPROP_OIS_AUTO: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_OIS_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_OIS_ON: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_DEFAULT: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_LATENCY: u64 = 8u64;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_PHOTO: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_POWER: u64 = 16u64;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_QUALITY: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_OPTIMIZATION_VIDEO: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_PHOTOCONFIRMATION_ON: u64 = 1u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    pub RequestedHistoryFrames: u32,
    pub MaxHistoryFrames: u32,
    pub SubMode: u32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_NORMAL: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_NONE: u32 = 0u32;
pub const KSCAMERA_EXTENDEDPROP_PHOTOMODE_SEQUENCE_SUB_VARIABLE: u32 = 1u32;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_16X: u64 = 8u64;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_2X: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_4X: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_8X: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_PHOTOTHUMBNAIL_DISABLE: u64 = 0u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_PROFILE {
    pub ProfileId: windows_core::GUID,
    pub Index: u32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_DYNAMIC: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_RELATIVEPANELOPTIMIZATION_ON: u64 = 1u64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROITYPE(pub i32);
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_FACE: KSCAMERA_EXTENDEDPROP_ROITYPE = KSCAMERA_EXTENDEDPROP_ROITYPE(1i32);
pub const KSCAMERA_EXTENDEDPROP_ROITYPE_UNKNOWN: KSCAMERA_EXTENDEDPROP_ROITYPE = KSCAMERA_EXTENDEDPROP_ROITYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    pub ControlId: u32,
    pub MaxNumberOfROIs: u32,
    pub Capability: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    pub Size: u32,
    pub ConfigCapCount: u32,
    pub Reserved: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_INFO {
    pub Region: super::super::Foundation::RECT,
    pub Flags: u64,
    pub Weight: i32,
    pub RegionOfInterestType: i32,
}
impl Default for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    pub ControlId: u32,
    pub ROICount: u32,
    pub Result: u32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    pub Size: u32,
    pub ControlCount: u32,
    pub Reserved: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    pub ROIInfo: KSCAMERA_EXTENDEDPROP_ROI_INFO,
    pub Reserved: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_AUTO: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BACKLIT: u64 = 1024u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_BEACH: u64 = 32u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_CANDLELIGHT: u64 = 128u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_LANDSCAPE: u64 = 256u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MACRO: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_MANUAL: u64 = 36028797018963968u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHT: u64 = 16u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_NIGHTPORTRAIT: u64 = 512u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_PORTRAIT: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SNOW: u64 = 8u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SPORT: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_SCENEMODE_SUNSET: u64 = 64u64;
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_DISABLED: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_SECUREMODE_ENABLED: u64 = 2u64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_EXTENDEDPROP_VALUE {
    pub Value: KSCAMERA_EXTENDEDPROP_VALUE_0,
}
impl Default for KSCAMERA_EXTENDEDPROP_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSCAMERA_EXTENDEDPROP_VALUE_0 {
    pub dbl: f64,
    pub ull: u64,
    pub ul: u32,
    pub ratio: u64,
    pub l: i32,
    pub ll: i64,
}
impl Default for KSCAMERA_EXTENDEDPROP_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_VFR_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_VFR_ON: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_AUTO: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOHDR_ON: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_AUTO: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_LOCK: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOPROCFLAG_MANUAL: u64 = 2u64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    pub Mode: u32,
    pub Min: i32,
    pub Max: i32,
    pub Step: i32,
    pub VideoProc: KSCAMERA_EXTENDEDPROP_VALUE,
    pub Reserved: u64,
}
impl Default for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_AUTO: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOSTABILIZATION_ON: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_AUTO: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_OFF: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTEMPORALDENOISING_ON: u64 = 4u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_OFF: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_VIDEOTORCH_ON_ADJUSTABLEPOWER: u64 = 2u64;
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_DISABLED: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_WARMSTART_MODE_ENABLED: u64 = 1u64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_WBPRESET(pub i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CANDLELIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(6i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_CLOUDY: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(1i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_DAYLIGHT: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(2i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLASH: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(3i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_FLUORESCENT: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(4i32);
pub const KSCAMERA_EXTENDEDPROP_WBPRESET_TUNGSTEN: KSCAMERA_EXTENDEDPROP_WBPRESET = KSCAMERA_EXTENDEDPROP_WBPRESET(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(pub i32);
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_PRESET: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(2i32);
pub const KSCAMERA_EXTENDEDPROP_WHITEBALANCE_TEMPERATURE: KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE = KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE(1i32);
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DEFAULT: u64 = 0u64;
pub const KSCAMERA_EXTENDEDPROP_ZOOM_DIRECT: u64 = 1u64;
pub const KSCAMERA_EXTENDEDPROP_ZOOM_SMOOTH: u64 = 2u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    pub PhotoResWidth: u32,
    pub PhotoResHeight: u32,
    pub PreviewFPSNum: u32,
    pub PreviewFPSDenom: u32,
    pub CaptureFPSNum: u32,
    pub CaptureFPSDenom: u32,
}
impl Default for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub MaskCoverageBoundingBox: super::super::Foundation::RECT,
    pub MaskResolution: super::super::Foundation::SIZE,
    pub ForegroundBoundingBox: super::super::Foundation::RECT,
    pub MaskData: [u8; 1],
}
impl Default for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_METADATA_CAPTURESTATS {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Flags: u32,
    pub Reserved: u32,
    pub ExposureTime: u64,
    pub ExposureCompensationFlags: u64,
    pub ExposureCompensationValue: i32,
    pub IsoSpeed: u32,
    pub FocusState: u32,
    pub LensPosition: u32,
    pub WhiteBalance: u32,
    pub Flash: u32,
    pub FlashPower: u32,
    pub ZoomFactor: u32,
    pub SceneMode: u64,
    pub SensorFramerate: u64,
}
impl Default for KSCAMERA_METADATA_CAPTURESTATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURECOMPENSATION: u32 = 2u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_EXPOSURETIME: u32 = 1u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASH: u32 = 64u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FLASHPOWER: u32 = 128u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_FOCUSSTATE: u32 = 8u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ISOSPEED: u32 = 4u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_LENSPOSITION: u32 = 16u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SCENEMODE: u32 = 512u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_SENSORFRAMERATE: u32 = 1024u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_WHITEBALANCE: u32 = 32u32;
pub const KSCAMERA_METADATA_CAPTURESTATS_FLAG_ZOOMFACTOR: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_METADATA_DIGITALWINDOW {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Window: KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING,
}
impl Default for KSCAMERA_METADATA_DIGITALWINDOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_METADATA_FRAMEILLUMINATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub Flags: u32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_METADATA_FRAMEILLUMINATION_FLAG_ON: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_METADATA_ITEMHEADER {
    pub MetadataId: u32,
    pub Size: u32,
}
impl Default for KSCAMERA_METADATA_ITEMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_METADATA_PHOTOCONFIRMATION {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub PhotoConfirmationIndex: u32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_MetadataId(pub i32);
pub const KSCAMERA_PERFRAMESETTING_AUTO: u64 = 4294967296u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    pub Size: u32,
    pub ItemCount: u32,
    pub Flags: u64,
}
impl Default for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
impl Default for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    pub Size: u32,
    pub Reserved: u32,
    pub Id: windows_core::GUID,
}
impl Default for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    pub Size: u32,
    pub Id: u32,
    pub ItemCount: u32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_HEADER {
    pub Size: u32,
    pub FrameCount: u32,
    pub Id: windows_core::GUID,
    pub Flags: u64,
    pub LoopCount: u32,
    pub Reserved: u32,
}
impl Default for KSCAMERA_PERFRAMESETTING_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_PERFRAMESETTING_ITEM_CUSTOM: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(7i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_COMPENSATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(3i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_EXPOSURE_TIME: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(1i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_FLASH: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(2i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_FOCUS: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    pub Size: u32,
    pub Type: u32,
    pub Flags: u64,
}
impl Default for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCAMERA_PERFRAMESETTING_ITEM_ISO: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(4i32);
pub const KSCAMERA_PERFRAMESETTING_ITEM_PHOTOCONFIRMATION: KSCAMERA_PERFRAMESETTING_ITEM_TYPE = KSCAMERA_PERFRAMESETTING_ITEM_TYPE(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCAMERA_PERFRAMESETTING_ITEM_TYPE(pub i32);
pub const KSCAMERA_PERFRAMESETTING_MANUAL: u64 = 8589934592u64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PROFILE_CONCURRENCYINFO {
    pub ReferenceGuid: windows_core::GUID,
    pub Reserved: u32,
    pub ProfileCount: u32,
    pub Profiles: *mut KSCAMERA_PROFILE_INFO,
}
impl Default for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PROFILE_INFO {
    pub ProfileId: windows_core::GUID,
    pub Index: u32,
    pub PinCount: u32,
    pub Pins: *mut KSCAMERA_PROFILE_PININFO,
}
impl Default for KSCAMERA_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PROFILE_MEDIAINFO {
    pub Resolution: KSCAMERA_PROFILE_MEDIAINFO_0,
    pub MaxFrameRate: KSCAMERA_PROFILE_MEDIAINFO_1,
    pub Flags: u64,
    pub Data0: u32,
    pub Data1: u32,
    pub Data2: u32,
    pub Data3: u32,
}
impl Default for KSCAMERA_PROFILE_MEDIAINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PROFILE_MEDIAINFO_1 {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl Default for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PROFILE_MEDIAINFO_0 {
    pub X: u32,
    pub Y: u32,
}
impl Default for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSCAMERA_PROFILE_PININFO {
    pub PinCategory: windows_core::GUID,
    pub Anonymous: KSCAMERA_PROFILE_PININFO_0,
    pub MediaInfoCount: u32,
    pub MediaInfos: *mut KSCAMERA_PROFILE_MEDIAINFO,
}
impl Default for KSCAMERA_PROFILE_PININFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSCAMERA_PROFILE_PININFO_0 {
    pub Anonymous: KSCAMERA_PROFILE_PININFO_0_0,
    pub Reserved: u32,
}
impl Default for KSCAMERA_PROFILE_PININFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCAMERA_PROFILE_PININFO_0_0 {
    pub PinIndex: u16,
    pub ProfileSensorType: u16,
}
impl Default for KSCAMERA_PROFILE_PININFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCATEGORY_ACOUSTIC_ECHO_CANCEL: windows_core::GUID = windows_core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const KSCATEGORY_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x6994ad04_93ef_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_BRIDGE: windows_core::GUID = windows_core::GUID::from_u128(0x085aff00_62ce_11cf_a5d6_28db04c10000);
pub const KSCATEGORY_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x65e8773d_8f56_11d0_a3b9_00a0c9223196);
pub const KSCATEGORY_CLOCK: windows_core::GUID = windows_core::GUID::from_u128(0x53172480_4791_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_COMMUNICATIONSTRANSFORM: windows_core::GUID = windows_core::GUID::from_u128(0xcf1dda2c_9743_11d0_a3ee_00a0c9223196);
pub const KSCATEGORY_CROSSBAR: windows_core::GUID = windows_core::GUID::from_u128(0xa799a801_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_DATACOMPRESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x1e84c900_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_DATADECOMPRESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x2721ae20_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_DATATRANSFORM: windows_core::GUID = windows_core::GUID::from_u128(0x2eb07ea0_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x19689bf6_c384_48fd_ad51_90e58c79f70b);
pub const KSCATEGORY_ESCALANTE_PLATFORM_DRIVER: windows_core::GUID = windows_core::GUID::from_u128(0x74f3aea8_9768_11d1_8e07_00a0c95ec22e);
pub const KSCATEGORY_FILESYSTEM: windows_core::GUID = windows_core::GUID::from_u128(0x760fed5e_9357_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_INTERFACETRANSFORM: windows_core::GUID = windows_core::GUID::from_u128(0xcf1dda2d_9743_11d0_a3ee_00a0c9223196);
pub const KSCATEGORY_MEDIUMTRANSFORM: windows_core::GUID = windows_core::GUID::from_u128(0xcf1dda2e_9743_11d0_a3ee_00a0c9223196);
pub const KSCATEGORY_MICROPHONE_ARRAY_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x830a44f2_a32d_476b_be97_42845673b35a);
pub const KSCATEGORY_MIXER: windows_core::GUID = windows_core::GUID::from_u128(0xad809c00_7b88_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_MULTIPLEXER: windows_core::GUID = windows_core::GUID::from_u128(0x7a5de1d3_01a1_452c_b481_4fa2b96271e8);
pub const KSCATEGORY_NETWORK: windows_core::GUID = windows_core::GUID::from_u128(0x67c9cc3c_69c4_11d2_8759_00a0c9223196);
pub const KSCATEGORY_NETWORK_CAMERA: windows_core::GUID = windows_core::GUID::from_u128(0xb8238652_b500_41eb_b4f3_4234f7f5ae99);
pub const KSCATEGORY_PROXY: windows_core::GUID = windows_core::GUID::from_u128(0x97ebaaca_95bd_11d0_a3ea_00a0c9223196);
pub const KSCATEGORY_QUALITY: windows_core::GUID = windows_core::GUID::from_u128(0x97ebaacb_95bd_11d0_a3ea_00a0c9223196);
pub const KSCATEGORY_REALTIME: windows_core::GUID = windows_core::GUID::from_u128(0xeb115ffc_10c8_4964_831d_6dcb02e6f23f);
pub const KSCATEGORY_RENDER: windows_core::GUID = windows_core::GUID::from_u128(0x65e8773e_8f56_11d0_a3b9_00a0c9223196);
pub const KSCATEGORY_SENSOR_CAMERA: windows_core::GUID = windows_core::GUID::from_u128(0x24e552d7_6523_47f7_a647_d3465bf1f5ca);
pub const KSCATEGORY_SENSOR_GROUP: windows_core::GUID = windows_core::GUID::from_u128(0x669c7214_0a88_4311_a7f3_4e79820e33bd);
pub const KSCATEGORY_SPLITTER: windows_core::GUID = windows_core::GUID::from_u128(0x0a4252a0_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_TEXT: windows_core::GUID = windows_core::GUID::from_u128(0x6994ad06_93ef_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_TOPOLOGY: windows_core::GUID = windows_core::GUID::from_u128(0xdda54a40_1e4c_11d1_a050_405705c10000);
pub const KSCATEGORY_TVAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xa799a802_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_TVTUNER: windows_core::GUID = windows_core::GUID::from_u128(0xa799a800_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_VBICODEC: windows_core::GUID = windows_core::GUID::from_u128(0x07dad660_22f1_11d1_a9f4_00c04fbbde8f);
pub const KSCATEGORY_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x6994ad05_93ef_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_VIDEO_CAMERA: windows_core::GUID = windows_core::GUID::from_u128(0xe5323777_f976_4f5b_9b55_b94699c46e44);
pub const KSCATEGORY_VIRTUAL: windows_core::GUID = windows_core::GUID::from_u128(0x3503eac4_1f26_11d1_8ab0_00a0c9223196);
pub const KSCATEGORY_VPMUX: windows_core::GUID = windows_core::GUID::from_u128(0xa799a803_a46d_11d0_a18c_00a02401dcd4);
pub const KSCATEGORY_WDMAUD_USE_PIN_NAME: windows_core::GUID = windows_core::GUID::from_u128(0x47a4fa20_a251_11d1_a050_0000f8004788);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCLOCK_CREATE {
    pub CreateFlags: u32,
}
impl Default for KSCLOCK_CREATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCOMPONENTID {
    pub Manufacturer: windows_core::GUID,
    pub Product: windows_core::GUID,
    pub Component: windows_core::GUID,
    pub Name: windows_core::GUID,
    pub Version: u32,
    pub Revision: u32,
}
impl Default for KSCOMPONENTID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCOMPONENTID_USBAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x8f1275f0_26e9_4264_ba4d_39fff01d94aa);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSCORRELATED_TIME {
    pub Time: i64,
    pub SystemTime: i64,
}
impl Default for KSCORRELATED_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSCREATE_ITEM_FREEONSTOP: u32 = 8u32;
pub const KSCREATE_ITEM_NOPARAMETERS: u32 = 4u32;
pub const KSCREATE_ITEM_SECURITYCHANGED: u32 = 1u32;
pub const KSCREATE_ITEM_WILDCARD: u32 = 2u32;
pub const KSCameraProfileSensorType_Custom: u32 = 128u32;
pub const KSCameraProfileSensorType_Depth: u32 = 4u32;
pub const KSCameraProfileSensorType_ImageSegmentation: u32 = 16u32;
pub const KSCameraProfileSensorType_Infrared: u32 = 2u32;
pub const KSCameraProfileSensorType_PoseTracking: u32 = 8u32;
pub const KSCameraProfileSensorType_RGB: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSDATAFORMAT {
    pub Anonymous: KSDATAFORMAT_0,
    pub Alignment: i64,
}
impl Default for KSDATAFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSDATAFORMAT_0 {
    pub FormatSize: u32,
    pub Flags: u32,
    pub SampleSize: u32,
    pub Reserved: u32,
    pub MajorFormat: windows_core::GUID,
    pub SubFormat: windows_core::GUID,
    pub Specifier: windows_core::GUID,
}
impl Default for KSDATAFORMAT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDATAFORMAT_BIT_ATTRIBUTES: u32 = 1u32;
pub const KSDATAFORMAT_BIT_TEMPORAL_COMPRESSION: u32 = 0u32;
pub const KSDATAFORMAT_SPECIFIER_AC3_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e4_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_ANALOGVIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x0482dde0_7817_11cf_8a03_00aa006ecb65);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_AC3_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b35_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b32_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG1_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b31_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b34_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DIALECT_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b33_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SPECIFIER_DSOUND: windows_core::GUID = windows_core::GUID::from_u128(0x518590a2_a184_11d0_8522_00c04fd9baf3);
pub const KSDATAFORMAT_SPECIFIER_FILEHANDLE: windows_core::GUID = windows_core::GUID::from_u128(0x65e8773c_8f56_11d0_a3b9_00a0c9223196);
pub const KSDATAFORMAT_SPECIFIER_FILENAME: windows_core::GUID = windows_core::GUID::from_u128(0xaa797b40_e974_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_SPECIFIER_H264_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x2017be05_6629_4248_aaed_7e1a47bc9b9c);
pub const KSDATAFORMAT_SPECIFIER_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x692fa379_d3e8_4651_b5b4_0b94b013eeaf);
pub const KSDATAFORMAT_SPECIFIER_JPEG_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x692fa379_d3e8_4651_b5b4_0b94b013eeaf);
pub const KSDATAFORMAT_SPECIFIER_LPCM_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e6_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_MPEG1_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x05589f82_c356_11ce_bf01_00aa0055595a);
pub const KSDATAFORMAT_SPECIFIER_MPEG2_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e5_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SPECIFIER_NONE: windows_core::GUID = windows_core::GUID::from_u128(0x0f6417d6_c318_11d0_a43f_00a0c9223196);
pub const KSDATAFORMAT_SPECIFIER_VBI: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76e0_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SPECIFIER_VC_ID: windows_core::GUID = windows_core::GUID::from_u128(0xad98d184_aac3_11d0_a41c_00a0c9223196);
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO: windows_core::GUID = windows_core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const KSDATAFORMAT_SPECIFIER_VIDEOINFO2: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76a0_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SPECIFIER_WAVEFORMATEX: windows_core::GUID = windows_core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const KSDATAFORMAT_SUBTYPE_AC3_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802c_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_ANALOG: windows_core::GUID = windows_core::GUID::from_u128(0x6dba3190_67bd_11cf_a0f7_0020afd156e4);
pub const KSDATAFORMAT_SUBTYPE_CC: windows_core::GUID = windows_core::GUID::from_u128(0x33214cc1_011f_11d2_b4b1_00a0d102cfbe);
pub const KSDATAFORMAT_SUBTYPE_D16: windows_core::GUID = windows_core::GUID::from_u128(0x00000050_0004_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_DSS_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xa0af4f82_e163_11d0_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_DSS_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xa0af4f81_e163_11d0_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_DTS_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8033_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_AAC: windows_core::GUID = windows_core::GUID::from_u128(0x00000006_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ATRAC: windows_core::GUID = windows_core::GUID::from_u128(0x00000008_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL: windows_core::GUID = windows_core::GUID::from_u128(0x00000092_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS: windows_core::GUID = windows_core::GUID::from_u128(0x0000000a_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL_PLUS_ATMOS: windows_core::GUID = windows_core::GUID::from_u128(0x0000010a_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT20: windows_core::GUID = windows_core::GUID::from_u128(0x0000010c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MAT21: windows_core::GUID = windows_core::GUID::from_u128(0x0000030c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_MLP: windows_core::GUID = windows_core::GUID::from_u128(0x0000000c_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DST: windows_core::GUID = windows_core::GUID::from_u128(0x0000000d_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS: windows_core::GUID = windows_core::GUID::from_u128(0x00000008_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E1: windows_core::GUID = windows_core::GUID::from_u128(0x0000010b_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTSX_E2: windows_core::GUID = windows_core::GUID::from_u128(0x0000030b_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_DTS_HD: windows_core::GUID = windows_core::GUID::from_u128(0x0000000b_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG1: windows_core::GUID = windows_core::GUID::from_u128(0x00000003_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG2: windows_core::GUID = windows_core::GUID::from_u128(0x00000004_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_MPEG3: windows_core::GUID = windows_core::GUID::from_u128(0x00000005_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_ONE_BIT_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x00000009_0cea_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IEC61937_WMA_PRO: windows_core::GUID = windows_core::GUID::from_u128(0x00000164_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_IMAGE_RGB32: windows_core::GUID = windows_core::GUID::from_u128(0x00000016_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_JPEG: windows_core::GUID = windows_core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const KSDATAFORMAT_SUBTYPE_L16: windows_core::GUID = windows_core::GUID::from_u128(0x00000051_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L16_CUSTOM: windows_core::GUID = windows_core::GUID::from_u128(0x00000051_8000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L16_IR: windows_core::GUID = windows_core::GUID::from_u128(0x00000051_0002_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L8: windows_core::GUID = windows_core::GUID::from_u128(0x00000032_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L8_CUSTOM: windows_core::GUID = windows_core::GUID::from_u128(0x00000032_8000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_L8_IR: windows_core::GUID = windows_core::GUID::from_u128(0x00000032_0002_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_LPCM_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8032_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_Line21_BytePair: windows_core::GUID = windows_core::GUID::from_u128(0x6e8d4a22_310c_11d0_b79a_00aa003767a7);
pub const KSDATAFORMAT_SUBTYPE_Line21_GOPPacket: windows_core::GUID = windows_core::GUID::from_u128(0x6e8d4a23_310c_11d0_b79a_00aa003767a7);
pub const KSDATAFORMAT_SUBTYPE_MIDI: windows_core::GUID = windows_core::GUID::from_u128(0x1d262760_e957_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_SUBTYPE_MIDI_BUS: windows_core::GUID = windows_core::GUID::from_u128(0x2ca15fa0_6cfe_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_SUBTYPE_MJPG_CUSTOM: windows_core::GUID = windows_core::GUID::from_u128(0x47504a4d_8000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MJPG_DEPTH: windows_core::GUID = windows_core::GUID::from_u128(0x47504a4d_0004_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MJPG_IR: windows_core::GUID = windows_core::GUID::from_u128(0x47504a4d_0002_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Packet: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb80_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Payload: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb81_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_MPEG1Video: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb86_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_MPEG2_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802b_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_MPEGLAYER3: windows_core::GUID = windows_core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_MPEG_HEAAC: windows_core::GUID = windows_core::GUID::from_u128(0x00001610_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_NABTS: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76e2_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SUBTYPE_NABTS_FEC: windows_core::GUID = windows_core::GUID::from_u128(0xe757bca1_39ac_11d1_a9f5_00c04fbbde8f);
pub const KSDATAFORMAT_SUBTYPE_NONE: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb8e_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_OVERLAY: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb7f_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_PCM: windows_core::GUID = windows_core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_RAW8: windows_core::GUID = windows_core::GUID::from_u128(0xca20d9a0_3e3e_11d1_9bf9_00c04fbbdebf);
pub const KSDATAFORMAT_SUBTYPE_RIFF: windows_core::GUID = windows_core::GUID::from_u128(0x4995daee_9ee6_11d0_a40e_00a0c9223196);
pub const KSDATAFORMAT_SUBTYPE_RIFFMIDI: windows_core::GUID = windows_core::GUID::from_u128(0x4995daf0_9ee6_11d0_a40e_00a0c9223196);
pub const KSDATAFORMAT_SUBTYPE_RIFFWAVE: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb8b_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_SUBTYPE_SDDS_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8034_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_AC3_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b25_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b22_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG1_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b21_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b24_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_STANDARD_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x36523b23_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_SUBTYPE_SUBPICTURE: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802d_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_SUBTYPE_TELETEXT: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76e3_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_SUBTYPE_VPVBI: windows_core::GUID = windows_core::GUID::from_u128(0x5a9b6a41_1a22_11d1_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_VPVideo: windows_core::GUID = windows_core::GUID::from_u128(0x5a9b6a40_1a22_11d1_bad9_00609744111a);
pub const KSDATAFORMAT_SUBTYPE_WAVEFORMATEX: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO2: windows_core::GUID = windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO3: windows_core::GUID = windows_core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_WMAUDIO_LOSSLESS: windows_core::GUID = windows_core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_ANALOGAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x0482dee1_7817_11cf_8a03_00aa006ecb65);
pub const KSDATAFORMAT_TYPE_ANALOGVIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x0482dde1_7817_11cf_8a03_00aa006ecb65);
pub const KSDATAFORMAT_TYPE_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_AUXLine21Data: windows_core::GUID = windows_core::GUID::from_u128(0x670aea80_3a82_11d0_b79b_00aa003767a7);
pub const KSDATAFORMAT_TYPE_DVD_ENCRYPTED_PACK: windows_core::GUID = windows_core::GUID::from_u128(0xed0b916a_044d_11d1_aa78_00c04fc31d60);
pub const KSDATAFORMAT_TYPE_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x72178c23_e45b_11d5_bc2a_00b0d0f3f4ab);
pub const KSDATAFORMAT_TYPE_MIDI: windows_core::GUID = windows_core::GUID::from_u128(0x7364696d_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_MPEG2_PES: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8020_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_TYPE_MPEG2_PROGRAM: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8022_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_TYPE_MPEG2_TRANSPORT: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8023_db46_11cf_b4d1_00805f6cbbea);
pub const KSDATAFORMAT_TYPE_MUSIC: windows_core::GUID = windows_core::GUID::from_u128(0xe725d360_62cc_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_TYPE_NABTS: windows_core::GUID = windows_core::GUID::from_u128(0xe757bca0_39ac_11d1_a9f5_00c04fbbde8f);
pub const KSDATAFORMAT_TYPE_STANDARD_ELEMENTARY_STREAM: windows_core::GUID = windows_core::GUID::from_u128(0x36523b11_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_TYPE_STANDARD_PACK_HEADER: windows_core::GUID = windows_core::GUID::from_u128(0x36523b13_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_TYPE_STANDARD_PES_PACKET: windows_core::GUID = windows_core::GUID::from_u128(0x36523b12_8ee5_11d1_8ca3_0060b057664a);
pub const KSDATAFORMAT_TYPE_STREAM: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb83_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_TYPE_TEXT: windows_core::GUID = windows_core::GUID::from_u128(0x73747874_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_TYPE_VBI: windows_core::GUID = windows_core::GUID::from_u128(0xf72a76e1_eb0a_11d0_ace4_0000c0cc16ba);
pub const KSDATAFORMAT_TYPE_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDATARANGE_AUDIO {
    pub DataRange: KSDATAFORMAT,
    pub MaximumChannels: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
}
impl Default for KSDATARANGE_AUDIO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDATARANGE_BIT_ATTRIBUTES: u32 = 1u32;
pub const KSDATARANGE_BIT_REQUIRED_ATTRIBUTES: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDATARANGE_MUSIC {
    pub DataRange: KSDATAFORMAT,
    pub Technology: windows_core::GUID,
    pub Channels: u32,
    pub Notes: u32,
    pub ChannelMask: u32,
}
impl Default for KSDATARANGE_MUSIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDEGRADESETID_Standard: windows_core::GUID = windows_core::GUID::from_u128(0x9f564180_704c_11d0_a5d6_28db04c10000);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDEGRADE_STANDARD(pub i32);
pub const KSDEGRADE_STANDARD_COMPUTATION: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(2i32);
pub const KSDEGRADE_STANDARD_QUALITY: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(1i32);
pub const KSDEGRADE_STANDARD_SAMPLE: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(0i32);
pub const KSDEGRADE_STANDARD_SKIP: KSDEGRADE_STANDARD = KSDEGRADE_STANDARD(3i32);
pub const KSDEVICE_DESCRIPTOR_VERSION: u32 = 256u32;
pub const KSDEVICE_DESCRIPTOR_VERSION_2: u32 = 272u32;
pub const KSDEVICE_FLAG_ENABLE_QUERYINTERFACE: u32 = 4u32;
pub const KSDEVICE_FLAG_ENABLE_REMOTE_WAKEUP: u32 = 1u32;
pub const KSDEVICE_FLAG_LOWPOWER_PASSTHROUGH: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDEVICE_PROFILE_INFO {
    pub Type: u32,
    pub Size: u32,
    pub Anonymous: KSDEVICE_PROFILE_INFO_0,
}
impl Default for KSDEVICE_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSDEVICE_PROFILE_INFO_0 {
    pub Camera: KSDEVICE_PROFILE_INFO_0_0,
}
impl Default for KSDEVICE_PROFILE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSDEVICE_PROFILE_INFO_0_0 {
    pub Info: KSCAMERA_PROFILE_INFO,
    pub Reserved: u32,
    pub ConcurrencyCount: u32,
    pub Concurrency: *mut KSCAMERA_PROFILE_CONCURRENCYINFO,
}
impl Default for KSDEVICE_PROFILE_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDEVICE_PROFILE_TYPE_CAMERA: u32 = 1u32;
pub const KSDEVICE_PROFILE_TYPE_UNKNOWN: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDEVICE_THERMAL_STATE(pub i32);
pub const KSDEVICE_THERMAL_STATE_HIGH: KSDEVICE_THERMAL_STATE = KSDEVICE_THERMAL_STATE(1i32);
pub const KSDEVICE_THERMAL_STATE_LOW: KSDEVICE_THERMAL_STATE = KSDEVICE_THERMAL_STATE(0i32);
pub const KSDISPATCH_FASTIO: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSDISPLAYCHANGE {
    pub PelsWidth: u32,
    pub PelsHeight: u32,
    pub BitsPerPel: u32,
    pub DeviceID: [u16; 1],
}
impl Default for KSDISPLAYCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDS3D_BUFFER_ALL {
    pub Position: DS3DVECTOR,
    pub Velocity: DS3DVECTOR,
    pub InsideConeAngle: u32,
    pub OutsideConeAngle: u32,
    pub ConeOrientation: DS3DVECTOR,
    pub ConeOutsideVolume: i32,
    pub MinDistance: f32,
    pub MaxDistance: f32,
    pub Mode: u32,
}
impl Default for KSDS3D_BUFFER_ALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSDS3D_BUFFER_CONE_ANGLES {
    pub InsideConeAngle: u32,
    pub OutsideConeAngle: u32,
}
impl Default for KSDS3D_BUFFER_CONE_ANGLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDS3D_COEFF_COUNT: KSDS3D_HRTF_COEFF_FORMAT = KSDS3D_HRTF_COEFF_FORMAT(2i32);
pub const KSDS3D_FILTER_METHOD_COUNT: KSDS3D_HRTF_FILTER_METHOD = KSDS3D_HRTF_FILTER_METHOD(2i32);
pub const KSDS3D_FILTER_QUALITY_COUNT: KSDS3D_HRTF_FILTER_QUALITY = KSDS3D_HRTF_FILTER_QUALITY(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDS3D_HRTF_COEFF_FORMAT(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSDS3D_HRTF_FILTER_FORMAT_MSG {
    pub FilterMethod: KSDS3D_HRTF_FILTER_METHOD,
    pub CoeffFormat: KSDS3D_HRTF_COEFF_FORMAT,
    pub Version: KSDS3D_HRTF_FILTER_VERSION,
    pub Reserved: u32,
}
impl Default for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDS3D_HRTF_FILTER_METHOD(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDS3D_HRTF_FILTER_QUALITY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDS3D_HRTF_FILTER_VERSION(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSDS3D_HRTF_INIT_MSG {
    pub Size: u32,
    pub Quality: KSDS3D_HRTF_FILTER_QUALITY,
    pub SampleRate: f32,
    pub MaxFilterSize: u32,
    pub FilterTransientMuteLength: u32,
    pub FilterOverlapBufferLength: u32,
    pub OutputOverlapBufferLength: u32,
    pub Reserved: u32,
}
impl Default for KSDS3D_HRTF_INIT_MSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSDS3D_HRTF_PARAMS_MSG {
    pub Size: u32,
    pub Enabled: u32,
    pub SwapChannels: super::super::Foundation::BOOL,
    pub ZeroAzimuth: super::super::Foundation::BOOL,
    pub CrossFadeOutput: super::super::Foundation::BOOL,
    pub FilterSize: u32,
}
impl Default for KSDS3D_HRTF_PARAMS_MSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSDS3D_ITD_PARAMS {
    pub Channel: i32,
    pub VolSmoothScale: f32,
    pub TotalDryAttenuation: f32,
    pub TotalWetAttenuation: f32,
    pub SmoothFrequency: i32,
    pub Delay: i32,
}
impl Default for KSDS3D_ITD_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSDS3D_ITD_PARAMS_MSG {
    pub Enabled: u32,
    pub LeftParams: KSDS3D_ITD_PARAMS,
    pub RightParams: KSDS3D_ITD_PARAMS,
    pub Reserved: u32,
}
impl Default for KSDS3D_ITD_PARAMS_MSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDS3D_LISTENER_ALL {
    pub Position: DS3DVECTOR,
    pub Velocity: DS3DVECTOR,
    pub OrientFront: DS3DVECTOR,
    pub OrientTop: DS3DVECTOR,
    pub DistanceFactor: f32,
    pub RolloffFactor: f32,
    pub DopplerFactor: f32,
}
impl Default for KSDS3D_LISTENER_ALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSDS3D_LISTENER_ORIENTATION {
    pub Front: DS3DVECTOR,
    pub Top: DS3DVECTOR,
}
impl Default for KSDS3D_LISTENER_ORIENTATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSDSOUND_3D_MODE_DISABLE: u32 = 2u32;
pub const KSDSOUND_3D_MODE_HEADRELATIVE: u32 = 1u32;
pub const KSDSOUND_3D_MODE_NORMAL: u32 = 0u32;
pub const KSDSOUND_BUFFER_CTRL_3D: u32 = 1u32;
pub const KSDSOUND_BUFFER_CTRL_FREQUENCY: u32 = 2u32;
pub const KSDSOUND_BUFFER_CTRL_HRTF_3D: u32 = 1073741824u32;
pub const KSDSOUND_BUFFER_CTRL_PAN: u32 = 4u32;
pub const KSDSOUND_BUFFER_CTRL_POSITIONNOTIFY: u32 = 16u32;
pub const KSDSOUND_BUFFER_CTRL_VOLUME: u32 = 8u32;
pub const KSDSOUND_BUFFER_LOCHARDWARE: u32 = 4u32;
pub const KSDSOUND_BUFFER_LOCSOFTWARE: u32 = 8u32;
pub const KSDSOUND_BUFFER_PRIMARY: u32 = 1u32;
pub const KSDSOUND_BUFFER_STATIC: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSERROR {
    pub Context: *mut core::ffi::c_void,
    pub Status: u32,
}
impl Default for KSERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSEVENTDATA {
    pub NotificationType: u32,
    pub Anonymous: KSEVENTDATA_0,
}
impl Default for KSEVENTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSEVENTDATA_0 {
    pub EventHandle: KSEVENTDATA_0_0,
    pub SemaphoreHandle: KSEVENTDATA_0_1,
    pub Alignment: KSEVENTDATA_0_2,
}
impl Default for KSEVENTDATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSEVENTDATA_0_2 {
    pub Unused: *mut core::ffi::c_void,
    pub Alignment: [isize; 2],
}
impl Default for KSEVENTDATA_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSEVENTDATA_0_0 {
    pub Event: super::super::Foundation::HANDLE,
    pub Reserved: [usize; 2],
}
impl Default for KSEVENTDATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSEVENTDATA_0_1 {
    pub Semaphore: super::super::Foundation::HANDLE,
    pub Reserved: u32,
    pub Adjustment: i32,
}
impl Default for KSEVENTDATA_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSEVENTF_DPC: u32 = 16u32;
pub const KSEVENTF_EVENT_HANDLE: u32 = 1u32;
pub const KSEVENTF_EVENT_OBJECT: u32 = 4u32;
pub const KSEVENTF_KSWORKITEM: u32 = 128u32;
pub const KSEVENTF_SEMAPHORE_HANDLE: u32 = 2u32;
pub const KSEVENTF_SEMAPHORE_OBJECT: u32 = 8u32;
pub const KSEVENTF_WORKITEM: u32 = 32u32;
pub const KSEVENTSETID_AudioControlChange: windows_core::GUID = windows_core::GUID::from_u128(0xe85e9698_fa2f_11d1_95bd_00c04fb925d3);
pub const KSEVENTSETID_CameraAsyncControl: windows_core::GUID = windows_core::GUID::from_u128(0x22a11754_9701_4088_b33f_6b9cbc52df5e);
pub const KSEVENTSETID_CameraEvent: windows_core::GUID = windows_core::GUID::from_u128(0x7899b2e0_6b43_4964_9d2a_a21f4061f576);
pub const KSEVENTSETID_Clock: windows_core::GUID = windows_core::GUID::from_u128(0x364d8e20_62c7_11cf_a5d6_28db04c10000);
pub const KSEVENTSETID_Connection: windows_core::GUID = windows_core::GUID::from_u128(0x7f4bcbe0_9ea5_11cf_a5d6_28db04c10000);
pub const KSEVENTSETID_Device: windows_core::GUID = windows_core::GUID::from_u128(0x288296ec_9f94_41b4_a153_aa31aeecb33f);
pub const KSEVENTSETID_DynamicFormatChange: windows_core::GUID = windows_core::GUID::from_u128(0x162ac456_83d7_4239_96df_c75ffa138bc6);
pub const KSEVENTSETID_EXTDEV_Command: windows_core::GUID = windows_core::GUID::from_u128(0x109c7988_b3cb_11d2_b48e_006097b3391b);
pub const KSEVENTSETID_ExtendedCameraControl: windows_core::GUID = windows_core::GUID::from_u128(0x571c92c9_13a2_47e3_a649_d2a778166384);
pub const KSEVENTSETID_LoopedStreaming: windows_core::GUID = windows_core::GUID::from_u128(0x4682b940_c6ef_11d0_96d8_00aa0051e51d);
pub const KSEVENTSETID_PinCapsChange: windows_core::GUID = windows_core::GUID::from_u128(0xdd4f192e_3b78_49ad_a534_2c315b822000);
pub const KSEVENTSETID_SoundDetector: windows_core::GUID = windows_core::GUID::from_u128(0x69785c9b_fc2d_49d6_ac32_4799f87de9f6);
pub const KSEVENTSETID_StreamAllocator: windows_core::GUID = windows_core::GUID::from_u128(0x75d95571_073c_11d0_a161_0020afd156e4);
pub const KSEVENTSETID_Telephony: windows_core::GUID = windows_core::GUID::from_u128(0xb77f12b4_ceb4_4484_8d5e_52c1e7d8762d);
pub const KSEVENTSETID_VIDCAPTOSTI: windows_core::GUID = windows_core::GUID::from_u128(0xdb47de20_f628_11d1_ba41_00a0c90d2b05);
pub const KSEVENTSETID_VIDCAP_TVAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0651_28e4_11d0_a18c_00a0c9118956);
pub const KSEVENTSETID_VPNotify: windows_core::GUID = windows_core::GUID::from_u128(0x20c5598e_d3c8_11d0_8dfc_00c04fd7c08b);
pub const KSEVENTSETID_VPVBINotify: windows_core::GUID = windows_core::GUID::from_u128(0xec529b01_1a1f_11d1_bad9_00609744111a);
pub const KSEVENTSETID_VolumeLimit: windows_core::GUID = windows_core::GUID::from_u128(0xda168465_3a7c_4858_9d4a_3e8e24701aef);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_AUDIO_CONTROL_CHANGE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_CAMERACONTROL(pub i32);
pub const KSEVENT_CAMERACONTROL_FOCUS: KSEVENT_CAMERACONTROL = KSEVENT_CAMERACONTROL(0i32);
pub const KSEVENT_CAMERACONTROL_ZOOM: KSEVENT_CAMERACONTROL = KSEVENT_CAMERACONTROL(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_CAMERAEVENT(pub i32);
pub const KSEVENT_CLOCK_INTERVAL_MARK: KSEVENT_CLOCK_POSITION = KSEVENT_CLOCK_POSITION(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_CLOCK_POSITION(pub i32);
pub const KSEVENT_CLOCK_POSITION_MARK: KSEVENT_CLOCK_POSITION = KSEVENT_CLOCK_POSITION(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_CONNECTION(pub i32);
pub const KSEVENT_CONNECTION_DATADISCONTINUITY: KSEVENT_CONNECTION = KSEVENT_CONNECTION(1i32);
pub const KSEVENT_CONNECTION_ENDOFSTREAM: KSEVENT_CONNECTION = KSEVENT_CONNECTION(4i32);
pub const KSEVENT_CONNECTION_POSITIONUPDATE: KSEVENT_CONNECTION = KSEVENT_CONNECTION(0i32);
pub const KSEVENT_CONNECTION_PRIORITY: KSEVENT_CONNECTION = KSEVENT_CONNECTION(3i32);
pub const KSEVENT_CONNECTION_TIMEDISCONTINUITY: KSEVENT_CONNECTION = KSEVENT_CONNECTION(2i32);
pub const KSEVENT_CONTROL_CHANGE: KSEVENT_AUDIO_CONTROL_CHANGE = KSEVENT_AUDIO_CONTROL_CHANGE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_CROSSBAR(pub i32);
pub const KSEVENT_CROSSBAR_CHANGED: KSEVENT_CROSSBAR = KSEVENT_CROSSBAR(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_DEVCMD(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_DEVICE(pub i32);
pub const KSEVENT_DEVICE_LOST: KSEVENT_DEVICE = KSEVENT_DEVICE(0i32);
pub const KSEVENT_DEVICE_PREEMPTED: KSEVENT_DEVICE = KSEVENT_DEVICE(1i32);
pub const KSEVENT_DEVICE_THERMAL_HIGH: KSEVENT_DEVICE = KSEVENT_DEVICE(2i32);
pub const KSEVENT_DEVICE_THERMAL_LOW: KSEVENT_DEVICE = KSEVENT_DEVICE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_DYNAMICFORMATCHANGE(pub i32);
pub const KSEVENT_DYNAMIC_FORMAT_CHANGE: KSEVENT_DYNAMICFORMATCHANGE = KSEVENT_DYNAMICFORMATCHANGE(0i32);
pub const KSEVENT_ENTRY_BUFFERED: u32 = 4u32;
pub const KSEVENT_ENTRY_DELETED: u32 = 1u32;
pub const KSEVENT_ENTRY_ONESHOT: u32 = 2u32;
pub const KSEVENT_EXTDEV_COMMAND_BUSRESET: KSEVENT_DEVCMD = KSEVENT_DEVCMD(2i32);
pub const KSEVENT_EXTDEV_COMMAND_CONTROL_INTERIM_READY: KSEVENT_DEVCMD = KSEVENT_DEVCMD(1i32);
pub const KSEVENT_EXTDEV_COMMAND_NOTIFY_INTERIM_READY: KSEVENT_DEVCMD = KSEVENT_DEVCMD(0i32);
pub const KSEVENT_EXTDEV_NOTIFY_MEDIUM_CHANGE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(7i32);
pub const KSEVENT_EXTDEV_NOTIFY_REMOVAL: KSEVENT_DEVCMD = KSEVENT_DEVCMD(6i32);
pub const KSEVENT_EXTDEV_OPERATION_MODE_UPDATE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(4i32);
pub const KSEVENT_EXTDEV_TIMECODE_UPDATE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(3i32);
pub const KSEVENT_EXTDEV_TRANSPORT_STATE_UPDATE: KSEVENT_DEVCMD = KSEVENT_DEVCMD(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_LOOPEDSTREAMING(pub i32);
pub const KSEVENT_LOOPEDSTREAMING_POSITION: KSEVENT_LOOPEDSTREAMING = KSEVENT_LOOPEDSTREAMING(0i32);
pub const KSEVENT_PHOTO_SAMPLE_SCANNED: KSEVENT_CAMERAEVENT = KSEVENT_CAMERAEVENT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_PINCAPS_CHANGENOTIFICATIONS(pub i32);
pub const KSEVENT_PINCAPS_FORMATCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = KSEVENT_PINCAPS_CHANGENOTIFICATIONS(0i32);
pub const KSEVENT_PINCAPS_JACKINFOCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = KSEVENT_PINCAPS_CHANGENOTIFICATIONS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_SOUNDDETECTOR(pub i32);
pub const KSEVENT_SOUNDDETECTOR_MATCHDETECTED: KSEVENT_SOUNDDETECTOR = KSEVENT_SOUNDDETECTOR(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_STREAMALLOCATOR(pub i32);
pub const KSEVENT_STREAMALLOCATOR_FREEFRAME: KSEVENT_STREAMALLOCATOR = KSEVENT_STREAMALLOCATOR(1i32);
pub const KSEVENT_STREAMALLOCATOR_INTERNAL_FREEFRAME: KSEVENT_STREAMALLOCATOR = KSEVENT_STREAMALLOCATOR(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_TELEPHONY(pub i32);
pub const KSEVENT_TELEPHONY_ENDPOINTPAIRS_CHANGED: KSEVENT_TELEPHONY = KSEVENT_TELEPHONY(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSEVENT_TIME_INTERVAL {
    pub EventData: KSEVENTDATA,
    pub TimeBase: i64,
    pub Interval: i64,
}
impl Default for KSEVENT_TIME_INTERVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSEVENT_TIME_MARK {
    pub EventData: KSEVENTDATA,
    pub MarkTime: i64,
}
impl Default for KSEVENT_TIME_MARK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_TUNER(pub i32);
pub const KSEVENT_TUNER_CHANGED: KSEVENT_TUNER = KSEVENT_TUNER(0i32);
pub const KSEVENT_TUNER_INITIATE_SCAN: KSEVENT_TUNER = KSEVENT_TUNER(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSEVENT_TUNER_INITIATE_SCAN_S {
    pub EventData: KSEVENTDATA,
    pub StartFrequency: u32,
    pub EndFrequency: u32,
}
impl Default for KSEVENT_TUNER_INITIATE_SCAN_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_TVAUDIO(pub i32);
pub const KSEVENT_TVAUDIO_CHANGED: KSEVENT_TVAUDIO = KSEVENT_TVAUDIO(0i32);
pub const KSEVENT_TYPE_BASICSUPPORT: u32 = 512u32;
pub const KSEVENT_TYPE_ENABLE: u32 = 1u32;
pub const KSEVENT_TYPE_ENABLEBUFFERED: u32 = 4u32;
pub const KSEVENT_TYPE_ONESHOT: u32 = 2u32;
pub const KSEVENT_TYPE_QUERYBUFFER: u32 = 1024u32;
pub const KSEVENT_TYPE_SETSUPPORT: u32 = 256u32;
pub const KSEVENT_TYPE_TOPOLOGY: u32 = 268435456u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_VIDCAPTOSTI(pub i32);
pub const KSEVENT_VIDCAPTOSTI_EXT_TRIGGER: KSEVENT_VIDCAPTOSTI = KSEVENT_VIDCAPTOSTI(0i32);
pub const KSEVENT_VIDCAP_AUTO_UPDATE: KSEVENT_VIDCAPTOSTI = KSEVENT_VIDCAPTOSTI(1i32);
pub const KSEVENT_VIDCAP_SEARCH: KSEVENT_VIDCAPTOSTI = KSEVENT_VIDCAPTOSTI(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_VIDEODECODER(pub i32);
pub const KSEVENT_VIDEODECODER_CHANGED: KSEVENT_VIDEODECODER = KSEVENT_VIDEODECODER(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_VOLUMELIMIT(pub i32);
pub const KSEVENT_VOLUMELIMIT_CHANGED: KSEVENT_VOLUMELIMIT = KSEVENT_VOLUMELIMIT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_VPNOTIFY(pub i32);
pub const KSEVENT_VPNOTIFY_FORMATCHANGE: KSEVENT_VPNOTIFY = KSEVENT_VPNOTIFY(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENT_VPVBINOTIFY(pub i32);
pub const KSEVENT_VPVBINOTIFY_FORMATCHANGE: KSEVENT_VPVBINOTIFY = KSEVENT_VPVBINOTIFY(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSE_NODE {
    pub Event: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl Default for KSE_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSE_PIN {
    pub Event: KSIDENTIFIER,
    pub PinId: u32,
    pub Reserved: u32,
}
impl Default for KSE_PIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSFILTER_FLAG_CRITICAL_PROCESSING: u32 = 2u32;
pub const KSFILTER_FLAG_DENY_USERMODE_ACCESS: u32 = 2147483648u32;
pub const KSFILTER_FLAG_DISPATCH_LEVEL_PROCESSING: u32 = 1u32;
pub const KSFILTER_FLAG_HYPERCRITICAL_PROCESSING: u32 = 4u32;
pub const KSFILTER_FLAG_PRIORITIZE_REFERENCEGUID: u32 = 16u32;
pub const KSFILTER_FLAG_RECEIVE_ZERO_LENGTH_SAMPLES: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSFRAMETIME {
    pub Duration: i64,
    pub FrameFlags: u32,
    pub Reserved: u32,
}
impl Default for KSFRAMETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSFRAMETIME_VARIABLESIZE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSGOP_USERDATA {
    pub sc: u32,
    pub reserved1: u32,
    pub cFields: u8,
    pub l21Data: [i8; 3],
}
impl Default for KSGOP_USERDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSIDENTIFIER {
    pub Anonymous: KSIDENTIFIER_0,
}
impl Default for KSIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSIDENTIFIER_0 {
    pub Anonymous: KSIDENTIFIER_0_0,
    pub Alignment: i64,
}
impl Default for KSIDENTIFIER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSIDENTIFIER_0_0 {
    pub Set: windows_core::GUID,
    pub Id: u32,
    pub Flags: u32,
}
impl Default for KSIDENTIFIER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSINTERFACESETID_FileIo: windows_core::GUID = windows_core::GUID::from_u128(0x8c6f932c_e771_11d0_b8ff_00a0c9223196);
pub const KSINTERFACESETID_Media: windows_core::GUID = windows_core::GUID::from_u128(0x3a13eb40_30a7_11d0_a5d6_28db04c10000);
pub const KSINTERFACESETID_Standard: windows_core::GUID = windows_core::GUID::from_u128(0x1a8766a0_62ce_11cf_a5d6_28db04c10000);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSINTERFACE_FILEIO(pub i32);
pub const KSINTERFACE_FILEIO_STREAMING: KSINTERFACE_FILEIO = KSINTERFACE_FILEIO(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSINTERFACE_MEDIA(pub i32);
pub const KSINTERFACE_MEDIA_MUSIC: KSINTERFACE_MEDIA = KSINTERFACE_MEDIA(0i32);
pub const KSINTERFACE_MEDIA_WAVE_BUFFERED: KSINTERFACE_MEDIA = KSINTERFACE_MEDIA(1i32);
pub const KSINTERFACE_MEDIA_WAVE_QUEUED: KSINTERFACE_MEDIA = KSINTERFACE_MEDIA(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSINTERFACE_STANDARD(pub i32);
pub const KSINTERFACE_STANDARD_CONTROL: KSINTERFACE_STANDARD = KSINTERFACE_STANDARD(2i32);
pub const KSINTERFACE_STANDARD_LOOPED_STREAMING: KSINTERFACE_STANDARD = KSINTERFACE_STANDARD(1i32);
pub const KSINTERFACE_STANDARD_STREAMING: KSINTERFACE_STANDARD = KSINTERFACE_STANDARD(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSINTERVAL {
    pub TimeBase: i64,
    pub Interval: i64,
}
impl Default for KSINTERVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSIOOPERATION(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSJACK_DESCRIPTION {
    pub ChannelMapping: u32,
    pub Color: u32,
    pub ConnectionType: EPcxConnectionType,
    pub GeoLocation: EPcxGeoLocation,
    pub GenLocation: EPcxGenLocation,
    pub PortConnection: EPxcPortConnection,
    pub IsConnected: super::super::Foundation::BOOL,
}
impl Default for KSJACK_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSJACK_DESCRIPTION2 {
    pub DeviceStateInfo: u32,
    pub JackCapabilities: u32,
}
impl Default for KSJACK_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSJACK_DESCRIPTION3 {
    pub ConfigId: u32,
}
impl Default for KSJACK_DESCRIPTION3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSJACK_SINK_CONNECTIONTYPE(pub i32);
pub const KSJACK_SINK_CONNECTIONTYPE_DISPLAYPORT: KSJACK_SINK_CONNECTIONTYPE = KSJACK_SINK_CONNECTIONTYPE(1i32);
pub const KSJACK_SINK_CONNECTIONTYPE_HDMI: KSJACK_SINK_CONNECTIONTYPE = KSJACK_SINK_CONNECTIONTYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSJACK_SINK_INFORMATION {
    pub ConnType: KSJACK_SINK_CONNECTIONTYPE,
    pub ManufacturerId: u16,
    pub ProductId: u16,
    pub AudioLatency: u16,
    pub HDCPCapable: super::super::Foundation::BOOL,
    pub AICapable: super::super::Foundation::BOOL,
    pub SinkDescriptionLength: u8,
    pub SinkDescription: [u16; 32],
    pub PortId: super::super::Foundation::LUID,
}
impl Default for KSJACK_SINK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSMEDIUMSETID_MidiBus: windows_core::GUID = windows_core::GUID::from_u128(0x05908040_3246_11d0_a5d6_28db04c10000);
pub const KSMEDIUMSETID_Standard: windows_core::GUID = windows_core::GUID::from_u128(0x4747b320_62ce_11cf_a5d6_28db04c10000);
pub const KSMEDIUMSETID_VPBus: windows_core::GUID = windows_core::GUID::from_u128(0xa18c15ec_ce43_11d0_abe7_00a0c9223196);
pub const KSMEDIUM_STANDARD_DEVIO: u32 = 0u32;
pub const KSMEDIUM_TYPE_ANYINSTANCE: u32 = 0u32;
pub const KSMEMORY_TYPE_DEVICE_UNKNOWN: windows_core::GUID = windows_core::GUID::from_u128(0x091bb639_603f_11d1_b067_00a0c9062802);
pub const KSMEMORY_TYPE_KERNEL_NONPAGED: windows_core::GUID = windows_core::GUID::from_u128(0x4a6d5fc4_7895_11d1_b069_00a0c9062802);
pub const KSMEMORY_TYPE_KERNEL_PAGED: windows_core::GUID = windows_core::GUID::from_u128(0xd833f8f8_7894_11d1_b069_00a0c9062802);
pub const KSMEMORY_TYPE_SYSTEM: windows_core::GUID = windows_core::GUID::from_u128(0x091bb638_603f_11d1_b067_00a0c9062802);
pub const KSMEMORY_TYPE_USER: windows_core::GUID = windows_core::GUID::from_u128(0x8cb0fc28_7893_11d1_b069_00a0c9062802);
pub const KSMETHODSETID_StreamAllocator: windows_core::GUID = windows_core::GUID::from_u128(0xcf6e4341_ec87_11cf_a130_0020afd156e4);
pub const KSMETHODSETID_StreamIo: windows_core::GUID = windows_core::GUID::from_u128(0x65d003ca_1523_11d2_b27a_00a0c9223196);
pub const KSMETHODSETID_Wavetable: windows_core::GUID = windows_core::GUID::from_u128(0xdcef31eb_d907_11d0_9583_00c04fb925d3);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMETHOD_STREAMALLOCATOR(pub i32);
pub const KSMETHOD_STREAMALLOCATOR_ALLOC: KSMETHOD_STREAMALLOCATOR = KSMETHOD_STREAMALLOCATOR(0i32);
pub const KSMETHOD_STREAMALLOCATOR_FREE: KSMETHOD_STREAMALLOCATOR = KSMETHOD_STREAMALLOCATOR(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMETHOD_STREAMIO(pub i32);
pub const KSMETHOD_STREAMIO_READ: KSMETHOD_STREAMIO = KSMETHOD_STREAMIO(0i32);
pub const KSMETHOD_STREAMIO_WRITE: KSMETHOD_STREAMIO = KSMETHOD_STREAMIO(1i32);
pub const KSMETHOD_TYPE_BASICSUPPORT: u32 = 512u32;
pub const KSMETHOD_TYPE_MODIFY: u32 = 3u32;
pub const KSMETHOD_TYPE_NONE: u32 = 0u32;
pub const KSMETHOD_TYPE_READ: u32 = 1u32;
pub const KSMETHOD_TYPE_SEND: u32 = 1u32;
pub const KSMETHOD_TYPE_SETSUPPORT: u32 = 256u32;
pub const KSMETHOD_TYPE_SOURCE: u32 = 4u32;
pub const KSMETHOD_TYPE_TOPOLOGY: u32 = 268435456u32;
pub const KSMETHOD_TYPE_WRITE: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMETHOD_WAVETABLE(pub i32);
pub const KSMETHOD_WAVETABLE_WAVE_ALLOC: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(0i32);
pub const KSMETHOD_WAVETABLE_WAVE_FIND: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(2i32);
pub const KSMETHOD_WAVETABLE_WAVE_FREE: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(1i32);
pub const KSMETHOD_WAVETABLE_WAVE_WRITE: KSMETHOD_WAVETABLE = KSMETHOD_WAVETABLE(3i32);
pub const KSMETHOD_WAVE_QUEUED_BREAKLOOP: u32 = 1u32;
pub const KSMFT_CATEGORY_AUDIO_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0x9ea73fb4_ef7a_4559_8d5d_719d8f0426c7);
pub const KSMFT_CATEGORY_AUDIO_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0x11064c48_3648_4ed0_932e_05ce8ac811b7);
pub const KSMFT_CATEGORY_AUDIO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x91c64bd0_f91e_4d8c_9276_db248279d975);
pub const KSMFT_CATEGORY_DEMULTIPLEXER: windows_core::GUID = windows_core::GUID::from_u128(0xa8700a7a_939b_44c5_99d7_76226b23b3f1);
pub const KSMFT_CATEGORY_MULTIPLEXER: windows_core::GUID = windows_core::GUID::from_u128(0x059c561e_05ae_4b61_b69d_55b61ee54a7b);
pub const KSMFT_CATEGORY_OTHER: windows_core::GUID = windows_core::GUID::from_u128(0x90175d57_b7ea_4901_aeb3_933a8747756f);
pub const KSMFT_CATEGORY_VIDEO_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0xd6c02d4b_6833_45b4_971a_05a4b04bab91);
pub const KSMFT_CATEGORY_VIDEO_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0x12e17c21_532c_4a6e_8a1c_40825a736397);
pub const KSMFT_CATEGORY_VIDEO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0xf79eac7d_e545_4387_bdee_d647d7bde42a);
pub const KSMFT_CATEGORY_VIDEO_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x302ea3fc_aa5f_47f9_9f7a_c2188bb16302);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMICARRAY_MICARRAYTYPE(pub i32);
pub const KSMICARRAY_MICARRAYTYPE_3D: KSMICARRAY_MICARRAYTYPE = KSMICARRAY_MICARRAYTYPE(2i32);
pub const KSMICARRAY_MICARRAYTYPE_LINEAR: KSMICARRAY_MICARRAYTYPE = KSMICARRAY_MICARRAYTYPE(0i32);
pub const KSMICARRAY_MICARRAYTYPE_PLANAR: KSMICARRAY_MICARRAYTYPE = KSMICARRAY_MICARRAYTYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMICARRAY_MICTYPE(pub i32);
pub const KSMICARRAY_MICTYPE_8SHAPED: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(5i32);
pub const KSMICARRAY_MICTYPE_CARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(2i32);
pub const KSMICARRAY_MICTYPE_HYPERCARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(4i32);
pub const KSMICARRAY_MICTYPE_OMNIDIRECTIONAL: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(0i32);
pub const KSMICARRAY_MICTYPE_SUBCARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(1i32);
pub const KSMICARRAY_MICTYPE_SUPERCARDIOID: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(3i32);
pub const KSMICARRAY_MICTYPE_VENDORDEFINED: KSMICARRAY_MICTYPE = KSMICARRAY_MICTYPE(15i32);
pub const KSMPEGVIDMODE_LTRBOX: u32 = 2u32;
pub const KSMPEGVIDMODE_PANSCAN: u32 = 1u32;
pub const KSMPEGVIDMODE_SCALE: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSMPEGVID_RECT {
    pub StartX: u32,
    pub StartY: u32,
    pub EndX: u32,
    pub EndY: u32,
}
impl Default for KSMPEGVID_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSMULTIPLE_DATA_PROP {
    pub Property: KSIDENTIFIER,
    pub MultipleItem: KSMULTIPLE_ITEM,
}
impl Default for KSMULTIPLE_DATA_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSMULTIPLE_ITEM {
    pub Size: u32,
    pub Count: u32,
}
impl Default for KSMULTIPLE_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSMUSICFORMAT {
    pub TimeDeltaMs: u32,
    pub ByteCount: u32,
}
impl Default for KSMUSICFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSMUSIC_TECHNOLOGY_FMSYNTH: windows_core::GUID = windows_core::GUID::from_u128(0x252c5c80_62e9_11cf_a5d6_28db04c10000);
pub const KSMUSIC_TECHNOLOGY_PORT: windows_core::GUID = windows_core::GUID::from_u128(0x86c92e60_62e8_11cf_a5d6_28db04c10000);
pub const KSMUSIC_TECHNOLOGY_SQSYNTH: windows_core::GUID = windows_core::GUID::from_u128(0x0ecf4380_62e9_11cf_a5d6_28db04c10000);
pub const KSMUSIC_TECHNOLOGY_SWSYNTH: windows_core::GUID = windows_core::GUID::from_u128(0x37407736_3620_11d1_85d3_0000f8754380);
pub const KSMUSIC_TECHNOLOGY_WAVETABLE: windows_core::GUID = windows_core::GUID::from_u128(0x394ec7c0_62e9_11cf_a5d6_28db04c10000);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSM_NODE {
    pub Method: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl Default for KSM_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSNAME_Allocator: windows_core::GUID = windows_core::GUID::from_u128(0x642f5d00_4791_11d0_a5d6_28db04c10000);
pub const KSNAME_Clock: windows_core::GUID = windows_core::GUID::from_u128(0x53172480_4791_11d0_a5d6_28db04c10000);
pub const KSNAME_Filter: windows_core::GUID = windows_core::GUID::from_u128(0x9b365890_165f_11d0_a195_0020afd156e4);
pub const KSNAME_Pin: windows_core::GUID = windows_core::GUID::from_u128(0x146f1a80_4791_11d0_a5d6_28db04c10000);
pub const KSNAME_TopologyNode: windows_core::GUID = windows_core::GUID::from_u128(0x0621061a_ee75_11d0_b915_00a0c9223196);
pub const KSNODEPIN_AEC_CAPTURE_IN: u32 = 2u32;
pub const KSNODEPIN_AEC_CAPTURE_OUT: u32 = 3u32;
pub const KSNODEPIN_AEC_RENDER_IN: u32 = 1u32;
pub const KSNODEPIN_AEC_RENDER_OUT: u32 = 0u32;
pub const KSNODEPIN_DEMUX_IN: u32 = 0u32;
pub const KSNODEPIN_DEMUX_OUT: u32 = 1u32;
pub const KSNODEPIN_STANDARD_IN: u32 = 1u32;
pub const KSNODEPIN_STANDARD_OUT: u32 = 0u32;
pub const KSNODEPIN_SUM_MUX_IN: u32 = 1u32;
pub const KSNODEPIN_SUM_MUX_OUT: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY {
    pub Property: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl Default for KSNODEPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER {
    pub NodeProperty: KSNODEPROPERTY,
    pub ListenerId: *mut core::ffi::c_void,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
impl Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_3D_LISTENER {
    pub NodeProperty: KSNODEPROPERTY,
    pub ListenerId: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_CHANNEL {
    pub NodeProperty: KSNODEPROPERTY,
    pub Channel: i32,
    pub Reserved: u32,
}
impl Default for KSNODEPROPERTY_AUDIO_CHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    pub NodeProperty: KSNODEPROPERTY,
    pub DevSpecificId: u32,
    pub DeviceInfo: u32,
    pub Length: u32,
}
impl Default for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY {
    pub NodeProperty: KSNODEPROPERTY,
    pub AppContext: *mut core::ffi::c_void,
    pub Length: u32,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
impl Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct KSNODEPROPERTY_AUDIO_PROPERTY {
    pub NodeProperty: KSNODEPROPERTY,
    pub AppContext: *mut core::ffi::c_void,
    pub Length: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSNODETYPE_1394_DA_STREAM: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_1394_DV_STREAM_SOUNDTRACK: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_3D_EFFECTS: windows_core::GUID = windows_core::GUID::from_u128(0x55515860_c559_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_ADC: windows_core::GUID = windows_core::GUID::from_u128(0x4d837fe0_c555_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_AGC: windows_core::GUID = windows_core::GUID::from_u128(0xe88c9ba0_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_ANALOG_CONNECTOR: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_ANALOG_TAPE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_AUDIO_ENGINE: windows_core::GUID = windows_core::GUID::from_u128(0x35caf6e4_f3b3_4168_bb4b_55e77a461c7e);
pub const KSNODETYPE_AUDIO_KEYWORDDETECTOR: windows_core::GUID = windows_core::GUID::from_u128(0x3817e0b8_df58_4375_b669_c49634331f9d);
pub const KSNODETYPE_AUDIO_LOOPBACK: windows_core::GUID = windows_core::GUID::from_u128(0x8f42c0b2_91ce_4bcf_9ccd_0e599037ab35);
pub const KSNODETYPE_AUDIO_MODULE: windows_core::GUID = windows_core::GUID::from_u128(0x45aab42e_caeb_4052_8aa9_b38cb5109619);
pub const KSNODETYPE_BIDIRECTIONAL_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_CABLE_TUNER_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ee_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_CD_PLAYER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_CHORUS: windows_core::GUID = windows_core::GUID::from_u128(0x20173f20_c559_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_COMMUNICATION_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DAC: windows_core::GUID = windows_core::GUID::from_u128(0x507ae360_c554_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_DAT_IO_DIGITAL_AUDIO_TAPE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DCC_IO_DIGITAL_COMPACT_CASSETTE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DELAY: windows_core::GUID = windows_core::GUID::from_u128(0x144981e0_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_DEMUX: windows_core::GUID = windows_core::GUID::from_u128(0xc0eb67d4_e807_11d0_958a_00c04fb925d3);
pub const KSNODETYPE_DESKTOP_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DESKTOP_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DEV_SPECIFIC: windows_core::GUID = windows_core::GUID::from_u128(0x941c7ac0_c559_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_DIGITAL_AUDIO_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DISPLAYPORT_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xe47e4031_3ea6_418d_8f9b_b73843ccba97);
pub const KSNODETYPE_DOWN_LINE_PHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ee3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DRM_DESCRAMBLE: windows_core::GUID = windows_core::GUID::from_u128(0xffbb6e3f_ccfe_4d84_90d9_421418b03a8e);
pub const KSNODETYPE_DSS_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ef_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DVD_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220eb_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_DYN_RANGE_COMPRESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x08c8a6a8_601f_4af8_8793_d905ff4ca97d);
pub const KSNODETYPE_ECHO_CANCELING_SPEAKERPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_ECHO_SUPPRESSING_SPEAKERPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_EMBEDDED_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_EQUALIZATION_NOISE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_EQUALIZER: windows_core::GUID = windows_core::GUID::from_u128(0x9d41b4a0_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_EXTERNAL_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_FM_RX: windows_core::GUID = windows_core::GUID::from_u128(0x834a733c_f485_41c0_a62b_513025014e40);
pub const KSNODETYPE_HANDSET: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_HDMI_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xd1b9cc2a_f519_417f_91c9_55fa65481001);
pub const KSNODETYPE_HEADPHONES: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_HEADSET: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_HEAD_MOUNTED_DISPLAY_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_INPUT_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LEGACY_AUDIO_CONNECTOR: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LEVEL_CALIBRATION_NOISE_SOURCE: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LINE_CONNECTOR: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_LOUDNESS: windows_core::GUID = windows_core::GUID::from_u128(0x41887440_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_LOW_FREQUENCY_EFFECTS_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MICROPHONE_ARRAY: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MIDI_ELEMENT: windows_core::GUID = windows_core::GUID::from_u128(0x01c6fe66_6e48_4c65_ac9b_52db5d656c7e);
pub const KSNODETYPE_MIDI_JACK: windows_core::GUID = windows_core::GUID::from_u128(0x265e0c3f_fa39_4df3_ab04_be01b91e299a);
pub const KSNODETYPE_MINIDISK: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MULTITRACK_RECORDER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220f2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_MUTE: windows_core::GUID = windows_core::GUID::from_u128(0x02b223c0_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_MUX: windows_core::GUID = windows_core::GUID::from_u128(0x2ceaf780_c556_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_NOISE_SUPPRESS: windows_core::GUID = windows_core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const KSNODETYPE_OMNI_DIRECTIONAL_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_OUTPUT_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PARAMETRIC_EQUALIZER: windows_core::GUID = windows_core::GUID::from_u128(0x19bb3a6a_ce2b_4442_87ec_6727c3cab477);
pub const KSNODETYPE_PEAKMETER: windows_core::GUID = windows_core::GUID::from_u128(0xa085651e_5f0d_4b36_a869_d195d6ab4b9e);
pub const KSNODETYPE_PERSONAL_MICROPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PHONE_LINE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ee1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PHONOGRAPH: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e8_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PROCESSING_MICROPHONE_ARRAY: windows_core::GUID = windows_core::GUID::from_u128(0xdff21be6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_PROLOGIC_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0x831c2c80_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_PROLOGIC_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x8074c5b2_3c66_11d2_b45a_3078302c2030);
pub const KSNODETYPE_RADIO_RECEIVER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220f0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_RADIO_TRANSMITTER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220f1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_REVERB: windows_core::GUID = windows_core::GUID::from_u128(0xef0328e0_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_ROOM_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SATELLITE_RECEIVER_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ed_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPDIF_INTERFACE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21fe5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPEAKER: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ce1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPEAKERPHONE_NO_ECHO_REDUCTION: windows_core::GUID = windows_core::GUID::from_u128(0xdff21de3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_SPEAKERS_STATIC_JACK: windows_core::GUID = windows_core::GUID::from_u128(0x28e04f87_4dbe_4f8d_8589_025d209dfb4a);
pub const KSNODETYPE_SRC: windows_core::GUID = windows_core::GUID::from_u128(0x9db7b9e0_c555_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_STEREO_WIDE: windows_core::GUID = windows_core::GUID::from_u128(0xa9e69800_c558_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_SUM: windows_core::GUID = windows_core::GUID::from_u128(0xda441a60_c556_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_SUPERMIX: windows_core::GUID = windows_core::GUID::from_u128(0xe573adc0_c555_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_SYNTHESIZER: windows_core::GUID = windows_core::GUID::from_u128(0xdff220f3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_TELEPHONE: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ee2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_TELEPHONY_BIDI: windows_core::GUID = windows_core::GUID::from_u128(0x686d7cc0_d903_4258_b443_3a3d3580741c);
pub const KSNODETYPE_TELEPHONY_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xdff21ee0_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_TONE: windows_core::GUID = windows_core::GUID::from_u128(0x7607e580_c557_11d0_8a2b_00a0c9255ac1);
pub const KSNODETYPE_TV_TUNER_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ec_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_UPDOWN_MIX: windows_core::GUID = windows_core::GUID::from_u128(0xb7edc5cf_7b63_4ee2_a100_29ee2cb6b2de);
pub const KSNODETYPE_VCR_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220e9_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_CAMERA_TERMINAL: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e6_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_DISC_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xdff220ea_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_INPUT_MTT: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e7_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_INPUT_TERMINAL: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e2_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_OUTPUT_MTT: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e8_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_OUTPUT_TERMINAL: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e3_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_PROCESSING: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e5_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_SELECTOR: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e4_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VIDEO_STREAMING: windows_core::GUID = windows_core::GUID::from_u128(0xdff229e1_f70f_11d0_b917_00a0c9223196);
pub const KSNODETYPE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x3a5acc00_c557_11d0_8a2b_00a0c9255ac1);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSNODE_CREATE {
    pub CreateFlags: u32,
    pub Node: u32,
}
impl Default for KSNODE_CREATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSNOTIFICATIONID_AudioModule: windows_core::GUID = windows_core::GUID::from_u128(0x9c2220f0_d9a6_4d5c_a036_573857fd50d2);
pub const KSNOTIFICATIONID_SoundDetector: windows_core::GUID = windows_core::GUID::from_u128(0x6389d844_bb32_4c4c_a802_f4b4b77afead);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPEEKOPERATION(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPIN_CINSTANCES {
    pub PossibleCount: u32,
    pub CurrentCount: u32,
}
impl Default for KSPIN_CINSTANCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPIN_COMMUNICATION(pub i32);
pub const KSPIN_COMMUNICATION_BOTH: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(3i32);
pub const KSPIN_COMMUNICATION_BRIDGE: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(4i32);
pub const KSPIN_COMMUNICATION_NONE: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(0i32);
pub const KSPIN_COMMUNICATION_SINK: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(1i32);
pub const KSPIN_COMMUNICATION_SOURCE: KSPIN_COMMUNICATION = KSPIN_COMMUNICATION(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPIN_CONNECT {
    pub Interface: KSIDENTIFIER,
    pub Medium: KSIDENTIFIER,
    pub PinId: u32,
    pub PinToHandle: super::super::Foundation::HANDLE,
    pub Priority: KSPRIORITY,
}
impl Default for KSPIN_CONNECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPIN_DATAFLOW(pub i32);
pub const KSPIN_DATAFLOW_IN: KSPIN_DATAFLOW = KSPIN_DATAFLOW(1i32);
pub const KSPIN_DATAFLOW_OUT: KSPIN_DATAFLOW = KSPIN_DATAFLOW(2i32);
pub const KSPIN_FLAG_ASYNCHRONOUS_PROCESSING: u32 = 8u32;
pub const KSPIN_FLAG_CRITICAL_PROCESSING: u32 = 2u32;
pub const KSPIN_FLAG_DENY_USERMODE_ACCESS: u32 = 2147483648u32;
pub const KSPIN_FLAG_DISPATCH_LEVEL_PROCESSING: u32 = 1u32;
pub const KSPIN_FLAG_DISTINCT_TRAILING_EDGE: u32 = 512u32;
pub const KSPIN_FLAG_DO_NOT_INITIATE_PROCESSING: u32 = 16u32;
pub const KSPIN_FLAG_DO_NOT_USE_STANDARD_TRANSPORT: u32 = 524288u32;
pub const KSPIN_FLAG_ENFORCE_FIFO: u32 = 128u32;
pub const KSPIN_FLAG_FIXED_FORMAT: u32 = 1048576u32;
pub const KSPIN_FLAG_FRAMES_NOT_REQUIRED_FOR_PROCESSING: u32 = 64u32;
pub const KSPIN_FLAG_GENERATE_EOS_EVENTS: u32 = 2097152u32;
pub const KSPIN_FLAG_GENERATE_MAPPINGS: u32 = 256u32;
pub const KSPIN_FLAG_HYPERCRITICAL_PROCESSING: u32 = 4u32;
pub const KSPIN_FLAG_IMPLEMENT_CLOCK: u32 = 4194304u32;
pub const KSPIN_FLAG_INITIATE_PROCESSING_ON_EVERY_ARRIVAL: u32 = 32u32;
pub const KSPIN_FLAG_PROCESS_IF_ANY_IN_RUN_STATE: u32 = 16777216u32;
pub const KSPIN_FLAG_PROCESS_IN_RUN_STATE_ONLY: u32 = 65536u32;
pub const KSPIN_FLAG_SOME_FRAMES_REQUIRED_FOR_PROCESSING: u32 = 8388608u32;
pub const KSPIN_FLAG_SPLITTER: u32 = 131072u32;
pub const KSPIN_FLAG_USE_STANDARD_TRANSPORT: u32 = 262144u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPIN_MDL_CACHING_EVENT(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPIN_MDL_CACHING_NOTIFICATION {
    pub Event: KSPIN_MDL_CACHING_EVENT,
    pub Buffer: *mut core::ffi::c_void,
}
impl Default for KSPIN_MDL_CACHING_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPIN_MDL_CACHING_NOTIFICATION32 {
    pub Event: KSPIN_MDL_CACHING_EVENT,
    pub Buffer: u32,
}
impl Default for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPIN_MDL_CACHING_NOTIFY_ADDSAMPLE: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(3i32);
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_NOWAIT: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(2i32);
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_WAIT: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(1i32);
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANUP: KSPIN_MDL_CACHING_EVENT = KSPIN_MDL_CACHING_EVENT(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPIN_PHYSICALCONNECTION {
    pub Size: u32,
    pub Pin: u32,
    pub SymbolicLinkName: [u16; 1],
}
impl Default for KSPIN_PHYSICALCONNECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPPROPERTY_ALLOCATOR_MDLCACHING(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPRIORITY {
    pub PriorityClass: u32,
    pub PrioritySubClass: u32,
}
impl Default for KSPRIORITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPRIORITY_EXCLUSIVE: u32 = 4294967295u32;
pub const KSPRIORITY_HIGH: u32 = 2147483648u32;
pub const KSPRIORITY_LOW: u32 = 1u32;
pub const KSPRIORITY_NORMAL: u32 = 1073741824u32;
pub const KSPROBE_ALLOCATEMDL: u32 = 16u32;
pub const KSPROBE_ALLOWFORMATCHANGE: u32 = 128u32;
pub const KSPROBE_MODIFY: u32 = 512u32;
pub const KSPROBE_PROBEANDLOCK: u32 = 32u32;
pub const KSPROBE_STREAMREAD: u32 = 0u32;
pub const KSPROBE_STREAMWRITE: u32 = 1u32;
pub const KSPROBE_SYSTEMADDRESS: u32 = 64u32;
pub const KSPROPERTYSETID_ExtendedCameraControl: windows_core::GUID = windows_core::GUID::from_u128(0x1cb79112_c0d2_4213_9ca6_cd4fdb927972);
pub const KSPROPERTYSETID_NetworkCameraControl: windows_core::GUID = windows_core::GUID::from_u128(0x0e780f09_5745_4e3a_bc9f_f226ea43a6ec);
pub const KSPROPERTYSETID_PerFrameSettingControl: windows_core::GUID = windows_core::GUID::from_u128(0xf1f3e261_dee6_4537_bff5_ee206db54aac);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_AC3(pub i32);
pub const KSPROPERTY_AC3_ALTERNATE_AUDIO: KSPROPERTY_AC3 = KSPROPERTY_AC3(2i32);
pub const KSPROPERTY_AC3_BIT_STREAM_MODE: KSPROPERTY_AC3 = KSPROPERTY_AC3(4i32);
pub const KSPROPERTY_AC3_DIALOGUE_LEVEL: KSPROPERTY_AC3 = KSPROPERTY_AC3(5i32);
pub const KSPROPERTY_AC3_DOWNMIX: KSPROPERTY_AC3 = KSPROPERTY_AC3(3i32);
pub const KSPROPERTY_AC3_ERROR_CONCEALMENT: KSPROPERTY_AC3 = KSPROPERTY_AC3(1i32);
pub const KSPROPERTY_AC3_LANGUAGE_CODE: KSPROPERTY_AC3 = KSPROPERTY_AC3(6i32);
pub const KSPROPERTY_AC3_ROOM_TYPE: KSPROPERTY_AC3 = KSPROPERTY_AC3(7i32);
pub const KSPROPERTY_ALLOCATOR_CLEANUP_CACHEDMDLPAGES: KSPPROPERTY_ALLOCATOR_MDLCACHING = KSPPROPERTY_ALLOCATOR_MDLCACHING(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL(pub i32);
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    pub InterleavedCapSupported: u32,
}
impl Default for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    pub InterleavedCapPossible: u32,
}
impl Default for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_ALLOCATOR_CONTROL_HONOR_COUNT: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(0i32);
pub const KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE: KSPROPERTY_ALLOCATOR_CONTROL = KSPROPERTY_ALLOCATOR_CONTROL(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    pub CX: u32,
    pub CY: u32,
}
impl Default for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_ATN_READER: KSPROPERTY_TIMECODE = KSPROPERTY_TIMECODE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_AUDDECOUT(pub i32);
pub const KSPROPERTY_AUDDECOUT_CUR_MODE: KSPROPERTY_AUDDECOUT = KSPROPERTY_AUDDECOUT(1i32);
pub const KSPROPERTY_AUDDECOUT_MODES: KSPROPERTY_AUDDECOUT = KSPROPERTY_AUDDECOUT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_AUDIO(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_AUDIOENGINE(pub i32);
pub const KSPROPERTY_AUDIOENGINE_BUFFER_SIZE_RANGE: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(7i32);
pub const KSPROPERTY_AUDIOENGINE_DESCRIPTOR: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(6i32);
pub const KSPROPERTY_AUDIOENGINE_DEVICECONTROLS: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(10i32);
pub const KSPROPERTY_AUDIOENGINE_DEVICEFORMAT: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(4i32);
pub const KSPROPERTY_AUDIOENGINE_GFXENABLE: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(1i32);
pub const KSPROPERTY_AUDIOENGINE_LFXENABLE: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(0i32);
pub const KSPROPERTY_AUDIOENGINE_LOOPBACK_PROTECTION: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(8i32);
pub const KSPROPERTY_AUDIOENGINE_MIXFORMAT: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(2i32);
pub const KSPROPERTY_AUDIOENGINE_SUPPORTEDDEVICEFORMATS: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(5i32);
pub const KSPROPERTY_AUDIOENGINE_VOLUMELEVEL: KSPROPERTY_AUDIOENGINE = KSPROPERTY_AUDIOENGINE(9i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_AUDIOMODULE(pub i32);
pub const KSPROPERTY_AUDIOMODULE_COMMAND: KSPROPERTY_AUDIOMODULE = KSPROPERTY_AUDIOMODULE(2i32);
pub const KSPROPERTY_AUDIOMODULE_DESCRIPTORS: KSPROPERTY_AUDIOMODULE = KSPROPERTY_AUDIOMODULE(1i32);
pub const KSPROPERTY_AUDIOMODULE_NOTIFICATION_DEVICE_ID: KSPROPERTY_AUDIOMODULE = KSPROPERTY_AUDIOMODULE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_AUDIOPOSTURE(pub i32);
pub const KSPROPERTY_AUDIOPOSTURE_ORIENTATION: KSPROPERTY_AUDIOPOSTURE = KSPROPERTY_AUDIOPOSTURE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_AUDIORESOURCEMANAGEMENT(pub i32);
pub const KSPROPERTY_AUDIORESOURCEMANAGEMENT_RESOURCEGROUP: KSPROPERTY_AUDIORESOURCEMANAGEMENT = KSPROPERTY_AUDIORESOURCEMANAGEMENT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_AUDIOSIGNALPROCESSING(pub i32);
pub const KSPROPERTY_AUDIOSIGNALPROCESSING_MODES: KSPROPERTY_AUDIOSIGNALPROCESSING = KSPROPERTY_AUDIOSIGNALPROCESSING(0i32);
pub const KSPROPERTY_AUDIO_3D_INTERFACE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(36i32);
pub const KSPROPERTY_AUDIO_AGC: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(21i32);
pub const KSPROPERTY_AUDIO_ALGORITHM_INSTANCE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(38i32);
pub const KSPROPERTY_AUDIO_BASS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(14i32);
pub const KSPROPERTY_AUDIO_BASS_BOOST: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(17i32);
pub const KSPROPERTY_AUDIO_BUFFER_DURATION: u32 = 1u32;
pub const KSPROPERTY_AUDIO_CHANNEL_CONFIG: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(3i32);
pub const KSPROPERTY_AUDIO_CHORUS_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(27i32);
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_DEPTH: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(47i32);
pub const KSPROPERTY_AUDIO_CHORUS_MODULATION_RATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(46i32);
pub const KSPROPERTY_AUDIO_COPY_PROTECTION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(2i32);
pub const KSPROPERTY_AUDIO_CPU_RESOURCES: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(33i32);
pub const KSPROPERTY_AUDIO_DELAY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(22i32);
pub const KSPROPERTY_AUDIO_DEMUX_DEST: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(29i32);
pub const KSPROPERTY_AUDIO_DEV_SPECIFIC: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(28i32);
pub const KSPROPERTY_AUDIO_DYNAMIC_RANGE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(6i32);
pub const KSPROPERTY_AUDIO_DYNAMIC_SAMPLING_RATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(9i32);
pub const KSPROPERTY_AUDIO_EQ_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(20i32);
pub const KSPROPERTY_AUDIO_EQ_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(18i32);
pub const KSPROPERTY_AUDIO_FILTER_STATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(39i32);
pub const KSPROPERTY_AUDIO_LATENCY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(1i32);
pub const KSPROPERTY_AUDIO_LINEAR_BUFFER_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(54i32);
pub const KSPROPERTY_AUDIO_LOUDNESS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(23i32);
pub const KSPROPERTY_AUDIO_MANUFACTURE_GUID: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(31i32);
pub const KSPROPERTY_AUDIO_MIC_ARRAY_GEOMETRY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(51i32);
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(58i32);
pub const KSPROPERTY_AUDIO_MIC_SENSITIVITY2: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(60i32);
pub const KSPROPERTY_AUDIO_MIC_SNR: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(59i32);
pub const KSPROPERTY_AUDIO_MID: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(15i32);
pub const KSPROPERTY_AUDIO_MIX_LEVEL_CAPS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(11i32);
pub const KSPROPERTY_AUDIO_MIX_LEVEL_TABLE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(10i32);
pub const KSPROPERTY_AUDIO_MUTE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(13i32);
pub const KSPROPERTY_AUDIO_MUX_SOURCE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(12i32);
pub const KSPROPERTY_AUDIO_NUM_EQ_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(19i32);
pub const KSPROPERTY_AUDIO_PEAKMETER: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(37i32);
pub const KSPROPERTY_AUDIO_PEAKMETER2: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(55i32);
pub const KSPROPERTY_AUDIO_PEQ_BAND_CENTER_FREQ: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(43i32);
pub const KSPROPERTY_AUDIO_PEQ_BAND_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(45i32);
pub const KSPROPERTY_AUDIO_PEQ_BAND_Q_FACTOR: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(44i32);
pub const KSPROPERTY_AUDIO_PEQ_MAX_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(41i32);
pub const KSPROPERTY_AUDIO_PEQ_NUM_BANDS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(42i32);
pub const KSPROPERTY_AUDIO_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(5i32);
pub const KSPROPERTY_AUDIO_POSITIONEX: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(50i32);
pub const KSPROPERTY_AUDIO_PREFERRED_STATUS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(40i32);
pub const KSPROPERTY_AUDIO_PRESENTATION_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(52i32);
pub const KSPROPERTY_AUDIO_PRODUCT_GUID: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(32i32);
pub const KSPROPERTY_AUDIO_QUALITY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(7i32);
pub const KSPROPERTY_AUDIO_REVERB_DELAY_FEEDBACK: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(49i32);
pub const KSPROPERTY_AUDIO_REVERB_LEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(26i32);
pub const KSPROPERTY_AUDIO_REVERB_TIME: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(48i32);
pub const KSPROPERTY_AUDIO_SAMPLING_RATE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(8i32);
pub const KSPROPERTY_AUDIO_STEREO_ENHANCE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(30i32);
pub const KSPROPERTY_AUDIO_STEREO_SPEAKER_GEOMETRY: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(34i32);
pub const KSPROPERTY_AUDIO_SURROUND_ENCODE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(35i32);
pub const KSPROPERTY_AUDIO_TREBLE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(16i32);
pub const KSPROPERTY_AUDIO_VOLUMELEVEL: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(4i32);
pub const KSPROPERTY_AUDIO_VOLUMELIMIT_ENGAGED: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(57i32);
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_LASTBUFFER_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(56i32);
pub const KSPROPERTY_AUDIO_WAVERT_CURRENT_WRITE_POSITION: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(53i32);
pub const KSPROPERTY_AUDIO_WIDENESS: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(25i32);
pub const KSPROPERTY_AUDIO_WIDE_MODE: KSPROPERTY_AUDIO = KSPROPERTY_AUDIO(24i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_BIBLIOGRAPHIC(pub i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYGEOGRAPHIC: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825570848i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808465952i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYRELATED: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808728352i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808727584i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYTOPICALTERM: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808793632i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDENTRYUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808662816i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ADDEDFORMAVAILABLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808662304i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_AWARDS: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(909653280i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_BIBLIOGRAPHYNOTE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(875574560i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_CATALOGINGSOURCE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808726560i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_CITATION: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808531232i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_CONTENTSNOTE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(892351776i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_CREATIONCREDIT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(942683424i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_GENERALNOTE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808465696i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMCURRICULUM: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(943011360i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_INDEXTERMGENRE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(892679712i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ISBN: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808595488i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_ISSN: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(842149920i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_LCCN: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808529952i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_LEADER: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(1380207648i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINCORPORATEBODY: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808530208i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINMEETINGNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825307424i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808464672i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_MAINUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808661280i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_PARTICIPANT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825308448i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_PHYSICALDESCRIPTION: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808465184i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_PUBLICATION: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808858144i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(809055264i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTPERSONALNAME: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808466464i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SERIESSTATEMENTUNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808663072i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SUMMARY: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808596768i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_SYSTEMDETAILS: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(942880032i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_TARGETAUDIENCE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(825373984i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_TITLESTATEMENT: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(892613152i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_UNIFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(808727072i32);
pub const KSPROPERTY_BIBLIOGRAPHIC_VARYINGFORMTITLE: KSPROPERTY_BIBLIOGRAPHIC = KSPROPERTY_BIBLIOGRAPHIC(909390368i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSPROPERTY_BOUNDS_LONG {
    pub Anonymous1: KSPROPERTY_BOUNDS_LONG_0,
    pub Anonymous2: KSPROPERTY_BOUNDS_LONG_1,
}
impl Default for KSPROPERTY_BOUNDS_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_BOUNDS_LONG_0 {
    pub SignedMinimum: i32,
    pub SignedMaximum: i32,
}
impl Default for KSPROPERTY_BOUNDS_LONG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_BOUNDS_LONG_1 {
    pub UnsignedMinimum: u32,
    pub UnsignedMaximum: u32,
}
impl Default for KSPROPERTY_BOUNDS_LONG_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSPROPERTY_BOUNDS_LONGLONG {
    pub Anonymous1: KSPROPERTY_BOUNDS_LONGLONG_0,
    pub Anonymous2: KSPROPERTY_BOUNDS_LONGLONG_1,
}
impl Default for KSPROPERTY_BOUNDS_LONGLONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_BOUNDS_LONGLONG_0 {
    pub SignedMinimum: i64,
    pub SignedMaximum: i64,
}
impl Default for KSPROPERTY_BOUNDS_LONGLONG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_BOUNDS_LONGLONG_1 {
    pub UnsignedMinimum: u64,
    pub UnsignedMaximum: u64,
}
impl Default for KSPROPERTY_BOUNDS_LONGLONG_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_BTAUDIO(pub i32);
pub const KSPROPERTY_CAMERACONTROL_AUTO_EXPOSURE_PRIORITY: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(19i32);
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(4i32);
pub const KSPROPERTY_CAMERACONTROL_EXPOSURE_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(14i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ADVANCEDPHOTO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(33i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_BACKGROUNDSEGMENTATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(41i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_CAMERAANGLEOFFSET: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(17i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(43i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_DIGITALWINDOW_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(42i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(44i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_END2: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(44i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EVCOMPENSATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(16i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EXPOSUREMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(12i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_EYEGAZECORRECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(40i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEAUTH_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(35i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FACEDETECTION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(29i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FIELDOFVIEW: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(15i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FLASHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(9i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(13i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSPRIORITY: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(19i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_FOCUSSTATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(20i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_HISTOGRAM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(31i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_IRTORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(38i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(14i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ISO_ADVANCED: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(26i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MAXVIDFPS_PHOTORES: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(5i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_MCC: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(25i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_METADATA: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(18i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OIS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(32i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_OPTIMIZATIONHINT: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(10i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOCONFIRMATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(23i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(1i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMAXFRAMERATE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(2i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(0i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTHUMBNAIL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(6i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PHOTOTRIGGERTIME: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(3i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_PROFILE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(34i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(pub i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_RELATIVEPANELOPTIMIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(39i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_CONFIGCAPS: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(21i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ROI_ISPCONTROL: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(22i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SCENEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(7i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_SECURE_MODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(36i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_TORCHMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(8i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VFR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(28i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOHDR: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(30i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOSTABILIZATION: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(27i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_VIDEOTEMPORALDENOISING: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(37i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WARMSTART: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(4i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_WHITEBALANCEMODE: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(11i32);
pub const KSPROPERTY_CAMERACONTROL_EXTENDED_ZOOM: KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY = KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY(24i32);
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ABSOLUTE: i32 = 0i32;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_ASYNCHRONOUS: i32 = 4i32;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_AUTO: i32 = 1i32;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_MANUAL: i32 = 2i32;
pub const KSPROPERTY_CAMERACONTROL_FLAGS_RELATIVE: i32 = 16i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_FLASH(pub i32);
pub const KSPROPERTY_CAMERACONTROL_FLASH_AUTO: i32 = 2i32;
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_AUTO: i32 = 1i32;
pub const KSPROPERTY_CAMERACONTROL_FLASH_FLAGS_MANUAL: i32 = 2i32;
pub const KSPROPERTY_CAMERACONTROL_FLASH_OFF: i32 = 0i32;
pub const KSPROPERTY_CAMERACONTROL_FLASH_ON: i32 = 1i32;
pub const KSPROPERTY_CAMERACONTROL_FLASH_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_FLASH = KSPROPERTY_CAMERACONTROL_FLASH(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_FLASH_S {
    pub Flash: u32,
    pub Capabilities: u32,
}
impl Default for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(18i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    pub Property: KSIDENTIFIER,
    pub lOcularFocalLength: i32,
    pub lObjectiveFocalLengthMin: i32,
    pub lObjectiveFocalLengthMax: i32,
}
impl Default for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_FOCUS: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(6i32);
pub const KSPROPERTY_CAMERACONTROL_FOCUS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY(pub i32);
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_EXCLUSIVE_WITH_RECORD: i32 = 1i32;
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY = KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    pub Capabilities: u32,
    pub Reserved0: u32,
}
impl Default for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_SEQUENCE_EXCLUSIVE_WITH_RECORD: i32 = 2i32;
pub const KSPROPERTY_CAMERACONTROL_IRIS: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(5i32);
pub const KSPROPERTY_CAMERACONTROL_IRIS_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(15i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    pub NodeProperty: KSNODEPROPERTY,
    pub lOcularFocalLength: i32,
    pub lObjectiveFocalLengthMin: i32,
    pub lObjectiveFocalLengthMax: i32,
}
impl Default for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl Default for KSPROPERTY_CAMERACONTROL_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_NODE_S2 {
    pub NodeProperty: KSP_NODE,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl Default for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_PAN: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(0i32);
pub const KSPROPERTY_CAMERACONTROL_PANTILT: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(9i32);
pub const KSPROPERTY_CAMERACONTROL_PANTILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(17i32);
pub const KSPROPERTY_CAMERACONTROL_PAN_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(10i32);
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CAPABILITY: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(0i32);
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_CLEAR: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(pub i32);
pub const KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_SET: KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY = KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY(1i32);
pub const KSPROPERTY_CAMERACONTROL_PRIVACY: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST(pub i32);
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_EXPOSURE: i32 = 512i32;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_FOCUS: i32 = 256i32;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONFIG_WB: i32 = 1024i32;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_CONVERGEMODE: i32 = 1073741824i32;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_ASYNC: i32 = -2147483648i32;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_AUTO: i32 = 1i32;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_FLAGS_MANUAL: i32 = 2i32;
pub const KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST = KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    pub FocusRect: super::super::Foundation::RECT,
    pub AutoFocusLock: super::super::Foundation::BOOL,
    pub AutoExposureLock: super::super::Foundation::BOOL,
    pub AutoWhitebalanceLock: super::super::Foundation::BOOL,
    pub Anonymous: KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0,
}
impl Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    pub Capabilities: u32,
    pub Configuration: u32,
}
impl Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_ROLL: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(2i32);
pub const KSPROPERTY_CAMERACONTROL_ROLL_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(12i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl Default for KSPROPERTY_CAMERACONTROL_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_S2 {
    pub Property: KSIDENTIFIER,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl Default for KSPROPERTY_CAMERACONTROL_S2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_SCANMODE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(7i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CAMERACONTROL_S_EX {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub FocusRect: super::super::Foundation::RECT,
}
impl Default for KSPROPERTY_CAMERACONTROL_S_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CAMERACONTROL_TILT: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(1i32);
pub const KSPROPERTY_CAMERACONTROL_TILT_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(11i32);
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_AUTO: i32 = 4i32;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_AUTO: i32 = 1i32;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_FLAGS_MANUAL: i32 = 2i32;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_HIGH: i32 = 1i32;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_LOW: i32 = 3i32;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_MEDIUM: i32 = 2i32;
pub const KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_OFF: i32 = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    pub VideoStabilizationMode: u32,
    pub Capabilities: u32,
}
impl Default for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE(pub i32);
pub const KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE_PROPERTY_ID: KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE = KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE(0i32);
pub const KSPROPERTY_CAMERACONTROL_ZOOM: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(3i32);
pub const KSPROPERTY_CAMERACONTROL_ZOOM_RELATIVE: KSPROPERTY_VIDCAP_CAMERACONTROL = KSPROPERTY_VIDCAP_CAMERACONTROL(13i32);
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_CLEAR: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(pub i32);
pub const KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_SET: KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS = KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CLOCK(pub i32);
pub const KSPROPERTY_CLOCK_CORRELATEDPHYSICALTIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(3i32);
pub const KSPROPERTY_CLOCK_CORRELATEDTIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(2i32);
pub const KSPROPERTY_CLOCK_PHYSICALTIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(1i32);
pub const KSPROPERTY_CLOCK_RESOLUTION: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(4i32);
pub const KSPROPERTY_CLOCK_STATE: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(5i32);
pub const KSPROPERTY_CLOCK_TIME: KSPROPERTY_CLOCK = KSPROPERTY_CLOCK(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CONNECTION(pub i32);
pub const KSPROPERTY_CONNECTION_ACQUIREORDERING: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(5i32);
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(3i32);
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING_EX: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(6i32);
pub const KSPROPERTY_CONNECTION_DATAFORMAT: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(2i32);
pub const KSPROPERTY_CONNECTION_PRIORITY: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(1i32);
pub const KSPROPERTY_CONNECTION_PROPOSEDATAFORMAT: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(4i32);
pub const KSPROPERTY_CONNECTION_STARTAT: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(7i32);
pub const KSPROPERTY_CONNECTION_STATE: KSPROPERTY_CONNECTION = KSPROPERTY_CONNECTION(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_COPYPROT(pub i32);
pub const KSPROPERTY_COPY_MACROVISION: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(5i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CROSSBAR_ACTIVE_S {
    pub Property: KSIDENTIFIER,
    pub IndexInputPin: u32,
    pub Active: u32,
}
impl Default for KSPROPERTY_CROSSBAR_ACTIVE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CROSSBAR_CAN_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(2i32);
pub const KSPROPERTY_CROSSBAR_CAPS: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CROSSBAR_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub NumberOfInputs: u32,
    pub NumberOfOutputs: u32,
}
impl Default for KSPROPERTY_CROSSBAR_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CROSSBAR_INPUT_ACTIVE: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(4i32);
pub const KSPROPERTY_CROSSBAR_PININFO: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CROSSBAR_PININFO_S {
    pub Property: KSIDENTIFIER,
    pub Direction: KSPIN_DATAFLOW,
    pub Index: u32,
    pub PinType: u32,
    pub RelatedPinIndex: u32,
    pub Medium: KSIDENTIFIER,
}
impl Default for KSPROPERTY_CROSSBAR_PININFO_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CROSSBAR_ROUTE: KSPROPERTY_VIDCAP_CROSSBAR = KSPROPERTY_VIDCAP_CROSSBAR(3i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_CROSSBAR_ROUTE_S {
    pub Property: KSIDENTIFIER,
    pub IndexInputPin: u32,
    pub IndexOutputPin: u32,
    pub CanRoute: u32,
}
impl Default for KSPROPERTY_CROSSBAR_ROUTE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_CURRENT_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_CYCLIC(pub i32);
pub const KSPROPERTY_CYCLIC_POSITION: KSPROPERTY_CYCLIC = KSPROPERTY_CYCLIC(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_DESCRIPTION {
    pub AccessFlags: u32,
    pub DescriptionSize: u32,
    pub PropTypeSet: KSIDENTIFIER,
    pub MembersListCount: u32,
    pub Reserved: u32,
}
impl Default for KSPROPERTY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_DIRECTSOUND3DBUFFER(pub i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_ALL: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(0i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEANGLES: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(3i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEORIENTATION: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(4i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_CONEOUTSIDEVOLUME: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(5i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MAXDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(7i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MINDISTANCE: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(6i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_MODE: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(8i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_POSITION: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(1i32);
pub const KSPROPERTY_DIRECTSOUND3DBUFFER_VELOCITY: KSPROPERTY_DIRECTSOUND3DBUFFER = KSPROPERTY_DIRECTSOUND3DBUFFER(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_DIRECTSOUND3DLISTENER(pub i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALL: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(0i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ALLOCATION: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(8i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_BATCH: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(7i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DISTANCEFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(4i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_DOPPLERFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(6i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ORIENTATION: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(3i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_POSITION: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(1i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_ROLLOFFFACTOR: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(5i32);
pub const KSPROPERTY_DIRECTSOUND3DLISTENER_VELOCITY: KSPROPERTY_DIRECTSOUND3DLISTENER = KSPROPERTY_DIRECTSOUND3DLISTENER(2i32);
pub const KSPROPERTY_DISPLAY_ADAPTER_GUID: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_DRMAUDIOSTREAM(pub i32);
pub const KSPROPERTY_DRMAUDIOSTREAM_CONTENTID: KSPROPERTY_DRMAUDIOSTREAM = KSPROPERTY_DRMAUDIOSTREAM(0i32);
pub const KSPROPERTY_DROPPEDFRAMES_CURRENT: KSPROPERTY_VIDCAP_DROPPEDFRAMES = KSPROPERTY_VIDCAP_DROPPEDFRAMES(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    pub Property: KSIDENTIFIER,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub AverageFrameSize: u32,
}
impl Default for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_DVDCOPY_CHLG_KEY: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(1i32);
pub const KSPROPERTY_DVDCOPY_DEC_KEY2: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(3i32);
pub const KSPROPERTY_DVDCOPY_DISC_KEY: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(128i32);
pub const KSPROPERTY_DVDCOPY_DVD_KEY1: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(2i32);
pub const KSPROPERTY_DVDCOPY_REGION: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(6i32);
pub const KSPROPERTY_DVDCOPY_SET_COPY_STATE: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(7i32);
pub const KSPROPERTY_DVDCOPY_TITLE_KEY: KSPROPERTY_COPYPROT = KSPROPERTY_COPYPROT(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_DVDSUBPIC(pub i32);
pub const KSPROPERTY_DVDSUBPIC_COMPOSIT_ON: KSPROPERTY_DVDSUBPIC = KSPROPERTY_DVDSUBPIC(2i32);
pub const KSPROPERTY_DVDSUBPIC_HLI: KSPROPERTY_DVDSUBPIC = KSPROPERTY_DVDSUBPIC(1i32);
pub const KSPROPERTY_DVDSUBPIC_PALETTE: KSPROPERTY_DVDSUBPIC = KSPROPERTY_DVDSUBPIC(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_EXTDEVICE(pub i32);
pub const KSPROPERTY_EXTDEVICE_CAPABILITIES: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(4i32);
pub const KSPROPERTY_EXTDEVICE_ID: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(0i32);
pub const KSPROPERTY_EXTDEVICE_PORT: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(3i32);
pub const KSPROPERTY_EXTDEVICE_POWER_STATE: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_EXTDEVICE_S {
    pub Property: KSIDENTIFIER,
    pub u: KSPROPERTY_EXTDEVICE_S_0,
}
impl Default for KSPROPERTY_EXTDEVICE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSPROPERTY_EXTDEVICE_S_0 {
    pub Capabilities: DEVCAPS,
    pub DevPort: u32,
    pub PowerState: u32,
    pub pawchString: [u16; 260],
    pub NodeUniqueID: [u32; 2],
}
impl Default for KSPROPERTY_EXTDEVICE_S_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_EXTDEVICE_VERSION: KSPROPERTY_EXTDEVICE = KSPROPERTY_EXTDEVICE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_EXTENSION_UNIT(pub i32);
pub const KSPROPERTY_EXTENSION_UNIT_CONTROL: KSPROPERTY_EXTENSION_UNIT = KSPROPERTY_EXTENSION_UNIT(1i32);
pub const KSPROPERTY_EXTENSION_UNIT_INFO: KSPROPERTY_EXTENSION_UNIT = KSPROPERTY_EXTENSION_UNIT(0i32);
pub const KSPROPERTY_EXTENSION_UNIT_PASS_THROUGH: KSPROPERTY_EXTENSION_UNIT = KSPROPERTY_EXTENSION_UNIT(65535i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_EXTXPORT(pub i32);
pub const KSPROPERTY_EXTXPORT_ATN_SEARCH: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(8i32);
pub const KSPROPERTY_EXTXPORT_CAPABILITIES: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(0i32);
pub const KSPROPERTY_EXTXPORT_INPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(1i32);
pub const KSPROPERTY_EXTXPORT_LOAD_MEDIUM: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(3i32);
pub const KSPROPERTY_EXTXPORT_MEDIUM_INFO: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(4i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_EXTXPORT_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub u: KSPROPERTY_EXTXPORT_NODE_S_0,
}
impl Default for KSPROPERTY_EXTXPORT_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSPROPERTY_EXTXPORT_NODE_S_0 {
    pub Capabilities: u32,
    pub SignalMode: u32,
    pub LoadMedium: u32,
    pub MediumInfo: MEDIUM_INFO,
    pub XPrtState: TRANSPORT_STATE,
    pub Timecode: KSPROPERTY_EXTXPORT_NODE_S_0_0,
    pub dwTimecode: u32,
    pub dwAbsTrackNumber: u32,
    pub RawAVC: KSPROPERTY_EXTXPORT_NODE_S_0_1,
}
impl Default for KSPROPERTY_EXTXPORT_NODE_S_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    pub PayloadSize: u32,
    pub Payload: [u8; 512],
}
impl Default for KSPROPERTY_EXTXPORT_NODE_S_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
impl Default for KSPROPERTY_EXTXPORT_NODE_S_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_EXTXPORT_OUTPUT_SIGNAL_MODE: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(2i32);
pub const KSPROPERTY_EXTXPORT_RTC_SEARCH: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(9i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_EXTXPORT_S {
    pub Property: KSIDENTIFIER,
    pub u: KSPROPERTY_EXTXPORT_S_0,
}
impl Default for KSPROPERTY_EXTXPORT_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSPROPERTY_EXTXPORT_S_0 {
    pub Capabilities: u32,
    pub SignalMode: u32,
    pub LoadMedium: u32,
    pub MediumInfo: MEDIUM_INFO,
    pub XPrtState: TRANSPORT_STATE,
    pub Timecode: KSPROPERTY_EXTXPORT_S_0_0,
    pub dwTimecode: u32,
    pub dwAbsTrackNumber: u32,
    pub RawAVC: KSPROPERTY_EXTXPORT_S_0_1,
}
impl Default for KSPROPERTY_EXTXPORT_S_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_EXTXPORT_S_0_1 {
    pub PayloadSize: u32,
    pub Payload: [u8; 512],
}
impl Default for KSPROPERTY_EXTXPORT_S_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_EXTXPORT_S_0_0 {
    pub frame: u8,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}
impl Default for KSPROPERTY_EXTXPORT_S_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_EXTXPORT_STATE: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(5i32);
pub const KSPROPERTY_EXTXPORT_STATE_NOTIFY: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(6i32);
pub const KSPROPERTY_EXTXPORT_TIMECODE_SEARCH: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(7i32);
pub const KSPROPERTY_FMRX_ANTENNAENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = KSPROPERTY_FMRX_TOPOLOGY(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_FMRX_CONTROL(pub i32);
pub const KSPROPERTY_FMRX_ENDPOINTID: KSPROPERTY_FMRX_TOPOLOGY = KSPROPERTY_FMRX_TOPOLOGY(0i32);
pub const KSPROPERTY_FMRX_STATE: KSPROPERTY_FMRX_CONTROL = KSPROPERTY_FMRX_CONTROL(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_FMRX_TOPOLOGY(pub i32);
pub const KSPROPERTY_FMRX_VOLUME: KSPROPERTY_FMRX_TOPOLOGY = KSPROPERTY_FMRX_TOPOLOGY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_GENERAL(pub i32);
pub const KSPROPERTY_GENERAL_COMPONENTID: KSPROPERTY_GENERAL = KSPROPERTY_GENERAL(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_HRTF3D(pub i32);
pub const KSPROPERTY_HRTF3D_FILTER_FORMAT: KSPROPERTY_HRTF3D = KSPROPERTY_HRTF3D(2i32);
pub const KSPROPERTY_HRTF3D_INITIALIZE: KSPROPERTY_HRTF3D = KSPROPERTY_HRTF3D(1i32);
pub const KSPROPERTY_HRTF3D_PARAMS: KSPROPERTY_HRTF3D = KSPROPERTY_HRTF3D(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_INTERLEAVEDAUDIO(pub i32);
pub const KSPROPERTY_INTERLEAVEDAUDIO_FORMATINFORMATION: KSPROPERTY_INTERLEAVEDAUDIO = KSPROPERTY_INTERLEAVEDAUDIO(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_ITD3D(pub i32);
pub const KSPROPERTY_ITD3D_PARAMS: KSPROPERTY_ITD3D = KSPROPERTY_ITD3D(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_JACK(pub i32);
pub const KSPROPERTY_JACK_CONTAINERID: KSPROPERTY_JACK = KSPROPERTY_JACK(4i32);
pub const KSPROPERTY_JACK_DESCRIPTION: KSPROPERTY_JACK = KSPROPERTY_JACK(1i32);
pub const KSPROPERTY_JACK_DESCRIPTION2: KSPROPERTY_JACK = KSPROPERTY_JACK(2i32);
pub const KSPROPERTY_JACK_DESCRIPTION3: KSPROPERTY_JACK = KSPROPERTY_JACK(5i32);
pub const KSPROPERTY_JACK_SINK_INFO: KSPROPERTY_JACK = KSPROPERTY_JACK(3i32);
pub const KSPROPERTY_MAP_CAPTURE_HANDLE_TO_VRAM_ADDRESS: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_MEDIAAVAILABLE {
    pub Earliest: i64,
    pub Latest: i64,
}
impl Default for KSPROPERTY_MEDIAAVAILABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_MEDIASEEKING(pub i32);
pub const KSPROPERTY_MEDIASEEKING_AVAILABLE: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(7i32);
pub const KSPROPERTY_MEDIASEEKING_CAPABILITIES: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(0i32);
pub const KSPROPERTY_MEDIASEEKING_CONVERTTIMEFORMAT: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(9i32);
pub const KSPROPERTY_MEDIASEEKING_DURATION: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(6i32);
pub const KSPROPERTY_MEDIASEEKING_FORMATS: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(1i32);
pub const KSPROPERTY_MEDIASEEKING_POSITION: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(3i32);
pub const KSPROPERTY_MEDIASEEKING_POSITIONS: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(5i32);
pub const KSPROPERTY_MEDIASEEKING_PREROLL: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(8i32);
pub const KSPROPERTY_MEDIASEEKING_STOPPOSITION: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(4i32);
pub const KSPROPERTY_MEDIASEEKING_TIMEFORMAT: KSPROPERTY_MEDIASEEKING = KSPROPERTY_MEDIASEEKING(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_MEMBERSHEADER {
    pub MembersFlags: u32,
    pub MembersSize: u32,
    pub MembersCount: u32,
    pub Flags: u32,
}
impl Default for KSPROPERTY_MEMBERSHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_MEMBER_FLAG_BASICSUPPORT_MULTICHANNEL: u32 = 2u32;
pub const KSPROPERTY_MEMBER_FLAG_BASICSUPPORT_UNIFORM: u32 = 4u32;
pub const KSPROPERTY_MEMBER_FLAG_DEFAULT: u32 = 1u32;
pub const KSPROPERTY_MEMBER_RANGES: u32 = 1u32;
pub const KSPROPERTY_MEMBER_STEPPEDRANGES: u32 = 2u32;
pub const KSPROPERTY_MEMBER_VALUES: u32 = 3u32;
pub const KSPROPERTY_MEMORY_TRANSPORT: i32 = 1i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_MPEG2VID(pub i32);
pub const KSPROPERTY_MPEG2VID_16_9_PANSCAN: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(4i32);
pub const KSPROPERTY_MPEG2VID_16_9_RECT: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(3i32);
pub const KSPROPERTY_MPEG2VID_4_3_RECT: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(2i32);
pub const KSPROPERTY_MPEG2VID_CUR_MODE: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(1i32);
pub const KSPROPERTY_MPEG2VID_MODES: KSPROPERTY_MPEG2VID = KSPROPERTY_MPEG2VID(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES(pub i32);
pub const KSPROPERTY_MPEG4_MEDIATYPE_SD_BOX: KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES = KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES(1i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_EVENTTOPICS_XML: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    pub Header: KSCAMERA_METADATA_ITEMHEADER,
    pub EventFilter: [u16; 1],
}
impl Default for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    pub MetadataItems: u32,
    pub Size: u32,
    pub PTZStatus: super::super::Foundation::BOOL,
    pub Events: super::super::Foundation::BOOL,
    pub Analytics: super::super::Foundation::BOOL,
    pub Reserved: super::super::Foundation::BOOL,
}
impl Default for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE(pub i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE_EVENTSINFO: KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE(0i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTP: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    pub Size: u32,
    pub Type: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE,
}
impl Default for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(pub i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_CUSTOM: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(2i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_DISABLE: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(0i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE_HOSTNTP: KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE = KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(pub i32);
pub const KSPROPERTY_NETWORKCAMERACONTROL_URI: KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY = KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY(1i32);
pub const KSPROPERTY_ONESHOT_DISCONNECT: KSPROPERTY_BTAUDIO = KSPROPERTY_BTAUDIO(1i32);
pub const KSPROPERTY_ONESHOT_RECONNECT: KSPROPERTY_BTAUDIO = KSPROPERTY_BTAUDIO(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_OVERLAYUPDATE(pub i32);
pub const KSPROPERTY_OVERLAYUPDATE_CLIPLIST: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(1i32);
pub const KSPROPERTY_OVERLAYUPDATE_COLORKEY: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(4i32);
pub const KSPROPERTY_OVERLAYUPDATE_COLORREF: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(268435456i32);
pub const KSPROPERTY_OVERLAYUPDATE_DISPLAYCHANGE: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(16i32);
pub const KSPROPERTY_OVERLAYUPDATE_INTERESTS: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(0i32);
pub const KSPROPERTY_OVERLAYUPDATE_PALETTE: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(2i32);
pub const KSPROPERTY_OVERLAYUPDATE_VIDEOPOSITION: KSPROPERTY_OVERLAYUPDATE = KSPROPERTY_OVERLAYUPDATE(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_PIN(pub i32);
pub const KSPROPERTY_PIN_CATEGORY: KSPROPERTY_PIN = KSPROPERTY_PIN(11i32);
pub const KSPROPERTY_PIN_CINSTANCES: KSPROPERTY_PIN = KSPROPERTY_PIN(0i32);
pub const KSPROPERTY_PIN_COMMUNICATION: KSPROPERTY_PIN = KSPROPERTY_PIN(7i32);
pub const KSPROPERTY_PIN_CONSTRAINEDDATARANGES: KSPROPERTY_PIN = KSPROPERTY_PIN(13i32);
pub const KSPROPERTY_PIN_CTYPES: KSPROPERTY_PIN = KSPROPERTY_PIN(1i32);
pub const KSPROPERTY_PIN_DATAFLOW: KSPROPERTY_PIN = KSPROPERTY_PIN(2i32);
pub const KSPROPERTY_PIN_DATAINTERSECTION: KSPROPERTY_PIN = KSPROPERTY_PIN(4i32);
pub const KSPROPERTY_PIN_DATARANGES: KSPROPERTY_PIN = KSPROPERTY_PIN(3i32);
pub const KSPROPERTY_PIN_FLAGS_ATTRIBUTE_RANGE_AWARE: u32 = 1u32;
pub const KSPROPERTY_PIN_FLAGS_MASK: u32 = 1u32;
pub const KSPROPERTY_PIN_GLOBALCINSTANCES: KSPROPERTY_PIN = KSPROPERTY_PIN(8i32);
pub const KSPROPERTY_PIN_INTERFACES: KSPROPERTY_PIN = KSPROPERTY_PIN(5i32);
pub const KSPROPERTY_PIN_MEDIUMS: KSPROPERTY_PIN = KSPROPERTY_PIN(6i32);
pub const KSPROPERTY_PIN_MODEDATAFORMATS: KSPROPERTY_PIN = KSPROPERTY_PIN(16i32);
pub const KSPROPERTY_PIN_NAME: KSPROPERTY_PIN = KSPROPERTY_PIN(12i32);
pub const KSPROPERTY_PIN_NECESSARYINSTANCES: KSPROPERTY_PIN = KSPROPERTY_PIN(9i32);
pub const KSPROPERTY_PIN_PHYSICALCONNECTION: KSPROPERTY_PIN = KSPROPERTY_PIN(10i32);
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT: KSPROPERTY_PIN = KSPROPERTY_PIN(14i32);
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT2: KSPROPERTY_PIN = KSPROPERTY_PIN(15i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_POSITIONS {
    pub Current: i64,
    pub Stop: i64,
    pub CurrentFlags: KS_SEEKING_FLAGS,
    pub StopFlags: KS_SEEKING_FLAGS,
}
impl Default for KSPROPERTY_POSITIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_PREFERRED_CAPTURE_SURFACE: KSPROPERTY_VIDMEM_TRANSPORT = KSPROPERTY_VIDMEM_TRANSPORT(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_QUALITY(pub i32);
pub const KSPROPERTY_QUALITY_ERROR: KSPROPERTY_QUALITY = KSPROPERTY_QUALITY(1i32);
pub const KSPROPERTY_QUALITY_REPORT: KSPROPERTY_QUALITY = KSPROPERTY_QUALITY(0i32);
pub const KSPROPERTY_RAW_AVC_CMD: KSPROPERTY_EXTXPORT = KSPROPERTY_EXTXPORT(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_RTAUDIO(pub i32);
pub const KSPROPERTY_RTAUDIO_BUFFER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(1i32);
pub const KSPROPERTY_RTAUDIO_BUFFER_WITH_NOTIFICATION: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(5i32);
pub const KSPROPERTY_RTAUDIO_CLOCKREGISTER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(4i32);
pub const KSPROPERTY_RTAUDIO_GETPOSITIONFUNCTION: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(0i32);
pub const KSPROPERTY_RTAUDIO_GETREADPACKET: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(11i32);
pub const KSPROPERTY_RTAUDIO_HWLATENCY: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(2i32);
pub const KSPROPERTY_RTAUDIO_PACKETCOUNT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(9i32);
pub const KSPROPERTY_RTAUDIO_PACKETVREGISTER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(13i32);
pub const KSPROPERTY_RTAUDIO_POSITIONREGISTER: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(3i32);
pub const KSPROPERTY_RTAUDIO_PRESENTATION_POSITION: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(10i32);
pub const KSPROPERTY_RTAUDIO_QUERY_NOTIFICATION_SUPPORT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(8i32);
pub const KSPROPERTY_RTAUDIO_REGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(6i32);
pub const KSPROPERTY_RTAUDIO_SETWRITEPACKET: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(12i32);
pub const KSPROPERTY_RTAUDIO_UNREGISTER_NOTIFICATION_EVENT: KSPROPERTY_RTAUDIO = KSPROPERTY_RTAUDIO(7i32);
pub const KSPROPERTY_RTC_READER: KSPROPERTY_TIMECODE = KSPROPERTY_TIMECODE(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_SELECTOR_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl Default for KSPROPERTY_SELECTOR_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_SELECTOR_NUM_SOURCES: KSPROPERTY_VIDCAP_SELECTOR = KSPROPERTY_VIDCAP_SELECTOR(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_SELECTOR_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl Default for KSPROPERTY_SELECTOR_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_SELECTOR_SOURCE_NODE_ID: KSPROPERTY_VIDCAP_SELECTOR = KSPROPERTY_VIDCAP_SELECTOR(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_SERIAL {
    pub PropTypeSet: KSIDENTIFIER,
    pub Id: u32,
    pub PropertyLength: u32,
}
impl Default for KSPROPERTY_SERIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_SERIALHDR {
    pub PropertySet: windows_core::GUID,
    pub Count: u32,
}
impl Default for KSPROPERTY_SERIALHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_SOUNDDETECTOR(pub i32);
pub const KSPROPERTY_SOUNDDETECTOR_ARMED: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(3i32);
pub const KSPROPERTY_SOUNDDETECTOR_MATCHRESULT: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(4i32);
pub const KSPROPERTY_SOUNDDETECTOR_PATTERNS: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(2i32);
pub const KSPROPERTY_SOUNDDETECTOR_RESET: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(5i32);
pub const KSPROPERTY_SOUNDDETECTOR_STREAMINGSUPPORT: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(6i32);
pub const KSPROPERTY_SOUNDDETECTOR_SUPPORTEDPATTERNS: KSPROPERTY_SOUNDDETECTOR = KSPROPERTY_SOUNDDETECTOR(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_SPHLI {
    pub HLISS: u16,
    pub Reserved: u16,
    pub StartPTM: u32,
    pub EndPTM: u32,
    pub StartX: u16,
    pub StartY: u16,
    pub StopX: u16,
    pub StopY: u16,
    pub ColCon: KS_COLCON,
}
impl Default for KSPROPERTY_SPHLI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPROPERTY_SPPAL {
    pub sppal: [KS_DVD_YUV; 16],
}
impl Default for KSPROPERTY_SPPAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_STEPPING_LONG {
    pub SteppingDelta: u32,
    pub Reserved: u32,
    pub Bounds: KSPROPERTY_BOUNDS_LONG,
}
impl Default for KSPROPERTY_STEPPING_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_STEPPING_LONGLONG {
    pub SteppingDelta: u64,
    pub Bounds: KSPROPERTY_BOUNDS_LONGLONG,
}
impl Default for KSPROPERTY_STEPPING_LONGLONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_STREAM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_STREAMINTERFACE(pub i32);
pub const KSPROPERTY_STREAMINTERFACE_HEADERSIZE: KSPROPERTY_STREAMINTERFACE = KSPROPERTY_STREAMINTERFACE(0i32);
pub const KSPROPERTY_STREAM_ALLOCATOR: KSPROPERTY_STREAM = KSPROPERTY_STREAM(0i32);
pub const KSPROPERTY_STREAM_DEGRADATION: KSPROPERTY_STREAM = KSPROPERTY_STREAM(2i32);
pub const KSPROPERTY_STREAM_FRAMETIME: KSPROPERTY_STREAM = KSPROPERTY_STREAM(7i32);
pub const KSPROPERTY_STREAM_MASTERCLOCK: KSPROPERTY_STREAM = KSPROPERTY_STREAM(3i32);
pub const KSPROPERTY_STREAM_PIPE_ID: KSPROPERTY_STREAM = KSPROPERTY_STREAM(10i32);
pub const KSPROPERTY_STREAM_PRESENTATIONEXTENT: KSPROPERTY_STREAM = KSPROPERTY_STREAM(6i32);
pub const KSPROPERTY_STREAM_PRESENTATIONTIME: KSPROPERTY_STREAM = KSPROPERTY_STREAM(5i32);
pub const KSPROPERTY_STREAM_QUALITY: KSPROPERTY_STREAM = KSPROPERTY_STREAM(1i32);
pub const KSPROPERTY_STREAM_RATE: KSPROPERTY_STREAM = KSPROPERTY_STREAM(9i32);
pub const KSPROPERTY_STREAM_RATECAPABILITY: KSPROPERTY_STREAM = KSPROPERTY_STREAM(8i32);
pub const KSPROPERTY_STREAM_TIMEFORMAT: KSPROPERTY_STREAM = KSPROPERTY_STREAM(4i32);
pub const KSPROPERTY_TELEPHONY_CALLCONTROL: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(2i32);
pub const KSPROPERTY_TELEPHONY_CALLHOLD: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(4i32);
pub const KSPROPERTY_TELEPHONY_CALLINFO: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_TELEPHONY_CONTROL(pub i32);
pub const KSPROPERTY_TELEPHONY_ENDPOINTIDPAIR: KSPROPERTY_TELEPHONY_TOPOLOGY = KSPROPERTY_TELEPHONY_TOPOLOGY(0i32);
pub const KSPROPERTY_TELEPHONY_MUTE_TX: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(5i32);
pub const KSPROPERTY_TELEPHONY_PROVIDERCHANGE: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(3i32);
pub const KSPROPERTY_TELEPHONY_PROVIDERID: KSPROPERTY_TELEPHONY_CONTROL = KSPROPERTY_TELEPHONY_CONTROL(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_TELEPHONY_TOPOLOGY(pub i32);
pub const KSPROPERTY_TELEPHONY_VOLUME: KSPROPERTY_TELEPHONY_TOPOLOGY = KSPROPERTY_TELEPHONY_TOPOLOGY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_TIMECODE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TIMECODE_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub TimecodeSamp: super::TIMECODE_SAMPLE,
}
impl Default for KSPROPERTY_TIMECODE_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TIMECODE_READER: KSPROPERTY_TIMECODE = KSPROPERTY_TIMECODE(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TIMECODE_S {
    pub Property: KSIDENTIFIER,
    pub TimecodeSamp: super::TIMECODE_SAMPLE,
}
impl Default for KSPROPERTY_TIMECODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_TOPOLOGY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_TOPOLOGYNODE(pub i32);
pub const KSPROPERTY_TOPOLOGYNODE_ENABLE: KSPROPERTY_TOPOLOGYNODE = KSPROPERTY_TOPOLOGYNODE(1i32);
pub const KSPROPERTY_TOPOLOGYNODE_RESET: KSPROPERTY_TOPOLOGYNODE = KSPROPERTY_TOPOLOGYNODE(2i32);
pub const KSPROPERTY_TOPOLOGY_CATEGORIES: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(0i32);
pub const KSPROPERTY_TOPOLOGY_CONNECTIONS: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(2i32);
pub const KSPROPERTY_TOPOLOGY_NAME: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(3i32);
pub const KSPROPERTY_TOPOLOGY_NODES: KSPROPERTY_TOPOLOGY = KSPROPERTY_TOPOLOGY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_TUNER(pub i32);
pub const KSPROPERTY_TUNER_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub ModesSupported: u32,
    pub VideoMedium: KSIDENTIFIER,
    pub TVAudioMedium: KSIDENTIFIER,
    pub RadioAudioMedium: KSIDENTIFIER,
}
impl Default for KSPROPERTY_TUNER_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_FREQUENCY: KSPROPERTY_TUNER = KSPROPERTY_TUNER(4i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_FREQUENCY_S {
    pub Property: KSIDENTIFIER,
    pub Frequency: u32,
    pub LastFrequency: u32,
    pub TuningFlags: u32,
    pub VideoSubChannel: u32,
    pub AudioSubChannel: u32,
    pub Channel: u32,
    pub Country: u32,
}
impl Default for KSPROPERTY_TUNER_FREQUENCY_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_IF_MEDIUM: KSPROPERTY_TUNER = KSPROPERTY_TUNER(7i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_IF_MEDIUM_S {
    pub Property: KSIDENTIFIER,
    pub IFMedium: KSIDENTIFIER,
}
impl Default for KSPROPERTY_TUNER_IF_MEDIUM_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_INPUT: KSPROPERTY_TUNER = KSPROPERTY_TUNER(5i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_INPUT_S {
    pub Property: KSIDENTIFIER,
    pub InputIndex: u32,
}
impl Default for KSPROPERTY_TUNER_INPUT_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_MODE: KSPROPERTY_TUNER = KSPROPERTY_TUNER(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_TUNER_MODES(pub i32);
pub const KSPROPERTY_TUNER_MODE_AM_RADIO: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(4i32);
pub const KSPROPERTY_TUNER_MODE_ATSC: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(16i32);
pub const KSPROPERTY_TUNER_MODE_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_MODE_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub Mode: u32,
    pub StandardsSupported: u32,
    pub MinFrequency: u32,
    pub MaxFrequency: u32,
    pub TuningGranularity: u32,
    pub NumberOfInputs: u32,
    pub SettlingTime: u32,
    pub Strategy: u32,
}
impl Default for KSPROPERTY_TUNER_MODE_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_MODE_DSS: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(8i32);
pub const KSPROPERTY_TUNER_MODE_FM_RADIO: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_MODE_S {
    pub Property: KSIDENTIFIER,
    pub Mode: u32,
}
impl Default for KSPROPERTY_TUNER_MODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_MODE_TV: KSPROPERTY_TUNER_MODES = KSPROPERTY_TUNER_MODES(1i32);
pub const KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(11i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub NetworkType: windows_core::GUID,
    pub BufferSize: u32,
    pub NetworkTunerCapabilities: *mut core::ffi::c_void,
}
impl Default for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_SCAN_CAPS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(8i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_SCAN_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub fSupportsHardwareAssistedScanning: super::super::Foundation::BOOL,
    pub SupportedBroadcastStandards: u32,
    pub GUIDBucket: *mut core::ffi::c_void,
    pub lengthofBucket: u32,
}
impl Default for KSPROPERTY_TUNER_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_SCAN_STATUS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(9i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_SCAN_STATUS_S {
    pub Property: KSIDENTIFIER,
    pub LockStatus: TunerLockType,
    pub CurrentFrequency: u32,
}
impl Default for KSPROPERTY_TUNER_SCAN_STATUS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_STANDARD: KSPROPERTY_TUNER = KSPROPERTY_TUNER(3i32);
pub const KSPROPERTY_TUNER_STANDARD_MODE: KSPROPERTY_TUNER = KSPROPERTY_TUNER(10i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_STANDARD_MODE_S {
    pub Property: KSIDENTIFIER,
    pub AutoDetect: super::super::Foundation::BOOL,
}
impl Default for KSPROPERTY_TUNER_STANDARD_MODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_STANDARD_S {
    pub Property: KSIDENTIFIER,
    pub Standard: u32,
}
impl Default for KSPROPERTY_TUNER_STANDARD_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TUNER_STATUS: KSPROPERTY_TUNER = KSPROPERTY_TUNER(6i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TUNER_STATUS_S {
    pub Property: KSIDENTIFIER,
    pub CurrentFrequency: u32,
    pub PLLOffset: u32,
    pub SignalStrength: u32,
    pub Busy: u32,
}
impl Default for KSPROPERTY_TUNER_STATUS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TVAUDIO_CAPS: KSPROPERTY_VIDCAP_TVAUDIO = KSPROPERTY_VIDCAP_TVAUDIO(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TVAUDIO_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub Capabilities: u32,
    pub InputMedium: KSIDENTIFIER,
    pub OutputMedium: KSIDENTIFIER,
}
impl Default for KSPROPERTY_TVAUDIO_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TVAUDIO_CURRENTLY_AVAILABLE_MODES: KSPROPERTY_VIDCAP_TVAUDIO = KSPROPERTY_VIDCAP_TVAUDIO(2i32);
pub const KSPROPERTY_TVAUDIO_MODE: KSPROPERTY_VIDCAP_TVAUDIO = KSPROPERTY_VIDCAP_TVAUDIO(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_TVAUDIO_S {
    pub Property: KSIDENTIFIER,
    pub Mode: u32,
}
impl Default for KSPROPERTY_TVAUDIO_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_TYPE_BASICSUPPORT: u32 = 512u32;
pub const KSPROPERTY_TYPE_COPYPAYLOAD: u32 = 2147483648u32;
pub const KSPROPERTY_TYPE_DEFAULTVALUES: u32 = 65536u32;
pub const KSPROPERTY_TYPE_FSFILTERSCOPE: u32 = 1073741824u32;
pub const KSPROPERTY_TYPE_GET: u32 = 1u32;
pub const KSPROPERTY_TYPE_GETPAYLOADSIZE: u32 = 4u32;
pub const KSPROPERTY_TYPE_HIGHPRIORITY: u32 = 134217728u32;
pub const KSPROPERTY_TYPE_RELATIONS: u32 = 1024u32;
pub const KSPROPERTY_TYPE_SERIALIZERAW: u32 = 8192u32;
pub const KSPROPERTY_TYPE_SERIALIZESET: u32 = 2048u32;
pub const KSPROPERTY_TYPE_SERIALIZESIZE: u32 = 32768u32;
pub const KSPROPERTY_TYPE_SET: u32 = 2u32;
pub const KSPROPERTY_TYPE_SETSUPPORT: u32 = 256u32;
pub const KSPROPERTY_TYPE_TOPOLOGY: u32 = 268435456u32;
pub const KSPROPERTY_TYPE_UNSERIALIZERAW: u32 = 16384u32;
pub const KSPROPERTY_TYPE_UNSERIALIZESET: u32 = 4096u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VBICAP(pub i32);
pub const KSPROPERTY_VBICAP_PROPERTIES_PROTECTION: KSPROPERTY_VBICAP = KSPROPERTY_VBICAP(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VBICODECFILTERING(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    pub Property: KSIDENTIFIER,
    pub Substreams: VBICODECFILTERING_CC_SUBSTREAMS,
}
impl Default for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    pub Property: KSIDENTIFIER,
    pub Substreams: VBICODECFILTERING_NABTS_SUBSTREAMS,
}
impl Default for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(2i32);
pub const KSPROPERTY_VBICODECFILTERING_SCANLINES_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    pub Property: KSIDENTIFIER,
    pub Scanlines: VBICODECFILTERING_SCANLINES,
}
impl Default for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VBICODECFILTERING_STATISTICS: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(5i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_CC_PIN,
}
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_CC,
}
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_COMMON,
}
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_NABTS_PIN,
}
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    pub Property: KSIDENTIFIER,
    pub Statistics: VBICODECFILTERING_STATISTICS_NABTS,
}
impl Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_DISCOVERED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(4i32);
pub const KSPROPERTY_VBICODECFILTERING_SUBSTREAMS_REQUESTED_BIT_ARRAY: KSPROPERTY_VBICODECFILTERING = KSPROPERTY_VBICODECFILTERING(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_CAMERACONTROL(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_CROSSBAR(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_DROPPEDFRAMES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_SELECTOR(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_TVAUDIO(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_VIDEOCONTROL(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_VIDEODECODER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_VIDEOENCODER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDCAP_VIDEOPROCAMP(pub i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_GETINFO: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub DefaultKeyFrameRate: i32,
    pub DefaultPFrameRate: i32,
    pub DefaultQuality: i32,
    pub NumberOfQualitySettings: i32,
    pub Capabilities: i32,
}
impl Default for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCOMPRESSION_KEYFRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(1i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_FRAME_SIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(5i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_OVERRIDE_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(4i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_PFRAMES_PER_KEYFRAME: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(2i32);
pub const KSPROPERTY_VIDEOCOMPRESSION_QUALITY: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(3i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCOMPRESSION_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Value: i32,
}
impl Default for KSPROPERTY_VIDEOCOMPRESSION_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCOMPRESSION_S1 {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Value: i32,
    pub Flags: u32,
}
impl Default for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCOMPRESSION_WINDOWSIZE: KSPROPERTY_VIDCAP_VIDEOCOMPRESSION = KSPROPERTY_VIDCAP_VIDEOCOMPRESSION(6i32);
pub const KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::super::Foundation::SIZE,
    pub CurrentActualFrameRate: i64,
    pub CurrentMaxAvailableFrameRate: i64,
}
impl Default for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCONTROL_CAPS: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCONTROL_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub VideoControlCaps: u32,
}
impl Default for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCONTROL_FRAME_RATES: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub RangeIndex: u32,
    pub Dimensions: super::super::Foundation::SIZE,
}
impl Default for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOCONTROL_MODE: KSPROPERTY_VIDCAP_VIDEOCONTROL = KSPROPERTY_VIDCAP_VIDEOCONTROL(3i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOCONTROL_MODE_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Mode: i32,
}
impl Default for KSPROPERTY_VIDEOCONTROL_MODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEODECODER_CAPS: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEODECODER_CAPS_S {
    pub Property: KSIDENTIFIER,
    pub StandardsSupported: u32,
    pub Capabilities: u32,
    pub SettlingTime: u32,
    pub HSyncPerVSync: u32,
}
impl Default for KSPROPERTY_VIDEODECODER_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEODECODER_OUTPUT_ENABLE: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(3i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEODECODER_S {
    pub Property: KSIDENTIFIER,
    pub Value: u32,
}
impl Default for KSPROPERTY_VIDEODECODER_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEODECODER_STANDARD: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(1i32);
pub const KSPROPERTY_VIDEODECODER_STATUS: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(2i32);
pub const KSPROPERTY_VIDEODECODER_STATUS2: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(5i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEODECODER_STATUS2_S {
    pub Property: KSIDENTIFIER,
    pub NumberOfLines: u32,
    pub SignalLocked: u32,
    pub ChromaLock: u32,
}
impl Default for KSPROPERTY_VIDEODECODER_STATUS2_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEODECODER_STATUS_S {
    pub Property: KSIDENTIFIER,
    pub NumberOfLines: u32,
    pub SignalLocked: u32,
}
impl Default for KSPROPERTY_VIDEODECODER_STATUS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEODECODER_VCR_TIMING: KSPROPERTY_VIDCAP_VIDEODECODER = KSPROPERTY_VIDCAP_VIDEODECODER(4i32);
pub const KSPROPERTY_VIDEOENCODER_CAPS: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(0i32);
pub const KSPROPERTY_VIDEOENCODER_CC_ENABLE: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(3i32);
pub const KSPROPERTY_VIDEOENCODER_COPYPROTECTION: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOENCODER_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl Default for KSPROPERTY_VIDEOENCODER_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOENCODER_STANDARD: KSPROPERTY_VIDCAP_VIDEOENCODER = KSPROPERTY_VIDCAP_VIDEOENCODER(1i32);
pub const KSPROPERTY_VIDEOPROCAMP_BACKLIGHT_COMPENSATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(8i32);
pub const KSPROPERTY_VIDEOPROCAMP_BRIGHTNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(0i32);
pub const KSPROPERTY_VIDEOPROCAMP_COLORENABLE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(6i32);
pub const KSPROPERTY_VIDEOPROCAMP_CONTRAST: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(1i32);
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(10i32);
pub const KSPROPERTY_VIDEOPROCAMP_DIGITAL_MULTIPLIER_LIMIT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(11i32);
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_AUTO: i32 = 1i32;
pub const KSPROPERTY_VIDEOPROCAMP_FLAGS_MANUAL: i32 = 2i32;
pub const KSPROPERTY_VIDEOPROCAMP_GAIN: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(9i32);
pub const KSPROPERTY_VIDEOPROCAMP_GAMMA: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(5i32);
pub const KSPROPERTY_VIDEOPROCAMP_HUE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S {
    pub NodeProperty: KSP_NODE,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl Default for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    pub NodeProperty: KSP_NODE,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl Default for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOPROCAMP_POWERLINE_FREQUENCY: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(13i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOPROCAMP_S {
    pub Property: KSIDENTIFIER,
    pub Value: i32,
    pub Flags: u32,
    pub Capabilities: u32,
}
impl Default for KSPROPERTY_VIDEOPROCAMP_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_VIDEOPROCAMP_S2 {
    pub Property: KSIDENTIFIER,
    pub Value1: i32,
    pub Flags: u32,
    pub Capabilities: u32,
    pub Value2: i32,
}
impl Default for KSPROPERTY_VIDEOPROCAMP_S2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSPROPERTY_VIDEOPROCAMP_SATURATION: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(3i32);
pub const KSPROPERTY_VIDEOPROCAMP_SHARPNESS: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(4i32);
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(7i32);
pub const KSPROPERTY_VIDEOPROCAMP_WHITEBALANCE_COMPONENT: KSPROPERTY_VIDCAP_VIDEOPROCAMP = KSPROPERTY_VIDCAP_VIDEOPROCAMP(12i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VIDMEM_TRANSPORT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_VPCONFIG(pub i32);
pub const KSPROPERTY_VPCONFIG_DDRAWHANDLE: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(12i32);
pub const KSPROPERTY_VPCONFIG_DDRAWSURFACEHANDLE: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(14i32);
pub const KSPROPERTY_VPCONFIG_DECIMATIONCAPABILITY: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(10i32);
pub const KSPROPERTY_VPCONFIG_GETCONNECTINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(1i32);
pub const KSPROPERTY_VPCONFIG_GETVIDEOFORMAT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(7i32);
pub const KSPROPERTY_VPCONFIG_INFORMVPINPUT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(5i32);
pub const KSPROPERTY_VPCONFIG_INVERTPOLARITY: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(9i32);
pub const KSPROPERTY_VPCONFIG_MAXPIXELRATE: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(4i32);
pub const KSPROPERTY_VPCONFIG_NUMCONNECTINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(0i32);
pub const KSPROPERTY_VPCONFIG_NUMVIDEOFORMAT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(6i32);
pub const KSPROPERTY_VPCONFIG_SCALEFACTOR: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(11i32);
pub const KSPROPERTY_VPCONFIG_SETCONNECTINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(2i32);
pub const KSPROPERTY_VPCONFIG_SETVIDEOFORMAT: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(8i32);
pub const KSPROPERTY_VPCONFIG_SURFACEPARAMS: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(15i32);
pub const KSPROPERTY_VPCONFIG_VIDEOPORTID: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(13i32);
pub const KSPROPERTY_VPCONFIG_VPDATAINFO: KSPROPERTY_VPCONFIG = KSPROPERTY_VPCONFIG(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_WAVE(pub i32);
pub const KSPROPERTY_WAVE_BUFFER: KSPROPERTY_WAVE = KSPROPERTY_WAVE(3i32);
pub const KSPROPERTY_WAVE_COMPATIBLE_CAPABILITIES: KSPROPERTY_WAVE = KSPROPERTY_WAVE(0i32);
pub const KSPROPERTY_WAVE_FREQUENCY: KSPROPERTY_WAVE = KSPROPERTY_WAVE(4i32);
pub const KSPROPERTY_WAVE_INPUT_CAPABILITIES: KSPROPERTY_WAVE = KSPROPERTY_WAVE(1i32);
pub const KSPROPERTY_WAVE_OUTPUT_CAPABILITIES: KSPROPERTY_WAVE = KSPROPERTY_WAVE(2i32);
pub const KSPROPERTY_WAVE_PAN: KSPROPERTY_WAVE = KSPROPERTY_WAVE(6i32);
pub const KSPROPERTY_WAVE_QUEUED_POSITION: u32 = 1u32;
pub const KSPROPERTY_WAVE_VOLUME: KSPROPERTY_WAVE = KSPROPERTY_WAVE(5i32);
pub const KSPROPSETID_AC3: windows_core::GUID = windows_core::GUID::from_u128(0xbfabe720_6e1f_11d0_bcf2_444553540000);
pub const KSPROPSETID_Audio: windows_core::GUID = windows_core::GUID::from_u128(0x45ffaaa0_6e1b_11d0_bcf2_444553540000);
pub const KSPROPSETID_AudioBufferDuration: windows_core::GUID = windows_core::GUID::from_u128(0x4e73c07f_23cc_4955_a7ea_3da502496290);
pub const KSPROPSETID_AudioDecoderOut: windows_core::GUID = windows_core::GUID::from_u128(0x6ca6e020_43bd_11d0_bd6a_003505c103a9);
pub const KSPROPSETID_AudioEngine: windows_core::GUID = windows_core::GUID::from_u128(0x3a2f82dc_886f_4baa_9eb4_082b9025c536);
pub const KSPROPSETID_AudioModule: windows_core::GUID = windows_core::GUID::from_u128(0xc034fdb0_ff75_47c8_aa3c_ee46716b50c6);
pub const KSPROPSETID_AudioPosture: windows_core::GUID = windows_core::GUID::from_u128(0xa3fb7b0d_474e_4f51_a379_51282dd4fa8f);
pub const KSPROPSETID_AudioResourceManagement: windows_core::GUID = windows_core::GUID::from_u128(0xd0b305e1_b2cc_484c_8f23_e5d28ad9cf88);
pub const KSPROPSETID_AudioSignalProcessing: windows_core::GUID = windows_core::GUID::from_u128(0x4f67b528_30c9_40de_b2fb_859ddd1f3470);
pub const KSPROPSETID_Bibliographic: windows_core::GUID = windows_core::GUID::from_u128(0x07ba150e_e2b1_11d0_ac17_00a0c9223196);
pub const KSPROPSETID_BtAudio: windows_core::GUID = windows_core::GUID::from_u128(0x7fa06c40_b8f6_4c7e_8556_e8c33a12e54d);
pub const KSPROPSETID_Clock: windows_core::GUID = windows_core::GUID::from_u128(0xdf12a4c0_ac17_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_Connection: windows_core::GUID = windows_core::GUID::from_u128(0x1d58c920_ac9b_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_CopyProt: windows_core::GUID = windows_core::GUID::from_u128(0x0e8a0a40_6aef_11d0_9ed0_00a024ca19b3);
pub const KSPROPSETID_Cyclic: windows_core::GUID = windows_core::GUID::from_u128(0x3ffeaea0_2bee_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_DirectSound3DBuffer: windows_core::GUID = windows_core::GUID::from_u128(0x437b3411_d060_11d0_8583_00c04fd9baf3);
pub const KSPROPSETID_DirectSound3DListener: windows_core::GUID = windows_core::GUID::from_u128(0x437b3414_d060_11d0_8583_00c04fd9baf3);
pub const KSPROPSETID_DrmAudioStream: windows_core::GUID = windows_core::GUID::from_u128(0x2f2c8ddd_4198_4fac_ba29_61bb05b7de06);
pub const KSPROPSETID_DvdSubPic: windows_core::GUID = windows_core::GUID::from_u128(0xac390460_43af_11d0_bd6a_003505c103a9);
pub const KSPROPSETID_FMRXControl: windows_core::GUID = windows_core::GUID::from_u128(0x947bba3a_e8ee_4786_90c4_8428185f05be);
pub const KSPROPSETID_FMRXTopology: windows_core::GUID = windows_core::GUID::from_u128(0x0c46ce8f_dc2d_4204_9dc9_f58963366563);
pub const KSPROPSETID_General: windows_core::GUID = windows_core::GUID::from_u128(0x1464eda5_6a8f_11d1_9aa7_00a0c9223196);
pub const KSPROPSETID_Hrtf3d: windows_core::GUID = windows_core::GUID::from_u128(0xb66decb0_a083_11d0_851e_00c04fd9baf3);
pub const KSPROPSETID_InterleavedAudio: windows_core::GUID = windows_core::GUID::from_u128(0xe9ebe550_d619_4c0a_976b_7062322b3006);
pub const KSPROPSETID_Itd3d: windows_core::GUID = windows_core::GUID::from_u128(0x6429f090_9fd9_11d0_a75b_00a0c90365e3);
pub const KSPROPSETID_Jack: windows_core::GUID = windows_core::GUID::from_u128(0x4509f757_2d46_4637_8e62_ce7db944f57b);
pub const KSPROPSETID_MPEG4_MediaType_Attributes: windows_core::GUID = windows_core::GUID::from_u128(0xff6c4bfa_07a9_4c7b_a237_672f9d68065f);
pub const KSPROPSETID_MediaSeeking: windows_core::GUID = windows_core::GUID::from_u128(0xee904f0c_d09b_11d0_abe9_00a0c9223196);
pub const KSPROPSETID_MemoryTransport: windows_core::GUID = windows_core::GUID::from_u128(0x0a3d1c5d_5243_4819_9ed0_aee8044cee2b);
pub const KSPROPSETID_Mpeg2Vid: windows_core::GUID = windows_core::GUID::from_u128(0xc8e11b60_0cc9_11d0_bd69_003505c103a9);
pub const KSPROPSETID_OverlayUpdate: windows_core::GUID = windows_core::GUID::from_u128(0x490ea5cf_7681_11d1_a21c_00a0c9223196);
pub const KSPROPSETID_Pin: windows_core::GUID = windows_core::GUID::from_u128(0x8c134960_51ad_11cf_878a_94f801c10000);
pub const KSPROPSETID_PinMDLCacheClearProp: windows_core::GUID = windows_core::GUID::from_u128(0xbd718a7b_97fc_40c7_88ce_d3ff06f55b16);
pub const KSPROPSETID_Quality: windows_core::GUID = windows_core::GUID::from_u128(0xd16ad380_ac1a_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_RtAudio: windows_core::GUID = windows_core::GUID::from_u128(0xa855a48c_2f78_4729_9051_1968746b9eef);
pub const KSPROPSETID_SoundDetector: windows_core::GUID = windows_core::GUID::from_u128(0x113c425e_fd17_4057_b422_ed4074f1afdf);
pub const KSPROPSETID_SoundDetector2: windows_core::GUID = windows_core::GUID::from_u128(0xfe07e322_450c_4bd5_84ca_a948500ea6aa);
pub const KSPROPSETID_Stream: windows_core::GUID = windows_core::GUID::from_u128(0x65aaba60_98ae_11cf_a10d_0020afd156e4);
pub const KSPROPSETID_StreamAllocator: windows_core::GUID = windows_core::GUID::from_u128(0xcf6e4342_ec87_11cf_a130_0020afd156e4);
pub const KSPROPSETID_StreamInterface: windows_core::GUID = windows_core::GUID::from_u128(0x1fdd8ee1_9cd3_11d0_82aa_0000f822fe8a);
pub const KSPROPSETID_TSRateChange: windows_core::GUID = windows_core::GUID::from_u128(0xa503c5c0_1d1d_11d1_ad80_444553540000);
pub const KSPROPSETID_TelephonyControl: windows_core::GUID = windows_core::GUID::from_u128(0xb6df7eb1_d099_489f_a6a0_c0106f0887a7);
pub const KSPROPSETID_TelephonyTopology: windows_core::GUID = windows_core::GUID::from_u128(0xabf25c7e_0e64_4e32_b190_d0f6d7c53e97);
pub const KSPROPSETID_Topology: windows_core::GUID = windows_core::GUID::from_u128(0x720d4ac0_7533_11d0_a5d6_28db04c10000);
pub const KSPROPSETID_TopologyNode: windows_core::GUID = windows_core::GUID::from_u128(0x45ffaaa1_6e1b_11d0_bcf2_444553540000);
pub const KSPROPSETID_VBICAP_PROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0xf162c607_7b35_496f_ad7f_2dca3b46b718);
pub const KSPROPSETID_VBICodecFiltering: windows_core::GUID = windows_core::GUID::from_u128(0xcafeb0ca_8715_11d0_bd6a_0035c0edbabe);
pub const KSPROPSETID_VPConfig: windows_core::GUID = windows_core::GUID::from_u128(0xbc29a660_30e3_11d0_9e69_00c04fd7c15b);
pub const KSPROPSETID_VPVBIConfig: windows_core::GUID = windows_core::GUID::from_u128(0xec529b00_1a1f_11d1_bad9_00609744111a);
pub const KSPROPSETID_VramCapture: windows_core::GUID = windows_core::GUID::from_u128(0xe73face3_2880_4902_b799_88d0cd634e0f);
pub const KSPROPSETID_Wave: windows_core::GUID = windows_core::GUID::from_u128(0x924e54b0_630f_11cf_ada7_08003e30494a);
pub const KSPROPTYPESETID_General: windows_core::GUID = windows_core::GUID::from_u128(0x97e99ba0_bdea_11cf_a5d6_28db04c10000);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSP_NODE {
    pub Property: KSIDENTIFIER,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl Default for KSP_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSP_PIN {
    pub Property: KSIDENTIFIER,
    pub PinId: u32,
    pub Anonymous: KSP_PIN_0,
}
impl Default for KSP_PIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSP_PIN_0 {
    pub Reserved: u32,
    pub Flags: u32,
}
impl Default for KSP_PIN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSP_TIMEFORMAT {
    pub Property: KSIDENTIFIER,
    pub SourceFormat: windows_core::GUID,
    pub TargetFormat: windows_core::GUID,
    pub Time: i64,
}
impl Default for KSP_TIMEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSQUALITY {
    pub Context: *mut core::ffi::c_void,
    pub Proportion: u32,
    pub DeltaTime: i64,
}
impl Default for KSQUALITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSQUALITY_MANAGER {
    pub QualityManager: super::super::Foundation::HANDLE,
    pub Context: *mut core::ffi::c_void,
}
impl Default for KSQUALITY_MANAGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSQUERYBUFFER {
    pub Event: KSIDENTIFIER,
    pub EventData: *mut KSEVENTDATA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for KSQUERYBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRATE {
    pub PresentationStart: i64,
    pub Duration: i64,
    pub Interface: KSIDENTIFIER,
    pub Rate: i32,
    pub Flags: u32,
}
impl Default for KSRATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRATE_CAPABILITY {
    pub Property: KSIDENTIFIER,
    pub Rate: KSRATE,
}
impl Default for KSRATE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSRATE_NOPRESENTATIONDURATION: u32 = 2u32;
pub const KSRATE_NOPRESENTATIONSTART: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRELATIVEEVENT {
    pub Size: u32,
    pub Flags: u32,
    pub Anonymous: KSRELATIVEEVENT_0,
    pub Reserved: *mut core::ffi::c_void,
    pub Event: KSIDENTIFIER,
    pub EventData: KSEVENTDATA,
}
impl Default for KSRELATIVEEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSRELATIVEEVENT_0 {
    pub ObjectHandle: super::super::Foundation::HANDLE,
    pub ObjectPointer: *mut core::ffi::c_void,
}
impl Default for KSRELATIVEEVENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSRELATIVEEVENT_FLAG_HANDLE: u32 = 1u32;
pub const KSRELATIVEEVENT_FLAG_POINTER: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSRESET(pub i32);
pub const KSRESET_BEGIN: KSRESET = KSRESET(0i32);
pub const KSRESET_END: KSRESET = KSRESET(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSRESOLUTION {
    pub Granularity: i64,
    pub Error: i64,
}
impl Default for KSRESOLUTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSRTAUDIO_BUFFER {
    pub BufferAddress: *mut core::ffi::c_void,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: super::super::Foundation::BOOL,
}
impl Default for KSRTAUDIO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSRTAUDIO_BUFFER32 {
    pub BufferAddress: u32,
    pub ActualBufferSize: u32,
    pub CallMemoryBarrier: super::super::Foundation::BOOL,
}
impl Default for KSRTAUDIO_BUFFER32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut core::ffi::c_void,
    pub RequestedBufferSize: u32,
}
impl Default for KSRTAUDIO_BUFFER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER_PROPERTY32 {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: u32,
    pub RequestedBufferSize: u32,
}
impl Default for KSRTAUDIO_BUFFER_PROPERTY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut core::ffi::c_void,
    pub RequestedBufferSize: u32,
    pub NotificationCount: u32,
}
impl Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: u32,
    pub RequestedBufferSize: u32,
    pub NotificationCount: u32,
}
impl Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSRTAUDIO_GETREADPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub PerformanceCounterValue: u64,
    pub MoreData: super::super::Foundation::BOOL,
}
impl Default for KSRTAUDIO_GETREADPACKET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSRTAUDIO_HWLATENCY {
    pub FifoSize: u32,
    pub ChipsetDelay: u32,
    pub CodecDelay: u32,
}
impl Default for KSRTAUDIO_HWLATENCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSRTAUDIO_HWREGISTER {
    pub Register: *mut core::ffi::c_void,
    pub Width: u32,
    pub Numerator: u64,
    pub Denominator: u64,
    pub Accuracy: u32,
}
impl Default for KSRTAUDIO_HWREGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSRTAUDIO_HWREGISTER32 {
    pub Register: u32,
    pub Width: u32,
    pub Numerator: u64,
    pub Denominator: u64,
    pub Accuracy: u32,
}
impl Default for KSRTAUDIO_HWREGISTER32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_HWREGISTER_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut core::ffi::c_void,
}
impl Default for KSRTAUDIO_HWREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_HWREGISTER_PROPERTY32 {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: u32,
}
impl Default for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub NotificationEvent: super::super::Foundation::HANDLE,
}
impl Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    pub Property: KSIDENTIFIER,
    pub NotificationEvent: u32,
}
impl Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSRTAUDIO_PACKETVREGISTER {
    pub CompletedPacketCount: *mut u64,
    pub CompletedPacketQPC: *mut u64,
    pub CompletedPacketHash: *mut u64,
}
impl Default for KSRTAUDIO_PACKETVREGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    pub Property: KSIDENTIFIER,
    pub BaseAddress: *mut core::ffi::c_void,
}
impl Default for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSRTAUDIO_SETWRITEPACKET_INFO {
    pub PacketNumber: u32,
    pub Flags: u32,
    pub EosPacketLength: u32,
}
impl Default for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSSOUNDDETECTORPROPERTY {
    pub Property: KSIDENTIFIER,
    pub EventId: windows_core::GUID,
}
impl Default for KSSOUNDDETECTORPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSSTATE(pub i32);
pub const KSSTATE_ACQUIRE: KSSTATE = KSSTATE(1i32);
pub const KSSTATE_PAUSE: KSSTATE = KSSTATE(2i32);
pub const KSSTATE_RUN: KSSTATE = KSSTATE(3i32);
pub const KSSTATE_STOP: KSSTATE = KSSTATE(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSSTREAMALLOCATOR_STATUS {
    pub Framing: KSALLOCATOR_FRAMING,
    pub AllocatedFrames: u32,
    pub Reserved: u32,
}
impl Default for KSSTREAMALLOCATOR_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSSTREAMALLOCATOR_STATUS_EX {
    pub Framing: KSALLOCATOR_FRAMING_EX,
    pub AllocatedFrames: u32,
    pub Reserved: u32,
}
impl Default for KSSTREAMALLOCATOR_STATUS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSSTREAM_FAILUREEXCEPTION: u32 = 8192u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSSTREAM_HEADER {
    pub Size: u32,
    pub TypeSpecificFlags: u32,
    pub PresentationTime: KSTIME,
    pub Duration: i64,
    pub FrameExtent: u32,
    pub DataUsed: u32,
    pub Data: *mut core::ffi::c_void,
    pub OptionsFlags: u32,
}
#[cfg(target_arch = "x86")]
impl Default for KSSTREAM_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSSTREAM_HEADER {
    pub Size: u32,
    pub TypeSpecificFlags: u32,
    pub PresentationTime: KSTIME,
    pub Duration: i64,
    pub FrameExtent: u32,
    pub DataUsed: u32,
    pub Data: *mut core::ffi::c_void,
    pub OptionsFlags: u32,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for KSSTREAM_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSSTREAM_HEADER_OPTIONSF_BUFFEREDTRANSFER: u32 = 1024u32;
pub const KSSTREAM_HEADER_OPTIONSF_DATADISCONTINUITY: u32 = 4u32;
pub const KSSTREAM_HEADER_OPTIONSF_DURATIONVALID: u32 = 256u32;
pub const KSSTREAM_HEADER_OPTIONSF_ENDOFPHOTOSEQUENCE: u32 = 8192u32;
pub const KSSTREAM_HEADER_OPTIONSF_ENDOFSTREAM: u32 = 512u32;
pub const KSSTREAM_HEADER_OPTIONSF_FLUSHONPAUSE: u32 = 128u32;
pub const KSSTREAM_HEADER_OPTIONSF_FRAMEINFO: u32 = 16384u32;
pub const KSSTREAM_HEADER_OPTIONSF_LOOPEDDATA: u32 = 2147483648u32;
pub const KSSTREAM_HEADER_OPTIONSF_METADATA: u32 = 4096u32;
pub const KSSTREAM_HEADER_OPTIONSF_PERSIST_SAMPLE: u32 = 32768u32;
pub const KSSTREAM_HEADER_OPTIONSF_PREROLL: u32 = 2u32;
pub const KSSTREAM_HEADER_OPTIONSF_SAMPLE_PERSISTED: u32 = 65536u32;
pub const KSSTREAM_HEADER_OPTIONSF_SECUREBUFFERTRANSFER: u32 = 262144u32;
pub const KSSTREAM_HEADER_OPTIONSF_SPLICEPOINT: u32 = 1u32;
pub const KSSTREAM_HEADER_OPTIONSF_TIMEDISCONTINUITY: u32 = 64u32;
pub const KSSTREAM_HEADER_OPTIONSF_TIMEVALID: u32 = 16u32;
pub const KSSTREAM_HEADER_OPTIONSF_TYPECHANGED: u32 = 8u32;
pub const KSSTREAM_HEADER_OPTIONSF_VRAM_DATA_TRANSFER: u32 = 2048u32;
pub const KSSTREAM_HEADER_TRACK_COMPLETION_NUMBERS: u32 = 131072u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSSTREAM_METADATA_INFO {
    pub BufferSize: u32,
    pub UsedSize: u32,
    pub Data: *mut core::ffi::c_void,
    pub SystemVa: *mut core::ffi::c_void,
    pub Flags: u32,
    pub Reserved: u32,
}
impl Default for KSSTREAM_METADATA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSSTREAM_NONPAGED_DATA: u32 = 256u32;
pub const KSSTREAM_PAGED_DATA: u32 = 0u32;
pub const KSSTREAM_READ: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct KSSTREAM_SEGMENT {
    pub KsInterfaceHandler: core::mem::ManuallyDrop<Option<IKsInterfaceHandler>>,
    pub KsDataTypeHandler: core::mem::ManuallyDrop<Option<IKsDataTypeHandler>>,
    pub IoOperation: KSIOOPERATION,
    pub CompletionEvent: super::super::Foundation::HANDLE,
}
impl Default for KSSTREAM_SEGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSSTREAM_SYNCHRONOUS: u32 = 4096u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSSTREAM_UVC_METADATA {
    pub StartOfFrameTimestamp: KSSTREAM_UVC_METADATATYPE_TIMESTAMP,
    pub EndOfFrameTimestamp: KSSTREAM_UVC_METADATATYPE_TIMESTAMP,
}
impl Default for KSSTREAM_UVC_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    pub PresentationTimeStamp: u32,
    pub SourceClockReference: u32,
    pub Anonymous: KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0,
    pub Reserved0: u16,
    pub Reserved1: u32,
}
impl Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    pub Anonymous: KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0,
    pub SCRToken: u16,
}
impl Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    pub _bitfield: u16,
}
impl Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSSTREAM_UVC_SECURE_ATTRIBUTE_SIZE: u32 = 8192u32;
pub const KSSTREAM_WRITE: u32 = 1u32;
pub const KSSTRING_Allocator: windows_core::PCWSTR = windows_core::w!("{642F5D00-4791-11D0-A5D6-28DB04C10000}");
pub const KSSTRING_AllocatorEx: windows_core::PCWSTR = windows_core::w!("{091BB63B-603F-11D1-B067-00A0C9062802}");
pub const KSSTRING_Clock: windows_core::PCWSTR = windows_core::w!("{53172480-4791-11D0-A5D6-28DB04C10000}");
pub const KSSTRING_Filter: windows_core::PCWSTR = windows_core::w!("{9B365890-165F-11D0-A195-0020AFD156E4}");
pub const KSSTRING_Pin: windows_core::PCWSTR = windows_core::w!("{146F1A80-4791-11D0-A5D6-28DB04C10000}");
pub const KSSTRING_TopologyNode: windows_core::PCWSTR = windows_core::w!("{0621061A-EE75-11D0-B915-00A0C9223196}");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSTELEPHONY_CALLCONTROL {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallControlOp: TELEPHONY_CALLCONTROLOP,
}
impl Default for KSTELEPHONY_CALLCONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSTELEPHONY_CALLINFO {
    pub CallType: TELEPHONY_CALLTYPE,
    pub CallState: TELEPHONY_CALLSTATE,
}
impl Default for KSTELEPHONY_CALLINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSTELEPHONY_PROVIDERCHANGE {
    pub CallType: TELEPHONY_CALLTYPE,
    pub ProviderChangeOp: TELEPHONY_PROVIDERCHANGEOP,
}
impl Default for KSTELEPHONY_PROVIDERCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSTIME {
    pub Time: i64,
    pub Numerator: u32,
    pub Denominator: u32,
}
impl Default for KSTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSTIME_FORMAT_BYTE: windows_core::GUID = windows_core::GUID::from_u128(0x7b785571_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_FIELD: windows_core::GUID = windows_core::GUID::from_u128(0x7b785573_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_FRAME: windows_core::GUID = windows_core::GUID::from_u128(0x7b785570_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_MEDIA_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x7b785574_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_SAMPLE: windows_core::GUID = windows_core::GUID::from_u128(0x7b785572_8c82_11cf_bc0c_00aa00ac74f6);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSTOPOLOGY {
    pub CategoriesCount: u32,
    pub Categories: *const windows_core::GUID,
    pub TopologyNodesCount: u32,
    pub TopologyNodes: *const windows_core::GUID,
    pub TopologyConnectionsCount: u32,
    pub TopologyConnections: *const KSTOPOLOGY_CONNECTION,
    pub TopologyNodesNames: *const windows_core::GUID,
    pub Reserved: u32,
}
impl Default for KSTOPOLOGY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSTOPOLOGY_CONNECTION {
    pub FromNode: u32,
    pub FromNodePin: u32,
    pub ToNode: u32,
    pub ToNodePin: u32,
}
impl Default for KSTOPOLOGY_CONNECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSTOPOLOGY_ENDPOINTID {
    pub TopologyName: [u16; 260],
    pub PinId: u32,
}
impl Default for KSTOPOLOGY_ENDPOINTID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSTOPOLOGY_ENDPOINTIDPAIR {
    pub RenderEndpoint: KSTOPOLOGY_ENDPOINTID,
    pub CaptureEndpoint: KSTOPOLOGY_ENDPOINTID,
}
impl Default for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSVPMAXPIXELRATE {
    pub Size: KS_AMVPSIZE,
    pub MaxPixelsPerSecond: u32,
    pub Reserved: u32,
}
impl Default for KSVPMAXPIXELRATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSVPSIZE_PROP {
    pub Property: KSIDENTIFIER,
    pub Size: KS_AMVPSIZE,
}
impl Default for KSVPSIZE_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSVPSURFACEPARAMS {
    pub dwPitch: u32,
    pub dwXOrigin: u32,
    pub dwYOrigin: u32,
}
impl Default for KSVPSURFACEPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSWAVETABLE_WAVE_DESC {
    pub Identifier: KSIDENTIFIER,
    pub Size: u32,
    pub Looped: super::super::Foundation::BOOL,
    pub LoopPoint: u32,
    pub InROM: super::super::Foundation::BOOL,
    pub Format: KSDATAFORMAT,
}
impl Default for KSWAVETABLE_WAVE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSWAVE_BUFFER {
    pub Attributes: u32,
    pub BufferSize: u32,
    pub BufferAddress: *mut core::ffi::c_void,
}
impl Default for KSWAVE_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSWAVE_BUFFER_ATTRIBUTEF_LOOPING: u32 = 1u32;
pub const KSWAVE_BUFFER_ATTRIBUTEF_STATIC: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSWAVE_COMPATCAPS {
    pub ulDeviceType: u32,
}
impl Default for KSWAVE_COMPATCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSWAVE_COMPATCAPS_INPUT: u32 = 0u32;
pub const KSWAVE_COMPATCAPS_OUTPUT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSWAVE_INPUT_CAPABILITIES {
    pub MaximumChannelsPerConnection: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
    pub TotalConnections: u32,
    pub ActiveConnections: u32,
}
impl Default for KSWAVE_INPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSWAVE_OUTPUT_CAPABILITIES {
    pub MaximumChannelsPerConnection: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
    pub TotalConnections: u32,
    pub StaticConnections: u32,
    pub StreamingConnections: u32,
    pub ActiveConnections: u32,
    pub ActiveStaticConnections: u32,
    pub ActiveStreamingConnections: u32,
    pub Total3DConnections: u32,
    pub Static3DConnections: u32,
    pub Streaming3DConnections: u32,
    pub Active3DConnections: u32,
    pub ActiveStatic3DConnections: u32,
    pub ActiveStreaming3DConnections: u32,
    pub TotalSampleMemory: u32,
    pub FreeSampleMemory: u32,
    pub LargestFreeContiguousSampleMemory: u32,
}
impl Default for KSWAVE_OUTPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSWAVE_VOLUME {
    pub LeftAttenuation: i32,
    pub RightAttenuation: i32,
}
impl Default for KSWAVE_VOLUME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_AMCONTROL_COLORINFO_PRESENT: u32 = 128u32;
pub const KS_AMCONTROL_PAD_TO_16x9: u32 = 4u32;
pub const KS_AMCONTROL_PAD_TO_4x3: u32 = 2u32;
pub const KS_AMCONTROL_USED: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_AMPixAspectRatio(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_AMVPDATAINFO {
    pub dwSize: u32,
    pub dwMicrosecondsPerField: u32,
    pub amvpDimInfo: KS_AMVPDIMINFO,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub bEnableDoubleClock: super::super::Foundation::BOOL,
    pub bEnableVACT: super::super::Foundation::BOOL,
    pub bDataIsInterlaced: super::super::Foundation::BOOL,
    pub lHalfLinesOdd: i32,
    pub bFieldPolarityInverted: super::super::Foundation::BOOL,
    pub dwNumLinesInVREF: u32,
    pub lHalfLinesEven: i32,
    pub dwReserved1: u32,
}
impl Default for KS_AMVPDATAINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_AMVPDIMINFO {
    pub dwFieldWidth: u32,
    pub dwFieldHeight: u32,
    pub dwVBIWidth: u32,
    pub dwVBIHeight: u32,
    pub rcValidRegion: super::super::Foundation::RECT,
}
impl Default for KS_AMVPDIMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_AMVPSIZE {
    pub dwWidth: u32,
    pub dwHeight: u32,
}
impl Default for KS_AMVPSIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_AMVP_BEST_BANDWIDTH: KS_AMVP_SELECTFORMATBY = KS_AMVP_SELECTFORMATBY(1i32);
pub const KS_AMVP_DO_NOT_CARE: KS_AMVP_SELECTFORMATBY = KS_AMVP_SELECTFORMATBY(0i32);
pub const KS_AMVP_INPUT_SAME_AS_OUTPUT: KS_AMVP_SELECTFORMATBY = KS_AMVP_SELECTFORMATBY(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_AMVP_MODE(pub i32);
pub const KS_AMVP_MODE_BOBINTERLEAVED: KS_AMVP_MODE = KS_AMVP_MODE(1i32);
pub const KS_AMVP_MODE_BOBNONINTERLEAVED: KS_AMVP_MODE = KS_AMVP_MODE(2i32);
pub const KS_AMVP_MODE_SKIPEVEN: KS_AMVP_MODE = KS_AMVP_MODE(3i32);
pub const KS_AMVP_MODE_SKIPODD: KS_AMVP_MODE = KS_AMVP_MODE(4i32);
pub const KS_AMVP_MODE_WEAVE: KS_AMVP_MODE = KS_AMVP_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_AMVP_SELECTFORMATBY(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_AM_ExactRateChange {
    pub OutputZeroTime: i64,
    pub Rate: i32,
}
impl Default for KS_AM_ExactRateChange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_AM_PROPERTY_TS_RATE_CHANGE(pub i32);
pub const KS_AM_RATE_ExactRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(2i32);
pub const KS_AM_RATE_MaxFullDataRate: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(3i32);
pub const KS_AM_RATE_SimpleRateChange: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(1i32);
pub const KS_AM_RATE_Step: KS_AM_PROPERTY_TS_RATE_CHANGE = KS_AM_PROPERTY_TS_RATE_CHANGE(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_AM_SimpleRateChange {
    pub StartTime: i64,
    pub Rate: i32,
}
impl Default for KS_AM_SimpleRateChange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_AM_UseNewCSSKey: i32 = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_ANALOGVIDEOINFO {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwActiveWidth: u32,
    pub dwActiveHeight: u32,
    pub AvgTimePerFrame: i64,
}
impl Default for KS_ANALOGVIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_AnalogVideoStandard(pub i32);
pub const KS_AnalogVideo_NTSC_433: KS_AnalogVideoStandard = KS_AnalogVideoStandard(4i32);
pub const KS_AnalogVideo_NTSC_M: KS_AnalogVideoStandard = KS_AnalogVideoStandard(1i32);
pub const KS_AnalogVideo_NTSC_M_J: KS_AnalogVideoStandard = KS_AnalogVideoStandard(2i32);
pub const KS_AnalogVideo_NTSC_Mask: u32 = 7u32;
pub const KS_AnalogVideo_None: KS_AnalogVideoStandard = KS_AnalogVideoStandard(0i32);
pub const KS_AnalogVideo_PAL_60: KS_AnalogVideoStandard = KS_AnalogVideoStandard(2048i32);
pub const KS_AnalogVideo_PAL_B: KS_AnalogVideoStandard = KS_AnalogVideoStandard(16i32);
pub const KS_AnalogVideo_PAL_D: KS_AnalogVideoStandard = KS_AnalogVideoStandard(32i32);
pub const KS_AnalogVideo_PAL_G: KS_AnalogVideoStandard = KS_AnalogVideoStandard(64i32);
pub const KS_AnalogVideo_PAL_H: KS_AnalogVideoStandard = KS_AnalogVideoStandard(128i32);
pub const KS_AnalogVideo_PAL_I: KS_AnalogVideoStandard = KS_AnalogVideoStandard(256i32);
pub const KS_AnalogVideo_PAL_M: KS_AnalogVideoStandard = KS_AnalogVideoStandard(512i32);
pub const KS_AnalogVideo_PAL_Mask: u32 = 1052656u32;
pub const KS_AnalogVideo_PAL_N: KS_AnalogVideoStandard = KS_AnalogVideoStandard(1024i32);
pub const KS_AnalogVideo_PAL_N_COMBO: KS_AnalogVideoStandard = KS_AnalogVideoStandard(1048576i32);
pub const KS_AnalogVideo_SECAM_B: KS_AnalogVideoStandard = KS_AnalogVideoStandard(4096i32);
pub const KS_AnalogVideo_SECAM_D: KS_AnalogVideoStandard = KS_AnalogVideoStandard(8192i32);
pub const KS_AnalogVideo_SECAM_G: KS_AnalogVideoStandard = KS_AnalogVideoStandard(16384i32);
pub const KS_AnalogVideo_SECAM_H: KS_AnalogVideoStandard = KS_AnalogVideoStandard(32768i32);
pub const KS_AnalogVideo_SECAM_K: KS_AnalogVideoStandard = KS_AnalogVideoStandard(65536i32);
pub const KS_AnalogVideo_SECAM_K1: KS_AnalogVideoStandard = KS_AnalogVideoStandard(131072i32);
pub const KS_AnalogVideo_SECAM_L: KS_AnalogVideoStandard = KS_AnalogVideoStandard(262144i32);
pub const KS_AnalogVideo_SECAM_L1: KS_AnalogVideoStandard = KS_AnalogVideoStandard(524288i32);
pub const KS_AnalogVideo_SECAM_Mask: u32 = 1044480u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
impl Default for KS_BITMAPINFOHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_BI_BITFIELDS: i32 = 3i32;
pub const KS_BI_JPEG: i32 = 4i32;
pub const KS_BI_RGB: i32 = 0i32;
pub const KS_BI_RLE4: i32 = 2i32;
pub const KS_BI_RLE8: i32 = 1i32;
pub const KS_CAMERACONTROL_ASYNC_RESET: KS_CameraControlAsyncOperation = KS_CameraControlAsyncOperation(3i32);
pub const KS_CAMERACONTROL_ASYNC_START: KS_CameraControlAsyncOperation = KS_CameraControlAsyncOperation(1i32);
pub const KS_CAMERACONTROL_ASYNC_STOP: KS_CameraControlAsyncOperation = KS_CameraControlAsyncOperation(2i32);
pub const KS_CAPTURE_ALLOC_INVALID: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(0i32);
pub const KS_CAPTURE_ALLOC_SECURE_BUFFER: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(16i32);
pub const KS_CAPTURE_ALLOC_SYSTEM: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(1i32);
pub const KS_CAPTURE_ALLOC_SYSTEM_AGP: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(4i32);
pub const KS_CAPTURE_ALLOC_VRAM: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(2i32);
pub const KS_CAPTURE_ALLOC_VRAM_MAPPED: CAPTURE_MEMORY_ALLOCATION_FLAGS = CAPTURE_MEMORY_ALLOCATION_FLAGS(8i32);
pub const KS_CC_SUBSTREAM_EVEN: i32 = 2i32;
pub const KS_CC_SUBSTREAM_FIELD1_MASK: i32 = 240i32;
pub const KS_CC_SUBSTREAM_FIELD2_MASK: i32 = 7936i32;
pub const KS_CC_SUBSTREAM_ODD: i32 = 1i32;
pub const KS_CC_SUBSTREAM_SERVICE_CC1: i32 = 16i32;
pub const KS_CC_SUBSTREAM_SERVICE_CC2: i32 = 32i32;
pub const KS_CC_SUBSTREAM_SERVICE_CC3: i32 = 256i32;
pub const KS_CC_SUBSTREAM_SERVICE_CC4: i32 = 512i32;
pub const KS_CC_SUBSTREAM_SERVICE_T1: i32 = 64i32;
pub const KS_CC_SUBSTREAM_SERVICE_T2: i32 = 128i32;
pub const KS_CC_SUBSTREAM_SERVICE_T3: i32 = 1024i32;
pub const KS_CC_SUBSTREAM_SERVICE_T4: i32 = 2048i32;
pub const KS_CC_SUBSTREAM_SERVICE_XDS: i32 = 4096i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_COLCON {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
}
impl Default for KS_COLCON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_COMPRESSION {
    pub RatioNumerator: u32,
    pub RatioDenominator: u32,
    pub RatioConstantMargin: u32,
}
impl Default for KS_COMPRESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_COPYPROTECT_RestrictDuplication: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_COPY_MACROVISION {
    pub MACROVISIONLevel: u32,
}
impl Default for KS_COPY_MACROVISION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_COPY_MACROVISION_LEVEL(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_CameraControlAsyncOperation(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_CompressionCaps(pub i32);
pub const KS_CompressionCaps_CanBFrame: KS_CompressionCaps = KS_CompressionCaps(8i32);
pub const KS_CompressionCaps_CanCrunch: KS_CompressionCaps = KS_CompressionCaps(2i32);
pub const KS_CompressionCaps_CanKeyFrame: KS_CompressionCaps = KS_CompressionCaps(4i32);
pub const KS_CompressionCaps_CanQuality: KS_CompressionCaps = KS_CompressionCaps(1i32);
pub const KS_CompressionCaps_CanWindow: KS_CompressionCaps = KS_CompressionCaps(16i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_H264VIDEOINFO {
    pub DataFormat: KSDATAFORMAT,
    pub H264VideoInfoHeader: KS_H264VIDEOINFO,
}
impl Default for KS_DATAFORMAT_H264VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_IMAGEINFO {
    pub DataFormat: KSDATAFORMAT,
    pub ImageInfoHeader: KS_BITMAPINFOHEADER,
}
impl Default for KS_DATAFORMAT_IMAGEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_MPEGVIDEOINFO2 {
    pub DataFormat: KSDATAFORMAT,
    pub MpegVideoInfoHeader2: KS_MPEGVIDEOINFO2,
}
impl Default for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_VBIINFOHEADER {
    pub DataFormat: KSDATAFORMAT,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
impl Default for KS_DATAFORMAT_VBIINFOHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER {
    pub DataFormat: KSDATAFORMAT,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER,
}
impl Default for KS_DATAFORMAT_VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_VIDEOINFOHEADER2 {
    pub DataFormat: KSDATAFORMAT,
    pub VideoInfoHeader2: KS_VIDEOINFOHEADER2,
}
impl Default for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATAFORMAT_VIDEOINFO_PALETTE {
    pub DataFormat: KSDATAFORMAT,
    pub VideoInfo: KS_VIDEOINFO,
}
impl Default for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_ANALOGVIDEO {
    pub DataRange: KSDATAFORMAT,
    pub AnalogVideoInfo: KS_ANALOGVIDEOINFO,
}
impl Default for KS_DATARANGE_ANALOGVIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_H264_VIDEO {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_H264VIDEOINFO,
}
impl Default for KS_DATARANGE_H264_VIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_IMAGE {
    pub DataRange: KSDATAFORMAT,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub ImageInfoHeader: KS_BITMAPINFOHEADER,
}
impl Default for KS_DATARANGE_IMAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_MPEG1_VIDEO {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_MPEG1VIDEOINFO,
}
impl Default for KS_DATARANGE_MPEG1_VIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_MPEG2_VIDEO {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_MPEGVIDEOINFO2,
}
impl Default for KS_DATARANGE_MPEG2_VIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_VIDEO {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER,
}
impl Default for KS_DATARANGE_VIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_VIDEO2 {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfoHeader: KS_VIDEOINFOHEADER2,
}
impl Default for KS_DATARANGE_VIDEO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_VIDEO_PALETTE {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VideoInfo: KS_VIDEOINFO,
}
impl Default for KS_DATARANGE_VIDEO_PALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_DATARANGE_VIDEO_VBI {
    pub DataRange: KSDATAFORMAT,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub StreamDescriptionFlags: u32,
    pub MemoryAllocationFlags: u32,
    pub ConfigCaps: KS_VIDEO_STREAM_CONFIG_CAPS,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
impl Default for KS_DATARANGE_VIDEO_VBI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_DVDCOPYSTATE(pub i32);
pub const KS_DVDCOPYSTATE_AUTHENTICATION_NOT_REQUIRED: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(2i32);
pub const KS_DVDCOPYSTATE_AUTHENTICATION_REQUIRED: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(3i32);
pub const KS_DVDCOPYSTATE_DONE: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(4i32);
pub const KS_DVDCOPYSTATE_INITIALIZE: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(0i32);
pub const KS_DVDCOPYSTATE_INITIALIZE_TITLE: KS_DVDCOPYSTATE = KS_DVDCOPYSTATE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_DVDCOPY_BUSKEY {
    pub BusKey: [u8; 5],
    pub Reserved: [u8; 1],
}
impl Default for KS_DVDCOPY_BUSKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_DVDCOPY_CHLGKEY {
    pub ChlgKey: [u8; 10],
    pub Reserved: [u8; 2],
}
impl Default for KS_DVDCOPY_CHLGKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_DVDCOPY_DISCKEY {
    pub DiscKey: [u8; 2048],
}
impl Default for KS_DVDCOPY_DISCKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_DVDCOPY_REGION {
    pub Reserved: u8,
    pub RegionData: u8,
    pub Reserved2: [u8; 2],
}
impl Default for KS_DVDCOPY_REGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_DVDCOPY_SET_COPY_STATE {
    pub DVDCopyState: u32,
}
impl Default for KS_DVDCOPY_SET_COPY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_DVDCOPY_TITLEKEY {
    pub KeyFlags: u32,
    pub ReservedNT: [u32; 2],
    pub TitleKey: [u8; 6],
    pub Reserved: [u8; 2],
}
impl Default for KS_DVDCOPY_TITLEKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_DVD_CGMS_COPY_ONCE: u32 = 16u32;
pub const KS_DVD_CGMS_COPY_PERMITTED: u32 = 0u32;
pub const KS_DVD_CGMS_COPY_PROTECT_MASK: u32 = 24u32;
pub const KS_DVD_CGMS_NO_COPY: u32 = 24u32;
pub const KS_DVD_CGMS_RESERVED_MASK: u32 = 120u32;
pub const KS_DVD_COPYRIGHTED: u32 = 64u32;
pub const KS_DVD_COPYRIGHT_MASK: u32 = 64u32;
pub const KS_DVD_NOT_COPYRIGHTED: u32 = 0u32;
pub const KS_DVD_SECTOR_NOT_PROTECTED: u32 = 0u32;
pub const KS_DVD_SECTOR_PROTECTED: u32 = 32u32;
pub const KS_DVD_SECTOR_PROTECT_MASK: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_DVD_YCrCb {
    pub Reserved: u8,
    pub Y: u8,
    pub Cr: u8,
    pub Cb: u8,
}
impl Default for KS_DVD_YCrCb {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_DVD_YUV {
    pub Reserved: u8,
    pub Y: u8,
    pub V: u8,
    pub U: u8,
}
impl Default for KS_DVD_YUV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_FRAME_INFO {
    pub ExtendedHeaderSize: u32,
    pub dwFrameFlags: u32,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub hDirectDraw: super::super::Foundation::HANDLE,
    pub hSurfaceHandle: super::super::Foundation::HANDLE,
    pub DirectDrawRect: super::super::Foundation::RECT,
    pub Anonymous1: KS_FRAME_INFO_0,
    pub Reserved2: u32,
    pub Anonymous2: KS_FRAME_INFO_1,
}
impl Default for KS_FRAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KS_FRAME_INFO_0 {
    pub lSurfacePitch: i32,
    pub Reserved1: u32,
}
impl Default for KS_FRAME_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KS_FRAME_INFO_1 {
    pub Anonymous: KS_FRAME_INFO_1_0,
    pub FrameCompletionNumber: u64,
}
impl Default for KS_FRAME_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_FRAME_INFO_1_0 {
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl Default for KS_FRAME_INFO_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_FRAMING_ITEM {
    pub MemoryType: windows_core::GUID,
    pub BusType: windows_core::GUID,
    pub MemoryFlags: u32,
    pub BusFlags: u32,
    pub Flags: u32,
    pub Frames: u32,
    pub Anonymous: KS_FRAMING_ITEM_0,
    pub MemoryTypeWeight: u32,
    pub PhysicalRange: KS_FRAMING_RANGE,
    pub FramingRange: KS_FRAMING_RANGE_WEIGHTED,
}
impl Default for KS_FRAMING_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KS_FRAMING_ITEM_0 {
    pub FileAlignment: u32,
    pub FramePitch: i32,
}
impl Default for KS_FRAMING_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_FRAMING_RANGE {
    pub MinFrameSize: u32,
    pub MaxFrameSize: u32,
    pub Stepping: u32,
}
impl Default for KS_FRAMING_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_FRAMING_RANGE_WEIGHTED {
    pub Range: KS_FRAMING_RANGE,
    pub InPlaceWeight: u32,
    pub NotInPlaceWeight: u32,
}
impl Default for KS_FRAMING_RANGE_WEIGHTED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_H264VIDEOINFO {
    pub wWidth: u16,
    pub wHeight: u16,
    pub wSARwidth: u16,
    pub wSARheight: u16,
    pub wProfile: u16,
    pub bLevelIDC: u8,
    pub wConstrainedToolset: u16,
    pub bmSupportedUsages: u32,
    pub bmCapabilities: u16,
    pub bmSVCCapabilities: u32,
    pub bmMVCCapabilities: u32,
    pub dwFrameInterval: u32,
    pub bMaxCodecConfigDelay: u8,
    pub bmSupportedSliceModes: u8,
    pub bmSupportedSyncFrameTypes: u8,
    pub bResolutionScaling: u8,
    pub bSimulcastSupport: u8,
    pub bmSupportedRateControlModes: u8,
    pub wMaxMBperSecOneResolutionNoScalability: u16,
    pub wMaxMBperSecTwoResolutionsNoScalability: u16,
    pub wMaxMBperSecThreeResolutionsNoScalability: u16,
    pub wMaxMBperSecFourResolutionsNoScalability: u16,
    pub wMaxMBperSecOneResolutionTemporalScalability: u16,
    pub wMaxMBperSecTwoResolutionsTemporalScalablility: u16,
    pub wMaxMBperSecThreeResolutionsTemporalScalability: u16,
    pub wMaxMBperSecFourResolutionsTemporalScalability: u16,
    pub wMaxMBperSecOneResolutionTemporalQualityScalability: u16,
    pub wMaxMBperSecTwoResolutionsTemporalQualityScalability: u16,
    pub wMaxMBperSecThreeResolutionsTemporalQualityScalablity: u16,
    pub wMaxMBperSecFourResolutionsTemporalQualityScalability: u16,
    pub wMaxMBperSecOneResolutionTemporalSpatialScalability: u16,
    pub wMaxMBperSecTwoResolutionsTemporalSpatialScalability: u16,
    pub wMaxMBperSecThreeResolutionsTemporalSpatialScalablity: u16,
    pub wMaxMBperSecFourResolutionsTemporalSpatialScalability: u16,
    pub wMaxMBperSecOneResolutionFullScalability: u16,
    pub wMaxMBperSecTwoResolutionsFullScalability: u16,
    pub wMaxMBperSecThreeResolutionsFullScalability: u16,
    pub wMaxMBperSecFourResolutionsFullScalability: u16,
}
impl Default for KS_H264VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_INTERLACE_1FieldPerSample: u32 = 2u32;
pub const KS_INTERLACE_DisplayModeBobOnly: u32 = 0u32;
pub const KS_INTERLACE_DisplayModeBobOrWeave: u32 = 128u32;
pub const KS_INTERLACE_DisplayModeMask: u32 = 192u32;
pub const KS_INTERLACE_DisplayModeWeaveOnly: u32 = 64u32;
pub const KS_INTERLACE_Field1First: u32 = 4u32;
pub const KS_INTERLACE_FieldPatBothIrregular: u32 = 48u32;
pub const KS_INTERLACE_FieldPatBothRegular: u32 = 32u32;
pub const KS_INTERLACE_FieldPatField1Only: u32 = 0u32;
pub const KS_INTERLACE_FieldPatField2Only: u32 = 16u32;
pub const KS_INTERLACE_FieldPatternMask: u32 = 48u32;
pub const KS_INTERLACE_IsInterlaced: u32 = 1u32;
pub const KS_INTERLACE_UNUSED: u32 = 8u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_LogicalMemoryType(pub i32);
pub const KS_MACROVISION_DISABLED: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(0i32);
pub const KS_MACROVISION_LEVEL1: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(1i32);
pub const KS_MACROVISION_LEVEL2: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(2i32);
pub const KS_MACROVISION_LEVEL3: KS_COPY_MACROVISION_LEVEL = KS_COPY_MACROVISION_LEVEL(3i32);
pub const KS_MAX_SIZE_MPEG1_SEQUENCE_INFO: u32 = 140u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_MPEG1VIDEOINFO {
    pub hdr: KS_VIDEOINFOHEADER,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub bSequenceHeader: [u8; 1],
}
impl Default for KS_MPEG1VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_MPEG2Level(pub i32);
pub const KS_MPEG2Level_High: KS_MPEG2Level = KS_MPEG2Level(3i32);
pub const KS_MPEG2Level_High1440: KS_MPEG2Level = KS_MPEG2Level(2i32);
pub const KS_MPEG2Level_Low: KS_MPEG2Level = KS_MPEG2Level(0i32);
pub const KS_MPEG2Level_Main: KS_MPEG2Level = KS_MPEG2Level(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_MPEG2Profile(pub i32);
pub const KS_MPEG2Profile_High: KS_MPEG2Profile = KS_MPEG2Profile(4i32);
pub const KS_MPEG2Profile_Main: KS_MPEG2Profile = KS_MPEG2Profile(1i32);
pub const KS_MPEG2Profile_SNRScalable: KS_MPEG2Profile = KS_MPEG2Profile(2i32);
pub const KS_MPEG2Profile_Simple: KS_MPEG2Profile = KS_MPEG2Profile(0i32);
pub const KS_MPEG2Profile_SpatiallyScalable: KS_MPEG2Profile = KS_MPEG2Profile(3i32);
pub const KS_MPEG2_27MhzTimebase: u32 = 256u32;
pub const KS_MPEG2_DSS_UserData: u32 = 64u32;
pub const KS_MPEG2_DVB_UserData: u32 = 128u32;
pub const KS_MPEG2_DVDLine21Field1: u32 = 2u32;
pub const KS_MPEG2_DVDLine21Field2: u32 = 4u32;
pub const KS_MPEG2_DoPanScan: u32 = 1u32;
pub const KS_MPEG2_FilmCameraMode: u32 = 16u32;
pub const KS_MPEG2_LetterboxAnalogOut: u32 = 32u32;
pub const KS_MPEG2_SourceIsLetterboxed: u32 = 8u32;
pub const KS_MPEG2_WidescreenAnalogOut: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_MPEGAUDIOINFO {
    pub dwFlags: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
}
impl Default for KS_MPEGAUDIOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_MPEGAUDIOINFO_27MhzTimebase: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_MPEGVIDEOINFO2 {
    pub hdr: KS_VIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub bSequenceHeader: [u32; 1],
}
impl Default for KS_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_MemoryTypeAnyHost: KS_LogicalMemoryType = KS_LogicalMemoryType(6i32);
pub const KS_MemoryTypeDeviceHostMapped: KS_LogicalMemoryType = KS_LogicalMemoryType(3i32);
pub const KS_MemoryTypeDeviceSpecific: KS_LogicalMemoryType = KS_LogicalMemoryType(4i32);
pub const KS_MemoryTypeDontCare: KS_LogicalMemoryType = KS_LogicalMemoryType(0i32);
pub const KS_MemoryTypeKernelNonPaged: KS_LogicalMemoryType = KS_LogicalMemoryType(2i32);
pub const KS_MemoryTypeKernelPaged: KS_LogicalMemoryType = KS_LogicalMemoryType(1i32);
pub const KS_MemoryTypeUser: KS_LogicalMemoryType = KS_LogicalMemoryType(5i32);
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_ADVERTISER_BASE: u32 = 2224u32;
pub const KS_NABTS_GROUPID_LOCAL_CABLE_SYSTEM_CONTENT_BASE: u32 = 2208u32;
pub const KS_NABTS_GROUPID_MICROSOFT_RESERVED_TEST_DATA_BASE: u32 = 2288u32;
pub const KS_NABTS_GROUPID_NETWORK_WIDE_ADVERTISER_BASE: u32 = 2160u32;
pub const KS_NABTS_GROUPID_NETWORK_WIDE_CONTENT_BASE: u32 = 2144u32;
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_ADVERTISER_BASE: u32 = 2064u32;
pub const KS_NABTS_GROUPID_ORIGINAL_CONTENT_BASE: u32 = 2048u32;
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_ADVERTISER_BASE: u32 = 2096u32;
pub const KS_NABTS_GROUPID_PRODUCTION_COMPANY_CONTENT_BASE: u32 = 2080u32;
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_ADVERTISER_BASE: u32 = 2128u32;
pub const KS_NABTS_GROUPID_SYNDICATED_SHOW_CONTENT_BASE: u32 = 2112u32;
pub const KS_NABTS_GROUPID_TELEVISION_STATION_ADVERTISER_BASE: u32 = 2192u32;
pub const KS_NABTS_GROUPID_TELEVISION_STATION_CONTENT_BASE: u32 = 2176u32;
pub const KS_Obsolete_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = KS_VideoControlFlags(16i32);
pub const KS_Obsolete_VideoControlFlag_Trigger: KS_VideoControlFlags = KS_VideoControlFlags(32i32);
pub const KS_PhysConn_Audio_1394: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4103i32);
pub const KS_PhysConn_Audio_AESDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4099i32);
pub const KS_PhysConn_Audio_AUX: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4102i32);
pub const KS_PhysConn_Audio_AudioDecoder: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4105i32);
pub const KS_PhysConn_Audio_Line: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4097i32);
pub const KS_PhysConn_Audio_Mic: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4098i32);
pub const KS_PhysConn_Audio_SCSI: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4101i32);
pub const KS_PhysConn_Audio_SPDIFDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4100i32);
pub const KS_PhysConn_Audio_Tuner: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4096i32);
pub const KS_PhysConn_Audio_USB: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4104i32);
pub const KS_PhysConn_Video_1394: KS_PhysicalConnectorType = KS_PhysicalConnectorType(10i32);
pub const KS_PhysConn_Video_AUX: KS_PhysicalConnectorType = KS_PhysicalConnectorType(9i32);
pub const KS_PhysConn_Video_Composite: KS_PhysicalConnectorType = KS_PhysicalConnectorType(2i32);
pub const KS_PhysConn_Video_ParallelDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(7i32);
pub const KS_PhysConn_Video_RGB: KS_PhysicalConnectorType = KS_PhysicalConnectorType(4i32);
pub const KS_PhysConn_Video_SCART: KS_PhysicalConnectorType = KS_PhysicalConnectorType(14i32);
pub const KS_PhysConn_Video_SCSI: KS_PhysicalConnectorType = KS_PhysicalConnectorType(8i32);
pub const KS_PhysConn_Video_SVideo: KS_PhysicalConnectorType = KS_PhysicalConnectorType(3i32);
pub const KS_PhysConn_Video_SerialDigital: KS_PhysicalConnectorType = KS_PhysicalConnectorType(6i32);
pub const KS_PhysConn_Video_Tuner: KS_PhysicalConnectorType = KS_PhysicalConnectorType(1i32);
pub const KS_PhysConn_Video_USB: KS_PhysicalConnectorType = KS_PhysicalConnectorType(11i32);
pub const KS_PhysConn_Video_VideoDecoder: KS_PhysicalConnectorType = KS_PhysicalConnectorType(12i32);
pub const KS_PhysConn_Video_VideoEncoder: KS_PhysicalConnectorType = KS_PhysicalConnectorType(13i32);
pub const KS_PhysConn_Video_YRYBY: KS_PhysicalConnectorType = KS_PhysicalConnectorType(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_PhysicalConnectorType(pub i32);
pub const KS_PixAspectRatio_NTSC16x9: KS_AMPixAspectRatio = KS_AMPixAspectRatio(1i32);
pub const KS_PixAspectRatio_NTSC4x3: KS_AMPixAspectRatio = KS_AMPixAspectRatio(0i32);
pub const KS_PixAspectRatio_PAL16x9: KS_AMPixAspectRatio = KS_AMPixAspectRatio(3i32);
pub const KS_PixAspectRatio_PAL4x3: KS_AMPixAspectRatio = KS_AMPixAspectRatio(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
impl Default for KS_RGBQUAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_SECURE_CAMERA_SCENARIO_ID: windows_core::GUID = windows_core::GUID::from_u128(0xae53fc6e_8d89_4488_9d2e_4d008731c5fd);
pub const KS_SEEKING_AbsolutePositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_SEEKING_CAPABILITIES(pub i32);
pub const KS_SEEKING_CanGetCurrentPos: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(8i32);
pub const KS_SEEKING_CanGetDuration: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(32i32);
pub const KS_SEEKING_CanGetStopPos: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(16i32);
pub const KS_SEEKING_CanPlayBackwards: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(64i32);
pub const KS_SEEKING_CanSeekAbsolute: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(1i32);
pub const KS_SEEKING_CanSeekBackwards: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(4i32);
pub const KS_SEEKING_CanSeekForwards: KS_SEEKING_CAPABILITIES = KS_SEEKING_CAPABILITIES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_SEEKING_FLAGS(pub i32);
pub const KS_SEEKING_IncrementalPositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(3i32);
pub const KS_SEEKING_NoPositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(0i32);
pub const KS_SEEKING_PositioningBitsMask: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(3i32);
pub const KS_SEEKING_RelativePositioning: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(2i32);
pub const KS_SEEKING_ReturnTime: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(8i32);
pub const KS_SEEKING_SeekToKeyFrame: KS_SEEKING_FLAGS = KS_SEEKING_FLAGS(4i32);
pub const KS_StreamingHint_CompQuality: KS_VideoStreamingHints = KS_VideoStreamingHints(2048i32);
pub const KS_StreamingHint_CompWindowSize: KS_VideoStreamingHints = KS_VideoStreamingHints(4096i32);
pub const KS_StreamingHint_FrameInterval: KS_VideoStreamingHints = KS_VideoStreamingHints(256i32);
pub const KS_StreamingHint_KeyFrameRate: KS_VideoStreamingHints = KS_VideoStreamingHints(512i32);
pub const KS_StreamingHint_PFrameRate: KS_VideoStreamingHints = KS_VideoStreamingHints(1024i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_TRUECOLORINFO {
    pub dwBitMasks: [u32; 3],
    pub bmiColors: [KS_RGBQUAD; 256],
}
impl Default for KS_TRUECOLORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_TUNER_STRATEGY(pub i32);
pub const KS_TUNER_STRATEGY_DRIVER_TUNES: KS_TUNER_STRATEGY = KS_TUNER_STRATEGY(4i32);
pub const KS_TUNER_STRATEGY_PLL: KS_TUNER_STRATEGY = KS_TUNER_STRATEGY(1i32);
pub const KS_TUNER_STRATEGY_SIGNAL_STRENGTH: KS_TUNER_STRATEGY = KS_TUNER_STRATEGY(2i32);
pub const KS_TUNER_TUNING_COARSE: KS_TUNER_TUNING_FLAGS = KS_TUNER_TUNING_FLAGS(3i32);
pub const KS_TUNER_TUNING_EXACT: KS_TUNER_TUNING_FLAGS = KS_TUNER_TUNING_FLAGS(1i32);
pub const KS_TUNER_TUNING_FINE: KS_TUNER_TUNING_FLAGS = KS_TUNER_TUNING_FLAGS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_TUNER_TUNING_FLAGS(pub i32);
pub const KS_TVAUDIO_MODE_LANG_A: u32 = 16u32;
pub const KS_TVAUDIO_MODE_LANG_B: u32 = 32u32;
pub const KS_TVAUDIO_MODE_LANG_C: u32 = 64u32;
pub const KS_TVAUDIO_MODE_MONO: u32 = 1u32;
pub const KS_TVAUDIO_MODE_STEREO: u32 = 2u32;
pub const KS_TVAUDIO_PRESET_LANG_A: u32 = 4096u32;
pub const KS_TVAUDIO_PRESET_LANG_B: u32 = 8192u32;
pub const KS_TVAUDIO_PRESET_LANG_C: u32 = 16384u32;
pub const KS_TVAUDIO_PRESET_STEREO: u32 = 512u32;
pub const KS_TVTUNER_CHANGE_BEGIN_TUNE: i32 = 1i32;
pub const KS_TVTUNER_CHANGE_END_TUNE: i32 = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_TVTUNER_CHANGE_INFO {
    pub dwFlags: u32,
    pub dwCountryCode: u32,
    pub dwAnalogVideoStandard: u32,
    pub dwChannel: u32,
}
impl Default for KS_TVTUNER_CHANGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_VBICAP_PROTECTION_MV_DETECTED: i32 = 4i32;
pub const KS_VBICAP_PROTECTION_MV_HARDWARE: i32 = 2i32;
pub const KS_VBICAP_PROTECTION_MV_PRESENT: i32 = 1i32;
pub const KS_VBIDATARATE_CC: i32 = 503493i32;
pub const KS_VBIDATARATE_NABTS: i32 = 5727272i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_VBIINFOHEADER {
    pub StartLine: u32,
    pub EndLine: u32,
    pub SamplingFrequency: u32,
    pub MinLineStartTime: u32,
    pub MaxLineStartTime: u32,
    pub ActualLineStartTime: u32,
    pub ActualLineEndTime: u32,
    pub VideoStandard: u32,
    pub SamplesPerLine: u32,
    pub StrideInBytes: u32,
    pub BufferSize: u32,
}
impl Default for KS_VBIINFOHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_VBI_FLAG_FIELD1: i32 = 1i32;
pub const KS_VBI_FLAG_FIELD2: i32 = 2i32;
pub const KS_VBI_FLAG_FRAME: i32 = 0i32;
pub const KS_VBI_FLAG_MV_DETECTED: i32 = 1024i32;
pub const KS_VBI_FLAG_MV_HARDWARE: i32 = 512i32;
pub const KS_VBI_FLAG_MV_PRESENT: i32 = 256i32;
pub const KS_VBI_FLAG_TVTUNER_CHANGE: i32 = 16i32;
pub const KS_VBI_FLAG_VBIINFOHEADER_CHANGE: i32 = 32i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_VBI_FRAME_INFO {
    pub ExtendedHeaderSize: u32,
    pub dwFrameFlags: u32,
    pub PictureNumber: i64,
    pub DropCount: i64,
    pub dwSamplingFrequency: u32,
    pub TvTunerChangeInfo: KS_TVTUNER_CHANGE_INFO,
    pub VBIInfoHeader: KS_VBIINFOHEADER,
}
impl Default for KS_VBI_FRAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_VIDEODECODER_FLAGS(pub i32);
pub const KS_VIDEODECODER_FLAGS_CAN_DISABLE_OUTPUT: KS_VIDEODECODER_FLAGS = KS_VIDEODECODER_FLAGS(1i32);
pub const KS_VIDEODECODER_FLAGS_CAN_INDICATE_LOCKED: KS_VIDEODECODER_FLAGS = KS_VIDEODECODER_FLAGS(4i32);
pub const KS_VIDEODECODER_FLAGS_CAN_USE_VCR_LOCKING: KS_VIDEODECODER_FLAGS = KS_VIDEODECODER_FLAGS(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_VIDEOINFO {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: KS_BITMAPINFOHEADER,
    pub Anonymous: KS_VIDEOINFO_0,
}
impl Default for KS_VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KS_VIDEOINFO_0 {
    pub bmiColors: [KS_RGBQUAD; 256],
    pub dwBitMasks: [u32; 3],
    pub TrueColorInfo: KS_TRUECOLORINFO,
}
impl Default for KS_VIDEOINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_VIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: KS_BITMAPINFOHEADER,
}
impl Default for KS_VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_VIDEOINFOHEADER2 {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub Anonymous: KS_VIDEOINFOHEADER2_0,
    pub dwReserved2: u32,
    pub bmiHeader: KS_BITMAPINFOHEADER,
}
impl Default for KS_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KS_VIDEOINFOHEADER2_0 {
    pub dwControlFlags: u32,
    pub dwReserved1: u32,
}
impl Default for KS_VIDEOINFOHEADER2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_VIDEOSTREAM_CAPTURE: u32 = 2u32;
pub const KS_VIDEOSTREAM_CC: u32 = 256u32;
pub const KS_VIDEOSTREAM_EDS: u32 = 512u32;
pub const KS_VIDEOSTREAM_IS_VPE: u32 = 32768u32;
pub const KS_VIDEOSTREAM_NABTS: u32 = 32u32;
pub const KS_VIDEOSTREAM_PREVIEW: u32 = 1u32;
pub const KS_VIDEOSTREAM_STILL: u32 = 4096u32;
pub const KS_VIDEOSTREAM_TELETEXT: u32 = 1024u32;
pub const KS_VIDEOSTREAM_VBI: u32 = 16u32;
pub const KS_VIDEO_ALLOC_VPE_AGP: u32 = 4u32;
pub const KS_VIDEO_ALLOC_VPE_DISPLAY: u32 = 2u32;
pub const KS_VIDEO_ALLOC_VPE_SYSTEM: u32 = 1u32;
pub const KS_VIDEO_FLAG_B_FRAME: i32 = 32i32;
pub const KS_VIDEO_FLAG_FIELD1: i32 = 1i32;
pub const KS_VIDEO_FLAG_FIELD1FIRST: i32 = 4i32;
pub const KS_VIDEO_FLAG_FIELD2: i32 = 2i32;
pub const KS_VIDEO_FLAG_FIELD_MASK: i32 = 3i32;
pub const KS_VIDEO_FLAG_FRAME: i32 = 0i32;
pub const KS_VIDEO_FLAG_IPB_MASK: i32 = 48i32;
pub const KS_VIDEO_FLAG_I_FRAME: i32 = 0i32;
pub const KS_VIDEO_FLAG_P_FRAME: i32 = 16i32;
pub const KS_VIDEO_FLAG_REPEAT_FIELD: i32 = 64i32;
pub const KS_VIDEO_FLAG_WEAVE: i32 = 8i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_VIDEO_STREAM_CONFIG_CAPS {
    pub guid: windows_core::GUID,
    pub VideoStandard: u32,
    pub InputSize: super::super::Foundation::SIZE,
    pub MinCroppingSize: super::super::Foundation::SIZE,
    pub MaxCroppingSize: super::super::Foundation::SIZE,
    pub CropGranularityX: i32,
    pub CropGranularityY: i32,
    pub CropAlignX: i32,
    pub CropAlignY: i32,
    pub MinOutputSize: super::super::Foundation::SIZE,
    pub MaxOutputSize: super::super::Foundation::SIZE,
    pub OutputGranularityX: i32,
    pub OutputGranularityY: i32,
    pub StretchTapsX: i32,
    pub StretchTapsY: i32,
    pub ShrinkTapsX: i32,
    pub ShrinkTapsY: i32,
    pub MinFrameInterval: i64,
    pub MaxFrameInterval: i64,
    pub MinBitsPerSecond: i32,
    pub MaxBitsPerSecond: i32,
}
impl Default for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KS_VideoControlFlag_ExternalTriggerEnable: KS_VideoControlFlags = KS_VideoControlFlags(4i32);
pub const KS_VideoControlFlag_FlipHorizontal: KS_VideoControlFlags = KS_VideoControlFlags(1i32);
pub const KS_VideoControlFlag_FlipVertical: KS_VideoControlFlags = KS_VideoControlFlags(2i32);
pub const KS_VideoControlFlag_IndependentImagePin: KS_VideoControlFlags = KS_VideoControlFlags(64i32);
pub const KS_VideoControlFlag_StartPhotoSequenceCapture: KS_VideoControlFlags = KS_VideoControlFlags(256i32);
pub const KS_VideoControlFlag_StillCapturePreviewFrame: KS_VideoControlFlags = KS_VideoControlFlags(128i32);
pub const KS_VideoControlFlag_StopPhotoSequenceCapture: KS_VideoControlFlags = KS_VideoControlFlags(512i32);
pub const KS_VideoControlFlag_Trigger: KS_VideoControlFlags = KS_VideoControlFlags(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_VideoControlFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_VideoStreamingHints(pub i32);
pub const KS_iBLUE: u32 = 2u32;
pub const KS_iEGA_COLORS: u32 = 16u32;
pub const KS_iGREEN: u32 = 1u32;
pub const KS_iMASK_COLORS: u32 = 3u32;
pub const KS_iMAXBITS: u32 = 8u32;
pub const KS_iPALETTE: u32 = 8u32;
pub const KS_iPALETTE_COLORS: u32 = 256u32;
pub const KS_iRED: u32 = 0u32;
pub const KS_iTRUECOLOR: u32 = 16u32;
pub const KsAllocatorMode_Kernel: KSALLOCATORMODE = KSALLOCATORMODE(1i32);
pub const KsAllocatorMode_User: KSALLOCATORMODE = KSALLOCATORMODE(0i32);
pub const KsIoOperation_Read: KSIOOPERATION = KSIOOPERATION(1i32);
pub const KsIoOperation_Write: KSIOOPERATION = KSIOOPERATION(0i32);
pub const KsPeekOperation_AddRef: KSPEEKOPERATION = KSPEEKOPERATION(1i32);
pub const KsPeekOperation_PeekOnly: KSPEEKOPERATION = KSPEEKOPERATION(0i32);
pub const LIGHT_FILTER: KSDS3D_HRTF_FILTER_QUALITY = KSDS3D_HRTF_FILTER_QUALITY(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOOPEDSTREAMING_POSITION_EVENT_DATA {
    pub KsEventData: KSEVENTDATA,
    pub Position: u64,
}
impl Default for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAX_NABTS_VBI_LINES_PER_FIELD: u32 = 11u32;
pub const MAX_RESOURCEGROUPID_LENGTH: u32 = 256u32;
pub const MAX_SINK_DESCRIPTION_NAME_LENGTH: u32 = 32u32;
pub const MAX_WST_VBI_LINES_PER_FIELD: u32 = 17u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEDIUM_INFO {
    pub MediaPresent: super::super::Foundation::BOOL,
    pub MediaType: u32,
    pub RecordInhibit: super::super::Foundation::BOOL,
}
impl Default for MEDIUM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MF_MDL_SHARED_PAYLOAD_KEY {
    pub combined: MF_MDL_SHARED_PAYLOAD_KEY_0,
    pub GMDLHandle: windows_core::GUID,
}
impl Default for MF_MDL_SHARED_PAYLOAD_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MF_MDL_SHARED_PAYLOAD_KEY_0 {
    pub pHandle: u32,
    pub fHandle: u32,
    pub uPayload: u64,
}
impl Default for MF_MDL_SHARED_PAYLOAD_KEY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIN_DEV_VER_FOR_FLAGS: u32 = 272u32;
pub const MIN_DEV_VER_FOR_QI: u32 = 256u32;
pub const MetadataId_BackgroundSegmentationMask: KSCAMERA_MetadataId = KSCAMERA_MetadataId(8i32);
pub const MetadataId_CameraExtrinsics: KSCAMERA_MetadataId = KSCAMERA_MetadataId(4i32);
pub const MetadataId_CameraIntrinsics: KSCAMERA_MetadataId = KSCAMERA_MetadataId(5i32);
pub const MetadataId_CaptureStats: KSCAMERA_MetadataId = KSCAMERA_MetadataId(3i32);
pub const MetadataId_Custom_Start: KSCAMERA_MetadataId = KSCAMERA_MetadataId(-2147483648i32);
pub const MetadataId_DigitalWindow: KSCAMERA_MetadataId = KSCAMERA_MetadataId(7i32);
pub const MetadataId_FrameIllumination: KSCAMERA_MetadataId = KSCAMERA_MetadataId(6i32);
pub const MetadataId_PhotoConfirmation: KSCAMERA_MetadataId = KSCAMERA_MetadataId(1i32);
pub const MetadataId_Standard_End: KSCAMERA_MetadataId = KSCAMERA_MetadataId(8i32);
pub const MetadataId_Standard_Start: KSCAMERA_MetadataId = KSCAMERA_MetadataId(1i32);
pub const MetadataId_UsbVideoHeader: KSCAMERA_MetadataId = KSCAMERA_MetadataId(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NABTSFEC_BUFFER {
    pub dataSize: u32,
    pub groupID: u16,
    pub Reserved: u16,
    pub data: [u8; 448],
}
impl Default for NABTSFEC_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NABTS_BUFFER {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub PictureNumber: i64,
    pub NabtsLines: [NABTS_BUFFER_LINE; 11],
}
impl Default for NABTS_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NABTS_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 36],
}
impl Default for NABTS_BUFFER_LINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NABTS_BUFFER_PICTURENUMBER_SUPPORT: u32 = 1u32;
pub const NABTS_BYTES_PER_LINE: u32 = 36u32;
pub const NABTS_LINES_PER_BUNDLE: u32 = 16u32;
pub const NABTS_PAYLOAD_PER_LINE: u32 = 28u32;
pub const NANOSECONDS: u32 = 10000000u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTIMAL_WEIGHT_TOTALS {
    pub MinTotalNominator: i64,
    pub MaxTotalNominator: i64,
    pub TotalDenominator: i64,
}
impl Default for OPTIMAL_WEIGHT_TOTALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PINNAME_DISPLAYPORT_OUT: windows_core::GUID = windows_core::GUID::from_u128(0x21fbb329_1a4a_48da_a076_2318a3c59b26);
pub const PINNAME_HDMI_OUT: windows_core::GUID = windows_core::GUID::from_u128(0x387bfc03_e7ef_4901_86e0_35b7c32b00ef);
pub const PINNAME_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x38a0cd98_d49b_4ce8_b48a_344667a17830);
pub const PINNAME_SPDIF_IN: windows_core::GUID = windows_core::GUID::from_u128(0x15dc9025_22ad_41b3_8875_f4ceb0299e20);
pub const PINNAME_SPDIF_OUT: windows_core::GUID = windows_core::GUID::from_u128(0x3a264481_e52c_4b82_8e7a_c8e2f91dc380);
pub const PINNAME_VIDEO_ANALOGVIDEOIN: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4283_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4281_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_CC: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4289_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_CC_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x1aad8061_012d_11d2_b4b1_00a0d102cfbe);
pub const PINNAME_VIDEO_EDS: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4287_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_NABTS: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4286_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_NABTS_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x29703660_498a_11d2_b4b1_00a0d102cfbe);
pub const PINNAME_VIDEO_PREVIEW: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4282_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_STILL: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c428a_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_TELETEXT: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4288_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_TIMECODE: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c428b_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_VBI: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4284_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_VIDEOPORT: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c4285_0353_11d1_905f_0000c0cc16ba);
pub const PINNAME_VIDEO_VIDEOPORT_VBI: windows_core::GUID = windows_core::GUID::from_u128(0xfb6c428c_0353_11d1_905f_0000c0cc16ba);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PIPE_ALLOCATOR_PLACE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PIPE_DIMENSIONS {
    pub AllocatorPin: KS_COMPRESSION,
    pub MaxExpansionPin: KS_COMPRESSION,
    pub EndPin: KS_COMPRESSION,
}
impl Default for PIPE_DIMENSIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PIPE_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PIPE_TERMINATION {
    pub Flags: u32,
    pub OutsideFactors: u32,
    pub Weigth: u32,
    pub PhysicalRange: KS_FRAMING_RANGE,
    pub OptimalRange: KS_FRAMING_RANGE_WEIGHTED,
    pub Compression: KS_COMPRESSION,
}
impl Default for PIPE_TERMINATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROPSETID_ALLOCATOR_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x53171960_148e_11d2_9979_0000c0cc16ba);
pub const PROPSETID_EXT_DEVICE: windows_core::GUID = windows_core::GUID::from_u128(0xb5730a90_1a2c_11cf_8c23_00aa006b6814);
pub const PROPSETID_EXT_TRANSPORT: windows_core::GUID = windows_core::GUID::from_u128(0xa03cd5f0_3045_11cf_8c44_00aa006b6814);
pub const PROPSETID_TIMECODE_READER: windows_core::GUID = windows_core::GUID::from_u128(0x9b496ce1_811b_11cf_8c77_00aa006b6814);
pub const PROPSETID_TUNER: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0605_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_CAMERACONTROL: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13370_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_CAMERACONTROL_FLASH: windows_core::GUID = windows_core::GUID::from_u128(0x785e8f49_63a2_4144_ab70_ffb278fa26ce);
pub const PROPSETID_VIDCAP_CAMERACONTROL_IMAGE_PIN_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x9d3d7bbf_5c6d_4138_bb00_584edd20f7c5);
pub const PROPSETID_VIDCAP_CAMERACONTROL_REGION_OF_INTEREST: windows_core::GUID = windows_core::GUID::from_u128(0x9d12d198_f86c_4fed_b023_5d87653da793);
pub const PROPSETID_VIDCAP_CAMERACONTROL_VIDEO_STABILIZATION: windows_core::GUID = windows_core::GUID::from_u128(0x43964bd3_7716_404e_8be1_d299b20e50fd);
pub const PROPSETID_VIDCAP_CROSSBAR: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0640_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_DROPPEDFRAMES: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13344_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_SELECTOR: windows_core::GUID = windows_core::GUID::from_u128(0x1abdaeca_68b6_4f83_9371_b413907c7b9f);
pub const PROPSETID_VIDCAP_TVAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0650_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOCOMPRESSION: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13343_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOCONTROL: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0670_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEODECODER: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13350_30ac_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x6a2e0610_28e4_11d0_a18c_00a0c9118956);
pub const PROPSETID_VIDCAP_VIDEOPROCAMP: windows_core::GUID = windows_core::GUID::from_u128(0xc6e13360_30ac_11d0_a18c_00a0c9118956);
pub const PipeFactor_Align: u32 = 512u32;
pub const PipeFactor_Buffers: u32 = 256u32;
pub const PipeFactor_FixedCompression: u32 = 64u32;
pub const PipeFactor_Flags: u32 = 8u32;
pub const PipeFactor_LogicalEnd: u32 = 2048u32;
pub const PipeFactor_MemoryTypes: u32 = 4u32;
pub const PipeFactor_None: u32 = 0u32;
pub const PipeFactor_OptimalRanges: u32 = 32u32;
pub const PipeFactor_PhysicalEnd: u32 = 1024u32;
pub const PipeFactor_PhysicalRanges: u32 = 16u32;
pub const PipeFactor_UnknownCompression: u32 = 128u32;
pub const PipeFactor_UserModeDownstream: u32 = 2u32;
pub const PipeFactor_UserModeUpstream: u32 = 1u32;
pub const PipeState_CompressionUnknown: PIPE_STATE = PIPE_STATE(3i32);
pub const PipeState_DontCare: PIPE_STATE = PIPE_STATE(0i32);
pub const PipeState_Finalized: PIPE_STATE = PIPE_STATE(4i32);
pub const PipeState_RangeFixed: PIPE_STATE = PIPE_STATE(2i32);
pub const PipeState_RangeNotFixed: PIPE_STATE = PIPE_STATE(1i32);
pub const Pipe_Allocator_FirstPin: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(1i32);
pub const Pipe_Allocator_LastPin: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(2i32);
pub const Pipe_Allocator_MiddlePin: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(3i32);
pub const Pipe_Allocator_None: PIPE_ALLOCATOR_PLACE = PIPE_ALLOCATOR_PLACE(0i32);
pub const RT_RCDATA: windows_core::PCWSTR = windows_core::PCWSTR(10u16 as _);
pub const RT_STRING: windows_core::PCWSTR = windows_core::PCWSTR(6u16 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECURE_BUFFER_INFO {
    pub guidBufferIdentifier: windows_core::GUID,
    pub cbBufferSize: u32,
    pub cbCaptured: u32,
    pub ullReserved: [u64; 16],
}
impl Default for SECURE_BUFFER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHORT_COEFF: KSDS3D_HRTF_COEFF_FORMAT = KSDS3D_HRTF_COEFF_FORMAT(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOUNDDETECTOR_PATTERNHEADER {
    pub Size: u32,
    pub PatternType: windows_core::GUID,
}
impl Default for SOUNDDETECTOR_PATTERNHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPEAKER_ALL: u32 = 2147483648u32;
pub const SPEAKER_BACK_CENTER: u32 = 256u32;
pub const SPEAKER_BACK_LEFT: u32 = 16u32;
pub const SPEAKER_BACK_RIGHT: u32 = 32u32;
pub const SPEAKER_FRONT_CENTER: u32 = 4u32;
pub const SPEAKER_FRONT_LEFT: u32 = 1u32;
pub const SPEAKER_FRONT_LEFT_OF_CENTER: u32 = 64u32;
pub const SPEAKER_FRONT_RIGHT: u32 = 2u32;
pub const SPEAKER_FRONT_RIGHT_OF_CENTER: u32 = 128u32;
pub const SPEAKER_LOW_FREQUENCY: u32 = 8u32;
pub const SPEAKER_RESERVED: u32 = 2147221504u32;
pub const SPEAKER_SIDE_LEFT: u32 = 512u32;
pub const SPEAKER_SIDE_RIGHT: u32 = 1024u32;
pub const SPEAKER_TOP_BACK_CENTER: u32 = 65536u32;
pub const SPEAKER_TOP_BACK_LEFT: u32 = 32768u32;
pub const SPEAKER_TOP_BACK_RIGHT: u32 = 131072u32;
pub const SPEAKER_TOP_CENTER: u32 = 2048u32;
pub const SPEAKER_TOP_FRONT_CENTER: u32 = 8192u32;
pub const SPEAKER_TOP_FRONT_LEFT: u32 = 4096u32;
pub const SPEAKER_TOP_FRONT_RIGHT: u32 = 16384u32;
pub const SYSAUDIO_FLAGS_CLEAR_PREFERRED: u32 = 2u32;
pub const SYSAUDIO_FLAGS_DONT_COMBINE_PINS: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TELEPHONY_CALLCONTROLOP(pub i32);
pub const TELEPHONY_CALLCONTROLOP_DISABLE: TELEPHONY_CALLCONTROLOP = TELEPHONY_CALLCONTROLOP(0i32);
pub const TELEPHONY_CALLCONTROLOP_ENABLE: TELEPHONY_CALLCONTROLOP = TELEPHONY_CALLCONTROLOP(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TELEPHONY_CALLSTATE(pub i32);
pub const TELEPHONY_CALLSTATE_DISABLED: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(0i32);
pub const TELEPHONY_CALLSTATE_ENABLED: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(1i32);
pub const TELEPHONY_CALLSTATE_HOLD: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(2i32);
pub const TELEPHONY_CALLSTATE_PROVIDERTRANSITION: TELEPHONY_CALLSTATE = TELEPHONY_CALLSTATE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TELEPHONY_CALLTYPE(pub i32);
pub const TELEPHONY_CALLTYPE_CIRCUITSWITCHED: TELEPHONY_CALLTYPE = TELEPHONY_CALLTYPE(0i32);
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_LTE: TELEPHONY_CALLTYPE = TELEPHONY_CALLTYPE(1i32);
pub const TELEPHONY_CALLTYPE_PACKETSWITCHED_WLAN: TELEPHONY_CALLTYPE = TELEPHONY_CALLTYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TELEPHONY_PROVIDERCHANGEOP(pub i32);
pub const TELEPHONY_PROVIDERCHANGEOP_BEGIN: TELEPHONY_PROVIDERCHANGEOP = TELEPHONY_PROVIDERCHANGEOP(1i32);
pub const TELEPHONY_PROVIDERCHANGEOP_CANCEL: TELEPHONY_PROVIDERCHANGEOP = TELEPHONY_PROVIDERCHANGEOP(2i32);
pub const TELEPHONY_PROVIDERCHANGEOP_END: TELEPHONY_PROVIDERCHANGEOP = TELEPHONY_PROVIDERCHANGEOP(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSPORTAUDIOPARMS {
    pub EnableOutput: i32,
    pub EnableRecord: i32,
    pub EnableSelsync: i32,
    pub Input: i32,
    pub MonitorSource: i32,
}
impl Default for TRANSPORTAUDIOPARMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSPORTBASICPARMS {
    pub TimeFormat: i32,
    pub TimeReference: i32,
    pub Superimpose: i32,
    pub EndStopAction: i32,
    pub RecordFormat: i32,
    pub StepFrames: i32,
    pub SetpField: i32,
    pub Preroll: i32,
    pub RecPreroll: i32,
    pub Postroll: i32,
    pub EditDelay: i32,
    pub PlayTCDelay: i32,
    pub RecTCDelay: i32,
    pub EditField: i32,
    pub FrameServo: i32,
    pub ColorFrameServo: i32,
    pub ServoRef: i32,
    pub WarnGenlock: i32,
    pub SetTracking: i32,
    pub VolumeName: [i8; 40],
    pub Ballistic: [i32; 20],
    pub Speed: i32,
    pub CounterFormat: i32,
    pub TunerChannel: i32,
    pub TunerNumber: i32,
    pub TimerEvent: i32,
    pub TimerStartDay: i32,
    pub TimerStartTime: i32,
    pub TimerStopDay: i32,
    pub TimerStopTime: i32,
}
impl Default for TRANSPORTBASICPARMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSPORTSTATUS {
    pub Mode: i32,
    pub LastError: i32,
    pub RecordInhibit: i32,
    pub ServoLock: i32,
    pub MediaPresent: i32,
    pub MediaLength: i32,
    pub MediaSize: i32,
    pub MediaTrackCount: i32,
    pub MediaTrackLength: i32,
    pub MediaTrackSide: i32,
    pub MediaType: i32,
    pub LinkMode: i32,
    pub NotifyOn: i32,
}
impl Default for TRANSPORTSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSPORTVIDEOPARMS {
    pub OutputMode: i32,
    pub Input: i32,
}
impl Default for TRANSPORTVIDEOPARMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSPORT_STATE {
    pub Mode: u32,
    pub State: u32,
}
impl Default for TRANSPORT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TUNER_ANALOG_CAPS_S {
    pub Mode: u32,
    pub StandardsSupported: u32,
    pub MinFrequency: u32,
    pub MaxFrequency: u32,
    pub TuningGranularity: u32,
    pub SettlingTime: u32,
    pub ScanSensingRange: u32,
    pub FineTuneSensingRange: u32,
}
impl Default for TUNER_ANALOG_CAPS_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TunerLockType(pub i32);
pub const Tuner_LockType_Locked: TunerLockType = TunerLockType(2i32);
pub const Tuner_LockType_None: TunerLockType = TunerLockType(0i32);
pub const Tuner_LockType_Within_Scan_Sensing_Range: TunerLockType = TunerLockType(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VBICAP_PROPERTIES_PROTECTION_S {
    pub Property: KSIDENTIFIER,
    pub StreamIndex: u32,
    pub Status: u32,
}
impl Default for VBICAP_PROPERTIES_PROTECTION_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_CC_SUBSTREAMS {
    pub SubstreamMask: u32,
}
impl Default for VBICODECFILTERING_CC_SUBSTREAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_NABTS_SUBSTREAMS {
    pub SubstreamMask: [u32; 128],
}
impl Default for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_SCANLINES {
    pub DwordBitArray: [u32; 32],
}
impl Default for VBICODECFILTERING_SCANLINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_CC {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
impl Default for VBICODECFILTERING_STATISTICS_CC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_CC_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl Default for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_COMMON {
    pub InputSRBsProcessed: u32,
    pub OutputSRBsProcessed: u32,
    pub SRBsIgnored: u32,
    pub InputSRBsMissing: u32,
    pub OutputSRBsMissing: u32,
    pub OutputFailures: u32,
    pub InternalErrors: u32,
    pub ExternalErrors: u32,
    pub InputDiscontinuities: u32,
    pub DSPFailures: u32,
    pub TvTunerChanges: u32,
    pub VBIHeaderChanges: u32,
    pub LineConfidenceAvg: u32,
    pub BytesOutput: u32,
}
impl Default for VBICODECFILTERING_STATISTICS_COMMON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_COMMON_PIN {
    pub SRBsProcessed: u32,
    pub SRBsIgnored: u32,
    pub SRBsMissing: u32,
    pub InternalErrors: u32,
    pub ExternalErrors: u32,
    pub Discontinuities: u32,
    pub LineConfidenceAvg: u32,
    pub BytesOutput: u32,
}
impl Default for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_NABTS {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
    pub FECBundleBadLines: u32,
    pub FECQueueOverflows: u32,
    pub FECCorrectedLines: u32,
    pub FECUncorrectableLines: u32,
    pub BundlesProcessed: u32,
    pub BundlesSent2IP: u32,
    pub FilteredLines: u32,
}
impl Default for VBICODECFILTERING_STATISTICS_NABTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_NABTS_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl Default for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON,
}
impl Default for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    pub Common: VBICODECFILTERING_STATISTICS_COMMON_PIN,
}
impl Default for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VRAM_SURFACE_INFO {
    pub hSurface: usize,
    pub VramPhysicalAddress: i64,
    pub cbCaptured: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwLinearSize: u32,
    pub lPitch: i32,
    pub ullReserved: [u64; 16],
}
impl Default for VRAM_SURFACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VRAM_SURFACE_INFO_PROPERTY_S {
    pub Property: KSIDENTIFIER,
    pub pVramSurfaceInfo: *mut VRAM_SURFACE_INFO,
}
impl Default for VRAM_SURFACE_INFO_PROPERTY_S {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WAVE_FORMAT_EXTENSIBLE: u32 = 65534u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WNF_KSCAMERA_STREAMSTATE_INFO {
    pub ProcessId: u32,
    pub SessionId: u32,
    pub StreamState: u32,
    pub Reserved: u32,
}
impl Default for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WST_BUFFER {
    pub ScanlinesRequested: VBICODECFILTERING_SCANLINES,
    pub WstLines: [WST_BUFFER_LINE; 17],
}
impl Default for WST_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WST_BUFFER_LINE {
    pub Confidence: u8,
    pub Bytes: [u8; 42],
}
impl Default for WST_BUFFER_LINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WST_BYTES_PER_LINE: u32 = 42u32;
pub const WST_TVTUNER_CHANGE_BEGIN_TUNE: i32 = 4096i32;
pub const WST_TVTUNER_CHANGE_END_TUNE: i32 = 8192i32;
pub const eConnType3Point5mm: EPcxConnectionType = EPcxConnectionType(1i32);
pub const eConnTypeAtapiInternal: EPcxConnectionType = EPcxConnectionType(3i32);
pub const eConnTypeCombination: EPcxConnectionType = EPcxConnectionType(11i32);
pub const eConnTypeMultichannelAnalogDIN: EPcxConnectionType = EPcxConnectionType(8i32);
pub const eConnTypeOptical: EPcxConnectionType = EPcxConnectionType(5i32);
pub const eConnTypeOtherAnalog: EPcxConnectionType = EPcxConnectionType(7i32);
pub const eConnTypeOtherDigital: EPcxConnectionType = EPcxConnectionType(6i32);
pub const eConnTypeQuarter: EPcxConnectionType = EPcxConnectionType(2i32);
pub const eConnTypeRCA: EPcxConnectionType = EPcxConnectionType(4i32);
pub const eConnTypeRJ11Modem: EPcxConnectionType = EPcxConnectionType(10i32);
pub const eConnTypeUnknown: EPcxConnectionType = EPcxConnectionType(0i32);
pub const eConnTypeXlrProfessional: EPcxConnectionType = EPcxConnectionType(9i32);
pub const eDeviceControlUseMissing: EDeviceControlUseType = EDeviceControlUseType(0i32);
pub const eDeviceControlUsePrimary: EDeviceControlUseType = EDeviceControlUseType(1i32);
pub const eDeviceControlUseSecondary: EDeviceControlUseType = EDeviceControlUseType(2i32);
pub const eGenLocInternal: EPcxGenLocation = EPcxGenLocation(1i32);
pub const eGenLocOther: EPcxGenLocation = EPcxGenLocation(3i32);
pub const eGenLocPrimaryBox: EPcxGenLocation = EPcxGenLocation(0i32);
pub const eGenLocSeparate: EPcxGenLocation = EPcxGenLocation(2i32);
pub const eGeoLocATAPI: EPcxGeoLocation = EPcxGeoLocation(13i32);
pub const eGeoLocBottom: EPcxGeoLocation = EPcxGeoLocation(6i32);
pub const eGeoLocDrivebay: EPcxGeoLocation = EPcxGeoLocation(10i32);
pub const eGeoLocFront: EPcxGeoLocation = EPcxGeoLocation(2i32);
pub const eGeoLocHDMI: EPcxGeoLocation = EPcxGeoLocation(11i32);
pub const eGeoLocInsideMobileLid: EPcxGeoLocation = EPcxGeoLocation(9i32);
pub const eGeoLocLeft: EPcxGeoLocation = EPcxGeoLocation(3i32);
pub const eGeoLocNotApplicable: EPcxGeoLocation = EPcxGeoLocation(14i32);
pub const eGeoLocOutsideMobileLid: EPcxGeoLocation = EPcxGeoLocation(12i32);
pub const eGeoLocRear: EPcxGeoLocation = EPcxGeoLocation(1i32);
pub const eGeoLocRearPanel: EPcxGeoLocation = EPcxGeoLocation(7i32);
pub const eGeoLocReserved6: EPcxGeoLocation = EPcxGeoLocation(15i32);
pub const eGeoLocRight: EPcxGeoLocation = EPcxGeoLocation(4i32);
pub const eGeoLocRiser: EPcxGeoLocation = EPcxGeoLocation(8i32);
pub const eGeoLocTop: EPcxGeoLocation = EPcxGeoLocation(5i32);
pub const ePortConnBothIntegratedAndJack: EPxcPortConnection = EPxcPortConnection(2i32);
pub const ePortConnIntegratedDevice: EPxcPortConnection = EPxcPortConnection(1i32);
pub const ePortConnJack: EPxcPortConnection = EPxcPortConnection(0i32);
pub const ePortConnUnknown: EPxcPortConnection = EPxcPortConnection(3i32);
