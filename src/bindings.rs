#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(trivial_casts)]
#![allow(clippy::all)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]

use windows::Win32::System::Com::IStream;
pub use windows::core::HRESULT;
pub type HANDLE = windows::Win32::Foundation::HANDLE;
pub type IID = windows::core::GUID;
pub type BYTE = ::std::os::raw::c_uchar;
pub type INT64 = ::std::os::raw::c_longlong;
pub type ULONG = ::std::os::raw::c_ulong;
pub type RPC_IF_HANDLE = *mut ::std::os::raw::c_void;
pub type WCHAR = ::std::os::raw::c_ushort;
pub type UINT = ::std::os::raw::c_uint;
pub type BOOLEAN = ::std::os::raw::c_uchar;
pub type WAITABLE_HANDLE = HANDLE;
pub type TIMESPAN = INT64;
pub type UINT64 = ::std::os::raw::c_ulonglong;
pub type UINT16 = ::std::os::raw::c_ushort;
pub type INT32 = ::std::os::raw::c_int;

unsafe extern "C" {
    pub static mut __MIDL_itf_Kinect2EINPC_0000_0000_v0_0_c_ifspec: RPC_IF_HANDLE;
}
unsafe extern "C" {
    pub static mut __MIDL_itf_Kinect2EINPC_0000_0000_v0_0_s_ifspec: RPC_IF_HANDLE;
}
unsafe extern "C" {
    pub static IID_INotifyPropertyChanged: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct INotifyPropertyChangedVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INotifyPropertyChanged,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut INotifyPropertyChanged) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut INotifyPropertyChanged) -> ULONG>,
    pub SubscribePropertyChanged: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INotifyPropertyChanged,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribePropertyChanged: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INotifyPropertyChanged,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetPropertyChangedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut INotifyPropertyChanged,
            waitableHandle: WAITABLE_HANDLE,
            bufferSize: UINT,
            propertyName: *mut WCHAR,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of INotifyPropertyChangedVtbl"]
        [::std::mem::size_of::<INotifyPropertyChangedVtbl>() - 48usize];
    ["Alignment of INotifyPropertyChangedVtbl"]
        [::std::mem::align_of::<INotifyPropertyChangedVtbl>() - 8usize];
    ["Offset of field: INotifyPropertyChangedVtbl::QueryInterface"]
        [::std::mem::offset_of!(INotifyPropertyChangedVtbl, QueryInterface) - 0usize];
    ["Offset of field: INotifyPropertyChangedVtbl::AddRef"]
        [::std::mem::offset_of!(INotifyPropertyChangedVtbl, AddRef) - 8usize];
    ["Offset of field: INotifyPropertyChangedVtbl::Release"]
        [::std::mem::offset_of!(INotifyPropertyChangedVtbl, Release) - 16usize];
    ["Offset of field: INotifyPropertyChangedVtbl::SubscribePropertyChanged"]
        [::std::mem::offset_of!(INotifyPropertyChangedVtbl, SubscribePropertyChanged) - 24usize];
    ["Offset of field: INotifyPropertyChangedVtbl::UnsubscribePropertyChanged"]
        [::std::mem::offset_of!(INotifyPropertyChangedVtbl, UnsubscribePropertyChanged) - 32usize];
    ["Offset of field: INotifyPropertyChangedVtbl::GetPropertyChangedEventData"]
        [::std::mem::offset_of!(INotifyPropertyChangedVtbl, GetPropertyChangedEventData) - 40usize];
};
#[repr(C)]
pub struct INotifyPropertyChanged {
    pub lpVtbl: *mut INotifyPropertyChangedVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of INotifyPropertyChanged"][::std::mem::size_of::<INotifyPropertyChanged>() - 8usize];
    ["Alignment of INotifyPropertyChanged"]
        [::std::mem::align_of::<INotifyPropertyChanged>() - 8usize];
    ["Offset of field: INotifyPropertyChanged::lpVtbl"]
        [::std::mem::offset_of!(INotifyPropertyChanged, lpVtbl) - 0usize];
};
impl Default for INotifyPropertyChanged {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IPropertyChangedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IPropertyChangedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IPropertyChangedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IPropertyChangedEventArgs) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IPropertyChangedEventArgs) -> ULONG>,
    pub get_PropertyName: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IPropertyChangedEventArgs,
            bufferSize: UINT,
            propertyName: *mut WCHAR,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IPropertyChangedEventArgsVtbl"]
        [::std::mem::size_of::<IPropertyChangedEventArgsVtbl>() - 32usize];
    ["Alignment of IPropertyChangedEventArgsVtbl"]
        [::std::mem::align_of::<IPropertyChangedEventArgsVtbl>() - 8usize];
    ["Offset of field: IPropertyChangedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IPropertyChangedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IPropertyChangedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IPropertyChangedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IPropertyChangedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IPropertyChangedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IPropertyChangedEventArgsVtbl::get_PropertyName"]
        [::std::mem::offset_of!(IPropertyChangedEventArgsVtbl, get_PropertyName) - 24usize];
};
#[repr(C)]
pub struct IPropertyChangedEventArgs {
    pub lpVtbl: *mut IPropertyChangedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IPropertyChangedEventArgs"]
        [::std::mem::size_of::<IPropertyChangedEventArgs>() - 8usize];
    ["Alignment of IPropertyChangedEventArgs"]
        [::std::mem::align_of::<IPropertyChangedEventArgs>() - 8usize];
    ["Offset of field: IPropertyChangedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IPropertyChangedEventArgs, lpVtbl) - 0usize];
};
impl Default for IPropertyChangedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum KinectCapabilities {
    None = 0,
    Vision = 1,
    Audio = 2,
    Face = 4,
    Expressions = 8,
    Gamechat = 16,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FrameSourceTypes {
    None = 0,
    Color = 1,
    Infrared = 2,
    LongExposureInfrared = 4,
    Depth = 8,
    BodyIndex = 16,
    Body = 32,
    Audio = 64,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ColorImageFormat {
    None = 0,
    Rgba = 1,
    Yuv = 2,
    Bgra = 3,
    Bayer = 4,
    Yuy2 = 5,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HandState {
    Unknown = 0,
    NotTracked = 1,
    Open = 2,
    Closed = 3,
    Lasso = 4,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Expression {
    Neutral = 0,
    Happy = 1,
    Count = 2,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DetectionResult {
    Unknown = 0,
    No = 1,
    Maybe = 2,
    Yes = 3,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TrackingConfidence {
    Low = 0,
    High = 1,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Activity {
    EyeLeftClosed = 0,
    EyeRightClosed = 1,
    MouthOpen = 2,
    MouthMoved = 3,
    LookingAway = 4,
    Count = 5,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Appearance {
    WearingGlasses = 0,
    Count = 1,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum JointType {
    SpineBase = 0,
    SpineMid = 1,
    Neck = 2,
    Head = 3,
    ShoulderLeft = 4,
    ElbowLeft = 5,
    WristLeft = 6,
    HandLeft = 7,
    ShoulderRight = 8,
    ElbowRight = 9,
    WristRight = 10,
    HandRight = 11,
    HipLeft = 12,
    KneeLeft = 13,
    AnkleLeft = 14,
    FootLeft = 15,
    HipRight = 16,
    KneeRight = 17,
    AnkleRight = 18,
    FootRight = 19,
    SpineShoulder = 20,
    HandTipLeft = 21,
    ThumbLeft = 22,
    HandTipRight = 23,
    ThumbRight = 24,
    Count = 25,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TrackingState {
    NotTracked = 0,
    Inferred = 1,
    Tracked = 2,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FrameEdges {
    None = 0,
    Right = 1,
    Left = 2,
    Top = 4,
    Bottom = 8,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FrameCapturedStatus {
    Unknown = 0,
    Queued = 1,
    Dropped = 2,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AudioBeamMode {
    Automatic = 0,
    Manual = 1,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum KinectAudioCalibrationState {
    Unknown = 0,
    CalibrationRequired = 1,
    Calibrated = 2,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Vector4"][::std::mem::size_of::<Vector4>() - 16usize];
    ["Alignment of Vector4"][::std::mem::align_of::<Vector4>() - 4usize];
    ["Offset of field: Vector4::x"][::std::mem::offset_of!(Vector4, x) - 0usize];
    ["Offset of field: Vector4::y"][::std::mem::offset_of!(Vector4, y) - 4usize];
    ["Offset of field: Vector4::z"][::std::mem::offset_of!(Vector4, z) - 8usize];
    ["Offset of field: Vector4::w"][::std::mem::offset_of!(Vector4, w) - 12usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct PointF {
    pub X: f32,
    pub Y: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PointF"][::std::mem::size_of::<PointF>() - 8usize];
    ["Alignment of PointF"][::std::mem::align_of::<PointF>() - 4usize];
    ["Offset of field: PointF::X"][::std::mem::offset_of!(PointF, X) - 0usize];
    ["Offset of field: PointF::Y"][::std::mem::offset_of!(PointF, Y) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct RectF {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RectF"][::std::mem::size_of::<RectF>() - 16usize];
    ["Alignment of RectF"][::std::mem::align_of::<RectF>() - 4usize];
    ["Offset of field: RectF::X"][::std::mem::offset_of!(RectF, X) - 0usize];
    ["Offset of field: RectF::Y"][::std::mem::offset_of!(RectF, Y) - 4usize];
    ["Offset of field: RectF::Width"][::std::mem::offset_of!(RectF, Width) - 8usize];
    ["Offset of field: RectF::Height"][::std::mem::offset_of!(RectF, Height) - 12usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct ColorSpacePoint {
    pub X: f32,
    pub Y: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ColorSpacePoint"][::std::mem::size_of::<ColorSpacePoint>() - 8usize];
    ["Alignment of ColorSpacePoint"][::std::mem::align_of::<ColorSpacePoint>() - 4usize];
    ["Offset of field: ColorSpacePoint::X"][::std::mem::offset_of!(ColorSpacePoint, X) - 0usize];
    ["Offset of field: ColorSpacePoint::Y"][::std::mem::offset_of!(ColorSpacePoint, Y) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct DepthSpacePoint {
    pub X: f32,
    pub Y: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DepthSpacePoint"][::std::mem::size_of::<DepthSpacePoint>() - 8usize];
    ["Alignment of DepthSpacePoint"][::std::mem::align_of::<DepthSpacePoint>() - 4usize];
    ["Offset of field: DepthSpacePoint::X"][::std::mem::offset_of!(DepthSpacePoint, X) - 0usize];
    ["Offset of field: DepthSpacePoint::Y"][::std::mem::offset_of!(DepthSpacePoint, Y) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct CameraSpacePoint {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CameraSpacePoint"][::std::mem::size_of::<CameraSpacePoint>() - 12usize];
    ["Alignment of CameraSpacePoint"][::std::mem::align_of::<CameraSpacePoint>() - 4usize];
    ["Offset of field: CameraSpacePoint::X"][::std::mem::offset_of!(CameraSpacePoint, X) - 0usize];
    ["Offset of field: CameraSpacePoint::Y"][::std::mem::offset_of!(CameraSpacePoint, Y) - 4usize];
    ["Offset of field: CameraSpacePoint::Z"][::std::mem::offset_of!(CameraSpacePoint, Z) - 8usize];
};
#[repr(C)]
pub struct Joint {
    pub JointType: JointType,
    pub Position: CameraSpacePoint,
    pub TrackingState: TrackingState,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Joint"][::std::mem::size_of::<Joint>() - 20usize];
    ["Alignment of Joint"][::std::mem::align_of::<Joint>() - 4usize];
    ["Offset of field: Joint::JointType"][::std::mem::offset_of!(Joint, JointType) - 0usize];
    ["Offset of field: Joint::Position"][::std::mem::offset_of!(Joint, Position) - 4usize];
    ["Offset of field: Joint::TrackingState"]
        [::std::mem::offset_of!(Joint, TrackingState) - 16usize];
};
impl Default for Joint {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct JointOrientation {
    pub JointType: JointType,
    pub Orientation: Vector4,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of JointOrientation"][::std::mem::size_of::<JointOrientation>() - 20usize];
    ["Alignment of JointOrientation"][::std::mem::align_of::<JointOrientation>() - 4usize];
    ["Offset of field: JointOrientation::JointType"]
        [::std::mem::offset_of!(JointOrientation, JointType) - 0usize];
    ["Offset of field: JointOrientation::Orientation"]
        [::std::mem::offset_of!(JointOrientation, Orientation) - 4usize];
};
impl Default for JointOrientation {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct CameraIntrinsics {
    pub FocalLengthX: f32,
    pub FocalLengthY: f32,
    pub PrincipalPointX: f32,
    pub PrincipalPointY: f32,
    pub RadialDistortionSecondOrder: f32,
    pub RadialDistortionFourthOrder: f32,
    pub RadialDistortionSixthOrder: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CameraIntrinsics"][::std::mem::size_of::<CameraIntrinsics>() - 28usize];
    ["Alignment of CameraIntrinsics"][::std::mem::align_of::<CameraIntrinsics>() - 4usize];
    ["Offset of field: CameraIntrinsics::FocalLengthX"]
        [::std::mem::offset_of!(CameraIntrinsics, FocalLengthX) - 0usize];
    ["Offset of field: CameraIntrinsics::FocalLengthY"]
        [::std::mem::offset_of!(CameraIntrinsics, FocalLengthY) - 4usize];
    ["Offset of field: CameraIntrinsics::PrincipalPointX"]
        [::std::mem::offset_of!(CameraIntrinsics, PrincipalPointX) - 8usize];
    ["Offset of field: CameraIntrinsics::PrincipalPointY"]
        [::std::mem::offset_of!(CameraIntrinsics, PrincipalPointY) - 12usize];
    ["Offset of field: CameraIntrinsics::RadialDistortionSecondOrder"]
        [::std::mem::offset_of!(CameraIntrinsics, RadialDistortionSecondOrder) - 16usize];
    ["Offset of field: CameraIntrinsics::RadialDistortionFourthOrder"]
        [::std::mem::offset_of!(CameraIntrinsics, RadialDistortionFourthOrder) - 20usize];
    ["Offset of field: CameraIntrinsics::RadialDistortionSixthOrder"]
        [::std::mem::offset_of!(CameraIntrinsics, RadialDistortionSixthOrder) - 24usize];
};
unsafe extern "C" {
    pub static mut __MIDL_itf_Kinect2ECOM_0000_0000_v0_0_c_ifspec: RPC_IF_HANDLE;
}
unsafe extern "C" {
    pub static mut __MIDL_itf_Kinect2ECOM_0000_0000_v0_0_s_ifspec: RPC_IF_HANDLE;
}
unsafe extern "C" {
    pub static IID_IKinectSensor: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectSensorVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectSensor) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectSensor) -> ULONG>,
    pub SubscribeIsAvailableChanged: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeIsAvailableChanged: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectSensor, waitableHandle: WAITABLE_HANDLE) -> HRESULT,
    >,
    pub GetIsAvailableChangedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IIsAvailableChangedEventArgs,
        ) -> HRESULT,
    >,
    pub Open: ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectSensor) -> HRESULT>,
    pub Close: ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectSensor) -> HRESULT>,
    pub get_IsOpen: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectSensor, isOpen: *mut BOOLEAN) -> HRESULT,
    >,
    pub get_IsAvailable: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectSensor, isAvailable: *mut BOOLEAN) -> HRESULT,
    >,
    pub get_ColorFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            colorFrameSource: *mut *mut IColorFrameSource,
        ) -> HRESULT,
    >,
    pub get_DepthFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            depthFrameSource: *mut *mut IDepthFrameSource,
        ) -> HRESULT,
    >,
    pub get_BodyFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            bodyFrameSource: *mut *mut IBodyFrameSource,
        ) -> HRESULT,
    >,
    pub get_BodyIndexFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            bodyIndexFrameSource: *mut *mut IBodyIndexFrameSource,
        ) -> HRESULT,
    >,
    pub get_InfraredFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            infraredFrameSource: *mut *mut IInfraredFrameSource,
        ) -> HRESULT,
    >,
    pub get_LongExposureInfraredFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            longExposureInfraredFrameSource: *mut *mut ILongExposureInfraredFrameSource,
        ) -> HRESULT,
    >,
    pub get_AudioSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            audioSource: *mut *mut IAudioSource,
        ) -> HRESULT,
    >,
    pub OpenMultiSourceFrameReader: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            enabledFrameSourceTypes: FrameSourceTypes,
            multiSourceFrameReader: *mut *mut IMultiSourceFrameReader,
        ) -> HRESULT,
    >,
    pub get_CoordinateMapper: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            coordinateMapper: *mut *mut ICoordinateMapper,
        ) -> HRESULT,
    >,
    pub get_UniqueKinectId: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectSensor,
            bufferSize: UINT,
            uniqueKinectId: *mut WCHAR,
        ) -> HRESULT,
    >,
    pub get_KinectCapabilities: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectSensor, capabilities: *mut ULONG) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectSensorVtbl"][::std::mem::size_of::<IKinectSensorVtbl>() - 168usize];
    ["Alignment of IKinectSensorVtbl"][::std::mem::align_of::<IKinectSensorVtbl>() - 8usize];
    ["Offset of field: IKinectSensorVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectSensorVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectSensorVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectSensorVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectSensorVtbl::Release"]
        [::std::mem::offset_of!(IKinectSensorVtbl, Release) - 16usize];
    ["Offset of field: IKinectSensorVtbl::SubscribeIsAvailableChanged"]
        [::std::mem::offset_of!(IKinectSensorVtbl, SubscribeIsAvailableChanged) - 24usize];
    ["Offset of field: IKinectSensorVtbl::UnsubscribeIsAvailableChanged"]
        [::std::mem::offset_of!(IKinectSensorVtbl, UnsubscribeIsAvailableChanged) - 32usize];
    ["Offset of field: IKinectSensorVtbl::GetIsAvailableChangedEventData"]
        [::std::mem::offset_of!(IKinectSensorVtbl, GetIsAvailableChangedEventData) - 40usize];
    ["Offset of field: IKinectSensorVtbl::Open"]
        [::std::mem::offset_of!(IKinectSensorVtbl, Open) - 48usize];
    ["Offset of field: IKinectSensorVtbl::Close"]
        [::std::mem::offset_of!(IKinectSensorVtbl, Close) - 56usize];
    ["Offset of field: IKinectSensorVtbl::get_IsOpen"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_IsOpen) - 64usize];
    ["Offset of field: IKinectSensorVtbl::get_IsAvailable"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_IsAvailable) - 72usize];
    ["Offset of field: IKinectSensorVtbl::get_ColorFrameSource"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_ColorFrameSource) - 80usize];
    ["Offset of field: IKinectSensorVtbl::get_DepthFrameSource"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_DepthFrameSource) - 88usize];
    ["Offset of field: IKinectSensorVtbl::get_BodyFrameSource"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_BodyFrameSource) - 96usize];
    ["Offset of field: IKinectSensorVtbl::get_BodyIndexFrameSource"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_BodyIndexFrameSource) - 104usize];
    ["Offset of field: IKinectSensorVtbl::get_InfraredFrameSource"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_InfraredFrameSource) - 112usize];
    ["Offset of field: IKinectSensorVtbl::get_LongExposureInfraredFrameSource"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_LongExposureInfraredFrameSource) - 120usize];
    ["Offset of field: IKinectSensorVtbl::get_AudioSource"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_AudioSource) - 128usize];
    ["Offset of field: IKinectSensorVtbl::OpenMultiSourceFrameReader"]
        [::std::mem::offset_of!(IKinectSensorVtbl, OpenMultiSourceFrameReader) - 136usize];
    ["Offset of field: IKinectSensorVtbl::get_CoordinateMapper"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_CoordinateMapper) - 144usize];
    ["Offset of field: IKinectSensorVtbl::get_UniqueKinectId"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_UniqueKinectId) - 152usize];
    ["Offset of field: IKinectSensorVtbl::get_KinectCapabilities"]
        [::std::mem::offset_of!(IKinectSensorVtbl, get_KinectCapabilities) - 160usize];
};
#[repr(C)]
pub struct IKinectSensor {
    pub lpVtbl: *mut IKinectSensorVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectSensor"][::std::mem::size_of::<IKinectSensor>() - 8usize];
    ["Alignment of IKinectSensor"][::std::mem::align_of::<IKinectSensor>() - 8usize];
    ["Offset of field: IKinectSensor::lpVtbl"]
        [::std::mem::offset_of!(IKinectSensor, lpVtbl) - 0usize];
};
impl Default for IKinectSensor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IIsAvailableChangedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IIsAvailableChangedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IIsAvailableChangedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IIsAvailableChangedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IIsAvailableChangedEventArgs) -> ULONG,
    >,
    pub get_IsAvailable: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IIsAvailableChangedEventArgs,
            isAvailable: *mut BOOLEAN,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IIsAvailableChangedEventArgsVtbl"]
        [::std::mem::size_of::<IIsAvailableChangedEventArgsVtbl>() - 32usize];
    ["Alignment of IIsAvailableChangedEventArgsVtbl"]
        [::std::mem::align_of::<IIsAvailableChangedEventArgsVtbl>() - 8usize];
    ["Offset of field: IIsAvailableChangedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IIsAvailableChangedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IIsAvailableChangedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IIsAvailableChangedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IIsAvailableChangedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IIsAvailableChangedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IIsAvailableChangedEventArgsVtbl::get_IsAvailable"]
        [::std::mem::offset_of!(IIsAvailableChangedEventArgsVtbl, get_IsAvailable) - 24usize];
};
#[repr(C)]
pub struct IIsAvailableChangedEventArgs {
    pub lpVtbl: *mut IIsAvailableChangedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IIsAvailableChangedEventArgs"]
        [::std::mem::size_of::<IIsAvailableChangedEventArgs>() - 8usize];
    ["Alignment of IIsAvailableChangedEventArgs"]
        [::std::mem::align_of::<IIsAvailableChangedEventArgs>() - 8usize];
    ["Offset of field: IIsAvailableChangedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IIsAvailableChangedEventArgs, lpVtbl) - 0usize];
};
impl Default for IIsAvailableChangedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IFrameDescription: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IFrameDescriptionVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameDescription,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IFrameDescription) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IFrameDescription) -> ULONG>,
    pub get_Width: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameDescription,
            width: *mut ::std::os::raw::c_int,
        ) -> HRESULT,
    >,
    pub get_Height: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameDescription,
            height: *mut ::std::os::raw::c_int,
        ) -> HRESULT,
    >,
    pub get_HorizontalFieldOfView: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameDescription,
            horizontalFieldOfView: *mut f32,
        ) -> HRESULT,
    >,
    pub get_VerticalFieldOfView: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameDescription,
            verticalFieldOfView: *mut f32,
        ) -> HRESULT,
    >,
    pub get_DiagonalFieldOfView: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameDescription,
            diagonalFieldOfView: *mut f32,
        ) -> HRESULT,
    >,
    pub get_LengthInPixels: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameDescription,
            lengthInPixels: *mut ::std::os::raw::c_uint,
        ) -> HRESULT,
    >,
    pub get_BytesPerPixel: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameDescription,
            bytesPerPixel: *mut ::std::os::raw::c_uint,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IFrameDescriptionVtbl"][::std::mem::size_of::<IFrameDescriptionVtbl>() - 80usize];
    ["Alignment of IFrameDescriptionVtbl"]
        [::std::mem::align_of::<IFrameDescriptionVtbl>() - 8usize];
    ["Offset of field: IFrameDescriptionVtbl::QueryInterface"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, QueryInterface) - 0usize];
    ["Offset of field: IFrameDescriptionVtbl::AddRef"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, AddRef) - 8usize];
    ["Offset of field: IFrameDescriptionVtbl::Release"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, Release) - 16usize];
    ["Offset of field: IFrameDescriptionVtbl::get_Width"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, get_Width) - 24usize];
    ["Offset of field: IFrameDescriptionVtbl::get_Height"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, get_Height) - 32usize];
    ["Offset of field: IFrameDescriptionVtbl::get_HorizontalFieldOfView"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, get_HorizontalFieldOfView) - 40usize];
    ["Offset of field: IFrameDescriptionVtbl::get_VerticalFieldOfView"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, get_VerticalFieldOfView) - 48usize];
    ["Offset of field: IFrameDescriptionVtbl::get_DiagonalFieldOfView"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, get_DiagonalFieldOfView) - 56usize];
    ["Offset of field: IFrameDescriptionVtbl::get_LengthInPixels"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, get_LengthInPixels) - 64usize];
    ["Offset of field: IFrameDescriptionVtbl::get_BytesPerPixel"]
        [::std::mem::offset_of!(IFrameDescriptionVtbl, get_BytesPerPixel) - 72usize];
};
#[repr(C)]
pub struct IFrameDescription {
    pub lpVtbl: *mut IFrameDescriptionVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IFrameDescription"][::std::mem::size_of::<IFrameDescription>() - 8usize];
    ["Alignment of IFrameDescription"][::std::mem::align_of::<IFrameDescription>() - 8usize];
    ["Offset of field: IFrameDescription::lpVtbl"]
        [::std::mem::offset_of!(IFrameDescription, lpVtbl) - 0usize];
};
impl Default for IFrameDescription {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IFrameCapturedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IFrameCapturedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameCapturedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IFrameCapturedEventArgs) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IFrameCapturedEventArgs) -> ULONG>,
    pub get_FrameType: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameCapturedEventArgs,
            frameType: *mut FrameSourceTypes,
        ) -> HRESULT,
    >,
    pub get_FrameStatus: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameCapturedEventArgs,
            frameStatus: *mut FrameCapturedStatus,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IFrameCapturedEventArgs,
            relativeTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IFrameCapturedEventArgsVtbl"]
        [::std::mem::size_of::<IFrameCapturedEventArgsVtbl>() - 48usize];
    ["Alignment of IFrameCapturedEventArgsVtbl"]
        [::std::mem::align_of::<IFrameCapturedEventArgsVtbl>() - 8usize];
    ["Offset of field: IFrameCapturedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IFrameCapturedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IFrameCapturedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IFrameCapturedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IFrameCapturedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IFrameCapturedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IFrameCapturedEventArgsVtbl::get_FrameType"]
        [::std::mem::offset_of!(IFrameCapturedEventArgsVtbl, get_FrameType) - 24usize];
    ["Offset of field: IFrameCapturedEventArgsVtbl::get_FrameStatus"]
        [::std::mem::offset_of!(IFrameCapturedEventArgsVtbl, get_FrameStatus) - 32usize];
    ["Offset of field: IFrameCapturedEventArgsVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IFrameCapturedEventArgsVtbl, get_RelativeTime) - 40usize];
};
#[repr(C)]
pub struct IFrameCapturedEventArgs {
    pub lpVtbl: *mut IFrameCapturedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IFrameCapturedEventArgs"][::std::mem::size_of::<IFrameCapturedEventArgs>() - 8usize];
    ["Alignment of IFrameCapturedEventArgs"]
        [::std::mem::align_of::<IFrameCapturedEventArgs>() - 8usize];
    ["Offset of field: IFrameCapturedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IFrameCapturedEventArgs, lpVtbl) - 0usize];
};
impl Default for IFrameCapturedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IMultiSourceFrameReader: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IMultiSourceFrameReaderVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameReader,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IMultiSourceFrameReader) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IMultiSourceFrameReader) -> ULONG>,
    pub SubscribeMultiSourceFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameReader,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeMultiSourceFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameReader,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetMultiSourceFrameArrivedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameReader,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IMultiSourceFrameArrivedEventArgs,
        ) -> HRESULT,
    >,
    pub AcquireLatestFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameReader,
            multiSourceFrame: *mut *mut IMultiSourceFrame,
        ) -> HRESULT,
    >,
    pub get_FrameSourceTypes: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameReader,
            enabledFrameSourceTypes: *mut ULONG,
        ) -> HRESULT,
    >,
    pub get_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IMultiSourceFrameReader, isPaused: *mut BOOLEAN) -> HRESULT,
    >,
    pub put_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IMultiSourceFrameReader, isPaused: BOOLEAN) -> HRESULT,
    >,
    pub get_KinectSensor: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameReader,
            sensor: *mut *mut IKinectSensor,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMultiSourceFrameReaderVtbl"]
        [::std::mem::size_of::<IMultiSourceFrameReaderVtbl>() - 88usize];
    ["Alignment of IMultiSourceFrameReaderVtbl"]
        [::std::mem::align_of::<IMultiSourceFrameReaderVtbl>() - 8usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::QueryInterface"]
        [::std::mem::offset_of!(IMultiSourceFrameReaderVtbl, QueryInterface) - 0usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::AddRef"]
        [::std::mem::offset_of!(IMultiSourceFrameReaderVtbl, AddRef) - 8usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::Release"]
        [::std::mem::offset_of!(IMultiSourceFrameReaderVtbl, Release) - 16usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::SubscribeMultiSourceFrameArrived"][::std::mem::offset_of!(
        IMultiSourceFrameReaderVtbl,
        SubscribeMultiSourceFrameArrived
    ) - 24usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::UnsubscribeMultiSourceFrameArrived"][::std::mem::offset_of!(
        IMultiSourceFrameReaderVtbl,
        UnsubscribeMultiSourceFrameArrived
    )
        - 32usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::GetMultiSourceFrameArrivedEventData"][::std::mem::offset_of!(
        IMultiSourceFrameReaderVtbl,
        GetMultiSourceFrameArrivedEventData
    )
        - 40usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::AcquireLatestFrame"]
        [::std::mem::offset_of!(IMultiSourceFrameReaderVtbl, AcquireLatestFrame) - 48usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::get_FrameSourceTypes"]
        [::std::mem::offset_of!(IMultiSourceFrameReaderVtbl, get_FrameSourceTypes) - 56usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::get_IsPaused"]
        [::std::mem::offset_of!(IMultiSourceFrameReaderVtbl, get_IsPaused) - 64usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::put_IsPaused"]
        [::std::mem::offset_of!(IMultiSourceFrameReaderVtbl, put_IsPaused) - 72usize];
    ["Offset of field: IMultiSourceFrameReaderVtbl::get_KinectSensor"]
        [::std::mem::offset_of!(IMultiSourceFrameReaderVtbl, get_KinectSensor) - 80usize];
};
#[repr(C)]
pub struct IMultiSourceFrameReader {
    pub lpVtbl: *mut IMultiSourceFrameReaderVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMultiSourceFrameReader"][::std::mem::size_of::<IMultiSourceFrameReader>() - 8usize];
    ["Alignment of IMultiSourceFrameReader"]
        [::std::mem::align_of::<IMultiSourceFrameReader>() - 8usize];
    ["Offset of field: IMultiSourceFrameReader::lpVtbl"]
        [::std::mem::offset_of!(IMultiSourceFrameReader, lpVtbl) - 0usize];
};
impl Default for IMultiSourceFrameReader {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IMultiSourceFrameArrivedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IMultiSourceFrameArrivedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameArrivedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IMultiSourceFrameArrivedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IMultiSourceFrameArrivedEventArgs) -> ULONG,
    >,
    pub get_FrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameArrivedEventArgs,
            frames: *mut *mut IMultiSourceFrameReference,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMultiSourceFrameArrivedEventArgsVtbl"]
        [::std::mem::size_of::<IMultiSourceFrameArrivedEventArgsVtbl>() - 32usize];
    ["Alignment of IMultiSourceFrameArrivedEventArgsVtbl"]
        [::std::mem::align_of::<IMultiSourceFrameArrivedEventArgsVtbl>() - 8usize];
    ["Offset of field: IMultiSourceFrameArrivedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IMultiSourceFrameArrivedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IMultiSourceFrameArrivedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IMultiSourceFrameArrivedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IMultiSourceFrameArrivedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IMultiSourceFrameArrivedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IMultiSourceFrameArrivedEventArgsVtbl::get_FrameReference"][::std::mem::offset_of!(
        IMultiSourceFrameArrivedEventArgsVtbl,
        get_FrameReference
    ) - 24usize];
};
#[repr(C)]
pub struct IMultiSourceFrameArrivedEventArgs {
    pub lpVtbl: *mut IMultiSourceFrameArrivedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMultiSourceFrameArrivedEventArgs"]
        [::std::mem::size_of::<IMultiSourceFrameArrivedEventArgs>() - 8usize];
    ["Alignment of IMultiSourceFrameArrivedEventArgs"]
        [::std::mem::align_of::<IMultiSourceFrameArrivedEventArgs>() - 8usize];
    ["Offset of field: IMultiSourceFrameArrivedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IMultiSourceFrameArrivedEventArgs, lpVtbl) - 0usize];
};
impl Default for IMultiSourceFrameArrivedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IMultiSourceFrameReference: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IMultiSourceFrameReferenceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameReference,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IMultiSourceFrameReference) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IMultiSourceFrameReference) -> ULONG>,
    pub AcquireFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrameReference,
            frame: *mut *mut IMultiSourceFrame,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMultiSourceFrameReferenceVtbl"]
        [::std::mem::size_of::<IMultiSourceFrameReferenceVtbl>() - 32usize];
    ["Alignment of IMultiSourceFrameReferenceVtbl"]
        [::std::mem::align_of::<IMultiSourceFrameReferenceVtbl>() - 8usize];
    ["Offset of field: IMultiSourceFrameReferenceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IMultiSourceFrameReferenceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IMultiSourceFrameReferenceVtbl::AddRef"]
        [::std::mem::offset_of!(IMultiSourceFrameReferenceVtbl, AddRef) - 8usize];
    ["Offset of field: IMultiSourceFrameReferenceVtbl::Release"]
        [::std::mem::offset_of!(IMultiSourceFrameReferenceVtbl, Release) - 16usize];
    ["Offset of field: IMultiSourceFrameReferenceVtbl::AcquireFrame"]
        [::std::mem::offset_of!(IMultiSourceFrameReferenceVtbl, AcquireFrame) - 24usize];
};
#[repr(C)]
pub struct IMultiSourceFrameReference {
    pub lpVtbl: *mut IMultiSourceFrameReferenceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMultiSourceFrameReference"]
        [::std::mem::size_of::<IMultiSourceFrameReference>() - 8usize];
    ["Alignment of IMultiSourceFrameReference"]
        [::std::mem::align_of::<IMultiSourceFrameReference>() - 8usize];
    ["Offset of field: IMultiSourceFrameReference::lpVtbl"]
        [::std::mem::offset_of!(IMultiSourceFrameReference, lpVtbl) - 0usize];
};
impl Default for IMultiSourceFrameReference {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IMultiSourceFrame: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IMultiSourceFrameVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrame,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IMultiSourceFrame) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IMultiSourceFrame) -> ULONG>,
    pub get_ColorFrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrame,
            colorFrameReference: *mut *mut IColorFrameReference,
        ) -> HRESULT,
    >,
    pub get_DepthFrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrame,
            depthFrameReference: *mut *mut IDepthFrameReference,
        ) -> HRESULT,
    >,
    pub get_BodyFrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrame,
            bodyFrameReference: *mut *mut IBodyFrameReference,
        ) -> HRESULT,
    >,
    pub get_BodyIndexFrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrame,
            bodyIndexFrameReference: *mut *mut IBodyIndexFrameReference,
        ) -> HRESULT,
    >,
    pub get_InfraredFrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrame,
            infraredFrameReference: *mut *mut IInfraredFrameReference,
        ) -> HRESULT,
    >,
    pub get_LongExposureInfraredFrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IMultiSourceFrame,
            longExposureInfraredFrameReference: *mut *mut ILongExposureInfraredFrameReference,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMultiSourceFrameVtbl"][::std::mem::size_of::<IMultiSourceFrameVtbl>() - 72usize];
    ["Alignment of IMultiSourceFrameVtbl"]
        [::std::mem::align_of::<IMultiSourceFrameVtbl>() - 8usize];
    ["Offset of field: IMultiSourceFrameVtbl::QueryInterface"]
        [::std::mem::offset_of!(IMultiSourceFrameVtbl, QueryInterface) - 0usize];
    ["Offset of field: IMultiSourceFrameVtbl::AddRef"]
        [::std::mem::offset_of!(IMultiSourceFrameVtbl, AddRef) - 8usize];
    ["Offset of field: IMultiSourceFrameVtbl::Release"]
        [::std::mem::offset_of!(IMultiSourceFrameVtbl, Release) - 16usize];
    ["Offset of field: IMultiSourceFrameVtbl::get_ColorFrameReference"]
        [::std::mem::offset_of!(IMultiSourceFrameVtbl, get_ColorFrameReference) - 24usize];
    ["Offset of field: IMultiSourceFrameVtbl::get_DepthFrameReference"]
        [::std::mem::offset_of!(IMultiSourceFrameVtbl, get_DepthFrameReference) - 32usize];
    ["Offset of field: IMultiSourceFrameVtbl::get_BodyFrameReference"]
        [::std::mem::offset_of!(IMultiSourceFrameVtbl, get_BodyFrameReference) - 40usize];
    ["Offset of field: IMultiSourceFrameVtbl::get_BodyIndexFrameReference"]
        [::std::mem::offset_of!(IMultiSourceFrameVtbl, get_BodyIndexFrameReference) - 48usize];
    ["Offset of field: IMultiSourceFrameVtbl::get_InfraredFrameReference"]
        [::std::mem::offset_of!(IMultiSourceFrameVtbl, get_InfraredFrameReference) - 56usize];
    ["Offset of field: IMultiSourceFrameVtbl::get_LongExposureInfraredFrameReference"][::std::mem::offset_of!(
        IMultiSourceFrameVtbl,
        get_LongExposureInfraredFrameReference
    ) - 64usize];
};
#[repr(C)]
pub struct IMultiSourceFrame {
    pub lpVtbl: *mut IMultiSourceFrameVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMultiSourceFrame"][::std::mem::size_of::<IMultiSourceFrame>() - 8usize];
    ["Alignment of IMultiSourceFrame"][::std::mem::align_of::<IMultiSourceFrame>() - 8usize];
    ["Offset of field: IMultiSourceFrame::lpVtbl"]
        [::std::mem::offset_of!(IMultiSourceFrame, lpVtbl) - 0usize];
};
impl Default for IMultiSourceFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IColorFrameReference: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IColorFrameReferenceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameReference,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IColorFrameReference) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IColorFrameReference) -> ULONG>,
    pub AcquireFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameReference,
            colorFrame: *mut *mut IColorFrame,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameReference,
            relativeTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrameReferenceVtbl"]
        [::std::mem::size_of::<IColorFrameReferenceVtbl>() - 40usize];
    ["Alignment of IColorFrameReferenceVtbl"]
        [::std::mem::align_of::<IColorFrameReferenceVtbl>() - 8usize];
    ["Offset of field: IColorFrameReferenceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IColorFrameReferenceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IColorFrameReferenceVtbl::AddRef"]
        [::std::mem::offset_of!(IColorFrameReferenceVtbl, AddRef) - 8usize];
    ["Offset of field: IColorFrameReferenceVtbl::Release"]
        [::std::mem::offset_of!(IColorFrameReferenceVtbl, Release) - 16usize];
    ["Offset of field: IColorFrameReferenceVtbl::AcquireFrame"]
        [::std::mem::offset_of!(IColorFrameReferenceVtbl, AcquireFrame) - 24usize];
    ["Offset of field: IColorFrameReferenceVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IColorFrameReferenceVtbl, get_RelativeTime) - 32usize];
};
#[repr(C)]
pub struct IColorFrameReference {
    pub lpVtbl: *mut IColorFrameReferenceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrameReference"][::std::mem::size_of::<IColorFrameReference>() - 8usize];
    ["Alignment of IColorFrameReference"][::std::mem::align_of::<IColorFrameReference>() - 8usize];
    ["Offset of field: IColorFrameReference::lpVtbl"]
        [::std::mem::offset_of!(IColorFrameReference, lpVtbl) - 0usize];
};
impl Default for IColorFrameReference {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IColorFrameArrivedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IColorFrameArrivedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameArrivedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IColorFrameArrivedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IColorFrameArrivedEventArgs) -> ULONG,
    >,
    pub get_FrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameArrivedEventArgs,
            colorFrameReference: *mut *mut IColorFrameReference,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrameArrivedEventArgsVtbl"]
        [::std::mem::size_of::<IColorFrameArrivedEventArgsVtbl>() - 32usize];
    ["Alignment of IColorFrameArrivedEventArgsVtbl"]
        [::std::mem::align_of::<IColorFrameArrivedEventArgsVtbl>() - 8usize];
    ["Offset of field: IColorFrameArrivedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IColorFrameArrivedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IColorFrameArrivedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IColorFrameArrivedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IColorFrameArrivedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IColorFrameArrivedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IColorFrameArrivedEventArgsVtbl::get_FrameReference"]
        [::std::mem::offset_of!(IColorFrameArrivedEventArgsVtbl, get_FrameReference) - 24usize];
};
#[repr(C)]
pub struct IColorFrameArrivedEventArgs {
    pub lpVtbl: *mut IColorFrameArrivedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrameArrivedEventArgs"]
        [::std::mem::size_of::<IColorFrameArrivedEventArgs>() - 8usize];
    ["Alignment of IColorFrameArrivedEventArgs"]
        [::std::mem::align_of::<IColorFrameArrivedEventArgs>() - 8usize];
    ["Offset of field: IColorFrameArrivedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IColorFrameArrivedEventArgs, lpVtbl) - 0usize];
};
impl Default for IColorFrameArrivedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IColorFrameSource: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IColorFrameSourceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameSource,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IColorFrameSource) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IColorFrameSource) -> ULONG>,
    pub SubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameSource,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameSource,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameCapturedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameSource,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IFrameCapturedEventArgs,
        ) -> HRESULT,
    >,
    pub get_IsActive: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IColorFrameSource, isActive: *mut BOOLEAN) -> HRESULT,
    >,
    pub OpenReader: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameSource,
            reader: *mut *mut IColorFrameReader,
        ) -> HRESULT,
    >,
    pub CreateFrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameSource,
            format: ColorImageFormat,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameSource,
            rawFrameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_KinectSensor: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameSource,
            sensor: *mut *mut IKinectSensor,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrameSourceVtbl"][::std::mem::size_of::<IColorFrameSourceVtbl>() - 88usize];
    ["Alignment of IColorFrameSourceVtbl"]
        [::std::mem::align_of::<IColorFrameSourceVtbl>() - 8usize];
    ["Offset of field: IColorFrameSourceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IColorFrameSourceVtbl::AddRef"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, AddRef) - 8usize];
    ["Offset of field: IColorFrameSourceVtbl::Release"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, Release) - 16usize];
    ["Offset of field: IColorFrameSourceVtbl::SubscribeFrameCaptured"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, SubscribeFrameCaptured) - 24usize];
    ["Offset of field: IColorFrameSourceVtbl::UnsubscribeFrameCaptured"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, UnsubscribeFrameCaptured) - 32usize];
    ["Offset of field: IColorFrameSourceVtbl::GetFrameCapturedEventData"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, GetFrameCapturedEventData) - 40usize];
    ["Offset of field: IColorFrameSourceVtbl::get_IsActive"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, get_IsActive) - 48usize];
    ["Offset of field: IColorFrameSourceVtbl::OpenReader"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, OpenReader) - 56usize];
    ["Offset of field: IColorFrameSourceVtbl::CreateFrameDescription"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, CreateFrameDescription) - 64usize];
    ["Offset of field: IColorFrameSourceVtbl::get_FrameDescription"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, get_FrameDescription) - 72usize];
    ["Offset of field: IColorFrameSourceVtbl::get_KinectSensor"]
        [::std::mem::offset_of!(IColorFrameSourceVtbl, get_KinectSensor) - 80usize];
};
#[repr(C)]
pub struct IColorFrameSource {
    pub lpVtbl: *mut IColorFrameSourceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrameSource"][::std::mem::size_of::<IColorFrameSource>() - 8usize];
    ["Alignment of IColorFrameSource"][::std::mem::align_of::<IColorFrameSource>() - 8usize];
    ["Offset of field: IColorFrameSource::lpVtbl"]
        [::std::mem::offset_of!(IColorFrameSource, lpVtbl) - 0usize];
};
impl Default for IColorFrameSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IColorFrameReader: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IColorFrameReaderVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameReader,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IColorFrameReader) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IColorFrameReader) -> ULONG>,
    pub SubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameReader,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameReader,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameArrivedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameReader,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IColorFrameArrivedEventArgs,
        ) -> HRESULT,
    >,
    pub AcquireLatestFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameReader,
            colorFrame: *mut *mut IColorFrame,
        ) -> HRESULT,
    >,
    pub get_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IColorFrameReader, isPaused: *mut BOOLEAN) -> HRESULT,
    >,
    pub put_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IColorFrameReader, isPaused: BOOLEAN) -> HRESULT,
    >,
    pub get_ColorFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrameReader,
            colorFrameSource: *mut *mut IColorFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrameReaderVtbl"][::std::mem::size_of::<IColorFrameReaderVtbl>() - 80usize];
    ["Alignment of IColorFrameReaderVtbl"]
        [::std::mem::align_of::<IColorFrameReaderVtbl>() - 8usize];
    ["Offset of field: IColorFrameReaderVtbl::QueryInterface"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, QueryInterface) - 0usize];
    ["Offset of field: IColorFrameReaderVtbl::AddRef"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, AddRef) - 8usize];
    ["Offset of field: IColorFrameReaderVtbl::Release"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, Release) - 16usize];
    ["Offset of field: IColorFrameReaderVtbl::SubscribeFrameArrived"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, SubscribeFrameArrived) - 24usize];
    ["Offset of field: IColorFrameReaderVtbl::UnsubscribeFrameArrived"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, UnsubscribeFrameArrived) - 32usize];
    ["Offset of field: IColorFrameReaderVtbl::GetFrameArrivedEventData"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, GetFrameArrivedEventData) - 40usize];
    ["Offset of field: IColorFrameReaderVtbl::AcquireLatestFrame"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, AcquireLatestFrame) - 48usize];
    ["Offset of field: IColorFrameReaderVtbl::get_IsPaused"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, get_IsPaused) - 56usize];
    ["Offset of field: IColorFrameReaderVtbl::put_IsPaused"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, put_IsPaused) - 64usize];
    ["Offset of field: IColorFrameReaderVtbl::get_ColorFrameSource"]
        [::std::mem::offset_of!(IColorFrameReaderVtbl, get_ColorFrameSource) - 72usize];
};
#[repr(C)]
pub struct IColorFrameReader {
    pub lpVtbl: *mut IColorFrameReaderVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrameReader"][::std::mem::size_of::<IColorFrameReader>() - 8usize];
    ["Alignment of IColorFrameReader"][::std::mem::align_of::<IColorFrameReader>() - 8usize];
    ["Offset of field: IColorFrameReader::lpVtbl"]
        [::std::mem::offset_of!(IColorFrameReader, lpVtbl) - 0usize];
};
impl Default for IColorFrameReader {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IColorFrame: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IColorFrameVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrame,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IColorFrame) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IColorFrame) -> ULONG>,
    pub get_RawColorImageFormat: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrame,
            rawColorImageFormat: *mut ColorImageFormat,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrame,
            rawFrameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub CopyRawFrameDataToArray: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrame,
            capacity: UINT,
            frameData: *mut BYTE,
        ) -> HRESULT,
    >,
    pub AccessRawUnderlyingBuffer: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrame,
            capacity: *mut UINT,
            buffer: *mut *mut BYTE,
        ) -> HRESULT,
    >,
    pub CopyConvertedFrameDataToArray: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrame,
            capacity: UINT,
            frameData: *mut BYTE,
            colorFormat: ColorImageFormat,
        ) -> HRESULT,
    >,
    pub CreateFrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrame,
            format: ColorImageFormat,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_ColorCameraSettings: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrame,
            colorCameraSettings: *mut *mut IColorCameraSettings,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IColorFrame, relativeTime: *mut TIMESPAN) -> HRESULT,
    >,
    pub get_ColorFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorFrame,
            colorFrameSource: *mut *mut IColorFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrameVtbl"][::std::mem::size_of::<IColorFrameVtbl>() - 96usize];
    ["Alignment of IColorFrameVtbl"][::std::mem::align_of::<IColorFrameVtbl>() - 8usize];
    ["Offset of field: IColorFrameVtbl::QueryInterface"]
        [::std::mem::offset_of!(IColorFrameVtbl, QueryInterface) - 0usize];
    ["Offset of field: IColorFrameVtbl::AddRef"]
        [::std::mem::offset_of!(IColorFrameVtbl, AddRef) - 8usize];
    ["Offset of field: IColorFrameVtbl::Release"]
        [::std::mem::offset_of!(IColorFrameVtbl, Release) - 16usize];
    ["Offset of field: IColorFrameVtbl::get_RawColorImageFormat"]
        [::std::mem::offset_of!(IColorFrameVtbl, get_RawColorImageFormat) - 24usize];
    ["Offset of field: IColorFrameVtbl::get_FrameDescription"]
        [::std::mem::offset_of!(IColorFrameVtbl, get_FrameDescription) - 32usize];
    ["Offset of field: IColorFrameVtbl::CopyRawFrameDataToArray"]
        [::std::mem::offset_of!(IColorFrameVtbl, CopyRawFrameDataToArray) - 40usize];
    ["Offset of field: IColorFrameVtbl::AccessRawUnderlyingBuffer"]
        [::std::mem::offset_of!(IColorFrameVtbl, AccessRawUnderlyingBuffer) - 48usize];
    ["Offset of field: IColorFrameVtbl::CopyConvertedFrameDataToArray"]
        [::std::mem::offset_of!(IColorFrameVtbl, CopyConvertedFrameDataToArray) - 56usize];
    ["Offset of field: IColorFrameVtbl::CreateFrameDescription"]
        [::std::mem::offset_of!(IColorFrameVtbl, CreateFrameDescription) - 64usize];
    ["Offset of field: IColorFrameVtbl::get_ColorCameraSettings"]
        [::std::mem::offset_of!(IColorFrameVtbl, get_ColorCameraSettings) - 72usize];
    ["Offset of field: IColorFrameVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IColorFrameVtbl, get_RelativeTime) - 80usize];
    ["Offset of field: IColorFrameVtbl::get_ColorFrameSource"]
        [::std::mem::offset_of!(IColorFrameVtbl, get_ColorFrameSource) - 88usize];
};
#[repr(C)]
pub struct IColorFrame {
    pub lpVtbl: *mut IColorFrameVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorFrame"][::std::mem::size_of::<IColorFrame>() - 8usize];
    ["Alignment of IColorFrame"][::std::mem::align_of::<IColorFrame>() - 8usize];
    ["Offset of field: IColorFrame::lpVtbl"][::std::mem::offset_of!(IColorFrame, lpVtbl) - 0usize];
};
impl Default for IColorFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IDepthFrameReference: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IDepthFrameReferenceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameReference,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IDepthFrameReference) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IDepthFrameReference) -> ULONG>,
    pub AcquireFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameReference,
            depthFrame: *mut *mut IDepthFrame,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameReference,
            relativeTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrameReferenceVtbl"]
        [::std::mem::size_of::<IDepthFrameReferenceVtbl>() - 40usize];
    ["Alignment of IDepthFrameReferenceVtbl"]
        [::std::mem::align_of::<IDepthFrameReferenceVtbl>() - 8usize];
    ["Offset of field: IDepthFrameReferenceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IDepthFrameReferenceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IDepthFrameReferenceVtbl::AddRef"]
        [::std::mem::offset_of!(IDepthFrameReferenceVtbl, AddRef) - 8usize];
    ["Offset of field: IDepthFrameReferenceVtbl::Release"]
        [::std::mem::offset_of!(IDepthFrameReferenceVtbl, Release) - 16usize];
    ["Offset of field: IDepthFrameReferenceVtbl::AcquireFrame"]
        [::std::mem::offset_of!(IDepthFrameReferenceVtbl, AcquireFrame) - 24usize];
    ["Offset of field: IDepthFrameReferenceVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IDepthFrameReferenceVtbl, get_RelativeTime) - 32usize];
};
#[repr(C)]
pub struct IDepthFrameReference {
    pub lpVtbl: *mut IDepthFrameReferenceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrameReference"][::std::mem::size_of::<IDepthFrameReference>() - 8usize];
    ["Alignment of IDepthFrameReference"][::std::mem::align_of::<IDepthFrameReference>() - 8usize];
    ["Offset of field: IDepthFrameReference::lpVtbl"]
        [::std::mem::offset_of!(IDepthFrameReference, lpVtbl) - 0usize];
};
impl Default for IDepthFrameReference {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IDepthFrameArrivedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IDepthFrameArrivedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameArrivedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IDepthFrameArrivedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IDepthFrameArrivedEventArgs) -> ULONG,
    >,
    pub get_FrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameArrivedEventArgs,
            depthFrameReference: *mut *mut IDepthFrameReference,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrameArrivedEventArgsVtbl"]
        [::std::mem::size_of::<IDepthFrameArrivedEventArgsVtbl>() - 32usize];
    ["Alignment of IDepthFrameArrivedEventArgsVtbl"]
        [::std::mem::align_of::<IDepthFrameArrivedEventArgsVtbl>() - 8usize];
    ["Offset of field: IDepthFrameArrivedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IDepthFrameArrivedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IDepthFrameArrivedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IDepthFrameArrivedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IDepthFrameArrivedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IDepthFrameArrivedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IDepthFrameArrivedEventArgsVtbl::get_FrameReference"]
        [::std::mem::offset_of!(IDepthFrameArrivedEventArgsVtbl, get_FrameReference) - 24usize];
};
#[repr(C)]
pub struct IDepthFrameArrivedEventArgs {
    pub lpVtbl: *mut IDepthFrameArrivedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrameArrivedEventArgs"]
        [::std::mem::size_of::<IDepthFrameArrivedEventArgs>() - 8usize];
    ["Alignment of IDepthFrameArrivedEventArgs"]
        [::std::mem::align_of::<IDepthFrameArrivedEventArgs>() - 8usize];
    ["Offset of field: IDepthFrameArrivedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IDepthFrameArrivedEventArgs, lpVtbl) - 0usize];
};
impl Default for IDepthFrameArrivedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IDepthFrameSource: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IDepthFrameSourceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameSource,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IDepthFrameSource) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IDepthFrameSource) -> ULONG>,
    pub SubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameSource,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameSource,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameCapturedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameSource,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IFrameCapturedEventArgs,
        ) -> HRESULT,
    >,
    pub get_IsActive: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IDepthFrameSource, isActive: *mut BOOLEAN) -> HRESULT,
    >,
    pub OpenReader: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameSource,
            reader: *mut *mut IDepthFrameReader,
        ) -> HRESULT,
    >,
    pub get_DepthMinReliableDistance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameSource,
            depthMinReliableDistance: *mut UINT16,
        ) -> HRESULT,
    >,
    pub get_DepthMaxReliableDistance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameSource,
            depthMaxReliableDistance: *mut UINT16,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameSource,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_KinectSensor: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameSource,
            sensor: *mut *mut IKinectSensor,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrameSourceVtbl"][::std::mem::size_of::<IDepthFrameSourceVtbl>() - 96usize];
    ["Alignment of IDepthFrameSourceVtbl"]
        [::std::mem::align_of::<IDepthFrameSourceVtbl>() - 8usize];
    ["Offset of field: IDepthFrameSourceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IDepthFrameSourceVtbl::AddRef"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, AddRef) - 8usize];
    ["Offset of field: IDepthFrameSourceVtbl::Release"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, Release) - 16usize];
    ["Offset of field: IDepthFrameSourceVtbl::SubscribeFrameCaptured"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, SubscribeFrameCaptured) - 24usize];
    ["Offset of field: IDepthFrameSourceVtbl::UnsubscribeFrameCaptured"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, UnsubscribeFrameCaptured) - 32usize];
    ["Offset of field: IDepthFrameSourceVtbl::GetFrameCapturedEventData"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, GetFrameCapturedEventData) - 40usize];
    ["Offset of field: IDepthFrameSourceVtbl::get_IsActive"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, get_IsActive) - 48usize];
    ["Offset of field: IDepthFrameSourceVtbl::OpenReader"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, OpenReader) - 56usize];
    ["Offset of field: IDepthFrameSourceVtbl::get_DepthMinReliableDistance"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, get_DepthMinReliableDistance) - 64usize];
    ["Offset of field: IDepthFrameSourceVtbl::get_DepthMaxReliableDistance"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, get_DepthMaxReliableDistance) - 72usize];
    ["Offset of field: IDepthFrameSourceVtbl::get_FrameDescription"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, get_FrameDescription) - 80usize];
    ["Offset of field: IDepthFrameSourceVtbl::get_KinectSensor"]
        [::std::mem::offset_of!(IDepthFrameSourceVtbl, get_KinectSensor) - 88usize];
};
#[repr(C)]
pub struct IDepthFrameSource {
    pub lpVtbl: *mut IDepthFrameSourceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrameSource"][::std::mem::size_of::<IDepthFrameSource>() - 8usize];
    ["Alignment of IDepthFrameSource"][::std::mem::align_of::<IDepthFrameSource>() - 8usize];
    ["Offset of field: IDepthFrameSource::lpVtbl"]
        [::std::mem::offset_of!(IDepthFrameSource, lpVtbl) - 0usize];
};
impl Default for IDepthFrameSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IDepthFrameReader: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IDepthFrameReaderVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameReader,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IDepthFrameReader) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IDepthFrameReader) -> ULONG>,
    pub SubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameReader,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameReader,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameArrivedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameReader,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IDepthFrameArrivedEventArgs,
        ) -> HRESULT,
    >,
    pub AcquireLatestFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameReader,
            depthFrame: *mut *mut IDepthFrame,
        ) -> HRESULT,
    >,
    pub get_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IDepthFrameReader, isPaused: *mut BOOLEAN) -> HRESULT,
    >,
    pub put_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IDepthFrameReader, isPaused: BOOLEAN) -> HRESULT,
    >,
    pub get_DepthFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrameReader,
            depthFrameSource: *mut *mut IDepthFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrameReaderVtbl"][::std::mem::size_of::<IDepthFrameReaderVtbl>() - 80usize];
    ["Alignment of IDepthFrameReaderVtbl"]
        [::std::mem::align_of::<IDepthFrameReaderVtbl>() - 8usize];
    ["Offset of field: IDepthFrameReaderVtbl::QueryInterface"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, QueryInterface) - 0usize];
    ["Offset of field: IDepthFrameReaderVtbl::AddRef"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, AddRef) - 8usize];
    ["Offset of field: IDepthFrameReaderVtbl::Release"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, Release) - 16usize];
    ["Offset of field: IDepthFrameReaderVtbl::SubscribeFrameArrived"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, SubscribeFrameArrived) - 24usize];
    ["Offset of field: IDepthFrameReaderVtbl::UnsubscribeFrameArrived"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, UnsubscribeFrameArrived) - 32usize];
    ["Offset of field: IDepthFrameReaderVtbl::GetFrameArrivedEventData"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, GetFrameArrivedEventData) - 40usize];
    ["Offset of field: IDepthFrameReaderVtbl::AcquireLatestFrame"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, AcquireLatestFrame) - 48usize];
    ["Offset of field: IDepthFrameReaderVtbl::get_IsPaused"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, get_IsPaused) - 56usize];
    ["Offset of field: IDepthFrameReaderVtbl::put_IsPaused"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, put_IsPaused) - 64usize];
    ["Offset of field: IDepthFrameReaderVtbl::get_DepthFrameSource"]
        [::std::mem::offset_of!(IDepthFrameReaderVtbl, get_DepthFrameSource) - 72usize];
};
#[repr(C)]
pub struct IDepthFrameReader {
    pub lpVtbl: *mut IDepthFrameReaderVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrameReader"][::std::mem::size_of::<IDepthFrameReader>() - 8usize];
    ["Alignment of IDepthFrameReader"][::std::mem::align_of::<IDepthFrameReader>() - 8usize];
    ["Offset of field: IDepthFrameReader::lpVtbl"]
        [::std::mem::offset_of!(IDepthFrameReader, lpVtbl) - 0usize];
};
impl Default for IDepthFrameReader {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IDepthFrame: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IDepthFrameVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrame,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IDepthFrame) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IDepthFrame) -> ULONG>,
    pub CopyFrameDataToArray: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrame,
            capacity: UINT,
            frameData: *mut UINT16,
        ) -> HRESULT,
    >,
    pub AccessUnderlyingBuffer: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrame,
            capacity: *mut UINT,
            buffer: *mut *mut UINT16,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrame,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IDepthFrame, relativeTime: *mut TIMESPAN) -> HRESULT,
    >,
    pub get_DepthFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrame,
            depthFrameSource: *mut *mut IDepthFrameSource,
        ) -> HRESULT,
    >,
    pub get_DepthMinReliableDistance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrame,
            depthMinReliableDistance: *mut UINT16,
        ) -> HRESULT,
    >,
    pub get_DepthMaxReliableDistance: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IDepthFrame,
            depthMaxReliableDistance: *mut UINT16,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrameVtbl"][::std::mem::size_of::<IDepthFrameVtbl>() - 80usize];
    ["Alignment of IDepthFrameVtbl"][::std::mem::align_of::<IDepthFrameVtbl>() - 8usize];
    ["Offset of field: IDepthFrameVtbl::QueryInterface"]
        [::std::mem::offset_of!(IDepthFrameVtbl, QueryInterface) - 0usize];
    ["Offset of field: IDepthFrameVtbl::AddRef"]
        [::std::mem::offset_of!(IDepthFrameVtbl, AddRef) - 8usize];
    ["Offset of field: IDepthFrameVtbl::Release"]
        [::std::mem::offset_of!(IDepthFrameVtbl, Release) - 16usize];
    ["Offset of field: IDepthFrameVtbl::CopyFrameDataToArray"]
        [::std::mem::offset_of!(IDepthFrameVtbl, CopyFrameDataToArray) - 24usize];
    ["Offset of field: IDepthFrameVtbl::AccessUnderlyingBuffer"]
        [::std::mem::offset_of!(IDepthFrameVtbl, AccessUnderlyingBuffer) - 32usize];
    ["Offset of field: IDepthFrameVtbl::get_FrameDescription"]
        [::std::mem::offset_of!(IDepthFrameVtbl, get_FrameDescription) - 40usize];
    ["Offset of field: IDepthFrameVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IDepthFrameVtbl, get_RelativeTime) - 48usize];
    ["Offset of field: IDepthFrameVtbl::get_DepthFrameSource"]
        [::std::mem::offset_of!(IDepthFrameVtbl, get_DepthFrameSource) - 56usize];
    ["Offset of field: IDepthFrameVtbl::get_DepthMinReliableDistance"]
        [::std::mem::offset_of!(IDepthFrameVtbl, get_DepthMinReliableDistance) - 64usize];
    ["Offset of field: IDepthFrameVtbl::get_DepthMaxReliableDistance"]
        [::std::mem::offset_of!(IDepthFrameVtbl, get_DepthMaxReliableDistance) - 72usize];
};
#[repr(C)]
pub struct IDepthFrame {
    pub lpVtbl: *mut IDepthFrameVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IDepthFrame"][::std::mem::size_of::<IDepthFrame>() - 8usize];
    ["Alignment of IDepthFrame"][::std::mem::align_of::<IDepthFrame>() - 8usize];
    ["Offset of field: IDepthFrame::lpVtbl"][::std::mem::offset_of!(IDepthFrame, lpVtbl) - 0usize];
};
impl Default for IDepthFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyFrameReference: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyFrameReferenceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameReference,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrameReference) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrameReference) -> ULONG>,
    pub AcquireFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameReference,
            bodyFrame: *mut *mut IBodyFrame,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameReference,
            relativeTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrameReferenceVtbl"][::std::mem::size_of::<IBodyFrameReferenceVtbl>() - 40usize];
    ["Alignment of IBodyFrameReferenceVtbl"]
        [::std::mem::align_of::<IBodyFrameReferenceVtbl>() - 8usize];
    ["Offset of field: IBodyFrameReferenceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyFrameReferenceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyFrameReferenceVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyFrameReferenceVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyFrameReferenceVtbl::Release"]
        [::std::mem::offset_of!(IBodyFrameReferenceVtbl, Release) - 16usize];
    ["Offset of field: IBodyFrameReferenceVtbl::AcquireFrame"]
        [::std::mem::offset_of!(IBodyFrameReferenceVtbl, AcquireFrame) - 24usize];
    ["Offset of field: IBodyFrameReferenceVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IBodyFrameReferenceVtbl, get_RelativeTime) - 32usize];
};
#[repr(C)]
pub struct IBodyFrameReference {
    pub lpVtbl: *mut IBodyFrameReferenceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrameReference"][::std::mem::size_of::<IBodyFrameReference>() - 8usize];
    ["Alignment of IBodyFrameReference"][::std::mem::align_of::<IBodyFrameReference>() - 8usize];
    ["Offset of field: IBodyFrameReference::lpVtbl"]
        [::std::mem::offset_of!(IBodyFrameReference, lpVtbl) - 0usize];
};
impl Default for IBodyFrameReference {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyFrameArrivedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyFrameArrivedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameArrivedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrameArrivedEventArgs) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrameArrivedEventArgs) -> ULONG>,
    pub get_FrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameArrivedEventArgs,
            bodyFrameReference: *mut *mut IBodyFrameReference,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrameArrivedEventArgsVtbl"]
        [::std::mem::size_of::<IBodyFrameArrivedEventArgsVtbl>() - 32usize];
    ["Alignment of IBodyFrameArrivedEventArgsVtbl"]
        [::std::mem::align_of::<IBodyFrameArrivedEventArgsVtbl>() - 8usize];
    ["Offset of field: IBodyFrameArrivedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyFrameArrivedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyFrameArrivedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyFrameArrivedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyFrameArrivedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IBodyFrameArrivedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IBodyFrameArrivedEventArgsVtbl::get_FrameReference"]
        [::std::mem::offset_of!(IBodyFrameArrivedEventArgsVtbl, get_FrameReference) - 24usize];
};
#[repr(C)]
pub struct IBodyFrameArrivedEventArgs {
    pub lpVtbl: *mut IBodyFrameArrivedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrameArrivedEventArgs"]
        [::std::mem::size_of::<IBodyFrameArrivedEventArgs>() - 8usize];
    ["Alignment of IBodyFrameArrivedEventArgs"]
        [::std::mem::align_of::<IBodyFrameArrivedEventArgs>() - 8usize];
    ["Offset of field: IBodyFrameArrivedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IBodyFrameArrivedEventArgs, lpVtbl) - 0usize];
};
impl Default for IBodyFrameArrivedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyFrameSource: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyFrameSourceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameSource,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrameSource) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrameSource) -> ULONG>,
    pub SubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameSource,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameSource,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameCapturedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameSource,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IFrameCapturedEventArgs,
        ) -> HRESULT,
    >,
    pub get_IsActive: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyFrameSource, isActive: *mut BOOLEAN) -> HRESULT,
    >,
    pub get_BodyCount: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyFrameSource, bodyCount: *mut INT32) -> HRESULT,
    >,
    pub OpenReader: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameSource,
            reader: *mut *mut IBodyFrameReader,
        ) -> HRESULT,
    >,
    pub get_KinectSensor: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameSource,
            sensor: *mut *mut IKinectSensor,
        ) -> HRESULT,
    >,
    pub OverrideHandTracking: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyFrameSource, trackingId: UINT64) -> HRESULT,
    >,
    pub OverrideAndReplaceHandTracking: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameSource,
            oldTrackingId: UINT64,
            newTrackingId: UINT64,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrameSourceVtbl"][::std::mem::size_of::<IBodyFrameSourceVtbl>() - 96usize];
    ["Alignment of IBodyFrameSourceVtbl"][::std::mem::align_of::<IBodyFrameSourceVtbl>() - 8usize];
    ["Offset of field: IBodyFrameSourceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyFrameSourceVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyFrameSourceVtbl::Release"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, Release) - 16usize];
    ["Offset of field: IBodyFrameSourceVtbl::SubscribeFrameCaptured"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, SubscribeFrameCaptured) - 24usize];
    ["Offset of field: IBodyFrameSourceVtbl::UnsubscribeFrameCaptured"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, UnsubscribeFrameCaptured) - 32usize];
    ["Offset of field: IBodyFrameSourceVtbl::GetFrameCapturedEventData"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, GetFrameCapturedEventData) - 40usize];
    ["Offset of field: IBodyFrameSourceVtbl::get_IsActive"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, get_IsActive) - 48usize];
    ["Offset of field: IBodyFrameSourceVtbl::get_BodyCount"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, get_BodyCount) - 56usize];
    ["Offset of field: IBodyFrameSourceVtbl::OpenReader"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, OpenReader) - 64usize];
    ["Offset of field: IBodyFrameSourceVtbl::get_KinectSensor"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, get_KinectSensor) - 72usize];
    ["Offset of field: IBodyFrameSourceVtbl::OverrideHandTracking"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, OverrideHandTracking) - 80usize];
    ["Offset of field: IBodyFrameSourceVtbl::OverrideAndReplaceHandTracking"]
        [::std::mem::offset_of!(IBodyFrameSourceVtbl, OverrideAndReplaceHandTracking) - 88usize];
};
#[repr(C)]
pub struct IBodyFrameSource {
    pub lpVtbl: *mut IBodyFrameSourceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrameSource"][::std::mem::size_of::<IBodyFrameSource>() - 8usize];
    ["Alignment of IBodyFrameSource"][::std::mem::align_of::<IBodyFrameSource>() - 8usize];
    ["Offset of field: IBodyFrameSource::lpVtbl"]
        [::std::mem::offset_of!(IBodyFrameSource, lpVtbl) - 0usize];
};
impl Default for IBodyFrameSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyFrameReader: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyFrameReaderVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameReader,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrameReader) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrameReader) -> ULONG>,
    pub SubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameReader,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameReader,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameArrivedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameReader,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IBodyFrameArrivedEventArgs,
        ) -> HRESULT,
    >,
    pub AcquireLatestFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameReader,
            bodyFrame: *mut *mut IBodyFrame,
        ) -> HRESULT,
    >,
    pub get_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyFrameReader, isPaused: *mut BOOLEAN) -> HRESULT,
    >,
    pub put_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyFrameReader, isPaused: BOOLEAN) -> HRESULT,
    >,
    pub get_BodyFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrameReader,
            bodyFrameSource: *mut *mut IBodyFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrameReaderVtbl"][::std::mem::size_of::<IBodyFrameReaderVtbl>() - 80usize];
    ["Alignment of IBodyFrameReaderVtbl"][::std::mem::align_of::<IBodyFrameReaderVtbl>() - 8usize];
    ["Offset of field: IBodyFrameReaderVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyFrameReaderVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyFrameReaderVtbl::Release"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, Release) - 16usize];
    ["Offset of field: IBodyFrameReaderVtbl::SubscribeFrameArrived"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, SubscribeFrameArrived) - 24usize];
    ["Offset of field: IBodyFrameReaderVtbl::UnsubscribeFrameArrived"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, UnsubscribeFrameArrived) - 32usize];
    ["Offset of field: IBodyFrameReaderVtbl::GetFrameArrivedEventData"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, GetFrameArrivedEventData) - 40usize];
    ["Offset of field: IBodyFrameReaderVtbl::AcquireLatestFrame"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, AcquireLatestFrame) - 48usize];
    ["Offset of field: IBodyFrameReaderVtbl::get_IsPaused"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, get_IsPaused) - 56usize];
    ["Offset of field: IBodyFrameReaderVtbl::put_IsPaused"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, put_IsPaused) - 64usize];
    ["Offset of field: IBodyFrameReaderVtbl::get_BodyFrameSource"]
        [::std::mem::offset_of!(IBodyFrameReaderVtbl, get_BodyFrameSource) - 72usize];
};
#[repr(C)]
pub struct IBodyFrameReader {
    pub lpVtbl: *mut IBodyFrameReaderVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrameReader"][::std::mem::size_of::<IBodyFrameReader>() - 8usize];
    ["Alignment of IBodyFrameReader"][::std::mem::align_of::<IBodyFrameReader>() - 8usize];
    ["Offset of field: IBodyFrameReader::lpVtbl"]
        [::std::mem::offset_of!(IBodyFrameReader, lpVtbl) - 0usize];
};
impl Default for IBodyFrameReader {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyFrame: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyFrameVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrame,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrame) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyFrame) -> ULONG>,
    pub GetAndRefreshBodyData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrame,
            capacity: UINT,
            bodies: *mut *mut IBody,
        ) -> HRESULT,
    >,
    pub get_FloorClipPlane: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyFrame, floorClipPlane: *mut Vector4) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyFrame, relativeTime: *mut TIMESPAN) -> HRESULT,
    >,
    pub get_BodyFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyFrame,
            bodyFrameSource: *mut *mut IBodyFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrameVtbl"][::std::mem::size_of::<IBodyFrameVtbl>() - 56usize];
    ["Alignment of IBodyFrameVtbl"][::std::mem::align_of::<IBodyFrameVtbl>() - 8usize];
    ["Offset of field: IBodyFrameVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyFrameVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyFrameVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyFrameVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyFrameVtbl::Release"]
        [::std::mem::offset_of!(IBodyFrameVtbl, Release) - 16usize];
    ["Offset of field: IBodyFrameVtbl::GetAndRefreshBodyData"]
        [::std::mem::offset_of!(IBodyFrameVtbl, GetAndRefreshBodyData) - 24usize];
    ["Offset of field: IBodyFrameVtbl::get_FloorClipPlane"]
        [::std::mem::offset_of!(IBodyFrameVtbl, get_FloorClipPlane) - 32usize];
    ["Offset of field: IBodyFrameVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IBodyFrameVtbl, get_RelativeTime) - 40usize];
    ["Offset of field: IBodyFrameVtbl::get_BodyFrameSource"]
        [::std::mem::offset_of!(IBodyFrameVtbl, get_BodyFrameSource) - 48usize];
};
#[repr(C)]
pub struct IBodyFrame {
    pub lpVtbl: *mut IBodyFrameVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyFrame"][::std::mem::size_of::<IBodyFrame>() - 8usize];
    ["Alignment of IBodyFrame"][::std::mem::align_of::<IBodyFrame>() - 8usize];
    ["Offset of field: IBodyFrame::lpVtbl"][::std::mem::offset_of!(IBodyFrame, lpVtbl) - 0usize];
};
impl Default for IBodyFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBody: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBody,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IBody) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IBody) -> ULONG>,
    pub GetJoints: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, capacity: UINT, joints: *mut Joint) -> HRESULT,
    >,
    pub GetJointOrientations: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBody,
            capacity: UINT,
            jointOrientations: *mut JointOrientation,
        ) -> HRESULT,
    >,
    pub get_Engaged: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, detectionResult: *mut DetectionResult) -> HRESULT,
    >,
    pub GetExpressionDetectionResults: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBody,
            capacity: UINT,
            detectionResults: *mut DetectionResult,
        ) -> HRESULT,
    >,
    pub GetActivityDetectionResults: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBody,
            capacity: UINT,
            detectionResults: *mut DetectionResult,
        ) -> HRESULT,
    >,
    pub GetAppearanceDetectionResults: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBody,
            capacity: UINT,
            detectionResults: *mut DetectionResult,
        ) -> HRESULT,
    >,
    pub get_HandLeftState: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, handState: *mut HandState) -> HRESULT,
    >,
    pub get_HandLeftConfidence: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, confidence: *mut TrackingConfidence) -> HRESULT,
    >,
    pub get_HandRightState: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, handState: *mut HandState) -> HRESULT,
    >,
    pub get_HandRightConfidence: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, confidence: *mut TrackingConfidence) -> HRESULT,
    >,
    pub get_ClippedEdges: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, clippedEdges: *mut ULONG) -> HRESULT,
    >,
    pub get_TrackingId: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, trackingId: *mut UINT64) -> HRESULT,
    >,
    pub get_IsTracked: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, tracked: *mut BOOLEAN) -> HRESULT,
    >,
    pub get_IsRestricted: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, isRestricted: *mut BOOLEAN) -> HRESULT,
    >,
    pub get_Lean: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, amount: *mut PointF) -> HRESULT,
    >,
    pub get_LeanTrackingState: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBody, trackingState: *mut TrackingState) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyVtbl"][::std::mem::size_of::<IBodyVtbl>() - 152usize];
    ["Alignment of IBodyVtbl"][::std::mem::align_of::<IBodyVtbl>() - 8usize];
    ["Offset of field: IBodyVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyVtbl::AddRef"][::std::mem::offset_of!(IBodyVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyVtbl::Release"][::std::mem::offset_of!(IBodyVtbl, Release) - 16usize];
    ["Offset of field: IBodyVtbl::GetJoints"]
        [::std::mem::offset_of!(IBodyVtbl, GetJoints) - 24usize];
    ["Offset of field: IBodyVtbl::GetJointOrientations"]
        [::std::mem::offset_of!(IBodyVtbl, GetJointOrientations) - 32usize];
    ["Offset of field: IBodyVtbl::get_Engaged"]
        [::std::mem::offset_of!(IBodyVtbl, get_Engaged) - 40usize];
    ["Offset of field: IBodyVtbl::GetExpressionDetectionResults"]
        [::std::mem::offset_of!(IBodyVtbl, GetExpressionDetectionResults) - 48usize];
    ["Offset of field: IBodyVtbl::GetActivityDetectionResults"]
        [::std::mem::offset_of!(IBodyVtbl, GetActivityDetectionResults) - 56usize];
    ["Offset of field: IBodyVtbl::GetAppearanceDetectionResults"]
        [::std::mem::offset_of!(IBodyVtbl, GetAppearanceDetectionResults) - 64usize];
    ["Offset of field: IBodyVtbl::get_HandLeftState"]
        [::std::mem::offset_of!(IBodyVtbl, get_HandLeftState) - 72usize];
    ["Offset of field: IBodyVtbl::get_HandLeftConfidence"]
        [::std::mem::offset_of!(IBodyVtbl, get_HandLeftConfidence) - 80usize];
    ["Offset of field: IBodyVtbl::get_HandRightState"]
        [::std::mem::offset_of!(IBodyVtbl, get_HandRightState) - 88usize];
    ["Offset of field: IBodyVtbl::get_HandRightConfidence"]
        [::std::mem::offset_of!(IBodyVtbl, get_HandRightConfidence) - 96usize];
    ["Offset of field: IBodyVtbl::get_ClippedEdges"]
        [::std::mem::offset_of!(IBodyVtbl, get_ClippedEdges) - 104usize];
    ["Offset of field: IBodyVtbl::get_TrackingId"]
        [::std::mem::offset_of!(IBodyVtbl, get_TrackingId) - 112usize];
    ["Offset of field: IBodyVtbl::get_IsTracked"]
        [::std::mem::offset_of!(IBodyVtbl, get_IsTracked) - 120usize];
    ["Offset of field: IBodyVtbl::get_IsRestricted"]
        [::std::mem::offset_of!(IBodyVtbl, get_IsRestricted) - 128usize];
    ["Offset of field: IBodyVtbl::get_Lean"]
        [::std::mem::offset_of!(IBodyVtbl, get_Lean) - 136usize];
    ["Offset of field: IBodyVtbl::get_LeanTrackingState"]
        [::std::mem::offset_of!(IBodyVtbl, get_LeanTrackingState) - 144usize];
};
#[repr(C)]
pub struct IBody {
    pub lpVtbl: *mut IBodyVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBody"][::std::mem::size_of::<IBody>() - 8usize];
    ["Alignment of IBody"][::std::mem::align_of::<IBody>() - 8usize];
    ["Offset of field: IBody::lpVtbl"][::std::mem::offset_of!(IBody, lpVtbl) - 0usize];
};
impl Default for IBody {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyIndexFrameReference: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyIndexFrameReferenceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameReference,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyIndexFrameReference) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyIndexFrameReference) -> ULONG>,
    pub AcquireFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameReference,
            bodyIndexFrame: *mut *mut IBodyIndexFrame,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameReference,
            relativeTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrameReferenceVtbl"]
        [::std::mem::size_of::<IBodyIndexFrameReferenceVtbl>() - 40usize];
    ["Alignment of IBodyIndexFrameReferenceVtbl"]
        [::std::mem::align_of::<IBodyIndexFrameReferenceVtbl>() - 8usize];
    ["Offset of field: IBodyIndexFrameReferenceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyIndexFrameReferenceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyIndexFrameReferenceVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyIndexFrameReferenceVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyIndexFrameReferenceVtbl::Release"]
        [::std::mem::offset_of!(IBodyIndexFrameReferenceVtbl, Release) - 16usize];
    ["Offset of field: IBodyIndexFrameReferenceVtbl::AcquireFrame"]
        [::std::mem::offset_of!(IBodyIndexFrameReferenceVtbl, AcquireFrame) - 24usize];
    ["Offset of field: IBodyIndexFrameReferenceVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IBodyIndexFrameReferenceVtbl, get_RelativeTime) - 32usize];
};
#[repr(C)]
pub struct IBodyIndexFrameReference {
    pub lpVtbl: *mut IBodyIndexFrameReferenceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrameReference"]
        [::std::mem::size_of::<IBodyIndexFrameReference>() - 8usize];
    ["Alignment of IBodyIndexFrameReference"]
        [::std::mem::align_of::<IBodyIndexFrameReference>() - 8usize];
    ["Offset of field: IBodyIndexFrameReference::lpVtbl"]
        [::std::mem::offset_of!(IBodyIndexFrameReference, lpVtbl) - 0usize];
};
impl Default for IBodyIndexFrameReference {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyIndexFrameArrivedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyIndexFrameArrivedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameArrivedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyIndexFrameArrivedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyIndexFrameArrivedEventArgs) -> ULONG,
    >,
    pub get_FrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameArrivedEventArgs,
            bodyIndexFrameReference: *mut *mut IBodyIndexFrameReference,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrameArrivedEventArgsVtbl"]
        [::std::mem::size_of::<IBodyIndexFrameArrivedEventArgsVtbl>() - 32usize];
    ["Alignment of IBodyIndexFrameArrivedEventArgsVtbl"]
        [::std::mem::align_of::<IBodyIndexFrameArrivedEventArgsVtbl>() - 8usize];
    ["Offset of field: IBodyIndexFrameArrivedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyIndexFrameArrivedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyIndexFrameArrivedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyIndexFrameArrivedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyIndexFrameArrivedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IBodyIndexFrameArrivedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IBodyIndexFrameArrivedEventArgsVtbl::get_FrameReference"]
        [::std::mem::offset_of!(IBodyIndexFrameArrivedEventArgsVtbl, get_FrameReference) - 24usize];
};
#[repr(C)]
pub struct IBodyIndexFrameArrivedEventArgs {
    pub lpVtbl: *mut IBodyIndexFrameArrivedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrameArrivedEventArgs"]
        [::std::mem::size_of::<IBodyIndexFrameArrivedEventArgs>() - 8usize];
    ["Alignment of IBodyIndexFrameArrivedEventArgs"]
        [::std::mem::align_of::<IBodyIndexFrameArrivedEventArgs>() - 8usize];
    ["Offset of field: IBodyIndexFrameArrivedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IBodyIndexFrameArrivedEventArgs, lpVtbl) - 0usize];
};
impl Default for IBodyIndexFrameArrivedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyIndexFrameSource: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyIndexFrameSourceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameSource,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyIndexFrameSource) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyIndexFrameSource) -> ULONG>,
    pub SubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameSource,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameSource,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameCapturedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameSource,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IFrameCapturedEventArgs,
        ) -> HRESULT,
    >,
    pub get_IsActive: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyIndexFrameSource, isActive: *mut BOOLEAN) -> HRESULT,
    >,
    pub OpenReader: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameSource,
            reader: *mut *mut IBodyIndexFrameReader,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameSource,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_KinectSensor: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameSource,
            sensor: *mut *mut IKinectSensor,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrameSourceVtbl"]
        [::std::mem::size_of::<IBodyIndexFrameSourceVtbl>() - 80usize];
    ["Alignment of IBodyIndexFrameSourceVtbl"]
        [::std::mem::align_of::<IBodyIndexFrameSourceVtbl>() - 8usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::Release"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, Release) - 16usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::SubscribeFrameCaptured"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, SubscribeFrameCaptured) - 24usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::UnsubscribeFrameCaptured"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, UnsubscribeFrameCaptured) - 32usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::GetFrameCapturedEventData"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, GetFrameCapturedEventData) - 40usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::get_IsActive"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, get_IsActive) - 48usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::OpenReader"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, OpenReader) - 56usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::get_FrameDescription"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, get_FrameDescription) - 64usize];
    ["Offset of field: IBodyIndexFrameSourceVtbl::get_KinectSensor"]
        [::std::mem::offset_of!(IBodyIndexFrameSourceVtbl, get_KinectSensor) - 72usize];
};
#[repr(C)]
pub struct IBodyIndexFrameSource {
    pub lpVtbl: *mut IBodyIndexFrameSourceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrameSource"][::std::mem::size_of::<IBodyIndexFrameSource>() - 8usize];
    ["Alignment of IBodyIndexFrameSource"]
        [::std::mem::align_of::<IBodyIndexFrameSource>() - 8usize];
    ["Offset of field: IBodyIndexFrameSource::lpVtbl"]
        [::std::mem::offset_of!(IBodyIndexFrameSource, lpVtbl) - 0usize];
};
impl Default for IBodyIndexFrameSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyIndexFrameReader: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyIndexFrameReaderVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameReader,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyIndexFrameReader) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyIndexFrameReader) -> ULONG>,
    pub SubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameReader,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameReader,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameArrivedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameReader,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IBodyIndexFrameArrivedEventArgs,
        ) -> HRESULT,
    >,
    pub AcquireLatestFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameReader,
            bodyIndexFrame: *mut *mut IBodyIndexFrame,
        ) -> HRESULT,
    >,
    pub get_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyIndexFrameReader, isPaused: *mut BOOLEAN) -> HRESULT,
    >,
    pub put_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyIndexFrameReader, isPaused: BOOLEAN) -> HRESULT,
    >,
    pub get_BodyIndexFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrameReader,
            bodyIndexFrameSource: *mut *mut IBodyIndexFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrameReaderVtbl"]
        [::std::mem::size_of::<IBodyIndexFrameReaderVtbl>() - 80usize];
    ["Alignment of IBodyIndexFrameReaderVtbl"]
        [::std::mem::align_of::<IBodyIndexFrameReaderVtbl>() - 8usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::Release"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, Release) - 16usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::SubscribeFrameArrived"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, SubscribeFrameArrived) - 24usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::UnsubscribeFrameArrived"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, UnsubscribeFrameArrived) - 32usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::GetFrameArrivedEventData"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, GetFrameArrivedEventData) - 40usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::AcquireLatestFrame"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, AcquireLatestFrame) - 48usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::get_IsPaused"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, get_IsPaused) - 56usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::put_IsPaused"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, put_IsPaused) - 64usize];
    ["Offset of field: IBodyIndexFrameReaderVtbl::get_BodyIndexFrameSource"]
        [::std::mem::offset_of!(IBodyIndexFrameReaderVtbl, get_BodyIndexFrameSource) - 72usize];
};
#[repr(C)]
pub struct IBodyIndexFrameReader {
    pub lpVtbl: *mut IBodyIndexFrameReaderVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrameReader"][::std::mem::size_of::<IBodyIndexFrameReader>() - 8usize];
    ["Alignment of IBodyIndexFrameReader"]
        [::std::mem::align_of::<IBodyIndexFrameReader>() - 8usize];
    ["Offset of field: IBodyIndexFrameReader::lpVtbl"]
        [::std::mem::offset_of!(IBodyIndexFrameReader, lpVtbl) - 0usize];
};
impl Default for IBodyIndexFrameReader {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IBodyIndexFrame: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyIndexFrameVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrame,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyIndexFrame) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyIndexFrame) -> ULONG>,
    pub CopyFrameDataToArray: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrame,
            capacity: UINT,
            frameData: *mut BYTE,
        ) -> HRESULT,
    >,
    pub AccessUnderlyingBuffer: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrame,
            capacity: *mut UINT,
            buffer: *mut *mut BYTE,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrame,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyIndexFrame, relativeTime: *mut TIMESPAN) -> HRESULT,
    >,
    pub get_BodyIndexFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyIndexFrame,
            bodyIndexFrameSource: *mut *mut IBodyIndexFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrameVtbl"][::std::mem::size_of::<IBodyIndexFrameVtbl>() - 64usize];
    ["Alignment of IBodyIndexFrameVtbl"][::std::mem::align_of::<IBodyIndexFrameVtbl>() - 8usize];
    ["Offset of field: IBodyIndexFrameVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyIndexFrameVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyIndexFrameVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyIndexFrameVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyIndexFrameVtbl::Release"]
        [::std::mem::offset_of!(IBodyIndexFrameVtbl, Release) - 16usize];
    ["Offset of field: IBodyIndexFrameVtbl::CopyFrameDataToArray"]
        [::std::mem::offset_of!(IBodyIndexFrameVtbl, CopyFrameDataToArray) - 24usize];
    ["Offset of field: IBodyIndexFrameVtbl::AccessUnderlyingBuffer"]
        [::std::mem::offset_of!(IBodyIndexFrameVtbl, AccessUnderlyingBuffer) - 32usize];
    ["Offset of field: IBodyIndexFrameVtbl::get_FrameDescription"]
        [::std::mem::offset_of!(IBodyIndexFrameVtbl, get_FrameDescription) - 40usize];
    ["Offset of field: IBodyIndexFrameVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IBodyIndexFrameVtbl, get_RelativeTime) - 48usize];
    ["Offset of field: IBodyIndexFrameVtbl::get_BodyIndexFrameSource"]
        [::std::mem::offset_of!(IBodyIndexFrameVtbl, get_BodyIndexFrameSource) - 56usize];
};
#[repr(C)]
pub struct IBodyIndexFrame {
    pub lpVtbl: *mut IBodyIndexFrameVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyIndexFrame"][::std::mem::size_of::<IBodyIndexFrame>() - 8usize];
    ["Alignment of IBodyIndexFrame"][::std::mem::align_of::<IBodyIndexFrame>() - 8usize];
    ["Offset of field: IBodyIndexFrame::lpVtbl"]
        [::std::mem::offset_of!(IBodyIndexFrame, lpVtbl) - 0usize];
};
impl Default for IBodyIndexFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IInfraredFrameReference: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IInfraredFrameReferenceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameReference,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IInfraredFrameReference) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IInfraredFrameReference) -> ULONG>,
    pub AcquireFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameReference,
            infraredFrame: *mut *mut IInfraredFrame,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameReference,
            relativeTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrameReferenceVtbl"]
        [::std::mem::size_of::<IInfraredFrameReferenceVtbl>() - 40usize];
    ["Alignment of IInfraredFrameReferenceVtbl"]
        [::std::mem::align_of::<IInfraredFrameReferenceVtbl>() - 8usize];
    ["Offset of field: IInfraredFrameReferenceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IInfraredFrameReferenceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IInfraredFrameReferenceVtbl::AddRef"]
        [::std::mem::offset_of!(IInfraredFrameReferenceVtbl, AddRef) - 8usize];
    ["Offset of field: IInfraredFrameReferenceVtbl::Release"]
        [::std::mem::offset_of!(IInfraredFrameReferenceVtbl, Release) - 16usize];
    ["Offset of field: IInfraredFrameReferenceVtbl::AcquireFrame"]
        [::std::mem::offset_of!(IInfraredFrameReferenceVtbl, AcquireFrame) - 24usize];
    ["Offset of field: IInfraredFrameReferenceVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IInfraredFrameReferenceVtbl, get_RelativeTime) - 32usize];
};
#[repr(C)]
pub struct IInfraredFrameReference {
    pub lpVtbl: *mut IInfraredFrameReferenceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrameReference"][::std::mem::size_of::<IInfraredFrameReference>() - 8usize];
    ["Alignment of IInfraredFrameReference"]
        [::std::mem::align_of::<IInfraredFrameReference>() - 8usize];
    ["Offset of field: IInfraredFrameReference::lpVtbl"]
        [::std::mem::offset_of!(IInfraredFrameReference, lpVtbl) - 0usize];
};
impl Default for IInfraredFrameReference {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IInfraredFrameArrivedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IInfraredFrameArrivedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameArrivedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IInfraredFrameArrivedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IInfraredFrameArrivedEventArgs) -> ULONG,
    >,
    pub get_FrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameArrivedEventArgs,
            infraredFrameReference: *mut *mut IInfraredFrameReference,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrameArrivedEventArgsVtbl"]
        [::std::mem::size_of::<IInfraredFrameArrivedEventArgsVtbl>() - 32usize];
    ["Alignment of IInfraredFrameArrivedEventArgsVtbl"]
        [::std::mem::align_of::<IInfraredFrameArrivedEventArgsVtbl>() - 8usize];
    ["Offset of field: IInfraredFrameArrivedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IInfraredFrameArrivedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IInfraredFrameArrivedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IInfraredFrameArrivedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IInfraredFrameArrivedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IInfraredFrameArrivedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IInfraredFrameArrivedEventArgsVtbl::get_FrameReference"]
        [::std::mem::offset_of!(IInfraredFrameArrivedEventArgsVtbl, get_FrameReference) - 24usize];
};
#[repr(C)]
pub struct IInfraredFrameArrivedEventArgs {
    pub lpVtbl: *mut IInfraredFrameArrivedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrameArrivedEventArgs"]
        [::std::mem::size_of::<IInfraredFrameArrivedEventArgs>() - 8usize];
    ["Alignment of IInfraredFrameArrivedEventArgs"]
        [::std::mem::align_of::<IInfraredFrameArrivedEventArgs>() - 8usize];
    ["Offset of field: IInfraredFrameArrivedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IInfraredFrameArrivedEventArgs, lpVtbl) - 0usize];
};
impl Default for IInfraredFrameArrivedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IInfraredFrameSource: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IInfraredFrameSourceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameSource,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IInfraredFrameSource) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IInfraredFrameSource) -> ULONG>,
    pub SubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameSource,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameSource,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameCapturedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameSource,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IFrameCapturedEventArgs,
        ) -> HRESULT,
    >,
    pub get_IsActive: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IInfraredFrameSource, isActive: *mut BOOLEAN) -> HRESULT,
    >,
    pub OpenReader: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameSource,
            reader: *mut *mut IInfraredFrameReader,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameSource,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_KinectSensor: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameSource,
            sensor: *mut *mut IKinectSensor,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrameSourceVtbl"]
        [::std::mem::size_of::<IInfraredFrameSourceVtbl>() - 80usize];
    ["Alignment of IInfraredFrameSourceVtbl"]
        [::std::mem::align_of::<IInfraredFrameSourceVtbl>() - 8usize];
    ["Offset of field: IInfraredFrameSourceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IInfraredFrameSourceVtbl::AddRef"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, AddRef) - 8usize];
    ["Offset of field: IInfraredFrameSourceVtbl::Release"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, Release) - 16usize];
    ["Offset of field: IInfraredFrameSourceVtbl::SubscribeFrameCaptured"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, SubscribeFrameCaptured) - 24usize];
    ["Offset of field: IInfraredFrameSourceVtbl::UnsubscribeFrameCaptured"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, UnsubscribeFrameCaptured) - 32usize];
    ["Offset of field: IInfraredFrameSourceVtbl::GetFrameCapturedEventData"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, GetFrameCapturedEventData) - 40usize];
    ["Offset of field: IInfraredFrameSourceVtbl::get_IsActive"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, get_IsActive) - 48usize];
    ["Offset of field: IInfraredFrameSourceVtbl::OpenReader"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, OpenReader) - 56usize];
    ["Offset of field: IInfraredFrameSourceVtbl::get_FrameDescription"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, get_FrameDescription) - 64usize];
    ["Offset of field: IInfraredFrameSourceVtbl::get_KinectSensor"]
        [::std::mem::offset_of!(IInfraredFrameSourceVtbl, get_KinectSensor) - 72usize];
};
#[repr(C)]
pub struct IInfraredFrameSource {
    pub lpVtbl: *mut IInfraredFrameSourceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrameSource"][::std::mem::size_of::<IInfraredFrameSource>() - 8usize];
    ["Alignment of IInfraredFrameSource"][::std::mem::align_of::<IInfraredFrameSource>() - 8usize];
    ["Offset of field: IInfraredFrameSource::lpVtbl"]
        [::std::mem::offset_of!(IInfraredFrameSource, lpVtbl) - 0usize];
};
impl Default for IInfraredFrameSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IInfraredFrameReader: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IInfraredFrameReaderVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameReader,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IInfraredFrameReader) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IInfraredFrameReader) -> ULONG>,
    pub SubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameReader,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameReader,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameArrivedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameReader,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IInfraredFrameArrivedEventArgs,
        ) -> HRESULT,
    >,
    pub AcquireLatestFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameReader,
            infraredFrame: *mut *mut IInfraredFrame,
        ) -> HRESULT,
    >,
    pub get_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IInfraredFrameReader, isPaused: *mut BOOLEAN) -> HRESULT,
    >,
    pub put_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IInfraredFrameReader, isPaused: BOOLEAN) -> HRESULT,
    >,
    pub get_InfraredFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrameReader,
            infraredFrameSource: *mut *mut IInfraredFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrameReaderVtbl"]
        [::std::mem::size_of::<IInfraredFrameReaderVtbl>() - 80usize];
    ["Alignment of IInfraredFrameReaderVtbl"]
        [::std::mem::align_of::<IInfraredFrameReaderVtbl>() - 8usize];
    ["Offset of field: IInfraredFrameReaderVtbl::QueryInterface"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, QueryInterface) - 0usize];
    ["Offset of field: IInfraredFrameReaderVtbl::AddRef"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, AddRef) - 8usize];
    ["Offset of field: IInfraredFrameReaderVtbl::Release"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, Release) - 16usize];
    ["Offset of field: IInfraredFrameReaderVtbl::SubscribeFrameArrived"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, SubscribeFrameArrived) - 24usize];
    ["Offset of field: IInfraredFrameReaderVtbl::UnsubscribeFrameArrived"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, UnsubscribeFrameArrived) - 32usize];
    ["Offset of field: IInfraredFrameReaderVtbl::GetFrameArrivedEventData"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, GetFrameArrivedEventData) - 40usize];
    ["Offset of field: IInfraredFrameReaderVtbl::AcquireLatestFrame"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, AcquireLatestFrame) - 48usize];
    ["Offset of field: IInfraredFrameReaderVtbl::get_IsPaused"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, get_IsPaused) - 56usize];
    ["Offset of field: IInfraredFrameReaderVtbl::put_IsPaused"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, put_IsPaused) - 64usize];
    ["Offset of field: IInfraredFrameReaderVtbl::get_InfraredFrameSource"]
        [::std::mem::offset_of!(IInfraredFrameReaderVtbl, get_InfraredFrameSource) - 72usize];
};
#[repr(C)]
pub struct IInfraredFrameReader {
    pub lpVtbl: *mut IInfraredFrameReaderVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrameReader"][::std::mem::size_of::<IInfraredFrameReader>() - 8usize];
    ["Alignment of IInfraredFrameReader"][::std::mem::align_of::<IInfraredFrameReader>() - 8usize];
    ["Offset of field: IInfraredFrameReader::lpVtbl"]
        [::std::mem::offset_of!(IInfraredFrameReader, lpVtbl) - 0usize];
};
impl Default for IInfraredFrameReader {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IInfraredFrame: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IInfraredFrameVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrame,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IInfraredFrame) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IInfraredFrame) -> ULONG>,
    pub CopyFrameDataToArray: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrame,
            capacity: UINT,
            frameData: *mut UINT16,
        ) -> HRESULT,
    >,
    pub AccessUnderlyingBuffer: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrame,
            capacity: *mut UINT,
            buffer: *mut *mut UINT16,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrame,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IInfraredFrame, relativeTime: *mut TIMESPAN) -> HRESULT,
    >,
    pub get_InfraredFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IInfraredFrame,
            infraredFrameSource: *mut *mut IInfraredFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrameVtbl"][::std::mem::size_of::<IInfraredFrameVtbl>() - 64usize];
    ["Alignment of IInfraredFrameVtbl"][::std::mem::align_of::<IInfraredFrameVtbl>() - 8usize];
    ["Offset of field: IInfraredFrameVtbl::QueryInterface"]
        [::std::mem::offset_of!(IInfraredFrameVtbl, QueryInterface) - 0usize];
    ["Offset of field: IInfraredFrameVtbl::AddRef"]
        [::std::mem::offset_of!(IInfraredFrameVtbl, AddRef) - 8usize];
    ["Offset of field: IInfraredFrameVtbl::Release"]
        [::std::mem::offset_of!(IInfraredFrameVtbl, Release) - 16usize];
    ["Offset of field: IInfraredFrameVtbl::CopyFrameDataToArray"]
        [::std::mem::offset_of!(IInfraredFrameVtbl, CopyFrameDataToArray) - 24usize];
    ["Offset of field: IInfraredFrameVtbl::AccessUnderlyingBuffer"]
        [::std::mem::offset_of!(IInfraredFrameVtbl, AccessUnderlyingBuffer) - 32usize];
    ["Offset of field: IInfraredFrameVtbl::get_FrameDescription"]
        [::std::mem::offset_of!(IInfraredFrameVtbl, get_FrameDescription) - 40usize];
    ["Offset of field: IInfraredFrameVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IInfraredFrameVtbl, get_RelativeTime) - 48usize];
    ["Offset of field: IInfraredFrameVtbl::get_InfraredFrameSource"]
        [::std::mem::offset_of!(IInfraredFrameVtbl, get_InfraredFrameSource) - 56usize];
};
#[repr(C)]
pub struct IInfraredFrame {
    pub lpVtbl: *mut IInfraredFrameVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IInfraredFrame"][::std::mem::size_of::<IInfraredFrame>() - 8usize];
    ["Alignment of IInfraredFrame"][::std::mem::align_of::<IInfraredFrame>() - 8usize];
    ["Offset of field: IInfraredFrame::lpVtbl"]
        [::std::mem::offset_of!(IInfraredFrame, lpVtbl) - 0usize];
};
impl Default for IInfraredFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_ILongExposureInfraredFrameReference: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct ILongExposureInfraredFrameReferenceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReference,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ILongExposureInfraredFrameReference) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ILongExposureInfraredFrameReference) -> ULONG,
    >,
    pub AcquireFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReference,
            longExposureInfraredFrame: *mut *mut ILongExposureInfraredFrame,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReference,
            relativeTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrameReferenceVtbl"]
        [::std::mem::size_of::<ILongExposureInfraredFrameReferenceVtbl>() - 40usize];
    ["Alignment of ILongExposureInfraredFrameReferenceVtbl"]
        [::std::mem::align_of::<ILongExposureInfraredFrameReferenceVtbl>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrameReferenceVtbl::QueryInterface"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReferenceVtbl, QueryInterface) - 0usize];
    ["Offset of field: ILongExposureInfraredFrameReferenceVtbl::AddRef"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReferenceVtbl, AddRef) - 8usize];
    ["Offset of field: ILongExposureInfraredFrameReferenceVtbl::Release"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReferenceVtbl, Release) - 16usize];
    ["Offset of field: ILongExposureInfraredFrameReferenceVtbl::AcquireFrame"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReferenceVtbl, AcquireFrame) - 24usize];
    ["Offset of field: ILongExposureInfraredFrameReferenceVtbl::get_RelativeTime"][::std::mem::offset_of!(
        ILongExposureInfraredFrameReferenceVtbl,
        get_RelativeTime
    ) - 32usize];
};
#[repr(C)]
pub struct ILongExposureInfraredFrameReference {
    pub lpVtbl: *mut ILongExposureInfraredFrameReferenceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrameReference"]
        [::std::mem::size_of::<ILongExposureInfraredFrameReference>() - 8usize];
    ["Alignment of ILongExposureInfraredFrameReference"]
        [::std::mem::align_of::<ILongExposureInfraredFrameReference>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrameReference::lpVtbl"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReference, lpVtbl) - 0usize];
};
impl Default for ILongExposureInfraredFrameReference {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_ILongExposureInfraredFrameArrivedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct ILongExposureInfraredFrameArrivedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameArrivedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ILongExposureInfraredFrameArrivedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ILongExposureInfraredFrameArrivedEventArgs) -> ULONG,
    >,
    pub get_FrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameArrivedEventArgs,
            longExposureInfraredFrameReference: *mut *mut ILongExposureInfraredFrameReference,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrameArrivedEventArgsVtbl"]
        [::std::mem::size_of::<ILongExposureInfraredFrameArrivedEventArgsVtbl>() - 32usize];
    ["Alignment of ILongExposureInfraredFrameArrivedEventArgsVtbl"]
        [::std::mem::align_of::<ILongExposureInfraredFrameArrivedEventArgsVtbl>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrameArrivedEventArgsVtbl::QueryInterface"][::std::mem::offset_of!(
        ILongExposureInfraredFrameArrivedEventArgsVtbl,
        QueryInterface
    ) - 0usize];
    ["Offset of field: ILongExposureInfraredFrameArrivedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameArrivedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: ILongExposureInfraredFrameArrivedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameArrivedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: ILongExposureInfraredFrameArrivedEventArgsVtbl::get_FrameReference"][::std::mem::offset_of!(
        ILongExposureInfraredFrameArrivedEventArgsVtbl,
        get_FrameReference
    )
        - 24usize];
};
#[repr(C)]
pub struct ILongExposureInfraredFrameArrivedEventArgs {
    pub lpVtbl: *mut ILongExposureInfraredFrameArrivedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrameArrivedEventArgs"]
        [::std::mem::size_of::<ILongExposureInfraredFrameArrivedEventArgs>() - 8usize];
    ["Alignment of ILongExposureInfraredFrameArrivedEventArgs"]
        [::std::mem::align_of::<ILongExposureInfraredFrameArrivedEventArgs>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrameArrivedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameArrivedEventArgs, lpVtbl) - 0usize];
};
impl Default for ILongExposureInfraredFrameArrivedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_ILongExposureInfraredFrameSource: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct ILongExposureInfraredFrameSourceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameSource,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ILongExposureInfraredFrameSource) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ILongExposureInfraredFrameSource) -> ULONG,
    >,
    pub SubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameSource,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameSource,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameCapturedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameSource,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IFrameCapturedEventArgs,
        ) -> HRESULT,
    >,
    pub get_IsActive: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameSource,
            isActive: *mut BOOLEAN,
        ) -> HRESULT,
    >,
    pub OpenReader: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameSource,
            reader: *mut *mut ILongExposureInfraredFrameReader,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameSource,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_KinectSensor: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameSource,
            sensor: *mut *mut IKinectSensor,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrameSourceVtbl"]
        [::std::mem::size_of::<ILongExposureInfraredFrameSourceVtbl>() - 80usize];
    ["Alignment of ILongExposureInfraredFrameSourceVtbl"]
        [::std::mem::align_of::<ILongExposureInfraredFrameSourceVtbl>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::QueryInterface"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameSourceVtbl, QueryInterface) - 0usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::AddRef"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameSourceVtbl, AddRef) - 8usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::Release"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameSourceVtbl, Release) - 16usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::SubscribeFrameCaptured"][::std::mem::offset_of!(
        ILongExposureInfraredFrameSourceVtbl,
        SubscribeFrameCaptured
    ) - 24usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::UnsubscribeFrameCaptured"][::std::mem::offset_of!(
        ILongExposureInfraredFrameSourceVtbl,
        UnsubscribeFrameCaptured
    )
        - 32usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::GetFrameCapturedEventData"][::std::mem::offset_of!(
        ILongExposureInfraredFrameSourceVtbl,
        GetFrameCapturedEventData
    )
        - 40usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::get_IsActive"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameSourceVtbl, get_IsActive) - 48usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::OpenReader"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameSourceVtbl, OpenReader) - 56usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::get_FrameDescription"][::std::mem::offset_of!(
        ILongExposureInfraredFrameSourceVtbl,
        get_FrameDescription
    ) - 64usize];
    ["Offset of field: ILongExposureInfraredFrameSourceVtbl::get_KinectSensor"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameSourceVtbl, get_KinectSensor) - 72usize];
};
#[repr(C)]
pub struct ILongExposureInfraredFrameSource {
    pub lpVtbl: *mut ILongExposureInfraredFrameSourceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrameSource"]
        [::std::mem::size_of::<ILongExposureInfraredFrameSource>() - 8usize];
    ["Alignment of ILongExposureInfraredFrameSource"]
        [::std::mem::align_of::<ILongExposureInfraredFrameSource>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrameSource::lpVtbl"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameSource, lpVtbl) - 0usize];
};
impl Default for ILongExposureInfraredFrameSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_ILongExposureInfraredFrameReader: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct ILongExposureInfraredFrameReaderVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReader,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ILongExposureInfraredFrameReader) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ILongExposureInfraredFrameReader) -> ULONG,
    >,
    pub SubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReader,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReader,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameArrivedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReader,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut ILongExposureInfraredFrameArrivedEventArgs,
        ) -> HRESULT,
    >,
    pub AcquireLatestFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReader,
            longExposureInfraredFrame: *mut *mut ILongExposureInfraredFrame,
        ) -> HRESULT,
    >,
    pub get_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReader,
            isPaused: *mut BOOLEAN,
        ) -> HRESULT,
    >,
    pub put_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReader,
            isPaused: BOOLEAN,
        ) -> HRESULT,
    >,
    pub get_LongExposureInfraredFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrameReader,
            LongExposureInfraredFrameSource: *mut *mut ILongExposureInfraredFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrameReaderVtbl"]
        [::std::mem::size_of::<ILongExposureInfraredFrameReaderVtbl>() - 80usize];
    ["Alignment of ILongExposureInfraredFrameReaderVtbl"]
        [::std::mem::align_of::<ILongExposureInfraredFrameReaderVtbl>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::QueryInterface"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReaderVtbl, QueryInterface) - 0usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::AddRef"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReaderVtbl, AddRef) - 8usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::Release"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReaderVtbl, Release) - 16usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::SubscribeFrameArrived"][::std::mem::offset_of!(
        ILongExposureInfraredFrameReaderVtbl,
        SubscribeFrameArrived
    ) - 24usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::UnsubscribeFrameArrived"][::std::mem::offset_of!(
        ILongExposureInfraredFrameReaderVtbl,
        UnsubscribeFrameArrived
    ) - 32usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::GetFrameArrivedEventData"][::std::mem::offset_of!(
        ILongExposureInfraredFrameReaderVtbl,
        GetFrameArrivedEventData
    )
        - 40usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::AcquireLatestFrame"][::std::mem::offset_of!(
        ILongExposureInfraredFrameReaderVtbl,
        AcquireLatestFrame
    ) - 48usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::get_IsPaused"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReaderVtbl, get_IsPaused) - 56usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::put_IsPaused"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReaderVtbl, put_IsPaused) - 64usize];
    ["Offset of field: ILongExposureInfraredFrameReaderVtbl::get_LongExposureInfraredFrameSource"]
        [::std::mem::offset_of!(
            ILongExposureInfraredFrameReaderVtbl,
            get_LongExposureInfraredFrameSource
        ) - 72usize];
};
#[repr(C)]
pub struct ILongExposureInfraredFrameReader {
    pub lpVtbl: *mut ILongExposureInfraredFrameReaderVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrameReader"]
        [::std::mem::size_of::<ILongExposureInfraredFrameReader>() - 8usize];
    ["Alignment of ILongExposureInfraredFrameReader"]
        [::std::mem::align_of::<ILongExposureInfraredFrameReader>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrameReader::lpVtbl"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameReader, lpVtbl) - 0usize];
};
impl Default for ILongExposureInfraredFrameReader {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_ILongExposureInfraredFrame: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct ILongExposureInfraredFrameVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrame,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut ILongExposureInfraredFrame) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut ILongExposureInfraredFrame) -> ULONG>,
    pub CopyFrameDataToArray: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrame,
            capacity: UINT,
            frameData: *mut UINT16,
        ) -> HRESULT,
    >,
    pub AccessUnderlyingBuffer: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrame,
            capacity: *mut UINT,
            buffer: *mut *mut UINT16,
        ) -> HRESULT,
    >,
    pub get_FrameDescription: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrame,
            frameDescription: *mut *mut IFrameDescription,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrame,
            relativeTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
    pub get_LongExposureInfraredFrameSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ILongExposureInfraredFrame,
            longExposureInfraredFrameSource: *mut *mut ILongExposureInfraredFrameSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrameVtbl"]
        [::std::mem::size_of::<ILongExposureInfraredFrameVtbl>() - 64usize];
    ["Alignment of ILongExposureInfraredFrameVtbl"]
        [::std::mem::align_of::<ILongExposureInfraredFrameVtbl>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrameVtbl::QueryInterface"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameVtbl, QueryInterface) - 0usize];
    ["Offset of field: ILongExposureInfraredFrameVtbl::AddRef"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameVtbl, AddRef) - 8usize];
    ["Offset of field: ILongExposureInfraredFrameVtbl::Release"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameVtbl, Release) - 16usize];
    ["Offset of field: ILongExposureInfraredFrameVtbl::CopyFrameDataToArray"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameVtbl, CopyFrameDataToArray) - 24usize];
    ["Offset of field: ILongExposureInfraredFrameVtbl::AccessUnderlyingBuffer"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameVtbl, AccessUnderlyingBuffer) - 32usize];
    ["Offset of field: ILongExposureInfraredFrameVtbl::get_FrameDescription"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameVtbl, get_FrameDescription) - 40usize];
    ["Offset of field: ILongExposureInfraredFrameVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(ILongExposureInfraredFrameVtbl, get_RelativeTime) - 48usize];
    ["Offset of field: ILongExposureInfraredFrameVtbl::get_LongExposureInfraredFrameSource"][::std::mem::offset_of!(
        ILongExposureInfraredFrameVtbl,
        get_LongExposureInfraredFrameSource
    )
        - 56usize];
};
#[repr(C)]
pub struct ILongExposureInfraredFrame {
    pub lpVtbl: *mut ILongExposureInfraredFrameVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ILongExposureInfraredFrame"]
        [::std::mem::size_of::<ILongExposureInfraredFrame>() - 8usize];
    ["Alignment of ILongExposureInfraredFrame"]
        [::std::mem::align_of::<ILongExposureInfraredFrame>() - 8usize];
    ["Offset of field: ILongExposureInfraredFrame::lpVtbl"]
        [::std::mem::offset_of!(ILongExposureInfraredFrame, lpVtbl) - 0usize];
};
impl Default for ILongExposureInfraredFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioBeam: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioBeamVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeam,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeam) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeam) -> ULONG>,
    pub get_AudioSource: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeam, audioSource: *mut *mut IAudioSource) -> HRESULT,
    >,
    pub get_AudioBeamMode: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeam, audioBeamMode: *mut AudioBeamMode) -> HRESULT,
    >,
    pub put_AudioBeamMode: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeam, audioBeamMode: AudioBeamMode) -> HRESULT,
    >,
    pub get_BeamAngle: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeam, beamAngle: *mut f32) -> HRESULT,
    >,
    pub put_BeamAngle: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeam, beamAngle: f32) -> HRESULT,
    >,
    pub get_BeamAngleConfidence: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeam, beamAngleConfidence: *mut f32) -> HRESULT,
    >,
    pub OpenInputStream: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeam, stream: *mut *mut IStream) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeam, relativeTime: *mut TIMESPAN) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamVtbl"][::std::mem::size_of::<IAudioBeamVtbl>() - 88usize];
    ["Alignment of IAudioBeamVtbl"][::std::mem::align_of::<IAudioBeamVtbl>() - 8usize];
    ["Offset of field: IAudioBeamVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioBeamVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioBeamVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioBeamVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioBeamVtbl::Release"]
        [::std::mem::offset_of!(IAudioBeamVtbl, Release) - 16usize];
    ["Offset of field: IAudioBeamVtbl::get_AudioSource"]
        [::std::mem::offset_of!(IAudioBeamVtbl, get_AudioSource) - 24usize];
    ["Offset of field: IAudioBeamVtbl::get_AudioBeamMode"]
        [::std::mem::offset_of!(IAudioBeamVtbl, get_AudioBeamMode) - 32usize];
    ["Offset of field: IAudioBeamVtbl::put_AudioBeamMode"]
        [::std::mem::offset_of!(IAudioBeamVtbl, put_AudioBeamMode) - 40usize];
    ["Offset of field: IAudioBeamVtbl::get_BeamAngle"]
        [::std::mem::offset_of!(IAudioBeamVtbl, get_BeamAngle) - 48usize];
    ["Offset of field: IAudioBeamVtbl::put_BeamAngle"]
        [::std::mem::offset_of!(IAudioBeamVtbl, put_BeamAngle) - 56usize];
    ["Offset of field: IAudioBeamVtbl::get_BeamAngleConfidence"]
        [::std::mem::offset_of!(IAudioBeamVtbl, get_BeamAngleConfidence) - 64usize];
    ["Offset of field: IAudioBeamVtbl::OpenInputStream"]
        [::std::mem::offset_of!(IAudioBeamVtbl, OpenInputStream) - 72usize];
    ["Offset of field: IAudioBeamVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IAudioBeamVtbl, get_RelativeTime) - 80usize];
};
#[repr(C)]
pub struct IAudioBeam {
    pub lpVtbl: *mut IAudioBeamVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeam"][::std::mem::size_of::<IAudioBeam>() - 8usize];
    ["Alignment of IAudioBeam"][::std::mem::align_of::<IAudioBeam>() - 8usize];
    ["Offset of field: IAudioBeam::lpVtbl"][::std::mem::offset_of!(IAudioBeam, lpVtbl) - 0usize];
};
impl Default for IAudioBeam {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioBeamList: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioBeamListVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamList,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamList) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamList) -> ULONG>,
    pub get_BeamCount: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamList, count: *mut UINT) -> HRESULT,
    >,
    pub OpenAudioBeam: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamList,
            index: UINT,
            audioBeam: *mut *mut IAudioBeam,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamListVtbl"][::std::mem::size_of::<IAudioBeamListVtbl>() - 40usize];
    ["Alignment of IAudioBeamListVtbl"][::std::mem::align_of::<IAudioBeamListVtbl>() - 8usize];
    ["Offset of field: IAudioBeamListVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioBeamListVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioBeamListVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioBeamListVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioBeamListVtbl::Release"]
        [::std::mem::offset_of!(IAudioBeamListVtbl, Release) - 16usize];
    ["Offset of field: IAudioBeamListVtbl::get_BeamCount"]
        [::std::mem::offset_of!(IAudioBeamListVtbl, get_BeamCount) - 24usize];
    ["Offset of field: IAudioBeamListVtbl::OpenAudioBeam"]
        [::std::mem::offset_of!(IAudioBeamListVtbl, OpenAudioBeam) - 32usize];
};
#[repr(C)]
pub struct IAudioBeamList {
    pub lpVtbl: *mut IAudioBeamListVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamList"][::std::mem::size_of::<IAudioBeamList>() - 8usize];
    ["Alignment of IAudioBeamList"][::std::mem::align_of::<IAudioBeamList>() - 8usize];
    ["Offset of field: IAudioBeamList::lpVtbl"]
        [::std::mem::offset_of!(IAudioBeamList, lpVtbl) - 0usize];
};
impl Default for IAudioBeamList {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioBeamFrameList: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioBeamFrameListVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameList,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamFrameList) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamFrameList) -> ULONG>,
    pub get_BeamCount: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamFrameList, count: *mut UINT) -> HRESULT,
    >,
    pub OpenAudioBeamFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameList,
            index: UINT,
            audioBeamFrame: *mut *mut IAudioBeamFrame,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrameListVtbl"][::std::mem::size_of::<IAudioBeamFrameListVtbl>() - 40usize];
    ["Alignment of IAudioBeamFrameListVtbl"]
        [::std::mem::align_of::<IAudioBeamFrameListVtbl>() - 8usize];
    ["Offset of field: IAudioBeamFrameListVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioBeamFrameListVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioBeamFrameListVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioBeamFrameListVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioBeamFrameListVtbl::Release"]
        [::std::mem::offset_of!(IAudioBeamFrameListVtbl, Release) - 16usize];
    ["Offset of field: IAudioBeamFrameListVtbl::get_BeamCount"]
        [::std::mem::offset_of!(IAudioBeamFrameListVtbl, get_BeamCount) - 24usize];
    ["Offset of field: IAudioBeamFrameListVtbl::OpenAudioBeamFrame"]
        [::std::mem::offset_of!(IAudioBeamFrameListVtbl, OpenAudioBeamFrame) - 32usize];
};
#[repr(C)]
pub struct IAudioBeamFrameList {
    pub lpVtbl: *mut IAudioBeamFrameListVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrameList"][::std::mem::size_of::<IAudioBeamFrameList>() - 8usize];
    ["Alignment of IAudioBeamFrameList"][::std::mem::align_of::<IAudioBeamFrameList>() - 8usize];
    ["Offset of field: IAudioBeamFrameList::lpVtbl"]
        [::std::mem::offset_of!(IAudioBeamFrameList, lpVtbl) - 0usize];
};
impl Default for IAudioBeamFrameList {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioBeamFrame: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioBeamFrameVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrame,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamFrame) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamFrame) -> ULONG>,
    pub get_AudioSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrame,
            audioSource: *mut *mut IAudioSource,
        ) -> HRESULT,
    >,
    pub get_Duration: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamFrame, duration: *mut TIMESPAN) -> HRESULT,
    >,
    pub get_AudioBeam: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrame,
            audioBeam: *mut *mut IAudioBeam,
        ) -> HRESULT,
    >,
    pub get_SubFrameCount: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamFrame, pSubFrameCount: *mut UINT) -> HRESULT,
    >,
    pub GetSubFrame: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrame,
            subFrameIndex: UINT,
            audioBeamSubFrame: *mut *mut IAudioBeamSubFrame,
        ) -> HRESULT,
    >,
    pub get_RelativeTimeStart: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamFrame, relativeTime: *mut TIMESPAN) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrameVtbl"][::std::mem::size_of::<IAudioBeamFrameVtbl>() - 72usize];
    ["Alignment of IAudioBeamFrameVtbl"][::std::mem::align_of::<IAudioBeamFrameVtbl>() - 8usize];
    ["Offset of field: IAudioBeamFrameVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioBeamFrameVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioBeamFrameVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioBeamFrameVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioBeamFrameVtbl::Release"]
        [::std::mem::offset_of!(IAudioBeamFrameVtbl, Release) - 16usize];
    ["Offset of field: IAudioBeamFrameVtbl::get_AudioSource"]
        [::std::mem::offset_of!(IAudioBeamFrameVtbl, get_AudioSource) - 24usize];
    ["Offset of field: IAudioBeamFrameVtbl::get_Duration"]
        [::std::mem::offset_of!(IAudioBeamFrameVtbl, get_Duration) - 32usize];
    ["Offset of field: IAudioBeamFrameVtbl::get_AudioBeam"]
        [::std::mem::offset_of!(IAudioBeamFrameVtbl, get_AudioBeam) - 40usize];
    ["Offset of field: IAudioBeamFrameVtbl::get_SubFrameCount"]
        [::std::mem::offset_of!(IAudioBeamFrameVtbl, get_SubFrameCount) - 48usize];
    ["Offset of field: IAudioBeamFrameVtbl::GetSubFrame"]
        [::std::mem::offset_of!(IAudioBeamFrameVtbl, GetSubFrame) - 56usize];
    ["Offset of field: IAudioBeamFrameVtbl::get_RelativeTimeStart"]
        [::std::mem::offset_of!(IAudioBeamFrameVtbl, get_RelativeTimeStart) - 64usize];
};
#[repr(C)]
pub struct IAudioBeamFrame {
    pub lpVtbl: *mut IAudioBeamFrameVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrame"][::std::mem::size_of::<IAudioBeamFrame>() - 8usize];
    ["Alignment of IAudioBeamFrame"][::std::mem::align_of::<IAudioBeamFrame>() - 8usize];
    ["Offset of field: IAudioBeamFrame::lpVtbl"]
        [::std::mem::offset_of!(IAudioBeamFrame, lpVtbl) - 0usize];
};
impl Default for IAudioBeamFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioBeamSubFrame: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioBeamSubFrameVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamSubFrame,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamSubFrame) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamSubFrame) -> ULONG>,
    pub get_FrameLengthInBytes: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamSubFrame, length: *mut UINT) -> HRESULT,
    >,
    pub get_Duration: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamSubFrame, duration: *mut TIMESPAN) -> HRESULT,
    >,
    pub get_BeamAngle: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamSubFrame, beamAngle: *mut f32) -> HRESULT,
    >,
    pub get_BeamAngleConfidence: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamSubFrame,
            beamAngleConfidence: *mut f32,
        ) -> HRESULT,
    >,
    pub get_AudioBeamMode: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamSubFrame,
            audioBeamMode: *mut AudioBeamMode,
        ) -> HRESULT,
    >,
    pub get_AudioBodyCorrelationCount: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamSubFrame, pCount: *mut UINT) -> HRESULT,
    >,
    pub GetAudioBodyCorrelation: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamSubFrame,
            index: UINT,
            ppAudioBodyCorrelation: *mut *mut IAudioBodyCorrelation,
        ) -> HRESULT,
    >,
    pub CopyFrameDataToArray: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamSubFrame,
            capacity: UINT,
            frameData: *mut BYTE,
        ) -> HRESULT,
    >,
    pub AccessUnderlyingBuffer: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamSubFrame,
            capacity: *mut UINT,
            buffer: *mut *mut BYTE,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamSubFrame, relativeTime: *mut TIMESPAN) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamSubFrameVtbl"][::std::mem::size_of::<IAudioBeamSubFrameVtbl>() - 104usize];
    ["Alignment of IAudioBeamSubFrameVtbl"]
        [::std::mem::align_of::<IAudioBeamSubFrameVtbl>() - 8usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::Release"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, Release) - 16usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::get_FrameLengthInBytes"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, get_FrameLengthInBytes) - 24usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::get_Duration"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, get_Duration) - 32usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::get_BeamAngle"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, get_BeamAngle) - 40usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::get_BeamAngleConfidence"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, get_BeamAngleConfidence) - 48usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::get_AudioBeamMode"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, get_AudioBeamMode) - 56usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::get_AudioBodyCorrelationCount"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, get_AudioBodyCorrelationCount) - 64usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::GetAudioBodyCorrelation"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, GetAudioBodyCorrelation) - 72usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::CopyFrameDataToArray"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, CopyFrameDataToArray) - 80usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::AccessUnderlyingBuffer"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, AccessUnderlyingBuffer) - 88usize];
    ["Offset of field: IAudioBeamSubFrameVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IAudioBeamSubFrameVtbl, get_RelativeTime) - 96usize];
};
#[repr(C)]
pub struct IAudioBeamSubFrame {
    pub lpVtbl: *mut IAudioBeamSubFrameVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamSubFrame"][::std::mem::size_of::<IAudioBeamSubFrame>() - 8usize];
    ["Alignment of IAudioBeamSubFrame"][::std::mem::align_of::<IAudioBeamSubFrame>() - 8usize];
    ["Offset of field: IAudioBeamSubFrame::lpVtbl"]
        [::std::mem::offset_of!(IAudioBeamSubFrame, lpVtbl) - 0usize];
};
impl Default for IAudioBeamSubFrame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioBeamFrameReference: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioBeamFrameReferenceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameReference,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamFrameReference) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamFrameReference) -> ULONG>,
    pub AcquireBeamFrames: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameReference,
            audioBeamFrameList: *mut *mut IAudioBeamFrameList,
        ) -> HRESULT,
    >,
    pub get_RelativeTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameReference,
            relativeTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrameReferenceVtbl"]
        [::std::mem::size_of::<IAudioBeamFrameReferenceVtbl>() - 40usize];
    ["Alignment of IAudioBeamFrameReferenceVtbl"]
        [::std::mem::align_of::<IAudioBeamFrameReferenceVtbl>() - 8usize];
    ["Offset of field: IAudioBeamFrameReferenceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioBeamFrameReferenceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioBeamFrameReferenceVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioBeamFrameReferenceVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioBeamFrameReferenceVtbl::Release"]
        [::std::mem::offset_of!(IAudioBeamFrameReferenceVtbl, Release) - 16usize];
    ["Offset of field: IAudioBeamFrameReferenceVtbl::AcquireBeamFrames"]
        [::std::mem::offset_of!(IAudioBeamFrameReferenceVtbl, AcquireBeamFrames) - 24usize];
    ["Offset of field: IAudioBeamFrameReferenceVtbl::get_RelativeTime"]
        [::std::mem::offset_of!(IAudioBeamFrameReferenceVtbl, get_RelativeTime) - 32usize];
};
#[repr(C)]
pub struct IAudioBeamFrameReference {
    pub lpVtbl: *mut IAudioBeamFrameReferenceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrameReference"]
        [::std::mem::size_of::<IAudioBeamFrameReference>() - 8usize];
    ["Alignment of IAudioBeamFrameReference"]
        [::std::mem::align_of::<IAudioBeamFrameReference>() - 8usize];
    ["Offset of field: IAudioBeamFrameReference::lpVtbl"]
        [::std::mem::offset_of!(IAudioBeamFrameReference, lpVtbl) - 0usize];
};
impl Default for IAudioBeamFrameReference {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioBodyCorrelation: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioBodyCorrelationVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBodyCorrelation,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBodyCorrelation) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBodyCorrelation) -> ULONG>,
    pub get_BodyTrackingId: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBodyCorrelation, trackingId: *mut UINT64) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBodyCorrelationVtbl"]
        [::std::mem::size_of::<IAudioBodyCorrelationVtbl>() - 32usize];
    ["Alignment of IAudioBodyCorrelationVtbl"]
        [::std::mem::align_of::<IAudioBodyCorrelationVtbl>() - 8usize];
    ["Offset of field: IAudioBodyCorrelationVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioBodyCorrelationVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioBodyCorrelationVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioBodyCorrelationVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioBodyCorrelationVtbl::Release"]
        [::std::mem::offset_of!(IAudioBodyCorrelationVtbl, Release) - 16usize];
    ["Offset of field: IAudioBodyCorrelationVtbl::get_BodyTrackingId"]
        [::std::mem::offset_of!(IAudioBodyCorrelationVtbl, get_BodyTrackingId) - 24usize];
};
#[repr(C)]
pub struct IAudioBodyCorrelation {
    pub lpVtbl: *mut IAudioBodyCorrelationVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBodyCorrelation"][::std::mem::size_of::<IAudioBodyCorrelation>() - 8usize];
    ["Alignment of IAudioBodyCorrelation"]
        [::std::mem::align_of::<IAudioBodyCorrelation>() - 8usize];
    ["Offset of field: IAudioBodyCorrelation::lpVtbl"]
        [::std::mem::offset_of!(IAudioBodyCorrelation, lpVtbl) - 0usize];
};
impl Default for IAudioBodyCorrelation {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioBeamFrameArrivedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioBeamFrameArrivedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameArrivedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamFrameArrivedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamFrameArrivedEventArgs) -> ULONG,
    >,
    pub get_FrameReference: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameArrivedEventArgs,
            audioBeamFrameReference: *mut *mut IAudioBeamFrameReference,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrameArrivedEventArgsVtbl"]
        [::std::mem::size_of::<IAudioBeamFrameArrivedEventArgsVtbl>() - 32usize];
    ["Alignment of IAudioBeamFrameArrivedEventArgsVtbl"]
        [::std::mem::align_of::<IAudioBeamFrameArrivedEventArgsVtbl>() - 8usize];
    ["Offset of field: IAudioBeamFrameArrivedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioBeamFrameArrivedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioBeamFrameArrivedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioBeamFrameArrivedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioBeamFrameArrivedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IAudioBeamFrameArrivedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IAudioBeamFrameArrivedEventArgsVtbl::get_FrameReference"]
        [::std::mem::offset_of!(IAudioBeamFrameArrivedEventArgsVtbl, get_FrameReference) - 24usize];
};
#[repr(C)]
pub struct IAudioBeamFrameArrivedEventArgs {
    pub lpVtbl: *mut IAudioBeamFrameArrivedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrameArrivedEventArgs"]
        [::std::mem::size_of::<IAudioBeamFrameArrivedEventArgs>() - 8usize];
    ["Alignment of IAudioBeamFrameArrivedEventArgs"]
        [::std::mem::align_of::<IAudioBeamFrameArrivedEventArgs>() - 8usize];
    ["Offset of field: IAudioBeamFrameArrivedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IAudioBeamFrameArrivedEventArgs, lpVtbl) - 0usize];
};
impl Default for IAudioBeamFrameArrivedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioBeamFrameReader: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioBeamFrameReaderVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameReader,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamFrameReader) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioBeamFrameReader) -> ULONG>,
    pub SubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameReader,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameArrived: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameReader,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetFrameArrivedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameReader,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IAudioBeamFrameArrivedEventArgs,
        ) -> HRESULT,
    >,
    pub AcquireLatestBeamFrames: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameReader,
            audioBeamFrameList: *mut *mut IAudioBeamFrameList,
        ) -> HRESULT,
    >,
    pub get_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamFrameReader, isPaused: *mut BOOLEAN) -> HRESULT,
    >,
    pub put_IsPaused: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioBeamFrameReader, isPaused: BOOLEAN) -> HRESULT,
    >,
    pub get_AudioSource: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioBeamFrameReader,
            audioSource: *mut *mut IAudioSource,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrameReaderVtbl"]
        [::std::mem::size_of::<IAudioBeamFrameReaderVtbl>() - 80usize];
    ["Alignment of IAudioBeamFrameReaderVtbl"]
        [::std::mem::align_of::<IAudioBeamFrameReaderVtbl>() - 8usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::Release"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, Release) - 16usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::SubscribeFrameArrived"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, SubscribeFrameArrived) - 24usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::UnsubscribeFrameArrived"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, UnsubscribeFrameArrived) - 32usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::GetFrameArrivedEventData"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, GetFrameArrivedEventData) - 40usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::AcquireLatestBeamFrames"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, AcquireLatestBeamFrames) - 48usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::get_IsPaused"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, get_IsPaused) - 56usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::put_IsPaused"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, put_IsPaused) - 64usize];
    ["Offset of field: IAudioBeamFrameReaderVtbl::get_AudioSource"]
        [::std::mem::offset_of!(IAudioBeamFrameReaderVtbl, get_AudioSource) - 72usize];
};
#[repr(C)]
pub struct IAudioBeamFrameReader {
    pub lpVtbl: *mut IAudioBeamFrameReaderVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioBeamFrameReader"][::std::mem::size_of::<IAudioBeamFrameReader>() - 8usize];
    ["Alignment of IAudioBeamFrameReader"]
        [::std::mem::align_of::<IAudioBeamFrameReader>() - 8usize];
    ["Offset of field: IAudioBeamFrameReader::lpVtbl"]
        [::std::mem::offset_of!(IAudioBeamFrameReader, lpVtbl) - 0usize];
};
impl Default for IAudioBeamFrameReader {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IAudioSource: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IAudioSourceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioSource,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioSource) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IAudioSource) -> ULONG>,
    pub SubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioSource,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeFrameCaptured: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioSource, waitableHandle: WAITABLE_HANDLE) -> HRESULT,
    >,
    pub GetFrameCapturedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioSource,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IFrameCapturedEventArgs,
        ) -> HRESULT,
    >,
    pub get_KinectSensor: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioSource, sensor: *mut *mut IKinectSensor) -> HRESULT,
    >,
    pub get_IsActive: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioSource, isActive: *mut BOOLEAN) -> HRESULT,
    >,
    pub get_SubFrameLengthInBytes: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioSource, length: *mut UINT) -> HRESULT,
    >,
    pub get_SubFrameDuration: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioSource, duration: *mut TIMESPAN) -> HRESULT,
    >,
    pub get_MaxSubFrameCount: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IAudioSource, count: *mut UINT) -> HRESULT,
    >,
    pub OpenReader: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioSource,
            reader: *mut *mut IAudioBeamFrameReader,
        ) -> HRESULT,
    >,
    pub get_AudioBeams: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioSource,
            audioBeamList: *mut *mut IAudioBeamList,
        ) -> HRESULT,
    >,
    pub get_AudioCalibrationState: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IAudioSource,
            audioCalibrationState: *mut KinectAudioCalibrationState,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioSourceVtbl"][::std::mem::size_of::<IAudioSourceVtbl>() - 112usize];
    ["Alignment of IAudioSourceVtbl"][::std::mem::align_of::<IAudioSourceVtbl>() - 8usize];
    ["Offset of field: IAudioSourceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IAudioSourceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IAudioSourceVtbl::AddRef"]
        [::std::mem::offset_of!(IAudioSourceVtbl, AddRef) - 8usize];
    ["Offset of field: IAudioSourceVtbl::Release"]
        [::std::mem::offset_of!(IAudioSourceVtbl, Release) - 16usize];
    ["Offset of field: IAudioSourceVtbl::SubscribeFrameCaptured"]
        [::std::mem::offset_of!(IAudioSourceVtbl, SubscribeFrameCaptured) - 24usize];
    ["Offset of field: IAudioSourceVtbl::UnsubscribeFrameCaptured"]
        [::std::mem::offset_of!(IAudioSourceVtbl, UnsubscribeFrameCaptured) - 32usize];
    ["Offset of field: IAudioSourceVtbl::GetFrameCapturedEventData"]
        [::std::mem::offset_of!(IAudioSourceVtbl, GetFrameCapturedEventData) - 40usize];
    ["Offset of field: IAudioSourceVtbl::get_KinectSensor"]
        [::std::mem::offset_of!(IAudioSourceVtbl, get_KinectSensor) - 48usize];
    ["Offset of field: IAudioSourceVtbl::get_IsActive"]
        [::std::mem::offset_of!(IAudioSourceVtbl, get_IsActive) - 56usize];
    ["Offset of field: IAudioSourceVtbl::get_SubFrameLengthInBytes"]
        [::std::mem::offset_of!(IAudioSourceVtbl, get_SubFrameLengthInBytes) - 64usize];
    ["Offset of field: IAudioSourceVtbl::get_SubFrameDuration"]
        [::std::mem::offset_of!(IAudioSourceVtbl, get_SubFrameDuration) - 72usize];
    ["Offset of field: IAudioSourceVtbl::get_MaxSubFrameCount"]
        [::std::mem::offset_of!(IAudioSourceVtbl, get_MaxSubFrameCount) - 80usize];
    ["Offset of field: IAudioSourceVtbl::OpenReader"]
        [::std::mem::offset_of!(IAudioSourceVtbl, OpenReader) - 88usize];
    ["Offset of field: IAudioSourceVtbl::get_AudioBeams"]
        [::std::mem::offset_of!(IAudioSourceVtbl, get_AudioBeams) - 96usize];
    ["Offset of field: IAudioSourceVtbl::get_AudioCalibrationState"]
        [::std::mem::offset_of!(IAudioSourceVtbl, get_AudioCalibrationState) - 104usize];
};
#[repr(C)]
pub struct IAudioSource {
    pub lpVtbl: *mut IAudioSourceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IAudioSource"][::std::mem::size_of::<IAudioSource>() - 8usize];
    ["Alignment of IAudioSource"][::std::mem::align_of::<IAudioSource>() - 8usize];
    ["Offset of field: IAudioSource::lpVtbl"]
        [::std::mem::offset_of!(IAudioSource, lpVtbl) - 0usize];
};
impl Default for IAudioSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_ICoordinateMapper: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct ICoordinateMapperVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut ICoordinateMapper) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut ICoordinateMapper) -> ULONG>,
    pub SubscribeCoordinateMappingChanged: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribeCoordinateMappingChanged: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetCoordinateMappingChangedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut ICoordinateMappingChangedEventArgs,
        ) -> HRESULT,
    >,
    pub MapCameraPointToDepthSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            cameraPoint: CameraSpacePoint,
            depthPoint: *mut DepthSpacePoint,
        ) -> HRESULT,
    >,
    pub MapCameraPointToColorSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            cameraPoint: CameraSpacePoint,
            colorPoint: *mut ColorSpacePoint,
        ) -> HRESULT,
    >,
    pub MapDepthPointToCameraSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            depthPoint: DepthSpacePoint,
            depth: UINT16,
            cameraPoint: *mut CameraSpacePoint,
        ) -> HRESULT,
    >,
    pub MapDepthPointToColorSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            depthPoint: DepthSpacePoint,
            depth: UINT16,
            colorPoint: *mut ColorSpacePoint,
        ) -> HRESULT,
    >,
    pub MapCameraPointsToDepthSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            cameraPointCount: UINT,
            cameraPoints: *const CameraSpacePoint,
            depthPointCount: UINT,
            depthPoints: *mut DepthSpacePoint,
        ) -> HRESULT,
    >,
    pub MapCameraPointsToColorSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            cameraPointCount: UINT,
            cameraPoints: *const CameraSpacePoint,
            colorPointCount: UINT,
            colorPoints: *mut ColorSpacePoint,
        ) -> HRESULT,
    >,
    pub MapDepthPointsToCameraSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            depthPointCount: UINT,
            depthPoints: *const DepthSpacePoint,
            depthCount: UINT,
            depths: *const UINT16,
            cameraPointCount: UINT,
            cameraPoints: *mut CameraSpacePoint,
        ) -> HRESULT,
    >,
    pub MapDepthPointsToColorSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            depthPointCount: UINT,
            depthPoints: *const DepthSpacePoint,
            depthCount: UINT,
            depths: *const UINT16,
            colorPointCount: UINT,
            colorPoints: *mut ColorSpacePoint,
        ) -> HRESULT,
    >,
    pub MapDepthFrameToCameraSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            depthPointCount: UINT,
            depthFrameData: *const UINT16,
            cameraPointCount: UINT,
            cameraSpacePoints: *mut CameraSpacePoint,
        ) -> HRESULT,
    >,
    pub MapDepthFrameToColorSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            depthPointCount: UINT,
            depthFrameData: *const UINT16,
            colorPointCount: UINT,
            colorSpacePoints: *mut ColorSpacePoint,
        ) -> HRESULT,
    >,
    pub MapColorFrameToDepthSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            depthDataPointCount: UINT,
            depthFrameData: *const UINT16,
            depthPointCount: UINT,
            depthSpacePoints: *mut DepthSpacePoint,
        ) -> HRESULT,
    >,
    pub MapColorFrameToCameraSpace: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            depthDataPointCount: UINT,
            depthFrameData: *const UINT16,
            cameraPointCount: UINT,
            cameraSpacePoints: *mut CameraSpacePoint,
        ) -> HRESULT,
    >,
    pub GetDepthFrameToCameraSpaceTable: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            tableEntryCount: *mut UINT,
            tableEntries: *mut *mut PointF,
        ) -> HRESULT,
    >,
    pub GetDepthCameraIntrinsics: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMapper,
            cameraIntrinsics: *mut CameraIntrinsics,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ICoordinateMapperVtbl"][::std::mem::size_of::<ICoordinateMapperVtbl>() - 160usize];
    ["Alignment of ICoordinateMapperVtbl"]
        [::std::mem::align_of::<ICoordinateMapperVtbl>() - 8usize];
    ["Offset of field: ICoordinateMapperVtbl::QueryInterface"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, QueryInterface) - 0usize];
    ["Offset of field: ICoordinateMapperVtbl::AddRef"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, AddRef) - 8usize];
    ["Offset of field: ICoordinateMapperVtbl::Release"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, Release) - 16usize];
    ["Offset of field: ICoordinateMapperVtbl::SubscribeCoordinateMappingChanged"][::std::mem::offset_of!(
        ICoordinateMapperVtbl,
        SubscribeCoordinateMappingChanged
    ) - 24usize];
    ["Offset of field: ICoordinateMapperVtbl::UnsubscribeCoordinateMappingChanged"][::std::mem::offset_of!(
        ICoordinateMapperVtbl,
        UnsubscribeCoordinateMappingChanged
    ) - 32usize];
    ["Offset of field: ICoordinateMapperVtbl::GetCoordinateMappingChangedEventData"][::std::mem::offset_of!(
        ICoordinateMapperVtbl,
        GetCoordinateMappingChangedEventData
    ) - 40usize];
    ["Offset of field: ICoordinateMapperVtbl::MapCameraPointToDepthSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapCameraPointToDepthSpace) - 48usize];
    ["Offset of field: ICoordinateMapperVtbl::MapCameraPointToColorSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapCameraPointToColorSpace) - 56usize];
    ["Offset of field: ICoordinateMapperVtbl::MapDepthPointToCameraSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapDepthPointToCameraSpace) - 64usize];
    ["Offset of field: ICoordinateMapperVtbl::MapDepthPointToColorSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapDepthPointToColorSpace) - 72usize];
    ["Offset of field: ICoordinateMapperVtbl::MapCameraPointsToDepthSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapCameraPointsToDepthSpace) - 80usize];
    ["Offset of field: ICoordinateMapperVtbl::MapCameraPointsToColorSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapCameraPointsToColorSpace) - 88usize];
    ["Offset of field: ICoordinateMapperVtbl::MapDepthPointsToCameraSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapDepthPointsToCameraSpace) - 96usize];
    ["Offset of field: ICoordinateMapperVtbl::MapDepthPointsToColorSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapDepthPointsToColorSpace) - 104usize];
    ["Offset of field: ICoordinateMapperVtbl::MapDepthFrameToCameraSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapDepthFrameToCameraSpace) - 112usize];
    ["Offset of field: ICoordinateMapperVtbl::MapDepthFrameToColorSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapDepthFrameToColorSpace) - 120usize];
    ["Offset of field: ICoordinateMapperVtbl::MapColorFrameToDepthSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapColorFrameToDepthSpace) - 128usize];
    ["Offset of field: ICoordinateMapperVtbl::MapColorFrameToCameraSpace"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, MapColorFrameToCameraSpace) - 136usize];
    ["Offset of field: ICoordinateMapperVtbl::GetDepthFrameToCameraSpaceTable"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, GetDepthFrameToCameraSpaceTable) - 144usize];
    ["Offset of field: ICoordinateMapperVtbl::GetDepthCameraIntrinsics"]
        [::std::mem::offset_of!(ICoordinateMapperVtbl, GetDepthCameraIntrinsics) - 152usize];
};
#[repr(C)]
pub struct ICoordinateMapper {
    pub lpVtbl: *mut ICoordinateMapperVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ICoordinateMapper"][::std::mem::size_of::<ICoordinateMapper>() - 8usize];
    ["Alignment of ICoordinateMapper"][::std::mem::align_of::<ICoordinateMapper>() - 8usize];
    ["Offset of field: ICoordinateMapper::lpVtbl"]
        [::std::mem::offset_of!(ICoordinateMapper, lpVtbl) - 0usize];
};
impl Default for ICoordinateMapper {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_ICoordinateMappingChangedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct ICoordinateMappingChangedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut ICoordinateMappingChangedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ICoordinateMappingChangedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut ICoordinateMappingChangedEventArgs) -> ULONG,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ICoordinateMappingChangedEventArgsVtbl"]
        [::std::mem::size_of::<ICoordinateMappingChangedEventArgsVtbl>() - 24usize];
    ["Alignment of ICoordinateMappingChangedEventArgsVtbl"]
        [::std::mem::align_of::<ICoordinateMappingChangedEventArgsVtbl>() - 8usize];
    ["Offset of field: ICoordinateMappingChangedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(ICoordinateMappingChangedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: ICoordinateMappingChangedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(ICoordinateMappingChangedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: ICoordinateMappingChangedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(ICoordinateMappingChangedEventArgsVtbl, Release) - 16usize];
};
#[repr(C)]
pub struct ICoordinateMappingChangedEventArgs {
    pub lpVtbl: *mut ICoordinateMappingChangedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ICoordinateMappingChangedEventArgs"]
        [::std::mem::size_of::<ICoordinateMappingChangedEventArgs>() - 8usize];
    ["Alignment of ICoordinateMappingChangedEventArgs"]
        [::std::mem::align_of::<ICoordinateMappingChangedEventArgs>() - 8usize];
    ["Offset of field: ICoordinateMappingChangedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(ICoordinateMappingChangedEventArgs, lpVtbl) - 0usize];
};
impl Default for ICoordinateMappingChangedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IColorCameraSettings: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IColorCameraSettingsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorCameraSettings,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IColorCameraSettings) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IColorCameraSettings) -> ULONG>,
    pub get_ExposureTime: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorCameraSettings,
            exposureTime: *mut TIMESPAN,
        ) -> HRESULT,
    >,
    pub get_FrameInterval: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IColorCameraSettings,
            frameInterval: *mut TIMESPAN,
        ) -> HRESULT,
    >,
    pub get_Gain: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IColorCameraSettings, gain: *mut f32) -> HRESULT,
    >,
    pub get_Gamma: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IColorCameraSettings, gamma: *mut f32) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorCameraSettingsVtbl"]
        [::std::mem::size_of::<IColorCameraSettingsVtbl>() - 56usize];
    ["Alignment of IColorCameraSettingsVtbl"]
        [::std::mem::align_of::<IColorCameraSettingsVtbl>() - 8usize];
    ["Offset of field: IColorCameraSettingsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IColorCameraSettingsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IColorCameraSettingsVtbl::AddRef"]
        [::std::mem::offset_of!(IColorCameraSettingsVtbl, AddRef) - 8usize];
    ["Offset of field: IColorCameraSettingsVtbl::Release"]
        [::std::mem::offset_of!(IColorCameraSettingsVtbl, Release) - 16usize];
    ["Offset of field: IColorCameraSettingsVtbl::get_ExposureTime"]
        [::std::mem::offset_of!(IColorCameraSettingsVtbl, get_ExposureTime) - 24usize];
    ["Offset of field: IColorCameraSettingsVtbl::get_FrameInterval"]
        [::std::mem::offset_of!(IColorCameraSettingsVtbl, get_FrameInterval) - 32usize];
    ["Offset of field: IColorCameraSettingsVtbl::get_Gain"]
        [::std::mem::offset_of!(IColorCameraSettingsVtbl, get_Gain) - 40usize];
    ["Offset of field: IColorCameraSettingsVtbl::get_Gamma"]
        [::std::mem::offset_of!(IColorCameraSettingsVtbl, get_Gamma) - 48usize];
};
#[repr(C)]
pub struct IColorCameraSettings {
    pub lpVtbl: *mut IColorCameraSettingsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IColorCameraSettings"][::std::mem::size_of::<IColorCameraSettings>() - 8usize];
    ["Alignment of IColorCameraSettings"][::std::mem::align_of::<IColorCameraSettings>() - 8usize];
    ["Offset of field: IColorCameraSettings::lpVtbl"]
        [::std::mem::offset_of!(IColorCameraSettings, lpVtbl) - 0usize];
};
impl Default for IColorCameraSettings {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn GetDefaultKinectSensor(defaultKinectSensor: *mut *mut IKinectSensor) -> HRESULT;
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PointerDeviceType {
    Touch = 0,
    Pen = 1,
    Mouse = 2,
    Kinect = 3,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HandType {
    NONE = 0,
    LEFT = 1,
    RIGHT = 2,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum KinectHoldingState {
    Started = 0,
    Completed = 1,
    Canceled = 2,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum KinectGestureSettings {
    None = 0,
    Tap = 1,
    ManipulationTranslateX = 64,
    ManipulationTranslateY = 128,
    ManipulationTranslateRailsX = 256,
    ManipulationTranslateRailsY = 512,
    ManipulationScale = 2048,
    ManipulationTranslateInertia = 4096,
    KinectHold = 65536,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum KinectInteractionMode {
    Normal = 0,
    Off = 1,
    Media = 2,
}
#[repr(i32)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum KinectEngagementMode {
    None = 0,
    SystemOnePerson = 1,
    SystemTwoPerson = 2,
    ManualOnePerson = 3,
    ManualTwoPerson = 4,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct KinectManipulationDelta {
    pub Translation: PointF,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of KinectManipulationDelta"][::std::mem::size_of::<KinectManipulationDelta>() - 20usize];
    ["Alignment of KinectManipulationDelta"]
        [::std::mem::align_of::<KinectManipulationDelta>() - 4usize];
    ["Offset of field: KinectManipulationDelta::Translation"]
        [::std::mem::offset_of!(KinectManipulationDelta, Translation) - 0usize];
    ["Offset of field: KinectManipulationDelta::Scale"]
        [::std::mem::offset_of!(KinectManipulationDelta, Scale) - 8usize];
    ["Offset of field: KinectManipulationDelta::Rotation"]
        [::std::mem::offset_of!(KinectManipulationDelta, Rotation) - 12usize];
    ["Offset of field: KinectManipulationDelta::Expansion"]
        [::std::mem::offset_of!(KinectManipulationDelta, Expansion) - 16usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct KinectManipulationVelocities {
    pub Linear: PointF,
    pub Angular: f32,
    pub Expansion: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of KinectManipulationVelocities"]
        [::std::mem::size_of::<KinectManipulationVelocities>() - 16usize];
    ["Alignment of KinectManipulationVelocities"]
        [::std::mem::align_of::<KinectManipulationVelocities>() - 4usize];
    ["Offset of field: KinectManipulationVelocities::Linear"]
        [::std::mem::offset_of!(KinectManipulationVelocities, Linear) - 0usize];
    ["Offset of field: KinectManipulationVelocities::Angular"]
        [::std::mem::offset_of!(KinectManipulationVelocities, Angular) - 8usize];
    ["Offset of field: KinectManipulationVelocities::Expansion"]
        [::std::mem::offset_of!(KinectManipulationVelocities, Expansion) - 12usize];
};
unsafe extern "C" {
    pub static mut __MIDL_itf_Kinect2ECOM_0000_0052_v0_0_c_ifspec: RPC_IF_HANDLE;
}
unsafe extern "C" {
    pub static mut __MIDL_itf_Kinect2ECOM_0000_0052_v0_0_s_ifspec: RPC_IF_HANDLE;
}
unsafe extern "C" {
    pub static IID_IBodyHandPair: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IBodyHandPairVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IBodyHandPair,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyHandPair) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IBodyHandPair) -> ULONG>,
    pub get_BodyTrackingId: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyHandPair, value: *mut UINT64) -> HRESULT,
    >,
    pub put_BodyTrackingId: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyHandPair, value: UINT64) -> HRESULT,
    >,
    pub get_HandType: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyHandPair, value: *mut HandType) -> HRESULT,
    >,
    pub put_HandType: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IBodyHandPair, value: HandType) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyHandPairVtbl"][::std::mem::size_of::<IBodyHandPairVtbl>() - 56usize];
    ["Alignment of IBodyHandPairVtbl"][::std::mem::align_of::<IBodyHandPairVtbl>() - 8usize];
    ["Offset of field: IBodyHandPairVtbl::QueryInterface"]
        [::std::mem::offset_of!(IBodyHandPairVtbl, QueryInterface) - 0usize];
    ["Offset of field: IBodyHandPairVtbl::AddRef"]
        [::std::mem::offset_of!(IBodyHandPairVtbl, AddRef) - 8usize];
    ["Offset of field: IBodyHandPairVtbl::Release"]
        [::std::mem::offset_of!(IBodyHandPairVtbl, Release) - 16usize];
    ["Offset of field: IBodyHandPairVtbl::get_BodyTrackingId"]
        [::std::mem::offset_of!(IBodyHandPairVtbl, get_BodyTrackingId) - 24usize];
    ["Offset of field: IBodyHandPairVtbl::put_BodyTrackingId"]
        [::std::mem::offset_of!(IBodyHandPairVtbl, put_BodyTrackingId) - 32usize];
    ["Offset of field: IBodyHandPairVtbl::get_HandType"]
        [::std::mem::offset_of!(IBodyHandPairVtbl, get_HandType) - 40usize];
    ["Offset of field: IBodyHandPairVtbl::put_HandType"]
        [::std::mem::offset_of!(IBodyHandPairVtbl, put_HandType) - 48usize];
};
#[repr(C)]
pub struct IBodyHandPair {
    pub lpVtbl: *mut IBodyHandPairVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IBodyHandPair"][::std::mem::size_of::<IBodyHandPair>() - 8usize];
    ["Alignment of IBodyHandPair"][::std::mem::align_of::<IBodyHandPair>() - 8usize];
    ["Offset of field: IBodyHandPair::lpVtbl"]
        [::std::mem::offset_of!(IBodyHandPair, lpVtbl) - 0usize];
};
impl Default for IBodyHandPair {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectCoreWindow: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectCoreWindowVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectCoreWindow) -> ULONG>,
    pub Release: ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectCoreWindow) -> ULONG>,
    pub SubscribePointerEntered: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribePointerEntered: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetPointerEnteredEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IKinectPointerEventArgs,
        ) -> HRESULT,
    >,
    pub SubscribePointerMoved: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribePointerMoved: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetPointerMovedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IKinectPointerEventArgs,
        ) -> HRESULT,
    >,
    pub SubscribePointerExited: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            waitableHandle: *mut WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub UnsubscribePointerExited: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            waitableHandle: WAITABLE_HANDLE,
        ) -> HRESULT,
    >,
    pub GetPointerExitedEventData: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectCoreWindow,
            waitableHandle: WAITABLE_HANDLE,
            eventData: *mut *mut IKinectPointerEventArgs,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectCoreWindowVtbl"][::std::mem::size_of::<IKinectCoreWindowVtbl>() - 96usize];
    ["Alignment of IKinectCoreWindowVtbl"]
        [::std::mem::align_of::<IKinectCoreWindowVtbl>() - 8usize];
    ["Offset of field: IKinectCoreWindowVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectCoreWindowVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectCoreWindowVtbl::Release"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, Release) - 16usize];
    ["Offset of field: IKinectCoreWindowVtbl::SubscribePointerEntered"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, SubscribePointerEntered) - 24usize];
    ["Offset of field: IKinectCoreWindowVtbl::UnsubscribePointerEntered"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, UnsubscribePointerEntered) - 32usize];
    ["Offset of field: IKinectCoreWindowVtbl::GetPointerEnteredEventData"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, GetPointerEnteredEventData) - 40usize];
    ["Offset of field: IKinectCoreWindowVtbl::SubscribePointerMoved"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, SubscribePointerMoved) - 48usize];
    ["Offset of field: IKinectCoreWindowVtbl::UnsubscribePointerMoved"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, UnsubscribePointerMoved) - 56usize];
    ["Offset of field: IKinectCoreWindowVtbl::GetPointerMovedEventData"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, GetPointerMovedEventData) - 64usize];
    ["Offset of field: IKinectCoreWindowVtbl::SubscribePointerExited"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, SubscribePointerExited) - 72usize];
    ["Offset of field: IKinectCoreWindowVtbl::UnsubscribePointerExited"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, UnsubscribePointerExited) - 80usize];
    ["Offset of field: IKinectCoreWindowVtbl::GetPointerExitedEventData"]
        [::std::mem::offset_of!(IKinectCoreWindowVtbl, GetPointerExitedEventData) - 88usize];
};
#[repr(C)]
pub struct IKinectCoreWindow {
    pub lpVtbl: *mut IKinectCoreWindowVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectCoreWindow"][::std::mem::size_of::<IKinectCoreWindow>() - 8usize];
    ["Alignment of IKinectCoreWindow"][::std::mem::align_of::<IKinectCoreWindow>() - 8usize];
    ["Offset of field: IKinectCoreWindow::lpVtbl"]
        [::std::mem::offset_of!(IKinectCoreWindow, lpVtbl) - 0usize];
};
impl Default for IKinectCoreWindow {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectGestureRecognizer: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectGestureRecognizerVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> ULONG>,
    pub RegisterSelectionTappedHandler: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            handler: *mut IKinectGestureRecognizerSelectionHandler,
        ) -> HRESULT,
    >,
    pub UnregisterSelectionTappedHandler:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub RegisterSelectionHoldingHandler: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            handler: *mut IKinectGestureRecognizerSelectionHandler,
        ) -> HRESULT,
    >,
    pub UnregisterSelectionHoldingHandler:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub RegisterSelectionPressingStartedHandler: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            handler: *mut IKinectGestureRecognizerSelectionHandler,
        ) -> HRESULT,
    >,
    pub UnregisterSelectionPressingStartedHandler:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub RegisterSelectionPressingUpdatedHandler: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            handler: *mut IKinectGestureRecognizerSelectionHandler,
        ) -> HRESULT,
    >,
    pub UnregisterSelectionPressingUpdatedHandler:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub RegisterSelectionPressingCompletedHandler: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            handler: *mut IKinectGestureRecognizerSelectionHandler,
        ) -> HRESULT,
    >,
    pub UnregisterSelectionPressingCompletedHandler:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub RegisterManipulationStartedHandler: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            handler: *mut IKinectGestureRecognizerManipulationHandler,
        ) -> HRESULT,
    >,
    pub UnregisterManipulationStartedHandler:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub RegisterManipulationUpdatedHandler: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            handler: *mut IKinectGestureRecognizerManipulationHandler,
        ) -> HRESULT,
    >,
    pub UnregisterManipulationUpdatedHandler:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub RegisterManipulationInertiaStartingHandler: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            handler: *mut IKinectGestureRecognizerManipulationHandler,
        ) -> HRESULT,
    >,
    pub UnregisterManipulationInertiaStartingHandler:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub RegisterManipulationCompletedHandler: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            handler: *mut IKinectGestureRecognizerManipulationHandler,
        ) -> HRESULT,
    >,
    pub UnregisterManipulationCompletedHandler:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub get_GestureSettings: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            value: *mut KinectGestureSettings,
        ) -> HRESULT,
    >,
    pub put_GestureSettings: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            value: KinectGestureSettings,
        ) -> HRESULT,
    >,
    pub get_IsInertial: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: *mut BOOLEAN) -> HRESULT,
    >,
    pub get_IsActive: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: *mut BOOLEAN) -> HRESULT,
    >,
    pub get_InertiaTranslationDeceleration: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: *mut f32) -> HRESULT,
    >,
    pub put_InertiaTranslationDeceleration: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: f32) -> HRESULT,
    >,
    pub get_InertiaTranslationDisplacement: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: *mut f32) -> HRESULT,
    >,
    pub put_InertiaTranslationDisplacement: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: f32) -> HRESULT,
    >,
    pub get_AutoProcessInertia: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: *mut BOOLEAN) -> HRESULT,
    >,
    pub put_AutoProcessInertia: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: BOOLEAN) -> HRESULT,
    >,
    pub get_BoundingRect: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: *mut RectF) -> HRESULT,
    >,
    pub put_BoundingRect: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizer, value: RectF) -> HRESULT,
    >,
    pub ProcessDownEvent: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            value: *mut IKinectPointerPoint,
        ) -> HRESULT,
    >,
    pub ProcessUpEvent: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            value: *mut IKinectPointerPoint,
        ) -> HRESULT,
    >,
    pub ProcessMoveEvents: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizer,
            value: *mut IKinectPointerPoint,
        ) -> HRESULT,
    >,
    pub ProcessInertia:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
    pub CompleteGesture:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectGestureRecognizer) -> HRESULT>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectGestureRecognizerVtbl"]
        [::std::mem::size_of::<IKinectGestureRecognizerVtbl>() - 304usize];
    ["Alignment of IKinectGestureRecognizerVtbl"]
        [::std::mem::align_of::<IKinectGestureRecognizerVtbl>() - 8usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::Release"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, Release) - 16usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::RegisterSelectionTappedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        RegisterSelectionTappedHandler
    ) - 24usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::UnregisterSelectionTappedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        UnregisterSelectionTappedHandler
    )
        - 32usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::RegisterSelectionHoldingHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        RegisterSelectionHoldingHandler
    ) - 40usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::UnregisterSelectionHoldingHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        UnregisterSelectionHoldingHandler
    )
        - 48usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::RegisterSelectionPressingStartedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        RegisterSelectionPressingStartedHandler
    )
        - 56usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::UnregisterSelectionPressingStartedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        UnregisterSelectionPressingStartedHandler
    )
        - 64usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::RegisterSelectionPressingUpdatedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        RegisterSelectionPressingUpdatedHandler
    )
        - 72usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::UnregisterSelectionPressingUpdatedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        UnregisterSelectionPressingUpdatedHandler
    )
        - 80usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::RegisterSelectionPressingCompletedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        RegisterSelectionPressingCompletedHandler
    )
        - 88usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::UnregisterSelectionPressingCompletedHandler"]
        [::std::mem::offset_of!(
            IKinectGestureRecognizerVtbl,
            UnregisterSelectionPressingCompletedHandler
        ) - 96usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::RegisterManipulationStartedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        RegisterManipulationStartedHandler
    )
        - 104usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::UnregisterManipulationStartedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        UnregisterManipulationStartedHandler
    )
        - 112usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::RegisterManipulationUpdatedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        RegisterManipulationUpdatedHandler
    )
        - 120usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::UnregisterManipulationUpdatedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        UnregisterManipulationUpdatedHandler
    )
        - 128usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::RegisterManipulationInertiaStartingHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        RegisterManipulationInertiaStartingHandler
    )
        - 136usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::UnregisterManipulationInertiaStartingHandler"]
        [::std::mem::offset_of!(
            IKinectGestureRecognizerVtbl,
            UnregisterManipulationInertiaStartingHandler
        ) - 144usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::RegisterManipulationCompletedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        RegisterManipulationCompletedHandler
    )
        - 152usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::UnregisterManipulationCompletedHandler"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        UnregisterManipulationCompletedHandler
    )
        - 160usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::get_GestureSettings"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, get_GestureSettings) - 168usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::put_GestureSettings"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, put_GestureSettings) - 176usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::get_IsInertial"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, get_IsInertial) - 184usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::get_IsActive"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, get_IsActive) - 192usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::get_InertiaTranslationDeceleration"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        get_InertiaTranslationDeceleration
    )
        - 200usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::put_InertiaTranslationDeceleration"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        put_InertiaTranslationDeceleration
    )
        - 208usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::get_InertiaTranslationDisplacement"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        get_InertiaTranslationDisplacement
    )
        - 216usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::put_InertiaTranslationDisplacement"][::std::mem::offset_of!(
        IKinectGestureRecognizerVtbl,
        put_InertiaTranslationDisplacement
    )
        - 224usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::get_AutoProcessInertia"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, get_AutoProcessInertia) - 232usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::put_AutoProcessInertia"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, put_AutoProcessInertia) - 240usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::get_BoundingRect"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, get_BoundingRect) - 248usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::put_BoundingRect"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, put_BoundingRect) - 256usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::ProcessDownEvent"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, ProcessDownEvent) - 264usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::ProcessUpEvent"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, ProcessUpEvent) - 272usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::ProcessMoveEvents"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, ProcessMoveEvents) - 280usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::ProcessInertia"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, ProcessInertia) - 288usize];
    ["Offset of field: IKinectGestureRecognizerVtbl::CompleteGesture"]
        [::std::mem::offset_of!(IKinectGestureRecognizerVtbl, CompleteGesture) - 296usize];
};
#[repr(C)]
pub struct IKinectGestureRecognizer {
    pub lpVtbl: *mut IKinectGestureRecognizerVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectGestureRecognizer"]
        [::std::mem::size_of::<IKinectGestureRecognizer>() - 8usize];
    ["Alignment of IKinectGestureRecognizer"]
        [::std::mem::align_of::<IKinectGestureRecognizer>() - 8usize];
    ["Offset of field: IKinectGestureRecognizer::lpVtbl"]
        [::std::mem::offset_of!(IKinectGestureRecognizer, lpVtbl) - 0usize];
};
impl Default for IKinectGestureRecognizer {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectGestureRecognizerSelectionHandler: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectGestureRecognizerSelectionHandlerVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerSelectionHandler,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizerSelectionHandler) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizerSelectionHandler) -> ULONG,
    >,
    pub OnTapped: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerSelectionHandler,
            args: *mut IKinectTappedEventArgs,
        ) -> HRESULT,
    >,
    pub OnHolding: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerSelectionHandler,
            args: *mut IKinectHoldingEventArgs,
        ) -> HRESULT,
    >,
    pub OnPressingStarted: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerSelectionHandler,
            args: *mut IKinectPressingStartedEventArgs,
        ) -> HRESULT,
    >,
    pub OnPressingUpdated: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerSelectionHandler,
            args: *mut IKinectPressingUpdatedEventArgs,
        ) -> HRESULT,
    >,
    pub OnPressingCompleted: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerSelectionHandler,
            args: *mut IKinectPressingCompletedEventArgs,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectGestureRecognizerSelectionHandlerVtbl"]
        [::std::mem::size_of::<IKinectGestureRecognizerSelectionHandlerVtbl>() - 64usize];
    ["Alignment of IKinectGestureRecognizerSelectionHandlerVtbl"]
        [::std::mem::align_of::<IKinectGestureRecognizerSelectionHandlerVtbl>() - 8usize];
    ["Offset of field: IKinectGestureRecognizerSelectionHandlerVtbl::QueryInterface"][::std::mem::offset_of!(
        IKinectGestureRecognizerSelectionHandlerVtbl,
        QueryInterface
    ) - 0usize];
    ["Offset of field: IKinectGestureRecognizerSelectionHandlerVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectGestureRecognizerSelectionHandlerVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectGestureRecognizerSelectionHandlerVtbl::Release"]
        [::std::mem::offset_of!(IKinectGestureRecognizerSelectionHandlerVtbl, Release) - 16usize];
    ["Offset of field: IKinectGestureRecognizerSelectionHandlerVtbl::OnTapped"]
        [::std::mem::offset_of!(IKinectGestureRecognizerSelectionHandlerVtbl, OnTapped) - 24usize];
    ["Offset of field: IKinectGestureRecognizerSelectionHandlerVtbl::OnHolding"]
        [::std::mem::offset_of!(IKinectGestureRecognizerSelectionHandlerVtbl, OnHolding) - 32usize];
    ["Offset of field: IKinectGestureRecognizerSelectionHandlerVtbl::OnPressingStarted"][::std::mem::offset_of!(
        IKinectGestureRecognizerSelectionHandlerVtbl,
        OnPressingStarted
    )
        - 40usize];
    ["Offset of field: IKinectGestureRecognizerSelectionHandlerVtbl::OnPressingUpdated"][::std::mem::offset_of!(
        IKinectGestureRecognizerSelectionHandlerVtbl,
        OnPressingUpdated
    )
        - 48usize];
    ["Offset of field: IKinectGestureRecognizerSelectionHandlerVtbl::OnPressingCompleted"][::std::mem::offset_of!(
        IKinectGestureRecognizerSelectionHandlerVtbl,
        OnPressingCompleted
    )
        - 56usize];
};
#[repr(C)]
pub struct IKinectGestureRecognizerSelectionHandler {
    pub lpVtbl: *mut IKinectGestureRecognizerSelectionHandlerVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectGestureRecognizerSelectionHandler"]
        [::std::mem::size_of::<IKinectGestureRecognizerSelectionHandler>() - 8usize];
    ["Alignment of IKinectGestureRecognizerSelectionHandler"]
        [::std::mem::align_of::<IKinectGestureRecognizerSelectionHandler>() - 8usize];
    ["Offset of field: IKinectGestureRecognizerSelectionHandler::lpVtbl"]
        [::std::mem::offset_of!(IKinectGestureRecognizerSelectionHandler, lpVtbl) - 0usize];
};
impl Default for IKinectGestureRecognizerSelectionHandler {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectGestureRecognizerManipulationHandler: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectGestureRecognizerManipulationHandlerVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerManipulationHandler,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizerManipulationHandler) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectGestureRecognizerManipulationHandler) -> ULONG,
    >,
    pub OnManipulationStarted: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerManipulationHandler,
            args: *mut IKinectManipulationStartedEventArgs,
        ) -> HRESULT,
    >,
    pub OnManipulationUpdated: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerManipulationHandler,
            args: *mut IKinectManipulationUpdatedEventArgs,
        ) -> HRESULT,
    >,
    pub OnManipulationInertiaStarting: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerManipulationHandler,
            args: *mut IKinectManipulationInertiaStartingEventArgs,
        ) -> HRESULT,
    >,
    pub OnManipulationCompleted: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectGestureRecognizerManipulationHandler,
            args: *mut IKinectManipulationCompletedEventArgs,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectGestureRecognizerManipulationHandlerVtbl"]
        [::std::mem::size_of::<IKinectGestureRecognizerManipulationHandlerVtbl>() - 56usize];
    ["Alignment of IKinectGestureRecognizerManipulationHandlerVtbl"]
        [::std::mem::align_of::<IKinectGestureRecognizerManipulationHandlerVtbl>() - 8usize];
    ["Offset of field: IKinectGestureRecognizerManipulationHandlerVtbl::QueryInterface"][::std::mem::offset_of!(
        IKinectGestureRecognizerManipulationHandlerVtbl,
        QueryInterface
    )
        - 0usize];
    ["Offset of field: IKinectGestureRecognizerManipulationHandlerVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectGestureRecognizerManipulationHandlerVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectGestureRecognizerManipulationHandlerVtbl::Release"][::std::mem::offset_of!(
        IKinectGestureRecognizerManipulationHandlerVtbl,
        Release
    ) - 16usize];
    ["Offset of field: IKinectGestureRecognizerManipulationHandlerVtbl::OnManipulationStarted"][::std::mem::offset_of!(
        IKinectGestureRecognizerManipulationHandlerVtbl,
        OnManipulationStarted
    )
        - 24usize];
    ["Offset of field: IKinectGestureRecognizerManipulationHandlerVtbl::OnManipulationUpdated"][::std::mem::offset_of!(
        IKinectGestureRecognizerManipulationHandlerVtbl,
        OnManipulationUpdated
    )
        - 32usize];
    [
        "Offset of field: IKinectGestureRecognizerManipulationHandlerVtbl::OnManipulationInertiaStarting",
    ][::std::mem::offset_of!(
        IKinectGestureRecognizerManipulationHandlerVtbl,
        OnManipulationInertiaStarting
    ) - 40usize];
    ["Offset of field: IKinectGestureRecognizerManipulationHandlerVtbl::OnManipulationCompleted"][::std::mem::offset_of!(
        IKinectGestureRecognizerManipulationHandlerVtbl,
        OnManipulationCompleted
    )
        - 48usize];
};
#[repr(C)]
pub struct IKinectGestureRecognizerManipulationHandler {
    pub lpVtbl: *mut IKinectGestureRecognizerManipulationHandlerVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectGestureRecognizerManipulationHandler"]
        [::std::mem::size_of::<IKinectGestureRecognizerManipulationHandler>() - 8usize];
    ["Alignment of IKinectGestureRecognizerManipulationHandler"]
        [::std::mem::align_of::<IKinectGestureRecognizerManipulationHandler>() - 8usize];
    ["Offset of field: IKinectGestureRecognizerManipulationHandler::lpVtbl"]
        [::std::mem::offset_of!(IKinectGestureRecognizerManipulationHandler, lpVtbl) - 0usize];
};
impl Default for IKinectGestureRecognizerManipulationHandler {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectHoldingEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectHoldingEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectHoldingEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectHoldingEventArgs) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectHoldingEventArgs) -> ULONG>,
    pub get_PointerDeviceType: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectHoldingEventArgs,
            value: *mut PointerDeviceType,
        ) -> HRESULT,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectHoldingEventArgs, value: *mut PointF) -> HRESULT,
    >,
    pub get_HoldingState: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectHoldingEventArgs,
            value: *mut KinectHoldingState,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectHoldingEventArgsVtbl"]
        [::std::mem::size_of::<IKinectHoldingEventArgsVtbl>() - 48usize];
    ["Alignment of IKinectHoldingEventArgsVtbl"]
        [::std::mem::align_of::<IKinectHoldingEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectHoldingEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectHoldingEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectHoldingEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectHoldingEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectHoldingEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IKinectHoldingEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IKinectHoldingEventArgsVtbl::get_PointerDeviceType"]
        [::std::mem::offset_of!(IKinectHoldingEventArgsVtbl, get_PointerDeviceType) - 24usize];
    ["Offset of field: IKinectHoldingEventArgsVtbl::get_Position"]
        [::std::mem::offset_of!(IKinectHoldingEventArgsVtbl, get_Position) - 32usize];
    ["Offset of field: IKinectHoldingEventArgsVtbl::get_HoldingState"]
        [::std::mem::offset_of!(IKinectHoldingEventArgsVtbl, get_HoldingState) - 40usize];
};
#[repr(C)]
pub struct IKinectHoldingEventArgs {
    pub lpVtbl: *mut IKinectHoldingEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectHoldingEventArgs"][::std::mem::size_of::<IKinectHoldingEventArgs>() - 8usize];
    ["Alignment of IKinectHoldingEventArgs"]
        [::std::mem::align_of::<IKinectHoldingEventArgs>() - 8usize];
    ["Offset of field: IKinectHoldingEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectHoldingEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectHoldingEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectManipulationCompletedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectManipulationCompletedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationCompletedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectManipulationCompletedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectManipulationCompletedEventArgs) -> ULONG,
    >,
    pub get_PointerDeviceType: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationCompletedEventArgs,
            value: *mut PointerDeviceType,
        ) -> HRESULT,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationCompletedEventArgs,
            value: *mut PointF,
        ) -> HRESULT,
    >,
    pub get_Cumulative: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationCompletedEventArgs,
            value: *mut KinectManipulationDelta,
        ) -> HRESULT,
    >,
    pub get_Velocities: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationCompletedEventArgs,
            value: *mut KinectManipulationVelocities,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectManipulationCompletedEventArgsVtbl"]
        [::std::mem::size_of::<IKinectManipulationCompletedEventArgsVtbl>() - 56usize];
    ["Alignment of IKinectManipulationCompletedEventArgsVtbl"]
        [::std::mem::align_of::<IKinectManipulationCompletedEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectManipulationCompletedEventArgsVtbl::QueryInterface"][::std::mem::offset_of!(
        IKinectManipulationCompletedEventArgsVtbl,
        QueryInterface
    ) - 0usize];
    ["Offset of field: IKinectManipulationCompletedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectManipulationCompletedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectManipulationCompletedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IKinectManipulationCompletedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IKinectManipulationCompletedEventArgsVtbl::get_PointerDeviceType"][::std::mem::offset_of!(
        IKinectManipulationCompletedEventArgsVtbl,
        get_PointerDeviceType
    )
        - 24usize];
    ["Offset of field: IKinectManipulationCompletedEventArgsVtbl::get_Position"]
        [::std::mem::offset_of!(IKinectManipulationCompletedEventArgsVtbl, get_Position) - 32usize];
    ["Offset of field: IKinectManipulationCompletedEventArgsVtbl::get_Cumulative"][::std::mem::offset_of!(
        IKinectManipulationCompletedEventArgsVtbl,
        get_Cumulative
    ) - 40usize];
    ["Offset of field: IKinectManipulationCompletedEventArgsVtbl::get_Velocities"][::std::mem::offset_of!(
        IKinectManipulationCompletedEventArgsVtbl,
        get_Velocities
    ) - 48usize];
};
#[repr(C)]
pub struct IKinectManipulationCompletedEventArgs {
    pub lpVtbl: *mut IKinectManipulationCompletedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectManipulationCompletedEventArgs"]
        [::std::mem::size_of::<IKinectManipulationCompletedEventArgs>() - 8usize];
    ["Alignment of IKinectManipulationCompletedEventArgs"]
        [::std::mem::align_of::<IKinectManipulationCompletedEventArgs>() - 8usize];
    ["Offset of field: IKinectManipulationCompletedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectManipulationCompletedEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectManipulationCompletedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectManipulationInertiaStartingEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectManipulationInertiaStartingEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationInertiaStartingEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectManipulationInertiaStartingEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectManipulationInertiaStartingEventArgs) -> ULONG,
    >,
    pub get_PointerDeviceType: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationInertiaStartingEventArgs,
            value: *mut PointerDeviceType,
        ) -> HRESULT,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationInertiaStartingEventArgs,
            value: *mut PointF,
        ) -> HRESULT,
    >,
    pub get_Delta: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationInertiaStartingEventArgs,
            value: *mut KinectManipulationDelta,
        ) -> HRESULT,
    >,
    pub get_Cumulative: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationInertiaStartingEventArgs,
            value: *mut KinectManipulationDelta,
        ) -> HRESULT,
    >,
    pub get_Velocities: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationInertiaStartingEventArgs,
            value: *mut KinectManipulationVelocities,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectManipulationInertiaStartingEventArgsVtbl"]
        [::std::mem::size_of::<IKinectManipulationInertiaStartingEventArgsVtbl>() - 64usize];
    ["Alignment of IKinectManipulationInertiaStartingEventArgsVtbl"]
        [::std::mem::align_of::<IKinectManipulationInertiaStartingEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectManipulationInertiaStartingEventArgsVtbl::QueryInterface"][::std::mem::offset_of!(
        IKinectManipulationInertiaStartingEventArgsVtbl,
        QueryInterface
    )
        - 0usize];
    ["Offset of field: IKinectManipulationInertiaStartingEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectManipulationInertiaStartingEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectManipulationInertiaStartingEventArgsVtbl::Release"][::std::mem::offset_of!(
        IKinectManipulationInertiaStartingEventArgsVtbl,
        Release
    ) - 16usize];
    ["Offset of field: IKinectManipulationInertiaStartingEventArgsVtbl::get_PointerDeviceType"][::std::mem::offset_of!(
        IKinectManipulationInertiaStartingEventArgsVtbl,
        get_PointerDeviceType
    )
        - 24usize];
    ["Offset of field: IKinectManipulationInertiaStartingEventArgsVtbl::get_Position"][::std::mem::offset_of!(
        IKinectManipulationInertiaStartingEventArgsVtbl,
        get_Position
    ) - 32usize];
    ["Offset of field: IKinectManipulationInertiaStartingEventArgsVtbl::get_Delta"][::std::mem::offset_of!(
        IKinectManipulationInertiaStartingEventArgsVtbl,
        get_Delta
    ) - 40usize];
    ["Offset of field: IKinectManipulationInertiaStartingEventArgsVtbl::get_Cumulative"][::std::mem::offset_of!(
        IKinectManipulationInertiaStartingEventArgsVtbl,
        get_Cumulative
    )
        - 48usize];
    ["Offset of field: IKinectManipulationInertiaStartingEventArgsVtbl::get_Velocities"][::std::mem::offset_of!(
        IKinectManipulationInertiaStartingEventArgsVtbl,
        get_Velocities
    )
        - 56usize];
};
#[repr(C)]
pub struct IKinectManipulationInertiaStartingEventArgs {
    pub lpVtbl: *mut IKinectManipulationInertiaStartingEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectManipulationInertiaStartingEventArgs"]
        [::std::mem::size_of::<IKinectManipulationInertiaStartingEventArgs>() - 8usize];
    ["Alignment of IKinectManipulationInertiaStartingEventArgs"]
        [::std::mem::align_of::<IKinectManipulationInertiaStartingEventArgs>() - 8usize];
    ["Offset of field: IKinectManipulationInertiaStartingEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectManipulationInertiaStartingEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectManipulationInertiaStartingEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectManipulationStartedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectManipulationStartedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationStartedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectManipulationStartedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectManipulationStartedEventArgs) -> ULONG,
    >,
    pub get_PointerDeviceType: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationStartedEventArgs,
            value: *mut PointerDeviceType,
        ) -> HRESULT,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationStartedEventArgs,
            value: *mut PointF,
        ) -> HRESULT,
    >,
    pub get_Cumulative: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationStartedEventArgs,
            value: *mut KinectManipulationDelta,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectManipulationStartedEventArgsVtbl"]
        [::std::mem::size_of::<IKinectManipulationStartedEventArgsVtbl>() - 48usize];
    ["Alignment of IKinectManipulationStartedEventArgsVtbl"]
        [::std::mem::align_of::<IKinectManipulationStartedEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectManipulationStartedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectManipulationStartedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectManipulationStartedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectManipulationStartedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectManipulationStartedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IKinectManipulationStartedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IKinectManipulationStartedEventArgsVtbl::get_PointerDeviceType"][::std::mem::offset_of!(
        IKinectManipulationStartedEventArgsVtbl,
        get_PointerDeviceType
    )
        - 24usize];
    ["Offset of field: IKinectManipulationStartedEventArgsVtbl::get_Position"]
        [::std::mem::offset_of!(IKinectManipulationStartedEventArgsVtbl, get_Position) - 32usize];
    ["Offset of field: IKinectManipulationStartedEventArgsVtbl::get_Cumulative"]
        [::std::mem::offset_of!(IKinectManipulationStartedEventArgsVtbl, get_Cumulative) - 40usize];
};
#[repr(C)]
pub struct IKinectManipulationStartedEventArgs {
    pub lpVtbl: *mut IKinectManipulationStartedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectManipulationStartedEventArgs"]
        [::std::mem::size_of::<IKinectManipulationStartedEventArgs>() - 8usize];
    ["Alignment of IKinectManipulationStartedEventArgs"]
        [::std::mem::align_of::<IKinectManipulationStartedEventArgs>() - 8usize];
    ["Offset of field: IKinectManipulationStartedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectManipulationStartedEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectManipulationStartedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectManipulationUpdatedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectManipulationUpdatedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationUpdatedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectManipulationUpdatedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectManipulationUpdatedEventArgs) -> ULONG,
    >,
    pub get_PointerDeviceType: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationUpdatedEventArgs,
            value: *mut PointerDeviceType,
        ) -> HRESULT,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationUpdatedEventArgs,
            value: *mut PointF,
        ) -> HRESULT,
    >,
    pub get_Delta: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationUpdatedEventArgs,
            value: *mut KinectManipulationDelta,
        ) -> HRESULT,
    >,
    pub get_Cumulative: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationUpdatedEventArgs,
            value: *mut KinectManipulationDelta,
        ) -> HRESULT,
    >,
    pub get_Velocities: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectManipulationUpdatedEventArgs,
            value: *mut KinectManipulationVelocities,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectManipulationUpdatedEventArgsVtbl"]
        [::std::mem::size_of::<IKinectManipulationUpdatedEventArgsVtbl>() - 64usize];
    ["Alignment of IKinectManipulationUpdatedEventArgsVtbl"]
        [::std::mem::align_of::<IKinectManipulationUpdatedEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectManipulationUpdatedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectManipulationUpdatedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectManipulationUpdatedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectManipulationUpdatedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectManipulationUpdatedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IKinectManipulationUpdatedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IKinectManipulationUpdatedEventArgsVtbl::get_PointerDeviceType"][::std::mem::offset_of!(
        IKinectManipulationUpdatedEventArgsVtbl,
        get_PointerDeviceType
    )
        - 24usize];
    ["Offset of field: IKinectManipulationUpdatedEventArgsVtbl::get_Position"]
        [::std::mem::offset_of!(IKinectManipulationUpdatedEventArgsVtbl, get_Position) - 32usize];
    ["Offset of field: IKinectManipulationUpdatedEventArgsVtbl::get_Delta"]
        [::std::mem::offset_of!(IKinectManipulationUpdatedEventArgsVtbl, get_Delta) - 40usize];
    ["Offset of field: IKinectManipulationUpdatedEventArgsVtbl::get_Cumulative"]
        [::std::mem::offset_of!(IKinectManipulationUpdatedEventArgsVtbl, get_Cumulative) - 48usize];
    ["Offset of field: IKinectManipulationUpdatedEventArgsVtbl::get_Velocities"]
        [::std::mem::offset_of!(IKinectManipulationUpdatedEventArgsVtbl, get_Velocities) - 56usize];
};
#[repr(C)]
pub struct IKinectManipulationUpdatedEventArgs {
    pub lpVtbl: *mut IKinectManipulationUpdatedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectManipulationUpdatedEventArgs"]
        [::std::mem::size_of::<IKinectManipulationUpdatedEventArgs>() - 8usize];
    ["Alignment of IKinectManipulationUpdatedEventArgs"]
        [::std::mem::align_of::<IKinectManipulationUpdatedEventArgs>() - 8usize];
    ["Offset of field: IKinectManipulationUpdatedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectManipulationUpdatedEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectManipulationUpdatedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectPointerDevice: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectPointerDeviceVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerDevice,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectPointerDevice) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectPointerDevice) -> ULONG>,
    pub get_PointerDeviceType: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerDevice,
            value: *mut PointerDeviceType,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPointerDeviceVtbl"]
        [::std::mem::size_of::<IKinectPointerDeviceVtbl>() - 32usize];
    ["Alignment of IKinectPointerDeviceVtbl"]
        [::std::mem::align_of::<IKinectPointerDeviceVtbl>() - 8usize];
    ["Offset of field: IKinectPointerDeviceVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectPointerDeviceVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectPointerDeviceVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectPointerDeviceVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectPointerDeviceVtbl::Release"]
        [::std::mem::offset_of!(IKinectPointerDeviceVtbl, Release) - 16usize];
    ["Offset of field: IKinectPointerDeviceVtbl::get_PointerDeviceType"]
        [::std::mem::offset_of!(IKinectPointerDeviceVtbl, get_PointerDeviceType) - 24usize];
};
#[repr(C)]
pub struct IKinectPointerDevice {
    pub lpVtbl: *mut IKinectPointerDeviceVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPointerDevice"][::std::mem::size_of::<IKinectPointerDevice>() - 8usize];
    ["Alignment of IKinectPointerDevice"][::std::mem::align_of::<IKinectPointerDevice>() - 8usize];
    ["Offset of field: IKinectPointerDevice::lpVtbl"]
        [::std::mem::offset_of!(IKinectPointerDevice, lpVtbl) - 0usize];
};
impl Default for IKinectPointerDevice {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectPointerEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectPointerEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectPointerEventArgs) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectPointerEventArgs) -> ULONG>,
    pub get_Handled: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPointerEventArgs, handled: *mut BOOLEAN) -> HRESULT,
    >,
    pub put_Handled: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPointerEventArgs, handled: BOOLEAN) -> HRESULT,
    >,
    pub get_CurrentPoint: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerEventArgs,
            pointer: *mut *mut IKinectPointerPoint,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPointerEventArgsVtbl"]
        [::std::mem::size_of::<IKinectPointerEventArgsVtbl>() - 48usize];
    ["Alignment of IKinectPointerEventArgsVtbl"]
        [::std::mem::align_of::<IKinectPointerEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectPointerEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectPointerEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectPointerEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectPointerEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectPointerEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IKinectPointerEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IKinectPointerEventArgsVtbl::get_Handled"]
        [::std::mem::offset_of!(IKinectPointerEventArgsVtbl, get_Handled) - 24usize];
    ["Offset of field: IKinectPointerEventArgsVtbl::put_Handled"]
        [::std::mem::offset_of!(IKinectPointerEventArgsVtbl, put_Handled) - 32usize];
    ["Offset of field: IKinectPointerEventArgsVtbl::get_CurrentPoint"]
        [::std::mem::offset_of!(IKinectPointerEventArgsVtbl, get_CurrentPoint) - 40usize];
};
#[repr(C)]
pub struct IKinectPointerEventArgs {
    pub lpVtbl: *mut IKinectPointerEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPointerEventArgs"][::std::mem::size_of::<IKinectPointerEventArgs>() - 8usize];
    ["Alignment of IKinectPointerEventArgs"]
        [::std::mem::align_of::<IKinectPointerEventArgs>() - 8usize];
    ["Offset of field: IKinectPointerEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectPointerEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectPointerEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectPointerPoint: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectPointerPointVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPoint,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectPointerPoint) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectPointerPoint) -> ULONG>,
    pub get_PointerDevice: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPoint,
            ppPointerDevice: *mut *mut IKinectPointerDevice,
        ) -> HRESULT,
    >,
    pub get_PointerId: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPointerPoint, pointerId: *mut UINT) -> HRESULT,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPointerPoint, position: *mut PointF) -> HRESULT,
    >,
    pub get_RawPosition: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPointerPoint, rawPosition: *mut PointF) -> HRESULT,
    >,
    pub get_Timestamp: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPointerPoint, timestamp: *mut UINT64) -> HRESULT,
    >,
    pub get_Properties: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPoint,
            ppProperties: *mut *mut IKinectPointerPointProperties,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPointerPointVtbl"][::std::mem::size_of::<IKinectPointerPointVtbl>() - 72usize];
    ["Alignment of IKinectPointerPointVtbl"]
        [::std::mem::align_of::<IKinectPointerPointVtbl>() - 8usize];
    ["Offset of field: IKinectPointerPointVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectPointerPointVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectPointerPointVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectPointerPointVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectPointerPointVtbl::Release"]
        [::std::mem::offset_of!(IKinectPointerPointVtbl, Release) - 16usize];
    ["Offset of field: IKinectPointerPointVtbl::get_PointerDevice"]
        [::std::mem::offset_of!(IKinectPointerPointVtbl, get_PointerDevice) - 24usize];
    ["Offset of field: IKinectPointerPointVtbl::get_PointerId"]
        [::std::mem::offset_of!(IKinectPointerPointVtbl, get_PointerId) - 32usize];
    ["Offset of field: IKinectPointerPointVtbl::get_Position"]
        [::std::mem::offset_of!(IKinectPointerPointVtbl, get_Position) - 40usize];
    ["Offset of field: IKinectPointerPointVtbl::get_RawPosition"]
        [::std::mem::offset_of!(IKinectPointerPointVtbl, get_RawPosition) - 48usize];
    ["Offset of field: IKinectPointerPointVtbl::get_Timestamp"]
        [::std::mem::offset_of!(IKinectPointerPointVtbl, get_Timestamp) - 56usize];
    ["Offset of field: IKinectPointerPointVtbl::get_Properties"]
        [::std::mem::offset_of!(IKinectPointerPointVtbl, get_Properties) - 64usize];
};
#[repr(C)]
pub struct IKinectPointerPoint {
    pub lpVtbl: *mut IKinectPointerPointVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPointerPoint"][::std::mem::size_of::<IKinectPointerPoint>() - 8usize];
    ["Alignment of IKinectPointerPoint"][::std::mem::align_of::<IKinectPointerPoint>() - 8usize];
    ["Offset of field: IKinectPointerPoint::lpVtbl"]
        [::std::mem::offset_of!(IKinectPointerPoint, lpVtbl) - 0usize];
};
impl Default for IKinectPointerPoint {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectPointerPointProperties: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectPointerPointPropertiesVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPointerPointProperties) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPointerPointProperties) -> ULONG,
    >,
    pub get_IsPrimary: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            isPrimary: *mut BOOLEAN,
        ) -> HRESULT,
    >,
    pub get_IsInRange: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            isInRange: *mut BOOLEAN,
        ) -> HRESULT,
    >,
    pub get_IsEngaged: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            isEngaged: *mut BOOLEAN,
        ) -> HRESULT,
    >,
    pub get_BodyTrackingId: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            bodyTrackingId: *mut UINT64,
        ) -> HRESULT,
    >,
    pub get_HandType: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            handType: *mut HandType,
        ) -> HRESULT,
    >,
    pub get_HandReachExtent: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            handReachExtent: *mut f32,
        ) -> HRESULT,
    >,
    pub get_BodyTimeCounter: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            bodyTimeCounter: *mut TIMESPAN,
        ) -> HRESULT,
    >,
    pub get_HandRotation: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            handRotation: *mut f32,
        ) -> HRESULT,
    >,
    pub get_PressExtent: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            pressExtent: *mut f32,
        ) -> HRESULT,
    >,
    pub get_UnclampedPosition: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPointerPointProperties,
            unclampedPosition: *mut PointF,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPointerPointPropertiesVtbl"]
        [::std::mem::size_of::<IKinectPointerPointPropertiesVtbl>() - 104usize];
    ["Alignment of IKinectPointerPointPropertiesVtbl"]
        [::std::mem::align_of::<IKinectPointerPointPropertiesVtbl>() - 8usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::Release"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, Release) - 16usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_IsPrimary"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, get_IsPrimary) - 24usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_IsInRange"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, get_IsInRange) - 32usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_IsEngaged"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, get_IsEngaged) - 40usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_BodyTrackingId"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, get_BodyTrackingId) - 48usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_HandType"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, get_HandType) - 56usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_HandReachExtent"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, get_HandReachExtent) - 64usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_BodyTimeCounter"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, get_BodyTimeCounter) - 72usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_HandRotation"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, get_HandRotation) - 80usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_PressExtent"]
        [::std::mem::offset_of!(IKinectPointerPointPropertiesVtbl, get_PressExtent) - 88usize];
    ["Offset of field: IKinectPointerPointPropertiesVtbl::get_UnclampedPosition"][::std::mem::offset_of!(
        IKinectPointerPointPropertiesVtbl,
        get_UnclampedPosition
    ) - 96usize];
};
#[repr(C)]
pub struct IKinectPointerPointProperties {
    pub lpVtbl: *mut IKinectPointerPointPropertiesVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPointerPointProperties"]
        [::std::mem::size_of::<IKinectPointerPointProperties>() - 8usize];
    ["Alignment of IKinectPointerPointProperties"]
        [::std::mem::align_of::<IKinectPointerPointProperties>() - 8usize];
    ["Offset of field: IKinectPointerPointProperties::lpVtbl"]
        [::std::mem::offset_of!(IKinectPointerPointProperties, lpVtbl) - 0usize];
};
impl Default for IKinectPointerPointProperties {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectPressingCompletedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectPressingCompletedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPressingCompletedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPressingCompletedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPressingCompletedEventArgs) -> ULONG,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPressingCompletedEventArgs,
            value: *mut PointF,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPressingCompletedEventArgsVtbl"]
        [::std::mem::size_of::<IKinectPressingCompletedEventArgsVtbl>() - 32usize];
    ["Alignment of IKinectPressingCompletedEventArgsVtbl"]
        [::std::mem::align_of::<IKinectPressingCompletedEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectPressingCompletedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectPressingCompletedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectPressingCompletedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectPressingCompletedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectPressingCompletedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IKinectPressingCompletedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IKinectPressingCompletedEventArgsVtbl::get_Position"]
        [::std::mem::offset_of!(IKinectPressingCompletedEventArgsVtbl, get_Position) - 24usize];
};
#[repr(C)]
pub struct IKinectPressingCompletedEventArgs {
    pub lpVtbl: *mut IKinectPressingCompletedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPressingCompletedEventArgs"]
        [::std::mem::size_of::<IKinectPressingCompletedEventArgs>() - 8usize];
    ["Alignment of IKinectPressingCompletedEventArgs"]
        [::std::mem::align_of::<IKinectPressingCompletedEventArgs>() - 8usize];
    ["Offset of field: IKinectPressingCompletedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectPressingCompletedEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectPressingCompletedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectPressingStartedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectPressingStartedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPressingStartedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPressingStartedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPressingStartedEventArgs) -> ULONG,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPressingStartedEventArgs,
            value: *mut PointF,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPressingStartedEventArgsVtbl"]
        [::std::mem::size_of::<IKinectPressingStartedEventArgsVtbl>() - 32usize];
    ["Alignment of IKinectPressingStartedEventArgsVtbl"]
        [::std::mem::align_of::<IKinectPressingStartedEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectPressingStartedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectPressingStartedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectPressingStartedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectPressingStartedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectPressingStartedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IKinectPressingStartedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IKinectPressingStartedEventArgsVtbl::get_Position"]
        [::std::mem::offset_of!(IKinectPressingStartedEventArgsVtbl, get_Position) - 24usize];
};
#[repr(C)]
pub struct IKinectPressingStartedEventArgs {
    pub lpVtbl: *mut IKinectPressingStartedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPressingStartedEventArgs"]
        [::std::mem::size_of::<IKinectPressingStartedEventArgs>() - 8usize];
    ["Alignment of IKinectPressingStartedEventArgs"]
        [::std::mem::align_of::<IKinectPressingStartedEventArgs>() - 8usize];
    ["Offset of field: IKinectPressingStartedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectPressingStartedEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectPressingStartedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectPressingUpdatedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectPressingUpdatedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPressingUpdatedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPressingUpdatedEventArgs) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectPressingUpdatedEventArgs) -> ULONG,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPressingUpdatedEventArgs,
            value: *mut PointF,
        ) -> HRESULT,
    >,
    pub get_PressExtent: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPressingUpdatedEventArgs,
            value: *mut f32,
        ) -> HRESULT,
    >,
    pub get_HoldProgress: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectPressingUpdatedEventArgs,
            value: *mut f32,
        ) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPressingUpdatedEventArgsVtbl"]
        [::std::mem::size_of::<IKinectPressingUpdatedEventArgsVtbl>() - 48usize];
    ["Alignment of IKinectPressingUpdatedEventArgsVtbl"]
        [::std::mem::align_of::<IKinectPressingUpdatedEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectPressingUpdatedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectPressingUpdatedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectPressingUpdatedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectPressingUpdatedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectPressingUpdatedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IKinectPressingUpdatedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IKinectPressingUpdatedEventArgsVtbl::get_Position"]
        [::std::mem::offset_of!(IKinectPressingUpdatedEventArgsVtbl, get_Position) - 24usize];
    ["Offset of field: IKinectPressingUpdatedEventArgsVtbl::get_PressExtent"]
        [::std::mem::offset_of!(IKinectPressingUpdatedEventArgsVtbl, get_PressExtent) - 32usize];
    ["Offset of field: IKinectPressingUpdatedEventArgsVtbl::get_HoldProgress"]
        [::std::mem::offset_of!(IKinectPressingUpdatedEventArgsVtbl, get_HoldProgress) - 40usize];
};
#[repr(C)]
pub struct IKinectPressingUpdatedEventArgs {
    pub lpVtbl: *mut IKinectPressingUpdatedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectPressingUpdatedEventArgs"]
        [::std::mem::size_of::<IKinectPressingUpdatedEventArgs>() - 8usize];
    ["Alignment of IKinectPressingUpdatedEventArgs"]
        [::std::mem::align_of::<IKinectPressingUpdatedEventArgs>() - 8usize];
    ["Offset of field: IKinectPressingUpdatedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectPressingUpdatedEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectPressingUpdatedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static IID_IKinectTappedEventArgs: IID;
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct IKinectTappedEventArgsVtbl {
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectTappedEventArgs,
            riid: *const IID,
            ppvObject: *mut *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectTappedEventArgs) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(This: *mut IKinectTappedEventArgs) -> ULONG>,
    pub get_PointerDeviceType: ::std::option::Option<
        unsafe extern "C" fn(
            This: *mut IKinectTappedEventArgs,
            value: *mut PointerDeviceType,
        ) -> HRESULT,
    >,
    pub get_Position: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectTappedEventArgs, value: *mut PointF) -> HRESULT,
    >,
    pub get_TapCount: ::std::option::Option<
        unsafe extern "C" fn(This: *mut IKinectTappedEventArgs, value: *mut UINT) -> HRESULT,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectTappedEventArgsVtbl"]
        [::std::mem::size_of::<IKinectTappedEventArgsVtbl>() - 48usize];
    ["Alignment of IKinectTappedEventArgsVtbl"]
        [::std::mem::align_of::<IKinectTappedEventArgsVtbl>() - 8usize];
    ["Offset of field: IKinectTappedEventArgsVtbl::QueryInterface"]
        [::std::mem::offset_of!(IKinectTappedEventArgsVtbl, QueryInterface) - 0usize];
    ["Offset of field: IKinectTappedEventArgsVtbl::AddRef"]
        [::std::mem::offset_of!(IKinectTappedEventArgsVtbl, AddRef) - 8usize];
    ["Offset of field: IKinectTappedEventArgsVtbl::Release"]
        [::std::mem::offset_of!(IKinectTappedEventArgsVtbl, Release) - 16usize];
    ["Offset of field: IKinectTappedEventArgsVtbl::get_PointerDeviceType"]
        [::std::mem::offset_of!(IKinectTappedEventArgsVtbl, get_PointerDeviceType) - 24usize];
    ["Offset of field: IKinectTappedEventArgsVtbl::get_Position"]
        [::std::mem::offset_of!(IKinectTappedEventArgsVtbl, get_Position) - 32usize];
    ["Offset of field: IKinectTappedEventArgsVtbl::get_TapCount"]
        [::std::mem::offset_of!(IKinectTappedEventArgsVtbl, get_TapCount) - 40usize];
};
#[repr(C)]
pub struct IKinectTappedEventArgs {
    pub lpVtbl: *mut IKinectTappedEventArgsVtbl,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKinectTappedEventArgs"][::std::mem::size_of::<IKinectTappedEventArgs>() - 8usize];
    ["Alignment of IKinectTappedEventArgs"]
        [::std::mem::align_of::<IKinectTappedEventArgs>() - 8usize];
    ["Offset of field: IKinectTappedEventArgs::lpVtbl"]
        [::std::mem::offset_of!(IKinectTappedEventArgs, lpVtbl) - 0usize];
};
impl Default for IKinectTappedEventArgs {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn GetKinectCoreWindowForCurrentThread(
        ppKinectCoreWindow: *mut *mut IKinectCoreWindow,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn CreateKinectGestureRecognizer(
        ppKinectGestureRecognizer: *mut *mut IKinectGestureRecognizer,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn OverrideKinectInteractionMode(mode: KinectInteractionMode) -> HRESULT;
}
unsafe extern "C" {
    pub fn SetKinectOnePersonSystemEngagement() -> HRESULT;
}
unsafe extern "C" {
    pub fn SetKinectTwoPersonSystemEngagement() -> HRESULT;
}
unsafe extern "C" {
    pub fn SetKinectOnePersonManualEngagement(pBodyHandPair: *mut IBodyHandPair) -> HRESULT;
}
unsafe extern "C" {
    pub fn SetKinectTwoPersonManualEngagement(
        pBodyHandPair1: *mut IBodyHandPair,
        pBodyHandPair2: *mut IBodyHandPair,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn GetKinectEngagementMode(pMode: *mut KinectEngagementMode) -> HRESULT;
}
unsafe extern "C" {
    pub fn GetKinectManualEngagedHandCount(pManualEngagedHandCount: *mut UINT) -> HRESULT;
}
unsafe extern "C" {
    pub fn GetKinectManualEngagedHand(
        manualEngagedHandIndex: UINT,
        ppManualEngagedHand: *mut *mut IBodyHandPair,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn GetMaximumKinectEngagedPersonCount(
        pMaximumKinectEngagedPersonCount: *mut UINT,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn CreateBodyHandPair(
        bodyTrackingId: UINT64,
        handType: HandType,
        ppBodyHandPair: *mut *mut IBodyHandPair,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub static mut __MIDL_itf_Kinect2ECOM_0000_0070_v0_0_c_ifspec: RPC_IF_HANDLE;
}
unsafe extern "C" {
    pub static mut __MIDL_itf_Kinect2ECOM_0000_0070_v0_0_s_ifspec: RPC_IF_HANDLE;
}
