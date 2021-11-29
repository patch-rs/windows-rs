#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AssignProcessToJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, hprocess: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AssignProcessToJobObject(hjob: super::super::Foundation::HANDLE, hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AssignProcessToJobObject(hjob.into_param().abi(), hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateJobObjectA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateJobObjectA(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateJobObjectA(::core::mem::transmute(lpjobattributes), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateJobObjectW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateJobObjectW(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateJobObjectW(::core::mem::transmute(lpjobattributes), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateJobSet(numjob: u32, userjobset: *const JOB_SET_ARRAY, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateJobSet(numjob: u32, userjobset: *const JOB_SET_ARRAY, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateJobSet(::core::mem::transmute(numjob), ::core::mem::transmute(userjobset), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FreeMemoryJobObject(buffer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeMemoryJobObject(buffer: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(FreeMemoryJobObject(::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(processhandle: Param0, jobhandle: Param1, result: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInJob(processhandle: super::super::Foundation::HANDLE, jobhandle: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsProcessInJob(processhandle.into_param().abi(), jobhandle.into_param().abi(), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOBOBJECTINFOCLASS(pub i32);
pub const JobObjectBasicAccountingInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(1i32);
pub const JobObjectBasicLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(2i32);
pub const JobObjectBasicProcessIdList: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(3i32);
pub const JobObjectBasicUIRestrictions: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(4i32);
pub const JobObjectSecurityLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(5i32);
pub const JobObjectEndOfJobTimeInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(6i32);
pub const JobObjectAssociateCompletionPortInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(7i32);
pub const JobObjectBasicAndIoAccountingInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(8i32);
pub const JobObjectExtendedLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(9i32);
pub const JobObjectJobSetInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(10i32);
pub const JobObjectGroupInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(11i32);
pub const JobObjectNotificationLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(12i32);
pub const JobObjectLimitViolationInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(13i32);
pub const JobObjectGroupInformationEx: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(14i32);
pub const JobObjectCpuRateControlInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(15i32);
pub const JobObjectCompletionFilter: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(16i32);
pub const JobObjectCompletionCounter: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(17i32);
pub const JobObjectReserved1Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(18i32);
pub const JobObjectReserved2Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(19i32);
pub const JobObjectReserved3Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(20i32);
pub const JobObjectReserved4Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(21i32);
pub const JobObjectReserved5Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(22i32);
pub const JobObjectReserved6Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(23i32);
pub const JobObjectReserved7Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(24i32);
pub const JobObjectReserved8Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(25i32);
pub const JobObjectReserved9Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(26i32);
pub const JobObjectReserved10Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(27i32);
pub const JobObjectReserved11Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(28i32);
pub const JobObjectReserved12Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(29i32);
pub const JobObjectReserved13Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(30i32);
pub const JobObjectReserved14Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(31i32);
pub const JobObjectNetRateControlInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(32i32);
pub const JobObjectNotificationLimitInformation2: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(33i32);
pub const JobObjectLimitViolationInformation2: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(34i32);
pub const JobObjectCreateSilo: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(35i32);
pub const JobObjectSiloBasicInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(36i32);
pub const JobObjectReserved15Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(37i32);
pub const JobObjectReserved16Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(38i32);
pub const JobObjectReserved17Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(39i32);
pub const JobObjectReserved18Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(40i32);
pub const JobObjectReserved19Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(41i32);
pub const JobObjectReserved20Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(42i32);
pub const JobObjectReserved21Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(43i32);
pub const JobObjectReserved22Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(44i32);
pub const JobObjectReserved23Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(45i32);
pub const JobObjectReserved24Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(46i32);
pub const JobObjectReserved25Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(47i32);
pub const MaxJobObjectInfoClass: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(48i32);
impl ::core::convert::From<i32> for JOBOBJECTINFOCLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECTINFOCLASS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    pub CompletionKey: *mut ::core::ffi::c_void,
    pub CompletionPort: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl JOBOBJECT_ASSOCIATE_COMPLETION_PORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_ASSOCIATE_COMPLETION_PORT").field("CompletionKey", &self.CompletionKey).field("CompletionPort", &self.CompletionPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn eq(&self, other: &Self) -> bool {
        self.CompletionKey == other.CompletionKey && self.CompletionPort == other.CompletionPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    pub TotalUserTime: i64,
    pub TotalKernelTime: i64,
    pub ThisPeriodTotalUserTime: i64,
    pub ThisPeriodTotalKernelTime: i64,
    pub TotalPageFaultCount: u32,
    pub TotalProcesses: u32,
    pub ActiveProcesses: u32,
    pub TotalTerminatedProcesses: u32,
}
impl JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_BASIC_ACCOUNTING_INFORMATION")
            .field("TotalUserTime", &self.TotalUserTime)
            .field("TotalKernelTime", &self.TotalKernelTime)
            .field("ThisPeriodTotalUserTime", &self.ThisPeriodTotalUserTime)
            .field("ThisPeriodTotalKernelTime", &self.ThisPeriodTotalKernelTime)
            .field("TotalPageFaultCount", &self.TotalPageFaultCount)
            .field("TotalProcesses", &self.TotalProcesses)
            .field("ActiveProcesses", &self.ActiveProcesses)
            .field("TotalTerminatedProcesses", &self.TotalTerminatedProcesses)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TotalUserTime == other.TotalUserTime && self.TotalKernelTime == other.TotalKernelTime && self.ThisPeriodTotalUserTime == other.ThisPeriodTotalUserTime && self.ThisPeriodTotalKernelTime == other.ThisPeriodTotalKernelTime && self.TotalPageFaultCount == other.TotalPageFaultCount && self.TotalProcesses == other.TotalProcesses && self.ActiveProcesses == other.ActiveProcesses && self.TotalTerminatedProcesses == other.TotalTerminatedProcesses
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {}
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Threading")]
pub struct JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    pub BasicInfo: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,
    pub IoInfo: super::Threading::IO_COUNTERS,
}
#[cfg(feature = "Win32_System_Threading")]
impl JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::default::Default for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::fmt::Debug for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION").field("BasicInfo", &self.BasicInfo).field("IoInfo", &self.IoInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BasicInfo == other.BasicInfo && self.IoInfo == other.IoInfo
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::Eq for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
    pub PerProcessUserTimeLimit: i64,
    pub PerJobUserTimeLimit: i64,
    pub LimitFlags: JOB_OBJECT_LIMIT,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub ActiveProcessLimit: u32,
    pub Affinity: usize,
    pub PriorityClass: u32,
    pub SchedulingClass: u32,
}
impl JOBOBJECT_BASIC_LIMIT_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_BASIC_LIMIT_INFORMATION")
            .field("PerProcessUserTimeLimit", &self.PerProcessUserTimeLimit)
            .field("PerJobUserTimeLimit", &self.PerJobUserTimeLimit)
            .field("LimitFlags", &self.LimitFlags)
            .field("MinimumWorkingSetSize", &self.MinimumWorkingSetSize)
            .field("MaximumWorkingSetSize", &self.MaximumWorkingSetSize)
            .field("ActiveProcessLimit", &self.ActiveProcessLimit)
            .field("Affinity", &self.Affinity)
            .field("PriorityClass", &self.PriorityClass)
            .field("SchedulingClass", &self.SchedulingClass)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PerProcessUserTimeLimit == other.PerProcessUserTimeLimit && self.PerJobUserTimeLimit == other.PerJobUserTimeLimit && self.LimitFlags == other.LimitFlags && self.MinimumWorkingSetSize == other.MinimumWorkingSetSize && self.MaximumWorkingSetSize == other.MaximumWorkingSetSize && self.ActiveProcessLimit == other.ActiveProcessLimit && self.Affinity == other.Affinity && self.PriorityClass == other.PriorityClass && self.SchedulingClass == other.SchedulingClass
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_LIMIT_INFORMATION {}
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_BASIC_PROCESS_ID_LIST {
    pub NumberOfAssignedProcesses: u32,
    pub NumberOfProcessIdsInList: u32,
    pub ProcessIdList: [usize; 1],
}
impl JOBOBJECT_BASIC_PROCESS_ID_LIST {}
impl ::core::default::Default for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_BASIC_PROCESS_ID_LIST").field("NumberOfAssignedProcesses", &self.NumberOfAssignedProcesses).field("NumberOfProcessIdsInList", &self.NumberOfProcessIdsInList).field("ProcessIdList", &self.ProcessIdList).finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfAssignedProcesses == other.NumberOfAssignedProcesses && self.NumberOfProcessIdsInList == other.NumberOfProcessIdsInList && self.ProcessIdList == other.ProcessIdList
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_PROCESS_ID_LIST {}
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_BASIC_UI_RESTRICTIONS {
    pub UIRestrictionsClass: JOB_OBJECT_UILIMIT,
}
impl JOBOBJECT_BASIC_UI_RESTRICTIONS {}
impl ::core::default::Default for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_BASIC_UI_RESTRICTIONS").field("UIRestrictionsClass", &self.UIRestrictionsClass).finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.UIRestrictionsClass == other.UIRestrictionsClass
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_UI_RESTRICTIONS {}
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    pub ControlFlags: JOB_OBJECT_CPU_RATE_CONTROL,
    pub Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0,
}
impl JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {}
unsafe impl ::windows::core::Abi for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    pub CpuRate: u32,
    pub Weight: u32,
    pub Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0,
}
impl JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {}
impl ::core::default::Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    pub MinRate: u16,
    pub MaxRate: u16,
}
impl JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {}
impl ::core::default::Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("MinRate", &self.MinRate).field("MaxRate", &self.MaxRate).finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.MinRate == other.MinRate && self.MaxRate == other.MaxRate
    }
}
impl ::core::cmp::Eq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    pub EndOfJobTimeAction: JOB_OBJECT_TERMINATE_AT_END_ACTION,
}
impl JOBOBJECT_END_OF_JOB_TIME_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_END_OF_JOB_TIME_INFORMATION").field("EndOfJobTimeAction", &self.EndOfJobTimeAction).finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.EndOfJobTimeAction == other.EndOfJobTimeAction
    }
}
impl ::core::cmp::Eq for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {}
unsafe impl ::windows::core::Abi for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Threading")]
pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    pub BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    pub IoInfo: super::Threading::IO_COUNTERS,
    pub ProcessMemoryLimit: usize,
    pub JobMemoryLimit: usize,
    pub PeakProcessMemoryUsed: usize,
    pub PeakJobMemoryUsed: usize,
}
#[cfg(feature = "Win32_System_Threading")]
impl JOBOBJECT_EXTENDED_LIMIT_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::default::Default for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::fmt::Debug for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_EXTENDED_LIMIT_INFORMATION").field("BasicLimitInformation", &self.BasicLimitInformation).field("IoInfo", &self.IoInfo).field("ProcessMemoryLimit", &self.ProcessMemoryLimit).field("JobMemoryLimit", &self.JobMemoryLimit).field("PeakProcessMemoryUsed", &self.PeakProcessMemoryUsed).field("PeakJobMemoryUsed", &self.PeakJobMemoryUsed).finish()
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::PartialEq for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BasicLimitInformation == other.BasicLimitInformation && self.IoInfo == other.IoInfo && self.ProcessMemoryLimit == other.ProcessMemoryLimit && self.JobMemoryLimit == other.JobMemoryLimit && self.PeakProcessMemoryUsed == other.PeakProcessMemoryUsed && self.PeakJobMemoryUsed == other.PeakJobMemoryUsed
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::Eq for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
unsafe impl ::windows::core::Abi for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(pub i32);
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_ENABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(1i32);
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_DISABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(2i32);
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_VALID_FLAGS: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(3i32);
impl ::core::convert::From<i32> for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    pub ControlFlags: u32,
    pub ReadStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
    pub WriteStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
}
impl JOBOBJECT_IO_ATTRIBUTION_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_IO_ATTRIBUTION_INFORMATION").field("ControlFlags", &self.ControlFlags).field("ReadStats", &self.ReadStats).field("WriteStats", &self.WriteStats).finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ControlFlags == other.ControlFlags && self.ReadStats == other.ReadStats && self.WriteStats == other.WriteStats
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_IO_ATTRIBUTION_STATS {
    pub IoCount: usize,
    pub TotalNonOverlappedQueueTime: u64,
    pub TotalNonOverlappedServiceTime: u64,
    pub TotalSize: u64,
}
impl JOBOBJECT_IO_ATTRIBUTION_STATS {}
impl ::core::default::Default for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_IO_ATTRIBUTION_STATS").field("IoCount", &self.IoCount).field("TotalNonOverlappedQueueTime", &self.TotalNonOverlappedQueueTime).field("TotalNonOverlappedServiceTime", &self.TotalNonOverlappedServiceTime).field("TotalSize", &self.TotalSize).finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.IoCount == other.IoCount && self.TotalNonOverlappedQueueTime == other.TotalNonOverlappedQueueTime && self.TotalNonOverlappedServiceTime == other.TotalNonOverlappedServiceTime && self.TotalSize == other.TotalSize
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_ATTRIBUTION_STATS {}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_ATTRIBUTION_STATS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: super::super::Foundation::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl JOBOBJECT_IO_RATE_CONTROL_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION").field("MaxIops", &self.MaxIops).field("MaxBandwidth", &self.MaxBandwidth).field("ReservationIops", &self.ReservationIops).field("VolumeName", &self.VolumeName).field("BaseIoSize", &self.BaseIoSize).field("ControlFlags", &self.ControlFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIops == other.MaxIops && self.MaxBandwidth == other.MaxBandwidth && self.ReservationIops == other.ReservationIops && self.VolumeName == other.VolumeName && self.BaseIoSize == other.BaseIoSize && self.ControlFlags == other.ControlFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: super::super::Foundation::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE").field("MaxIops", &self.MaxIops).field("MaxBandwidth", &self.MaxBandwidth).field("ReservationIops", &self.ReservationIops).field("VolumeName", &self.VolumeName).field("BaseIoSize", &self.BaseIoSize).field("ControlFlags", &self.ControlFlags).field("VolumeNameLength", &self.VolumeNameLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIops == other.MaxIops && self.MaxBandwidth == other.MaxBandwidth && self.ReservationIops == other.ReservationIops && self.VolumeName == other.VolumeName && self.BaseIoSize == other.BaseIoSize && self.ControlFlags == other.ControlFlags && self.VolumeNameLength == other.VolumeNameLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: super::super::Foundation::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
    pub CriticalReservationIops: i64,
    pub ReservationBandwidth: i64,
    pub CriticalReservationBandwidth: i64,
    pub MaxTimePercent: i64,
    pub ReservationTimePercent: i64,
    pub CriticalReservationTimePercent: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2")
            .field("MaxIops", &self.MaxIops)
            .field("MaxBandwidth", &self.MaxBandwidth)
            .field("ReservationIops", &self.ReservationIops)
            .field("VolumeName", &self.VolumeName)
            .field("BaseIoSize", &self.BaseIoSize)
            .field("ControlFlags", &self.ControlFlags)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("CriticalReservationIops", &self.CriticalReservationIops)
            .field("ReservationBandwidth", &self.ReservationBandwidth)
            .field("CriticalReservationBandwidth", &self.CriticalReservationBandwidth)
            .field("MaxTimePercent", &self.MaxTimePercent)
            .field("ReservationTimePercent", &self.ReservationTimePercent)
            .field("CriticalReservationTimePercent", &self.CriticalReservationTimePercent)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIops == other.MaxIops && self.MaxBandwidth == other.MaxBandwidth && self.ReservationIops == other.ReservationIops && self.VolumeName == other.VolumeName && self.BaseIoSize == other.BaseIoSize && self.ControlFlags == other.ControlFlags && self.VolumeNameLength == other.VolumeNameLength && self.CriticalReservationIops == other.CriticalReservationIops && self.ReservationBandwidth == other.ReservationBandwidth && self.CriticalReservationBandwidth == other.CriticalReservationBandwidth && self.MaxTimePercent == other.MaxTimePercent && self.ReservationTimePercent == other.ReservationTimePercent && self.CriticalReservationTimePercent == other.CriticalReservationTimePercent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: super::super::Foundation::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
    pub CriticalReservationIops: i64,
    pub ReservationBandwidth: i64,
    pub CriticalReservationBandwidth: i64,
    pub MaxTimePercent: i64,
    pub ReservationTimePercent: i64,
    pub CriticalReservationTimePercent: i64,
    pub SoftMaxIops: i64,
    pub SoftMaxBandwidth: i64,
    pub SoftMaxTimePercent: i64,
    pub LimitExcessNotifyIops: i64,
    pub LimitExcessNotifyBandwidth: i64,
    pub LimitExcessNotifyTimePercent: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3")
            .field("MaxIops", &self.MaxIops)
            .field("MaxBandwidth", &self.MaxBandwidth)
            .field("ReservationIops", &self.ReservationIops)
            .field("VolumeName", &self.VolumeName)
            .field("BaseIoSize", &self.BaseIoSize)
            .field("ControlFlags", &self.ControlFlags)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("CriticalReservationIops", &self.CriticalReservationIops)
            .field("ReservationBandwidth", &self.ReservationBandwidth)
            .field("CriticalReservationBandwidth", &self.CriticalReservationBandwidth)
            .field("MaxTimePercent", &self.MaxTimePercent)
            .field("ReservationTimePercent", &self.ReservationTimePercent)
            .field("CriticalReservationTimePercent", &self.CriticalReservationTimePercent)
            .field("SoftMaxIops", &self.SoftMaxIops)
            .field("SoftMaxBandwidth", &self.SoftMaxBandwidth)
            .field("SoftMaxTimePercent", &self.SoftMaxTimePercent)
            .field("LimitExcessNotifyIops", &self.LimitExcessNotifyIops)
            .field("LimitExcessNotifyBandwidth", &self.LimitExcessNotifyBandwidth)
            .field("LimitExcessNotifyTimePercent", &self.LimitExcessNotifyTimePercent)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIops == other.MaxIops
            && self.MaxBandwidth == other.MaxBandwidth
            && self.ReservationIops == other.ReservationIops
            && self.VolumeName == other.VolumeName
            && self.BaseIoSize == other.BaseIoSize
            && self.ControlFlags == other.ControlFlags
            && self.VolumeNameLength == other.VolumeNameLength
            && self.CriticalReservationIops == other.CriticalReservationIops
            && self.ReservationBandwidth == other.ReservationBandwidth
            && self.CriticalReservationBandwidth == other.CriticalReservationBandwidth
            && self.MaxTimePercent == other.MaxTimePercent
            && self.ReservationTimePercent == other.ReservationTimePercent
            && self.CriticalReservationTimePercent == other.CriticalReservationTimePercent
            && self.SoftMaxIops == other.SoftMaxIops
            && self.SoftMaxBandwidth == other.SoftMaxBandwidth
            && self.SoftMaxTimePercent == other.SoftMaxTimePercent
            && self.LimitExcessNotifyIops == other.LimitExcessNotifyIops
            && self.LimitExcessNotifyBandwidth == other.LimitExcessNotifyBandwidth
            && self.LimitExcessNotifyTimePercent == other.LimitExcessNotifyTimePercent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_JOBSET_INFORMATION {
    pub MemberLevel: u32,
}
impl JOBOBJECT_JOBSET_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_JOBSET_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_JOBSET_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_JOBSET_INFORMATION").field("MemberLevel", &self.MemberLevel).finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_JOBSET_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MemberLevel == other.MemberLevel
    }
}
impl ::core::cmp::Eq for JOBOBJECT_JOBSET_INFORMATION {}
unsafe impl ::windows::core::Abi for JOBOBJECT_JOBSET_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    pub LimitFlags: JOB_OBJECT_LIMIT,
    pub ViolationLimitFlags: JOB_OBJECT_LIMIT,
    pub IoReadBytes: u64,
    pub IoReadBytesLimit: u64,
    pub IoWriteBytes: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTime: i64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemory: u64,
    pub JobMemoryLimit: u64,
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub RateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl JOBOBJECT_LIMIT_VIOLATION_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_LIMIT_VIOLATION_INFORMATION")
            .field("LimitFlags", &self.LimitFlags)
            .field("ViolationLimitFlags", &self.ViolationLimitFlags)
            .field("IoReadBytes", &self.IoReadBytes)
            .field("IoReadBytesLimit", &self.IoReadBytesLimit)
            .field("IoWriteBytes", &self.IoWriteBytes)
            .field("IoWriteBytesLimit", &self.IoWriteBytesLimit)
            .field("PerJobUserTime", &self.PerJobUserTime)
            .field("PerJobUserTimeLimit", &self.PerJobUserTimeLimit)
            .field("JobMemory", &self.JobMemory)
            .field("JobMemoryLimit", &self.JobMemoryLimit)
            .field("RateControlTolerance", &self.RateControlTolerance)
            .field("RateControlToleranceLimit", &self.RateControlToleranceLimit)
            .finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LimitFlags == other.LimitFlags && self.ViolationLimitFlags == other.ViolationLimitFlags && self.IoReadBytes == other.IoReadBytes && self.IoReadBytesLimit == other.IoReadBytesLimit && self.IoWriteBytes == other.IoWriteBytes && self.IoWriteBytesLimit == other.IoWriteBytesLimit && self.PerJobUserTime == other.PerJobUserTime && self.PerJobUserTimeLimit == other.PerJobUserTimeLimit && self.JobMemory == other.JobMemory && self.JobMemoryLimit == other.JobMemoryLimit && self.RateControlTolerance == other.RateControlTolerance && self.RateControlToleranceLimit == other.RateControlToleranceLimit
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    pub LimitFlags: JOB_OBJECT_LIMIT,
    pub ViolationLimitFlags: JOB_OBJECT_LIMIT,
    pub IoReadBytes: u64,
    pub IoReadBytesLimit: u64,
    pub IoWriteBytes: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTime: i64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemory: u64,
    pub Anonymous1: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0,
    pub Anonymous2: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1,
    pub Anonymous3: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2,
    pub JobLowMemoryLimit: u64,
    pub IoRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub IoRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub NetRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub NetRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    pub JobHighMemoryLimit: u64,
    pub JobMemoryLimit: u64,
}
impl JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    pub RateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    pub MaxBandwidth: u64,
    pub ControlFlags: JOB_OBJECT_NET_RATE_CONTROL_FLAGS,
    pub DscpTag: u8,
}
impl JOBOBJECT_NET_RATE_CONTROL_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_NET_RATE_CONTROL_INFORMATION").field("MaxBandwidth", &self.MaxBandwidth).field("ControlFlags", &self.ControlFlags).field("DscpTag", &self.DscpTag).finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MaxBandwidth == other.MaxBandwidth && self.ControlFlags == other.ControlFlags && self.DscpTag == other.DscpTag
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {}
unsafe impl ::windows::core::Abi for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    pub IoReadBytesLimit: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemoryLimit: u64,
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub RateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub LimitFlags: JOB_OBJECT_LIMIT,
}
impl JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION").field("IoReadBytesLimit", &self.IoReadBytesLimit).field("IoWriteBytesLimit", &self.IoWriteBytesLimit).field("PerJobUserTimeLimit", &self.PerJobUserTimeLimit).field("JobMemoryLimit", &self.JobMemoryLimit).field("RateControlTolerance", &self.RateControlTolerance).field("RateControlToleranceInterval", &self.RateControlToleranceInterval).field("LimitFlags", &self.LimitFlags).finish()
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IoReadBytesLimit == other.IoReadBytesLimit && self.IoWriteBytesLimit == other.IoWriteBytesLimit && self.PerJobUserTimeLimit == other.PerJobUserTimeLimit && self.JobMemoryLimit == other.JobMemoryLimit && self.RateControlTolerance == other.RateControlTolerance && self.RateControlToleranceInterval == other.RateControlToleranceInterval && self.LimitFlags == other.LimitFlags
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    pub IoReadBytesLimit: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTimeLimit: i64,
    pub Anonymous1: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0,
    pub Anonymous2: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1,
    pub Anonymous3: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2,
    pub LimitFlags: JOB_OBJECT_LIMIT,
    pub IoRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub JobLowMemoryLimit: u64,
    pub IoRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub NetRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub NetRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
}
impl JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    pub JobHighMemoryLimit: u64,
    pub JobMemoryLimit: u64,
}
impl JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    pub RateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub CpuRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
}
impl JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOBOBJECT_RATE_CONTROL_TOLERANCE(pub i32);
pub const ToleranceLow: JOBOBJECT_RATE_CONTROL_TOLERANCE = JOBOBJECT_RATE_CONTROL_TOLERANCE(1i32);
pub const ToleranceMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE = JOBOBJECT_RATE_CONTROL_TOLERANCE(2i32);
pub const ToleranceHigh: JOBOBJECT_RATE_CONTROL_TOLERANCE = JOBOBJECT_RATE_CONTROL_TOLERANCE(3i32);
impl ::core::convert::From<i32> for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(pub i32);
pub const ToleranceIntervalShort: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(1i32);
pub const ToleranceIntervalMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(2i32);
pub const ToleranceIntervalLong: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(3i32);
impl ::core::convert::From<i32> for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    pub SecurityLimitFlags: JOB_OBJECT_SECURITY,
    pub JobToken: super::super::Foundation::HANDLE,
    pub SidsToDisable: *mut super::super::Security::TOKEN_GROUPS,
    pub PrivilegesToDelete: *mut super::super::Security::TOKEN_PRIVILEGES,
    pub RestrictedSids: *mut super::super::Security::TOKEN_GROUPS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl JOBOBJECT_SECURITY_LIMIT_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOBOBJECT_SECURITY_LIMIT_INFORMATION").field("SecurityLimitFlags", &self.SecurityLimitFlags).field("JobToken", &self.JobToken).field("SidsToDisable", &self.SidsToDisable).field("PrivilegesToDelete", &self.PrivilegesToDelete).field("RestrictedSids", &self.RestrictedSids).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityLimitFlags == other.SecurityLimitFlags && self.JobToken == other.JobToken && self.SidsToDisable == other.SidsToDisable && self.PrivilegesToDelete == other.PrivilegesToDelete && self.RestrictedSids == other.RestrictedSids
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for JOBOBJECT_SECURITY_LIMIT_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_CPU_RATE_CONTROL(pub u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_ENABLE: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(1u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(2u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(4u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(8u32);
pub const JOB_OBJECT__CPU_RATE_CONTROL_MIN_MAX_RATE: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(16u32);
impl ::core::convert::From<u32> for JOB_OBJECT_CPU_RATE_CONTROL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_CPU_RATE_CONTROL {
    type Abi = Self;
}
impl ::core::ops::BitOr for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_CPU_RATE_CONTROL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_CPU_RATE_CONTROL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_IO_RATE_CONTROL_FLAGS(pub i32);
pub const JOB_OBJECT_IO_RATE_CONTROL_ENABLE: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(1i32);
pub const JOB_OBJECT_IO_RATE_CONTROL_STANDALONE_VOLUME: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(2i32);
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ALL: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(4i32);
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ON_SOFT_CAP: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(8i32);
pub const JOB_OBJECT_IO_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(15i32);
impl ::core::convert::From<i32> for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_LIMIT(pub u32);
pub const JOB_OBJECT_LIMIT_WORKINGSET: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1u32);
pub const JOB_OBJECT_LIMIT_PROCESS_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2u32);
pub const JOB_OBJECT_LIMIT_JOB_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(4u32);
pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(8u32);
pub const JOB_OBJECT_LIMIT_AFFINITY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(16u32);
pub const JOB_OBJECT_LIMIT_PRIORITY_CLASS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32u32);
pub const JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(64u32);
pub const JOB_OBJECT_LIMIT_SCHEDULING_CLASS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(128u32);
pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(256u32);
pub const JOB_OBJECT_LIMIT_JOB_MEMORY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(512u32);
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(512u32);
pub const JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1024u32);
pub const JOB_OBJECT_LIMIT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2048u32);
pub const JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(4096u32);
pub const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(8192u32);
pub const JOB_OBJECT_LIMIT_SUBSET_AFFINITY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(16384u32);
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_LOW: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32768u32);
pub const JOB_OBJECT_LIMIT_JOB_READ_BYTES: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(65536u32);
pub const JOB_OBJECT_LIMIT_JOB_WRITE_BYTES: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(131072u32);
pub const JOB_OBJECT_LIMIT_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(262144u32);
pub const JOB_OBJECT_LIMIT_CPU_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(262144u32);
pub const JOB_OBJECT_LIMIT_IO_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(524288u32);
pub const JOB_OBJECT_LIMIT_NET_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1048576u32);
pub const JOB_OBJECT_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(524287u32);
pub const JOB_OBJECT_BASIC_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(255u32);
pub const JOB_OBJECT_EXTENDED_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32767u32);
pub const JOB_OBJECT_NOTIFICATION_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2064900u32);
impl ::core::convert::From<u32> for JOB_OBJECT_LIMIT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_LIMIT {
    type Abi = Self;
}
impl ::core::ops::BitOr for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_LIMIT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_LIMIT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_NET_RATE_CONTROL_FLAGS(pub i32);
pub const JOB_OBJECT_NET_RATE_CONTROL_ENABLE: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(1i32);
pub const JOB_OBJECT_NET_RATE_CONTROL_MAX_BANDWIDTH: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(2i32);
pub const JOB_OBJECT_NET_RATE_CONTROL_DSCP_TAG: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(4i32);
pub const JOB_OBJECT_NET_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(7i32);
impl ::core::convert::From<i32> for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_SECURITY(pub u32);
pub const JOB_OBJECT_SECURITY_NO_ADMIN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(1u32);
pub const JOB_OBJECT_SECURITY_RESTRICTED_TOKEN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(2u32);
pub const JOB_OBJECT_SECURITY_ONLY_TOKEN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(4u32);
pub const JOB_OBJECT_SECURITY_FILTER_TOKENS: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(8u32);
pub const JOB_OBJECT_SECURITY_VALID_FLAGS: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(15u32);
impl ::core::convert::From<u32> for JOB_OBJECT_SECURITY {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_SECURITY {
    type Abi = Self;
}
impl ::core::ops::BitOr for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_SECURITY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_SECURITY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_TERMINATE_AT_END_ACTION(pub u32);
pub const JOB_OBJECT_TERMINATE_AT_END_OF_JOB: JOB_OBJECT_TERMINATE_AT_END_ACTION = JOB_OBJECT_TERMINATE_AT_END_ACTION(0u32);
pub const JOB_OBJECT_POST_AT_END_OF_JOB: JOB_OBJECT_TERMINATE_AT_END_ACTION = JOB_OBJECT_TERMINATE_AT_END_ACTION(1u32);
impl ::core::convert::From<u32> for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    type Abi = Self;
}
impl ::core::ops::BitOr for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JOB_OBJECT_UILIMIT(pub u32);
pub const JOB_OBJECT_UILIMIT_NONE: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(0u32);
pub const JOB_OBJECT_UILIMIT_HANDLES: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(1u32);
pub const JOB_OBJECT_UILIMIT_READCLIPBOARD: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(2u32);
pub const JOB_OBJECT_UILIMIT_WRITECLIPBOARD: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(4u32);
pub const JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(8u32);
pub const JOB_OBJECT_UILIMIT_DISPLAYSETTINGS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(16u32);
pub const JOB_OBJECT_UILIMIT_GLOBALATOMS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(32u32);
pub const JOB_OBJECT_UILIMIT_DESKTOP: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(64u32);
pub const JOB_OBJECT_UILIMIT_EXITWINDOWS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(128u32);
impl ::core::convert::From<u32> for JOB_OBJECT_UILIMIT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_UILIMIT {
    type Abi = Self;
}
impl ::core::ops::BitOr for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_UILIMIT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_UILIMIT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JOB_SET_ARRAY {
    pub JobHandle: super::super::Foundation::HANDLE,
    pub MemberLevel: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JOB_SET_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOB_SET_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOB_SET_ARRAY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("JOB_SET_ARRAY").field("JobHandle", &self.JobHandle).field("MemberLevel", &self.MemberLevel).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOB_SET_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.JobHandle == other.JobHandle && self.MemberLevel == other.MemberLevel && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOB_SET_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOB_SET_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenJobObjectA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenJobObjectA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenJobObjectA(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenJobObjectW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenJobObjectW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenJobObjectW(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryInformationJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *mut ::core::ffi::c_void, cbjobobjectinformationlength: u32, lpreturnlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryInformationJobObject(hjob: super::super::Foundation::HANDLE, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *mut ::core::ffi::c_void, cbjobobjectinformationlength: u32, lpreturnlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryInformationJobObject(hjob.into_param().abi(), ::core::mem::transmute(jobobjectinformationclass), ::core::mem::transmute(lpjobobjectinformation), ::core::mem::transmute(cbjobobjectinformationlength), ::core::mem::transmute(lpreturnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryIoRateControlInformationJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hjob: Param0, volumename: Param1, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryIoRateControlInformationJobObject(hjob: super::super::Foundation::HANDLE, volumename: super::super::Foundation::PWSTR, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut u32) -> u32;
        }
        ::core::mem::transmute(QueryIoRateControlInformationJobObject(hjob.into_param().abi(), volumename.into_param().abi(), ::core::mem::transmute(infoblocks), ::core::mem::transmute(infoblockcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetInformationJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *const ::core::ffi::c_void, cbjobobjectinformationlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetInformationJobObject(hjob: super::super::Foundation::HANDLE, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *const ::core::ffi::c_void, cbjobobjectinformationlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetInformationJobObject(hjob.into_param().abi(), ::core::mem::transmute(jobobjectinformationclass), ::core::mem::transmute(lpjobobjectinformation), ::core::mem::transmute(cbjobobjectinformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetIoRateControlInformationJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIoRateControlInformationJobObject(hjob: super::super::Foundation::HANDLE, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32;
        }
        ::core::mem::transmute(SetIoRateControlInformationJobObject(hjob.into_param().abi(), ::core::mem::transmute(ioratecontrolinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, uexitcode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateJobObject(hjob: super::super::Foundation::HANDLE, uexitcode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TerminateJobObject(hjob.into_param().abi(), ::core::mem::transmute(uexitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserHandleGrantAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(huserhandle: Param0, hjob: Param1, bgrant: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserHandleGrantAccess(huserhandle: super::super::Foundation::HANDLE, hjob: super::super::Foundation::HANDLE, bgrant: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UserHandleGrantAccess(huserhandle.into_param().abi(), hjob.into_param().abi(), bgrant.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
