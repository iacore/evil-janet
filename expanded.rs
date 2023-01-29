#![feature(prelude_import)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2018::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
extern crate alloc;
use core::{cmp::Ordering, fmt, hash::{Hash, Hasher}};
#[repr(C)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
#[automatically_derived]
impl<T: ::core::default::Default> ::core::default::Default
for __IncompleteArrayField<T> {
    #[inline]
    fn default() -> __IncompleteArrayField<T> {
        __IncompleteArrayField(
            ::core::default::Default::default(),
            ::core::default::Default::default(),
        )
    }
}
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const JANET_VERSION_MAJOR: u32 = 1;
pub const JANET_VERSION_MINOR: u32 = 25;
pub const JANET_VERSION_PATCH: u32 = 1;
pub const JANET_VERSION_EXTRA: &[u8; 1usize] = b"\0";
pub const JANET_VERSION: &[u8; 7usize] = b"1.25.1\0";
pub const JANET_BUILD: &[u8; 6usize] = b"local\0";
pub const JANET_APPLE: u32 = 1;
pub const JANET_POSIX: u32 = 1;
pub const JANET_64: u32 = 1;
pub const JANET_LITTLE_ENDIAN: u32 = 1;
pub const JANET_INTMAX_DOUBLE: f64 = 9007199254740992.0;
pub const JANET_INTMIN_DOUBLE: f64 = -9007199254740992.0;
pub const JANET_INTMAX_INT64: u64 = 9007199254740992;
pub const JANET_INTMIN_INT64: i64 = -9007199254740992;
pub const JANET_RECURSION_GUARD: u32 = 1024;
pub const JANET_MAX_PROTO_DEPTH: u32 = 200;
pub const JANET_MAX_MACRO_EXPAND: u32 = 200;
pub const JANET_STACK_MAX: u32 = 2147483647;
pub const JANET_NANBOX_BIT: u32 = 0;
pub const JANET_SINGLE_THREADED_BIT: u32 = 0;
pub const JANET_CURRENT_CONFIG_BITS: u32 = 0;
pub const JANET_HANDLE_NONE: i32 = -1;
pub const JANET_STREAM_CLOSED: u32 = 1;
pub const JANET_STREAM_SOCKET: u32 = 2;
pub const JANET_STREAM_IOCP: u32 = 4;
pub const JANET_STREAM_READABLE: u32 = 512;
pub const JANET_STREAM_WRITABLE: u32 = 1024;
pub const JANET_STREAM_ACCEPTABLE: u32 = 2048;
pub const JANET_STREAM_UDPSERVER: u32 = 4096;
pub const JANET_STACKFRAME_TAILCALL: u32 = 1;
pub const JANET_STACKFRAME_ENTRANCE: u32 = 2;
pub const JANET_FRAME_SIZE: u32 = 4;
pub const JANET_FUNCDEF_FLAG_VARARG: u32 = 65536;
pub const JANET_FUNCDEF_FLAG_NEEDSENV: u32 = 131072;
pub const JANET_FUNCDEF_FLAG_HASNAME: u32 = 524288;
pub const JANET_FUNCDEF_FLAG_HASSOURCE: u32 = 1048576;
pub const JANET_FUNCDEF_FLAG_HASDEFS: u32 = 2097152;
pub const JANET_FUNCDEF_FLAG_HASENVS: u32 = 4194304;
pub const JANET_FUNCDEF_FLAG_HASSOURCEMAP: u32 = 8388608;
pub const JANET_FUNCDEF_FLAG_STRUCTARG: u32 = 16777216;
pub const JANET_FUNCDEF_FLAG_HASCLOBITSET: u32 = 33554432;
pub const JANET_FUNCDEF_FLAG_TAG: u32 = 65535;
pub const JANET_FUNCFLAG_TRACE: u32 = 65536;
pub const JANET_EV_TCTAG_NIL: u32 = 0;
pub const JANET_EV_TCTAG_INTEGER: u32 = 1;
pub const JANET_EV_TCTAG_STRING: u32 = 2;
pub const JANET_EV_TCTAG_STRINGF: u32 = 3;
pub const JANET_EV_TCTAG_KEYWORD: u32 = 4;
pub const JANET_EV_TCTAG_ERR_STRING: u32 = 5;
pub const JANET_EV_TCTAG_ERR_STRINGF: u32 = 6;
pub const JANET_EV_TCTAG_ERR_KEYWORD: u32 = 7;
pub const JANET_EV_TCTAG_BOOLEAN: u32 = 8;
pub const JANET_TUPLE_FLAG_BRACKETCTOR: u32 = 65536;
pub const JANET_MARSHAL_UNSAFE: u32 = 131072;
pub const JANET_MARSHAL_NO_CYCLES: u32 = 262144;
pub const JANET_PRETTY_COLOR: u32 = 1;
pub const JANET_PRETTY_ONELINE: u32 = 2;
pub const JANET_PRETTY_NOTRUNC: u32 = 4;
pub const JANET_FILE_WRITE: u32 = 1;
pub const JANET_FILE_READ: u32 = 2;
pub const JANET_FILE_APPEND: u32 = 4;
pub const JANET_FILE_UPDATE: u32 = 8;
pub const JANET_FILE_NOT_CLOSEABLE: u32 = 16;
pub const JANET_FILE_CLOSED: u32 = 32;
pub const JANET_FILE_BINARY: u32 = 64;
pub const JANET_FILE_SERIALIZABLE: u32 = 128;
pub const JANET_FILE_NONIL: u32 = 512;
#[repr(C)]
pub struct JanetBuildConfig {
    pub major: ::libc::c_uint,
    pub minor: ::libc::c_uint,
    pub patch: ::libc::c_uint,
    pub bits: ::libc::c_uint,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetBuildConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "JanetBuildConfig",
            "major",
            &&self.major,
            "minor",
            &&self.minor,
            "patch",
            &&self.patch,
            "bits",
            &&self.bits,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetBuildConfig {}
#[automatically_derived]
impl ::core::clone::Clone for JanetBuildConfig {
    #[inline]
    fn clone(&self) -> JanetBuildConfig {
        let _: ::core::clone::AssertParamIsClone<::libc::c_uint>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_uint>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_uint>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_uint>;
        *self
    }
}
#[repr(C)]
pub struct JanetOSMutex {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetOSMutex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "JanetOSMutex",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetOSMutex {}
#[automatically_derived]
impl ::core::clone::Clone for JanetOSMutex {
    #[inline]
    fn clone(&self) -> JanetOSMutex {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
#[repr(C)]
pub struct JanetOSRWLock {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetOSRWLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "JanetOSRWLock",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetOSRWLock {}
#[automatically_derived]
impl ::core::clone::Clone for JanetOSRWLock {
    #[inline]
    fn clone(&self) -> JanetOSRWLock {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
pub type __int64_t = ::libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type va_list = __builtin_va_list;
pub type jmp_buf = [::libc::c_int; 48usize];
extern "C" {
    pub fn setjmp(arg1: *mut ::libc::c_int) -> ::libc::c_int;
}
extern "C" {
    pub fn longjmp(arg1: *mut ::libc::c_int, arg2: ::libc::c_int) -> !;
}
extern "C" {
    pub fn _setjmp(arg1: *mut ::libc::c_int) -> ::libc::c_int;
}
extern "C" {
    pub fn _longjmp(arg1: *mut ::libc::c_int, arg2: ::libc::c_int) -> !;
}
extern "C" {
    pub fn sigsetjmp(arg1: *mut ::libc::c_int, arg2: ::libc::c_int) -> ::libc::c_int;
}
extern "C" {
    pub fn siglongjmp(arg1: *mut ::libc::c_int, arg2: ::libc::c_int) -> !;
}
pub type fpos_t = __darwin_off_t;
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::libc::c_uchar,
    pub _size: ::libc::c_int,
}
#[automatically_derived]
impl ::core::fmt::Debug for __sbuf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "__sbuf",
            "_base",
            &&self._base,
            "_size",
            &&self._size,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for __sbuf {}
#[automatically_derived]
impl ::core::clone::Clone for __sbuf {
    #[inline]
    fn clone(&self) -> __sbuf {
        let _: ::core::clone::AssertParamIsClone<*mut ::libc::c_uchar>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        *self
    }
}
#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for __sFILEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "__sFILEX",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for __sFILEX {}
#[automatically_derived]
impl ::core::clone::Clone for __sFILEX {
    #[inline]
    fn clone(&self) -> __sFILEX {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::libc::c_uchar,
    pub _r: ::libc::c_int,
    pub _w: ::libc::c_int,
    pub _flags: ::libc::c_short,
    pub _file: ::libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::libc::c_int,
    pub _cookie: *mut ::libc::c_void,
    pub _close: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut ::libc::c_void) -> ::libc::c_int,
    >,
    pub _read: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::libc::c_void,
            arg2: *mut ::libc::c_char,
            arg3: ::libc::c_int,
        ) -> ::libc::c_int,
    >,
    pub _seek: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::libc::c_void,
            arg2: fpos_t,
            arg3: ::libc::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::libc::c_void,
            arg2: *const ::libc::c_char,
            arg3: ::libc::c_int,
        ) -> ::libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::libc::c_int,
    pub _ubuf: [::libc::c_uchar; 3usize],
    pub _nbuf: [::libc::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::libc::c_int,
    pub _offset: fpos_t,
}
#[automatically_derived]
impl ::core::fmt::Debug for __sFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "_p",
            "_r",
            "_w",
            "_flags",
            "_file",
            "_bf",
            "_lbfsize",
            "_cookie",
            "_close",
            "_read",
            "_seek",
            "_write",
            "_ub",
            "_extra",
            "_ur",
            "_ubuf",
            "_nbuf",
            "_lb",
            "_blksize",
            "_offset",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self._p,
            &&self._r,
            &&self._w,
            &&self._flags,
            &&self._file,
            &&self._bf,
            &&self._lbfsize,
            &&self._cookie,
            &&self._close,
            &&self._read,
            &&self._seek,
            &&self._write,
            &&self._ub,
            &&self._extra,
            &&self._ur,
            &&self._ubuf,
            &&self._nbuf,
            &&self._lb,
            &&self._blksize,
            &&self._offset,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "__sFILE", names, values)
    }
}
#[automatically_derived]
impl ::core::marker::Copy for __sFILE {}
#[automatically_derived]
impl ::core::clone::Clone for __sFILE {
    #[inline]
    fn clone(&self) -> __sFILE {
        let _: ::core::clone::AssertParamIsClone<*mut ::libc::c_uchar>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_short>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_short>;
        let _: ::core::clone::AssertParamIsClone<__sbuf>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<*mut ::libc::c_void>;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(arg1: *mut ::libc::c_void) -> ::libc::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::libc::c_void,
                    arg2: *mut ::libc::c_char,
                    arg3: ::libc::c_int,
                ) -> ::libc::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::libc::c_void,
                    arg2: fpos_t,
                    arg3: ::libc::c_int,
                ) -> fpos_t,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    arg1: *mut ::libc::c_void,
                    arg2: *const ::libc::c_char,
                    arg3: ::libc::c_int,
                ) -> ::libc::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<*mut __sFILEX>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<[::libc::c_uchar; 3usize]>;
        let _: ::core::clone::AssertParamIsClone<[::libc::c_uchar; 1usize]>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<fpos_t>;
        *self
    }
}
pub type FILE = __sFILE;
extern "C" {
    pub static janet_type_names: [*const ::libc::c_char; 16usize];
}
extern "C" {
    pub static janet_signal_names: [*const ::libc::c_char; 14usize];
}
extern "C" {
    pub static janet_status_names: [*const ::libc::c_char; 16usize];
}
pub type JanetHandle = ::libc::c_int;
pub const JanetSignal_JANET_SIGNAL_OK: JanetSignal = 0;
pub const JanetSignal_JANET_SIGNAL_ERROR: JanetSignal = 1;
pub const JanetSignal_JANET_SIGNAL_DEBUG: JanetSignal = 2;
pub const JanetSignal_JANET_SIGNAL_YIELD: JanetSignal = 3;
pub const JanetSignal_JANET_SIGNAL_USER0: JanetSignal = 4;
pub const JanetSignal_JANET_SIGNAL_USER1: JanetSignal = 5;
pub const JanetSignal_JANET_SIGNAL_USER2: JanetSignal = 6;
pub const JanetSignal_JANET_SIGNAL_USER3: JanetSignal = 7;
pub const JanetSignal_JANET_SIGNAL_USER4: JanetSignal = 8;
pub const JanetSignal_JANET_SIGNAL_USER5: JanetSignal = 9;
pub const JanetSignal_JANET_SIGNAL_USER6: JanetSignal = 10;
pub const JanetSignal_JANET_SIGNAL_USER7: JanetSignal = 11;
pub const JanetSignal_JANET_SIGNAL_USER8: JanetSignal = 12;
pub const JanetSignal_JANET_SIGNAL_USER9: JanetSignal = 13;
pub type JanetSignal = ::libc::c_uint;
pub const JanetFiberStatus_JANET_STATUS_DEAD: JanetFiberStatus = 0;
pub const JanetFiberStatus_JANET_STATUS_ERROR: JanetFiberStatus = 1;
pub const JanetFiberStatus_JANET_STATUS_DEBUG: JanetFiberStatus = 2;
pub const JanetFiberStatus_JANET_STATUS_PENDING: JanetFiberStatus = 3;
pub const JanetFiberStatus_JANET_STATUS_USER0: JanetFiberStatus = 4;
pub const JanetFiberStatus_JANET_STATUS_USER1: JanetFiberStatus = 5;
pub const JanetFiberStatus_JANET_STATUS_USER2: JanetFiberStatus = 6;
pub const JanetFiberStatus_JANET_STATUS_USER3: JanetFiberStatus = 7;
pub const JanetFiberStatus_JANET_STATUS_USER4: JanetFiberStatus = 8;
pub const JanetFiberStatus_JANET_STATUS_USER5: JanetFiberStatus = 9;
pub const JanetFiberStatus_JANET_STATUS_USER6: JanetFiberStatus = 10;
pub const JanetFiberStatus_JANET_STATUS_USER7: JanetFiberStatus = 11;
pub const JanetFiberStatus_JANET_STATUS_USER8: JanetFiberStatus = 12;
pub const JanetFiberStatus_JANET_STATUS_USER9: JanetFiberStatus = 13;
pub const JanetFiberStatus_JANET_STATUS_NEW: JanetFiberStatus = 14;
pub const JanetFiberStatus_JANET_STATUS_ALIVE: JanetFiberStatus = 15;
pub type JanetFiberStatus = ::libc::c_uint;
#[repr(C)]
pub struct JanetVM {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetVM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "JanetVM",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetVM {}
#[automatically_derived]
impl ::core::clone::Clone for JanetVM {
    #[inline]
    fn clone(&self) -> JanetVM {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
pub const JanetType_JANET_NUMBER: JanetType = 0;
pub const JanetType_JANET_NIL: JanetType = 1;
pub const JanetType_JANET_BOOLEAN: JanetType = 2;
pub const JanetType_JANET_FIBER: JanetType = 3;
pub const JanetType_JANET_STRING: JanetType = 4;
pub const JanetType_JANET_SYMBOL: JanetType = 5;
pub const JanetType_JANET_KEYWORD: JanetType = 6;
pub const JanetType_JANET_ARRAY: JanetType = 7;
pub const JanetType_JANET_TUPLE: JanetType = 8;
pub const JanetType_JANET_TABLE: JanetType = 9;
pub const JanetType_JANET_STRUCT: JanetType = 10;
pub const JanetType_JANET_BUFFER: JanetType = 11;
pub const JanetType_JANET_FUNCTION: JanetType = 12;
pub const JanetType_JANET_CFUNCTION: JanetType = 13;
pub const JanetType_JANET_ABSTRACT: JanetType = 14;
pub const JanetType_JANET_POINTER: JanetType = 15;
pub type JanetType = ::libc::c_uint;
#[repr(C)]
pub struct Janet {
    pub as_: Janet__bindgen_ty_1,
    pub type_: JanetType,
}
#[automatically_derived]
impl ::core::marker::Copy for Janet {}
#[automatically_derived]
impl ::core::clone::Clone for Janet {
    #[inline]
    fn clone(&self) -> Janet {
        let _: ::core::clone::AssertParamIsClone<Janet__bindgen_ty_1>;
        let _: ::core::clone::AssertParamIsClone<JanetType>;
        *self
    }
}
#[repr(C)]
pub union Janet__bindgen_ty_1 {
    pub u64_: u64,
    pub number: f64,
    pub integer: i32,
    pub pointer: *mut ::libc::c_void,
    pub cpointer: *const ::libc::c_void,
}
#[automatically_derived]
impl ::core::marker::Copy for Janet__bindgen_ty_1 {}
#[automatically_derived]
impl ::core::clone::Clone for Janet__bindgen_ty_1 {
    #[inline]
    fn clone(&self) -> Janet__bindgen_ty_1 {
        let _: ::core::clone::AssertParamIsCopy<Self>;
        *self
    }
}
pub type JanetCFunction = ::core::option::Option<
    unsafe extern "C" fn(argc: i32, argv: *mut Janet) -> Janet,
>;
pub type JanetString = *const u8;
pub type JanetSymbol = *const u8;
pub type JanetKeyword = *const u8;
pub type JanetTuple = *const Janet;
pub type JanetStruct = *const JanetKV;
pub type JanetAbstract = *mut ::libc::c_void;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_INIT: JanetAsyncEvent = 0;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_MARK: JanetAsyncEvent = 1;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_DEINIT: JanetAsyncEvent = 2;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_CLOSE: JanetAsyncEvent = 3;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_ERR: JanetAsyncEvent = 4;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_HUP: JanetAsyncEvent = 5;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_READ: JanetAsyncEvent = 6;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_WRITE: JanetAsyncEvent = 7;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_CANCEL: JanetAsyncEvent = 8;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_COMPLETE: JanetAsyncEvent = 9;
pub const JanetAsyncEvent_JANET_ASYNC_EVENT_USER: JanetAsyncEvent = 10;
pub type JanetAsyncEvent = ::libc::c_uint;
pub const JanetAsyncStatus_JANET_ASYNC_STATUS_NOT_DONE: JanetAsyncStatus = 0;
pub const JanetAsyncStatus_JANET_ASYNC_STATUS_DONE: JanetAsyncStatus = 1;
pub type JanetAsyncStatus = ::libc::c_uint;
pub type JanetListener = ::core::option::Option<
    unsafe extern "C" fn(
        state: *mut JanetListenerState,
        event: JanetAsyncEvent,
    ) -> JanetAsyncStatus,
>;
#[repr(C)]
pub struct JanetStream {
    pub handle: JanetHandle,
    pub flags: u32,
    pub state: *mut JanetListenerState,
    pub methods: *const ::libc::c_void,
    pub _mask: ::libc::c_int,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "JanetStream",
            "handle",
            &&self.handle,
            "flags",
            &&self.flags,
            "state",
            &&self.state,
            "methods",
            &&self.methods,
            "_mask",
            &&self._mask,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetStream {}
#[automatically_derived]
impl ::core::clone::Clone for JanetStream {
    #[inline]
    fn clone(&self) -> JanetStream {
        let _: ::core::clone::AssertParamIsClone<JanetHandle>;
        let _: ::core::clone::AssertParamIsClone<u32>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetListenerState>;
        let _: ::core::clone::AssertParamIsClone<*const ::libc::c_void>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        *self
    }
}
#[repr(C)]
pub struct JanetListenerState {
    pub machine: JanetListener,
    pub fiber: *mut JanetFiber,
    pub stream: *mut JanetStream,
    pub event: *mut ::libc::c_void,
    pub _index: usize,
    pub _mask: ::libc::c_int,
    pub _next: *mut JanetListenerState,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetListenerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "machine",
            "fiber",
            "stream",
            "event",
            "_index",
            "_mask",
            "_next",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.machine,
            &&self.fiber,
            &&self.stream,
            &&self.event,
            &&self._index,
            &&self._mask,
            &&self._next,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "JanetListenerState",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetListenerState {}
#[automatically_derived]
impl ::core::clone::Clone for JanetListenerState {
    #[inline]
    fn clone(&self) -> JanetListenerState {
        let _: ::core::clone::AssertParamIsClone<JanetListener>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetFiber>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetStream>;
        let _: ::core::clone::AssertParamIsClone<*mut ::libc::c_void>;
        let _: ::core::clone::AssertParamIsClone<usize>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetListenerState>;
        *self
    }
}
extern "C" {
    /// START SECTION NON-C API
    pub fn janet_struct_head(st: *const JanetKV) -> *mut JanetStructHead;
}
extern "C" {
    pub fn janet_abstract_head(
        abstract_: *const ::libc::c_void,
    ) -> *mut JanetAbstractHead;
}
extern "C" {
    pub fn janet_string_head(s: *const u8) -> *mut JanetStringHead;
}
extern "C" {
    pub fn janet_tuple_head(tuple: *const Janet) -> *mut JanetTupleHead;
}
extern "C" {
    pub fn janet_type(x: Janet) -> JanetType;
}
extern "C" {
    pub fn janet_checktype(x: Janet, type_: JanetType) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_checktypes(x: Janet, typeflags: ::libc::c_int) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_truthy(x: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_unwrap_struct(x: Janet) -> *const JanetKV;
}
extern "C" {
    pub fn janet_unwrap_tuple(x: Janet) -> *const Janet;
}
extern "C" {
    pub fn janet_unwrap_fiber(x: Janet) -> *mut JanetFiber;
}
extern "C" {
    pub fn janet_unwrap_array(x: Janet) -> *mut JanetArray;
}
extern "C" {
    pub fn janet_unwrap_table(x: Janet) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_unwrap_buffer(x: Janet) -> *mut JanetBuffer;
}
extern "C" {
    pub fn janet_unwrap_string(x: Janet) -> *const u8;
}
extern "C" {
    pub fn janet_unwrap_symbol(x: Janet) -> *const u8;
}
extern "C" {
    pub fn janet_unwrap_keyword(x: Janet) -> *const u8;
}
extern "C" {
    pub fn janet_unwrap_abstract(x: Janet) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_unwrap_pointer(x: Janet) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_unwrap_function(x: Janet) -> *mut JanetFunction;
}
extern "C" {
    pub fn janet_unwrap_cfunction(x: Janet) -> JanetCFunction;
}
extern "C" {
    pub fn janet_unwrap_boolean(x: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_unwrap_number(x: Janet) -> f64;
}
extern "C" {
    pub fn janet_unwrap_integer(x: Janet) -> i32;
}
extern "C" {
    pub fn janet_wrap_nil() -> Janet;
}
extern "C" {
    pub fn janet_wrap_number(x: f64) -> Janet;
}
extern "C" {
    pub fn janet_wrap_true() -> Janet;
}
extern "C" {
    pub fn janet_wrap_false() -> Janet;
}
extern "C" {
    pub fn janet_wrap_boolean(x: ::libc::c_int) -> Janet;
}
extern "C" {
    pub fn janet_wrap_string(x: *const u8) -> Janet;
}
extern "C" {
    pub fn janet_wrap_symbol(x: *const u8) -> Janet;
}
extern "C" {
    pub fn janet_wrap_keyword(x: *const u8) -> Janet;
}
extern "C" {
    pub fn janet_wrap_array(x: *mut JanetArray) -> Janet;
}
extern "C" {
    pub fn janet_wrap_tuple(x: *const Janet) -> Janet;
}
extern "C" {
    pub fn janet_wrap_struct(x: *const JanetKV) -> Janet;
}
extern "C" {
    pub fn janet_wrap_fiber(x: *mut JanetFiber) -> Janet;
}
extern "C" {
    pub fn janet_wrap_buffer(x: *mut JanetBuffer) -> Janet;
}
extern "C" {
    pub fn janet_wrap_function(x: *mut JanetFunction) -> Janet;
}
extern "C" {
    pub fn janet_wrap_cfunction(x: JanetCFunction) -> Janet;
}
extern "C" {
    pub fn janet_wrap_table(x: *mut JanetTable) -> Janet;
}
extern "C" {
    pub fn janet_wrap_abstract(x: *mut ::libc::c_void) -> Janet;
}
extern "C" {
    pub fn janet_wrap_pointer(x: *mut ::libc::c_void) -> Janet;
}
extern "C" {
    pub fn janet_wrap_integer(x: i32) -> Janet;
}
extern "C" {
    pub fn janet_checkint(x: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_checkint64(x: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_checkuint64(x: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_checksize(x: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_checkabstract(x: Janet, at: *const JanetAbstractType) -> JanetAbstract;
}
#[repr(C)]
pub struct JanetGCObject {
    pub flags: i32,
    pub data: JanetGCObject__bindgen_ty_1,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetGCObject {}
#[automatically_derived]
impl ::core::clone::Clone for JanetGCObject {
    #[inline]
    fn clone(&self) -> JanetGCObject {
        let _: ::core::clone::AssertParamIsClone<i32>;
        let _: ::core::clone::AssertParamIsClone<JanetGCObject__bindgen_ty_1>;
        *self
    }
}
#[repr(C)]
pub union JanetGCObject__bindgen_ty_1 {
    pub next: *mut JanetGCObject,
    pub refcount: i32,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetGCObject__bindgen_ty_1 {}
#[automatically_derived]
impl ::core::clone::Clone for JanetGCObject__bindgen_ty_1 {
    #[inline]
    fn clone(&self) -> JanetGCObject__bindgen_ty_1 {
        let _: ::core::clone::AssertParamIsCopy<Self>;
        *self
    }
}
#[repr(C)]
pub struct JanetFiber {
    pub gc: JanetGCObject,
    pub flags: i32,
    pub frame: i32,
    pub stackstart: i32,
    pub stacktop: i32,
    pub capacity: i32,
    pub maxstack: i32,
    pub env: *mut JanetTable,
    pub data: *mut Janet,
    pub child: *mut JanetFiber,
    pub last_value: Janet,
    pub waiting: *mut JanetListenerState,
    pub sched_id: u32,
    pub supervisor_channel: *mut ::libc::c_void,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetFiber {}
#[automatically_derived]
impl ::core::clone::Clone for JanetFiber {
    #[inline]
    fn clone(&self) -> JanetFiber {
        let _: ::core::clone::AssertParamIsClone<JanetGCObject>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetTable>;
        let _: ::core::clone::AssertParamIsClone<*mut Janet>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetFiber>;
        let _: ::core::clone::AssertParamIsClone<Janet>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetListenerState>;
        let _: ::core::clone::AssertParamIsClone<u32>;
        let _: ::core::clone::AssertParamIsClone<*mut ::libc::c_void>;
        *self
    }
}
#[repr(C)]
pub struct JanetStackFrame {
    pub func: *mut JanetFunction,
    pub pc: *mut u32,
    pub env: *mut JanetFuncEnv,
    pub prevframe: i32,
    pub flags: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetStackFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "JanetStackFrame",
            "func",
            &&self.func,
            "pc",
            &&self.pc,
            "env",
            &&self.env,
            "prevframe",
            &&self.prevframe,
            "flags",
            &&self.flags,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetStackFrame {}
#[automatically_derived]
impl ::core::clone::Clone for JanetStackFrame {
    #[inline]
    fn clone(&self) -> JanetStackFrame {
        let _: ::core::clone::AssertParamIsClone<*mut JanetFunction>;
        let _: ::core::clone::AssertParamIsClone<*mut u32>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetFuncEnv>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub struct JanetArray {
    pub gc: JanetGCObject,
    pub count: i32,
    pub capacity: i32,
    pub data: *mut Janet,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetArray {}
#[automatically_derived]
impl ::core::clone::Clone for JanetArray {
    #[inline]
    fn clone(&self) -> JanetArray {
        let _: ::core::clone::AssertParamIsClone<JanetGCObject>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        let _: ::core::clone::AssertParamIsClone<*mut Janet>;
        *self
    }
}
#[repr(C)]
pub struct JanetBuffer {
    pub gc: JanetGCObject,
    pub count: i32,
    pub capacity: i32,
    pub data: *mut u8,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetBuffer {}
#[automatically_derived]
impl ::core::clone::Clone for JanetBuffer {
    #[inline]
    fn clone(&self) -> JanetBuffer {
        let _: ::core::clone::AssertParamIsClone<JanetGCObject>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        let _: ::core::clone::AssertParamIsClone<*mut u8>;
        *self
    }
}
#[repr(C)]
pub struct JanetTable {
    pub gc: JanetGCObject,
    pub count: i32,
    pub capacity: i32,
    pub deleted: i32,
    pub data: *mut JanetKV,
    pub proto: *mut JanetTable,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetTable {}
#[automatically_derived]
impl ::core::clone::Clone for JanetTable {
    #[inline]
    fn clone(&self) -> JanetTable {
        let _: ::core::clone::AssertParamIsClone<JanetGCObject>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetKV>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetTable>;
        *self
    }
}
#[repr(C)]
pub struct JanetKV {
    pub key: Janet,
    pub value: Janet,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetKV {}
#[automatically_derived]
impl ::core::clone::Clone for JanetKV {
    #[inline]
    fn clone(&self) -> JanetKV {
        let _: ::core::clone::AssertParamIsClone<Janet>;
        *self
    }
}
#[repr(C)]
pub struct JanetTupleHead {
    pub gc: JanetGCObject,
    pub length: i32,
    pub hash: i32,
    pub sm_line: i32,
    pub sm_column: i32,
    pub data: __IncompleteArrayField<Janet>,
}
#[repr(C)]
pub struct JanetStructHead {
    pub gc: JanetGCObject,
    pub length: i32,
    pub hash: i32,
    pub capacity: i32,
    pub proto: *const JanetKV,
    pub data: __IncompleteArrayField<JanetKV>,
}
#[repr(C)]
pub struct JanetStringHead {
    pub gc: JanetGCObject,
    pub length: i32,
    pub hash: i32,
    pub data: __IncompleteArrayField<u8>,
}
#[repr(C)]
pub struct JanetAbstractHead {
    pub gc: JanetGCObject,
    pub type_: *const JanetAbstractType,
    pub size: usize,
    pub data: __IncompleteArrayField<::libc::c_longlong>,
}
#[repr(C)]
pub struct JanetSourceMapping {
    pub line: i32,
    pub column: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetSourceMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "JanetSourceMapping",
            "line",
            &&self.line,
            "column",
            &&self.column,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetSourceMapping {}
#[automatically_derived]
impl ::core::clone::Clone for JanetSourceMapping {
    #[inline]
    fn clone(&self) -> JanetSourceMapping {
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub struct JanetFuncDef {
    pub gc: JanetGCObject,
    pub environments: *mut i32,
    pub constants: *mut Janet,
    pub defs: *mut *mut JanetFuncDef,
    pub bytecode: *mut u32,
    pub closure_bitset: *mut u32,
    pub sourcemap: *mut JanetSourceMapping,
    pub source: JanetString,
    pub name: JanetString,
    pub flags: i32,
    pub slotcount: i32,
    pub arity: i32,
    pub min_arity: i32,
    pub max_arity: i32,
    pub constants_length: i32,
    pub bytecode_length: i32,
    pub environments_length: i32,
    pub defs_length: i32,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetFuncDef {}
#[automatically_derived]
impl ::core::clone::Clone for JanetFuncDef {
    #[inline]
    fn clone(&self) -> JanetFuncDef {
        let _: ::core::clone::AssertParamIsClone<JanetGCObject>;
        let _: ::core::clone::AssertParamIsClone<*mut i32>;
        let _: ::core::clone::AssertParamIsClone<*mut Janet>;
        let _: ::core::clone::AssertParamIsClone<*mut *mut JanetFuncDef>;
        let _: ::core::clone::AssertParamIsClone<*mut u32>;
        let _: ::core::clone::AssertParamIsClone<*mut u32>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetSourceMapping>;
        let _: ::core::clone::AssertParamIsClone<JanetString>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub struct JanetFuncEnv {
    pub gc: JanetGCObject,
    pub as_: JanetFuncEnv__bindgen_ty_1,
    pub length: i32,
    pub offset: i32,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetFuncEnv {}
#[automatically_derived]
impl ::core::clone::Clone for JanetFuncEnv {
    #[inline]
    fn clone(&self) -> JanetFuncEnv {
        let _: ::core::clone::AssertParamIsClone<JanetGCObject>;
        let _: ::core::clone::AssertParamIsClone<JanetFuncEnv__bindgen_ty_1>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub union JanetFuncEnv__bindgen_ty_1 {
    pub fiber: *mut JanetFiber,
    pub values: *mut Janet,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetFuncEnv__bindgen_ty_1 {}
#[automatically_derived]
impl ::core::clone::Clone for JanetFuncEnv__bindgen_ty_1 {
    #[inline]
    fn clone(&self) -> JanetFuncEnv__bindgen_ty_1 {
        let _: ::core::clone::AssertParamIsCopy<Self>;
        *self
    }
}
#[repr(C)]
pub struct JanetFunction {
    pub gc: JanetGCObject,
    pub def: *mut JanetFuncDef,
    pub envs: __IncompleteArrayField<*mut JanetFuncEnv>,
}
#[repr(C)]
pub struct JanetParseState {
    _unused: [u8; 0],
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetParseState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "JanetParseState",
            "_unused",
            &&self._unused,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetParseState {}
#[automatically_derived]
impl ::core::clone::Clone for JanetParseState {
    #[inline]
    fn clone(&self) -> JanetParseState {
        let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
        *self
    }
}
pub const JanetParserStatus_JANET_PARSE_ROOT: JanetParserStatus = 0;
pub const JanetParserStatus_JANET_PARSE_ERROR: JanetParserStatus = 1;
pub const JanetParserStatus_JANET_PARSE_PENDING: JanetParserStatus = 2;
pub const JanetParserStatus_JANET_PARSE_DEAD: JanetParserStatus = 3;
pub type JanetParserStatus = ::libc::c_uint;
#[repr(C)]
pub struct JanetParser {
    pub args: *mut Janet,
    pub error: *const ::libc::c_char,
    pub states: *mut JanetParseState,
    pub buf: *mut u8,
    pub argcount: usize,
    pub argcap: usize,
    pub statecount: usize,
    pub statecap: usize,
    pub bufcount: usize,
    pub bufcap: usize,
    pub line: usize,
    pub column: usize,
    pub pending: usize,
    pub lookback: ::libc::c_int,
    pub flag: ::libc::c_int,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "args",
            "error",
            "states",
            "buf",
            "argcount",
            "argcap",
            "statecount",
            "statecap",
            "bufcount",
            "bufcap",
            "line",
            "column",
            "pending",
            "lookback",
            "flag",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.args,
            &&self.error,
            &&self.states,
            &&self.buf,
            &&self.argcount,
            &&self.argcap,
            &&self.statecount,
            &&self.statecap,
            &&self.bufcount,
            &&self.bufcap,
            &&self.line,
            &&self.column,
            &&self.pending,
            &&self.lookback,
            &&self.flag,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "JanetParser",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetParser {}
#[automatically_derived]
impl ::core::clone::Clone for JanetParser {
    #[inline]
    fn clone(&self) -> JanetParser {
        let _: ::core::clone::AssertParamIsClone<*mut Janet>;
        let _: ::core::clone::AssertParamIsClone<*const ::libc::c_char>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetParseState>;
        let _: ::core::clone::AssertParamIsClone<*mut u8>;
        let _: ::core::clone::AssertParamIsClone<usize>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        *self
    }
}
#[repr(C)]
pub struct JanetMarshalContext {
    pub m_state: *mut ::libc::c_void,
    pub u_state: *mut ::libc::c_void,
    pub flags: ::libc::c_int,
    pub data: *const u8,
    pub at: *const JanetAbstractType,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetMarshalContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "JanetMarshalContext",
            "m_state",
            &&self.m_state,
            "u_state",
            &&self.u_state,
            "flags",
            &&self.flags,
            "data",
            &&self.data,
            "at",
            &&self.at,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetMarshalContext {}
#[automatically_derived]
impl ::core::clone::Clone for JanetMarshalContext {
    #[inline]
    fn clone(&self) -> JanetMarshalContext {
        let _: ::core::clone::AssertParamIsClone<*mut ::libc::c_void>;
        let _: ::core::clone::AssertParamIsClone<*mut ::libc::c_void>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<*const u8>;
        let _: ::core::clone::AssertParamIsClone<*const JanetAbstractType>;
        *self
    }
}
#[repr(C)]
pub struct JanetAbstractType {
    pub name: *const ::libc::c_char,
    pub gc: ::core::option::Option<
        unsafe extern "C" fn(data: *mut ::libc::c_void, len: usize) -> ::libc::c_int,
    >,
    pub gcmark: ::core::option::Option<
        unsafe extern "C" fn(data: *mut ::libc::c_void, len: usize) -> ::libc::c_int,
    >,
    pub get: ::core::option::Option<
        unsafe extern "C" fn(
            data: *mut ::libc::c_void,
            key: Janet,
            out: *mut Janet,
        ) -> ::libc::c_int,
    >,
    pub put: ::core::option::Option<
        unsafe extern "C" fn(data: *mut ::libc::c_void, key: Janet, value: Janet),
    >,
    pub marshal: ::core::option::Option<
        unsafe extern "C" fn(p: *mut ::libc::c_void, ctx: *mut JanetMarshalContext),
    >,
    pub unmarshal: ::core::option::Option<
        unsafe extern "C" fn(ctx: *mut JanetMarshalContext) -> *mut ::libc::c_void,
    >,
    pub tostring: ::core::option::Option<
        unsafe extern "C" fn(p: *mut ::libc::c_void, buffer: *mut JanetBuffer),
    >,
    pub compare: ::core::option::Option<
        unsafe extern "C" fn(
            lhs: *mut ::libc::c_void,
            rhs: *mut ::libc::c_void,
        ) -> ::libc::c_int,
    >,
    pub hash: ::core::option::Option<
        unsafe extern "C" fn(p: *mut ::libc::c_void, len: usize) -> i32,
    >,
    pub next: ::core::option::Option<
        unsafe extern "C" fn(p: *mut ::libc::c_void, key: Janet) -> Janet,
    >,
    pub call: ::core::option::Option<
        unsafe extern "C" fn(
            p: *mut ::libc::c_void,
            argc: i32,
            argv: *mut Janet,
        ) -> Janet,
    >,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetAbstractType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "name",
            "gc",
            "gcmark",
            "get",
            "put",
            "marshal",
            "unmarshal",
            "tostring",
            "compare",
            "hash",
            "next",
            "call",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &&self.name,
            &&self.gc,
            &&self.gcmark,
            &&self.get,
            &&self.put,
            &&self.marshal,
            &&self.unmarshal,
            &&self.tostring,
            &&self.compare,
            &&self.hash,
            &&self.next,
            &&self.call,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "JanetAbstractType",
            names,
            values,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetAbstractType {}
#[automatically_derived]
impl ::core::clone::Clone for JanetAbstractType {
    #[inline]
    fn clone(&self) -> JanetAbstractType {
        let _: ::core::clone::AssertParamIsClone<*const ::libc::c_char>;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    data: *mut ::libc::c_void,
                    len: usize,
                ) -> ::libc::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    data: *mut ::libc::c_void,
                    len: usize,
                ) -> ::libc::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    data: *mut ::libc::c_void,
                    key: Janet,
                    out: *mut Janet,
                ) -> ::libc::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(data: *mut ::libc::c_void, key: Janet, value: Janet),
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    p: *mut ::libc::c_void,
                    ctx: *mut JanetMarshalContext,
                ),
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    ctx: *mut JanetMarshalContext,
                ) -> *mut ::libc::c_void,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(p: *mut ::libc::c_void, buffer: *mut JanetBuffer),
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    lhs: *mut ::libc::c_void,
                    rhs: *mut ::libc::c_void,
                ) -> ::libc::c_int,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(p: *mut ::libc::c_void, len: usize) -> i32,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(p: *mut ::libc::c_void, key: Janet) -> Janet,
            >,
        >;
        let _: ::core::clone::AssertParamIsClone<
            ::core::option::Option<
                unsafe extern "C" fn(
                    p: *mut ::libc::c_void,
                    argc: i32,
                    argv: *mut Janet,
                ) -> Janet,
            >,
        >;
        *self
    }
}
#[repr(C)]
pub struct JanetReg {
    pub name: *const ::libc::c_char,
    pub cfun: JanetCFunction,
    pub documentation: *const ::libc::c_char,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetReg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "JanetReg",
            "name",
            &&self.name,
            "cfun",
            &&self.cfun,
            "documentation",
            &&self.documentation,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetReg {}
#[automatically_derived]
impl ::core::clone::Clone for JanetReg {
    #[inline]
    fn clone(&self) -> JanetReg {
        let _: ::core::clone::AssertParamIsClone<*const ::libc::c_char>;
        let _: ::core::clone::AssertParamIsClone<JanetCFunction>;
        let _: ::core::clone::AssertParamIsClone<*const ::libc::c_char>;
        *self
    }
}
#[repr(C)]
pub struct JanetRegExt {
    pub name: *const ::libc::c_char,
    pub cfun: JanetCFunction,
    pub documentation: *const ::libc::c_char,
    pub source_file: *const ::libc::c_char,
    pub source_line: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetRegExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "JanetRegExt",
            "name",
            &&self.name,
            "cfun",
            &&self.cfun,
            "documentation",
            &&self.documentation,
            "source_file",
            &&self.source_file,
            "source_line",
            &&self.source_line,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetRegExt {}
#[automatically_derived]
impl ::core::clone::Clone for JanetRegExt {
    #[inline]
    fn clone(&self) -> JanetRegExt {
        let _: ::core::clone::AssertParamIsClone<*const ::libc::c_char>;
        let _: ::core::clone::AssertParamIsClone<JanetCFunction>;
        let _: ::core::clone::AssertParamIsClone<*const ::libc::c_char>;
        let _: ::core::clone::AssertParamIsClone<*const ::libc::c_char>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub struct JanetMethod {
    pub name: *const ::libc::c_char,
    pub cfun: JanetCFunction,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "JanetMethod",
            "name",
            &&self.name,
            "cfun",
            &&self.cfun,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetMethod {}
#[automatically_derived]
impl ::core::clone::Clone for JanetMethod {
    #[inline]
    fn clone(&self) -> JanetMethod {
        let _: ::core::clone::AssertParamIsClone<*const ::libc::c_char>;
        let _: ::core::clone::AssertParamIsClone<JanetCFunction>;
        *self
    }
}
#[repr(C)]
pub struct JanetView {
    pub items: *const Janet,
    pub len: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "JanetView",
            "items",
            &&self.items,
            "len",
            &&self.len,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetView {}
#[automatically_derived]
impl ::core::clone::Clone for JanetView {
    #[inline]
    fn clone(&self) -> JanetView {
        let _: ::core::clone::AssertParamIsClone<*const Janet>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub struct JanetByteView {
    pub bytes: *const u8,
    pub len: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetByteView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "JanetByteView",
            "bytes",
            &&self.bytes,
            "len",
            &&self.len,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetByteView {}
#[automatically_derived]
impl ::core::clone::Clone for JanetByteView {
    #[inline]
    fn clone(&self) -> JanetByteView {
        let _: ::core::clone::AssertParamIsClone<*const u8>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub struct JanetDictView {
    pub kvs: *const JanetKV,
    pub len: i32,
    pub cap: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetDictView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "JanetDictView",
            "kvs",
            &&self.kvs,
            "len",
            &&self.len,
            "cap",
            &&self.cap,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetDictView {}
#[automatically_derived]
impl ::core::clone::Clone for JanetDictView {
    #[inline]
    fn clone(&self) -> JanetDictView {
        let _: ::core::clone::AssertParamIsClone<*const JanetKV>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub struct JanetRange {
    pub start: i32,
    pub end: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "JanetRange",
            "start",
            &&self.start,
            "end",
            &&self.end,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetRange {}
#[automatically_derived]
impl ::core::clone::Clone for JanetRange {
    #[inline]
    fn clone(&self) -> JanetRange {
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub struct JanetRNG {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
    pub counter: u32,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetRNG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "JanetRNG",
            "a",
            &&self.a,
            "b",
            &&self.b,
            "c",
            &&self.c,
            "d",
            &&self.d,
            "counter",
            &&self.counter,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetRNG {}
#[automatically_derived]
impl ::core::clone::Clone for JanetRNG {
    #[inline]
    fn clone(&self) -> JanetRNG {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}
#[repr(C)]
pub struct JanetFile {
    pub file: *mut FILE,
    pub flags: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "JanetFile",
            "file",
            &&self.file,
            "flags",
            &&self.flags,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetFile {}
#[automatically_derived]
impl ::core::clone::Clone for JanetFile {
    #[inline]
    fn clone(&self) -> JanetFile {
        let _: ::core::clone::AssertParamIsClone<*mut FILE>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[repr(C)]
pub struct JanetTryState {
    pub stackn: i32,
    pub gc_handle: ::libc::c_int,
    pub vm_fiber: *mut JanetFiber,
    pub vm_jmp_buf: *mut jmp_buf,
    pub vm_return_reg: *mut Janet,
    pub buf: jmp_buf,
    pub payload: Janet,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetTryState {}
#[automatically_derived]
impl ::core::clone::Clone for JanetTryState {
    #[inline]
    fn clone(&self) -> JanetTryState {
        let _: ::core::clone::AssertParamIsClone<i32>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetFiber>;
        let _: ::core::clone::AssertParamIsClone<*mut jmp_buf>;
        let _: ::core::clone::AssertParamIsClone<*mut Janet>;
        let _: ::core::clone::AssertParamIsClone<jmp_buf>;
        let _: ::core::clone::AssertParamIsClone<Janet>;
        *self
    }
}
pub const JanetOpArgType_JANET_OAT_SLOT: JanetOpArgType = 0;
pub const JanetOpArgType_JANET_OAT_ENVIRONMENT: JanetOpArgType = 1;
pub const JanetOpArgType_JANET_OAT_CONSTANT: JanetOpArgType = 2;
pub const JanetOpArgType_JANET_OAT_INTEGER: JanetOpArgType = 3;
pub const JanetOpArgType_JANET_OAT_TYPE: JanetOpArgType = 4;
pub const JanetOpArgType_JANET_OAT_SIMPLETYPE: JanetOpArgType = 5;
pub const JanetOpArgType_JANET_OAT_LABEL: JanetOpArgType = 6;
pub const JanetOpArgType_JANET_OAT_FUNCDEF: JanetOpArgType = 7;
/// START SECTION OPCODES
pub type JanetOpArgType = ::libc::c_uint;
pub const JanetInstructionType_JINT_0: JanetInstructionType = 0;
pub const JanetInstructionType_JINT_S: JanetInstructionType = 1;
pub const JanetInstructionType_JINT_L: JanetInstructionType = 2;
pub const JanetInstructionType_JINT_SS: JanetInstructionType = 3;
pub const JanetInstructionType_JINT_SL: JanetInstructionType = 4;
pub const JanetInstructionType_JINT_ST: JanetInstructionType = 5;
pub const JanetInstructionType_JINT_SI: JanetInstructionType = 6;
pub const JanetInstructionType_JINT_SD: JanetInstructionType = 7;
pub const JanetInstructionType_JINT_SU: JanetInstructionType = 8;
pub const JanetInstructionType_JINT_SSS: JanetInstructionType = 9;
pub const JanetInstructionType_JINT_SSI: JanetInstructionType = 10;
pub const JanetInstructionType_JINT_SSU: JanetInstructionType = 11;
pub const JanetInstructionType_JINT_SES: JanetInstructionType = 12;
pub const JanetInstructionType_JINT_SC: JanetInstructionType = 13;
pub type JanetInstructionType = ::libc::c_uint;
pub const JanetOpCode_JOP_NOOP: JanetOpCode = 0;
pub const JanetOpCode_JOP_ERROR: JanetOpCode = 1;
pub const JanetOpCode_JOP_TYPECHECK: JanetOpCode = 2;
pub const JanetOpCode_JOP_RETURN: JanetOpCode = 3;
pub const JanetOpCode_JOP_RETURN_NIL: JanetOpCode = 4;
pub const JanetOpCode_JOP_ADD_IMMEDIATE: JanetOpCode = 5;
pub const JanetOpCode_JOP_ADD: JanetOpCode = 6;
pub const JanetOpCode_JOP_SUBTRACT: JanetOpCode = 7;
pub const JanetOpCode_JOP_MULTIPLY_IMMEDIATE: JanetOpCode = 8;
pub const JanetOpCode_JOP_MULTIPLY: JanetOpCode = 9;
pub const JanetOpCode_JOP_DIVIDE_IMMEDIATE: JanetOpCode = 10;
pub const JanetOpCode_JOP_DIVIDE: JanetOpCode = 11;
pub const JanetOpCode_JOP_MODULO: JanetOpCode = 12;
pub const JanetOpCode_JOP_REMAINDER: JanetOpCode = 13;
pub const JanetOpCode_JOP_BAND: JanetOpCode = 14;
pub const JanetOpCode_JOP_BOR: JanetOpCode = 15;
pub const JanetOpCode_JOP_BXOR: JanetOpCode = 16;
pub const JanetOpCode_JOP_BNOT: JanetOpCode = 17;
pub const JanetOpCode_JOP_SHIFT_LEFT: JanetOpCode = 18;
pub const JanetOpCode_JOP_SHIFT_LEFT_IMMEDIATE: JanetOpCode = 19;
pub const JanetOpCode_JOP_SHIFT_RIGHT: JanetOpCode = 20;
pub const JanetOpCode_JOP_SHIFT_RIGHT_IMMEDIATE: JanetOpCode = 21;
pub const JanetOpCode_JOP_SHIFT_RIGHT_UNSIGNED: JanetOpCode = 22;
pub const JanetOpCode_JOP_SHIFT_RIGHT_UNSIGNED_IMMEDIATE: JanetOpCode = 23;
pub const JanetOpCode_JOP_MOVE_FAR: JanetOpCode = 24;
pub const JanetOpCode_JOP_MOVE_NEAR: JanetOpCode = 25;
pub const JanetOpCode_JOP_JUMP: JanetOpCode = 26;
pub const JanetOpCode_JOP_JUMP_IF: JanetOpCode = 27;
pub const JanetOpCode_JOP_JUMP_IF_NOT: JanetOpCode = 28;
pub const JanetOpCode_JOP_JUMP_IF_NIL: JanetOpCode = 29;
pub const JanetOpCode_JOP_JUMP_IF_NOT_NIL: JanetOpCode = 30;
pub const JanetOpCode_JOP_GREATER_THAN: JanetOpCode = 31;
pub const JanetOpCode_JOP_GREATER_THAN_IMMEDIATE: JanetOpCode = 32;
pub const JanetOpCode_JOP_LESS_THAN: JanetOpCode = 33;
pub const JanetOpCode_JOP_LESS_THAN_IMMEDIATE: JanetOpCode = 34;
pub const JanetOpCode_JOP_EQUALS: JanetOpCode = 35;
pub const JanetOpCode_JOP_EQUALS_IMMEDIATE: JanetOpCode = 36;
pub const JanetOpCode_JOP_COMPARE: JanetOpCode = 37;
pub const JanetOpCode_JOP_LOAD_NIL: JanetOpCode = 38;
pub const JanetOpCode_JOP_LOAD_TRUE: JanetOpCode = 39;
pub const JanetOpCode_JOP_LOAD_FALSE: JanetOpCode = 40;
pub const JanetOpCode_JOP_LOAD_INTEGER: JanetOpCode = 41;
pub const JanetOpCode_JOP_LOAD_CONSTANT: JanetOpCode = 42;
pub const JanetOpCode_JOP_LOAD_UPVALUE: JanetOpCode = 43;
pub const JanetOpCode_JOP_LOAD_SELF: JanetOpCode = 44;
pub const JanetOpCode_JOP_SET_UPVALUE: JanetOpCode = 45;
pub const JanetOpCode_JOP_CLOSURE: JanetOpCode = 46;
pub const JanetOpCode_JOP_PUSH: JanetOpCode = 47;
pub const JanetOpCode_JOP_PUSH_2: JanetOpCode = 48;
pub const JanetOpCode_JOP_PUSH_3: JanetOpCode = 49;
pub const JanetOpCode_JOP_PUSH_ARRAY: JanetOpCode = 50;
pub const JanetOpCode_JOP_CALL: JanetOpCode = 51;
pub const JanetOpCode_JOP_TAILCALL: JanetOpCode = 52;
pub const JanetOpCode_JOP_RESUME: JanetOpCode = 53;
pub const JanetOpCode_JOP_SIGNAL: JanetOpCode = 54;
pub const JanetOpCode_JOP_PROPAGATE: JanetOpCode = 55;
pub const JanetOpCode_JOP_IN: JanetOpCode = 56;
pub const JanetOpCode_JOP_GET: JanetOpCode = 57;
pub const JanetOpCode_JOP_PUT: JanetOpCode = 58;
pub const JanetOpCode_JOP_GET_INDEX: JanetOpCode = 59;
pub const JanetOpCode_JOP_PUT_INDEX: JanetOpCode = 60;
pub const JanetOpCode_JOP_LENGTH: JanetOpCode = 61;
pub const JanetOpCode_JOP_MAKE_ARRAY: JanetOpCode = 62;
pub const JanetOpCode_JOP_MAKE_BUFFER: JanetOpCode = 63;
pub const JanetOpCode_JOP_MAKE_STRING: JanetOpCode = 64;
pub const JanetOpCode_JOP_MAKE_STRUCT: JanetOpCode = 65;
pub const JanetOpCode_JOP_MAKE_TABLE: JanetOpCode = 66;
pub const JanetOpCode_JOP_MAKE_TUPLE: JanetOpCode = 67;
pub const JanetOpCode_JOP_MAKE_BRACKET_TUPLE: JanetOpCode = 68;
pub const JanetOpCode_JOP_GREATER_THAN_EQUAL: JanetOpCode = 69;
pub const JanetOpCode_JOP_LESS_THAN_EQUAL: JanetOpCode = 70;
pub const JanetOpCode_JOP_NEXT: JanetOpCode = 71;
pub const JanetOpCode_JOP_NOT_EQUALS: JanetOpCode = 72;
pub const JanetOpCode_JOP_NOT_EQUALS_IMMEDIATE: JanetOpCode = 73;
pub const JanetOpCode_JOP_CANCEL: JanetOpCode = 74;
pub const JanetOpCode_JOP_INSTRUCTION_COUNT: JanetOpCode = 75;
pub type JanetOpCode = ::libc::c_uint;
extern "C" {
    pub static mut janet_instructions: [JanetInstructionType; 75usize];
}
extern "C" {
    pub static janet_stream_type: JanetAbstractType;
}
extern "C" {
    pub static janet_channel_type: JanetAbstractType;
}
extern "C" {
    pub fn janet_loop();
}
extern "C" {
    pub fn janet_loop_done() -> ::libc::c_int;
}
extern "C" {
    pub fn janet_loop1() -> *mut JanetFiber;
}
extern "C" {
    pub fn janet_loop1_interrupt(vm: *mut JanetVM);
}
extern "C" {
    pub fn janet_stream(
        handle: JanetHandle,
        flags: u32,
        methods: *const JanetMethod,
    ) -> *mut JanetStream;
}
extern "C" {
    pub fn janet_stream_close(stream: *mut JanetStream);
}
extern "C" {
    pub fn janet_cfun_stream_close(argc: i32, argv: *mut Janet) -> Janet;
}
extern "C" {
    pub fn janet_cfun_stream_read(argc: i32, argv: *mut Janet) -> Janet;
}
extern "C" {
    pub fn janet_cfun_stream_chunk(argc: i32, argv: *mut Janet) -> Janet;
}
extern "C" {
    pub fn janet_cfun_stream_write(argc: i32, argv: *mut Janet) -> Janet;
}
extern "C" {
    pub fn janet_stream_flags(stream: *mut JanetStream, flags: u32);
}
extern "C" {
    pub fn janet_schedule(fiber: *mut JanetFiber, value: Janet);
}
extern "C" {
    pub fn janet_cancel(fiber: *mut JanetFiber, value: Janet);
}
extern "C" {
    pub fn janet_schedule_signal(fiber: *mut JanetFiber, value: Janet, sig: JanetSignal);
}
extern "C" {
    pub fn janet_listen(
        stream: *mut JanetStream,
        behavior: JanetListener,
        mask: ::libc::c_int,
        size: usize,
        user: *mut ::libc::c_void,
    ) -> *mut JanetListenerState;
}
extern "C" {
    pub fn janet_await() -> !;
}
extern "C" {
    pub fn janet_sleep_await(sec: f64) -> !;
}
extern "C" {
    pub fn janet_addtimeout(sec: f64);
}
extern "C" {
    pub fn janet_ev_inc_refcount();
}
extern "C" {
    pub fn janet_ev_dec_refcount();
}
extern "C" {
    pub fn janet_abstract_begin_threaded(
        atype: *const JanetAbstractType,
        size: usize,
    ) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_abstract_end_threaded(x: *mut ::libc::c_void) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_abstract_threaded(
        atype: *const JanetAbstractType,
        size: usize,
    ) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_abstract_incref(abst: *mut ::libc::c_void) -> i32;
}
extern "C" {
    pub fn janet_abstract_decref(abst: *mut ::libc::c_void) -> i32;
}
extern "C" {
    pub fn janet_os_mutex_size() -> usize;
}
extern "C" {
    pub fn janet_os_rwlock_size() -> usize;
}
extern "C" {
    pub fn janet_os_mutex_init(mutex: *mut JanetOSMutex);
}
extern "C" {
    pub fn janet_os_mutex_deinit(mutex: *mut JanetOSMutex);
}
extern "C" {
    pub fn janet_os_mutex_lock(mutex: *mut JanetOSMutex);
}
extern "C" {
    pub fn janet_os_mutex_unlock(mutex: *mut JanetOSMutex);
}
extern "C" {
    pub fn janet_os_rwlock_init(rwlock: *mut JanetOSRWLock);
}
extern "C" {
    pub fn janet_os_rwlock_deinit(rwlock: *mut JanetOSRWLock);
}
extern "C" {
    pub fn janet_os_rwlock_rlock(rwlock: *mut JanetOSRWLock);
}
extern "C" {
    pub fn janet_os_rwlock_wlock(rwlock: *mut JanetOSRWLock);
}
extern "C" {
    pub fn janet_os_rwlock_runlock(rwlock: *mut JanetOSRWLock);
}
extern "C" {
    pub fn janet_os_rwlock_wunlock(rwlock: *mut JanetOSRWLock);
}
extern "C" {
    pub fn janet_ev_lasterr() -> Janet;
}
#[repr(C)]
pub struct JanetEVGenericMessage {
    pub tag: ::libc::c_int,
    pub argi: ::libc::c_int,
    pub argp: *mut ::libc::c_void,
    pub argj: Janet,
    pub fiber: *mut JanetFiber,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetEVGenericMessage {}
#[automatically_derived]
impl ::core::clone::Clone for JanetEVGenericMessage {
    #[inline]
    fn clone(&self) -> JanetEVGenericMessage {
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        let _: ::core::clone::AssertParamIsClone<*mut ::libc::c_void>;
        let _: ::core::clone::AssertParamIsClone<Janet>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetFiber>;
        *self
    }
}
pub type JanetThreadedSubroutine = ::core::option::Option<
    unsafe extern "C" fn(arguments: JanetEVGenericMessage) -> JanetEVGenericMessage,
>;
pub type JanetCallback = ::core::option::Option<
    unsafe extern "C" fn(return_value: JanetEVGenericMessage),
>;
pub type JanetThreadedCallback = ::core::option::Option<
    unsafe extern "C" fn(return_value: JanetEVGenericMessage),
>;
extern "C" {
    pub fn janet_ev_threaded_call(
        fp: JanetThreadedSubroutine,
        arguments: JanetEVGenericMessage,
        cb: JanetThreadedCallback,
    );
}
extern "C" {
    pub fn janet_ev_threaded_await(
        fp: JanetThreadedSubroutine,
        tag: ::libc::c_int,
        argi: ::libc::c_int,
        argp: *mut ::libc::c_void,
    ) -> !;
}
extern "C" {
    pub fn janet_ev_post_event(
        vm: *mut JanetVM,
        cb: JanetCallback,
        msg: JanetEVGenericMessage,
    );
}
extern "C" {
    pub fn janet_ev_default_threaded_callback(return_value: JanetEVGenericMessage);
}
extern "C" {
    pub fn janet_ev_read(stream: *mut JanetStream, buf: *mut JanetBuffer, nbytes: i32);
}
extern "C" {
    pub fn janet_ev_readchunk(
        stream: *mut JanetStream,
        buf: *mut JanetBuffer,
        nbytes: i32,
    );
}
extern "C" {
    pub fn janet_ev_recv(
        stream: *mut JanetStream,
        buf: *mut JanetBuffer,
        nbytes: i32,
        flags: ::libc::c_int,
    );
}
extern "C" {
    pub fn janet_ev_recvchunk(
        stream: *mut JanetStream,
        buf: *mut JanetBuffer,
        nbytes: i32,
        flags: ::libc::c_int,
    );
}
extern "C" {
    pub fn janet_ev_recvfrom(
        stream: *mut JanetStream,
        buf: *mut JanetBuffer,
        nbytes: i32,
        flags: ::libc::c_int,
    );
}
extern "C" {
    pub fn janet_ev_write_buffer(stream: *mut JanetStream, buf: *mut JanetBuffer);
}
extern "C" {
    pub fn janet_ev_write_string(stream: *mut JanetStream, str_: JanetString);
}
extern "C" {
    pub fn janet_ev_send_buffer(
        stream: *mut JanetStream,
        buf: *mut JanetBuffer,
        flags: ::libc::c_int,
    );
}
extern "C" {
    pub fn janet_ev_send_string(
        stream: *mut JanetStream,
        str_: JanetString,
        flags: ::libc::c_int,
    );
}
extern "C" {
    pub fn janet_ev_sendto_buffer(
        stream: *mut JanetStream,
        buf: *mut JanetBuffer,
        dest: *mut ::libc::c_void,
        flags: ::libc::c_int,
    );
}
extern "C" {
    pub fn janet_ev_sendto_string(
        stream: *mut JanetStream,
        str_: JanetString,
        dest: *mut ::libc::c_void,
        flags: ::libc::c_int,
    );
}
extern "C" {
    pub static janet_parser_type: JanetAbstractType;
}
extern "C" {
    pub fn janet_parser_init(parser: *mut JanetParser);
}
extern "C" {
    pub fn janet_parser_deinit(parser: *mut JanetParser);
}
extern "C" {
    pub fn janet_parser_consume(parser: *mut JanetParser, c: u8);
}
extern "C" {
    pub fn janet_parser_status(parser: *mut JanetParser) -> JanetParserStatus;
}
extern "C" {
    pub fn janet_parser_produce(parser: *mut JanetParser) -> Janet;
}
extern "C" {
    pub fn janet_parser_produce_wrapped(parser: *mut JanetParser) -> Janet;
}
extern "C" {
    pub fn janet_parser_error(parser: *mut JanetParser) -> *const ::libc::c_char;
}
extern "C" {
    pub fn janet_parser_flush(parser: *mut JanetParser);
}
extern "C" {
    pub fn janet_parser_eof(parser: *mut JanetParser);
}
extern "C" {
    pub fn janet_parser_has_more(parser: *mut JanetParser) -> ::libc::c_int;
}
pub const JanetAssembleStatus_JANET_ASSEMBLE_OK: JanetAssembleStatus = 0;
pub const JanetAssembleStatus_JANET_ASSEMBLE_ERROR: JanetAssembleStatus = 1;
pub type JanetAssembleStatus = ::libc::c_uint;
#[repr(C)]
pub struct JanetAssembleResult {
    pub funcdef: *mut JanetFuncDef,
    pub error: JanetString,
    pub status: JanetAssembleStatus,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetAssembleResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "JanetAssembleResult",
            "funcdef",
            &&self.funcdef,
            "error",
            &&self.error,
            "status",
            &&self.status,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetAssembleResult {}
#[automatically_derived]
impl ::core::clone::Clone for JanetAssembleResult {
    #[inline]
    fn clone(&self) -> JanetAssembleResult {
        let _: ::core::clone::AssertParamIsClone<*mut JanetFuncDef>;
        let _: ::core::clone::AssertParamIsClone<JanetString>;
        let _: ::core::clone::AssertParamIsClone<JanetAssembleStatus>;
        *self
    }
}
extern "C" {
    pub fn janet_asm(source: Janet, flags: ::libc::c_int) -> JanetAssembleResult;
}
extern "C" {
    pub fn janet_disasm(def: *mut JanetFuncDef) -> Janet;
}
extern "C" {
    pub fn janet_asm_decode_instruction(instr: u32) -> Janet;
}
pub const JanetCompileStatus_JANET_COMPILE_OK: JanetCompileStatus = 0;
pub const JanetCompileStatus_JANET_COMPILE_ERROR: JanetCompileStatus = 1;
pub type JanetCompileStatus = ::libc::c_uint;
#[repr(C)]
pub struct JanetCompileResult {
    pub funcdef: *mut JanetFuncDef,
    pub error: JanetString,
    pub macrofiber: *mut JanetFiber,
    pub error_mapping: JanetSourceMapping,
    pub status: JanetCompileStatus,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetCompileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "JanetCompileResult",
            "funcdef",
            &&self.funcdef,
            "error",
            &&self.error,
            "macrofiber",
            &&self.macrofiber,
            "error_mapping",
            &&self.error_mapping,
            "status",
            &&self.status,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetCompileResult {}
#[automatically_derived]
impl ::core::clone::Clone for JanetCompileResult {
    #[inline]
    fn clone(&self) -> JanetCompileResult {
        let _: ::core::clone::AssertParamIsClone<*mut JanetFuncDef>;
        let _: ::core::clone::AssertParamIsClone<JanetString>;
        let _: ::core::clone::AssertParamIsClone<*mut JanetFiber>;
        let _: ::core::clone::AssertParamIsClone<JanetSourceMapping>;
        let _: ::core::clone::AssertParamIsClone<JanetCompileStatus>;
        *self
    }
}
extern "C" {
    pub fn janet_compile(
        source: Janet,
        env: *mut JanetTable,
        where_: JanetString,
    ) -> JanetCompileResult;
}
extern "C" {
    pub fn janet_compile_lint(
        source: Janet,
        env: *mut JanetTable,
        where_: JanetString,
        lints: *mut JanetArray,
    ) -> JanetCompileResult;
}
extern "C" {
    pub fn janet_core_env(replacements: *mut JanetTable) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_core_lookup_table(replacements: *mut JanetTable) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_dobytes(
        env: *mut JanetTable,
        bytes: *const u8,
        len: i32,
        sourcePath: *const ::libc::c_char,
        out: *mut Janet,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_dostring(
        env: *mut JanetTable,
        str_: *const ::libc::c_char,
        sourcePath: *const ::libc::c_char,
        out: *mut Janet,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_loop_fiber(fiber: *mut JanetFiber) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_scan_number(str_: *const u8, len: i32, out: *mut f64) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_scan_number_base(
        str_: *const u8,
        len: i32,
        base: i32,
        out: *mut f64,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_scan_int64(str_: *const u8, len: i32, out: *mut i64) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_scan_uint64(str_: *const u8, len: i32, out: *mut u64) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_debug_break(def: *mut JanetFuncDef, pc: i32);
}
extern "C" {
    pub fn janet_debug_unbreak(def: *mut JanetFuncDef, pc: i32);
}
extern "C" {
    pub fn janet_debug_find(
        def_out: *mut *mut JanetFuncDef,
        pc_out: *mut i32,
        source: JanetString,
        line: i32,
        column: i32,
    );
}
extern "C" {
    pub static janet_rng_type: JanetAbstractType;
}
extern "C" {
    pub fn janet_default_rng() -> *mut JanetRNG;
}
extern "C" {
    pub fn janet_rng_seed(rng: *mut JanetRNG, seed: u32);
}
extern "C" {
    pub fn janet_rng_longseed(rng: *mut JanetRNG, bytes: *const u8, len: i32);
}
extern "C" {
    pub fn janet_rng_u32(rng: *mut JanetRNG) -> u32;
}
extern "C" {
    pub fn janet_rng_double(rng: *mut JanetRNG) -> f64;
}
extern "C" {
    pub fn janet_array(capacity: i32) -> *mut JanetArray;
}
extern "C" {
    pub fn janet_array_n(elements: *const Janet, n: i32) -> *mut JanetArray;
}
extern "C" {
    pub fn janet_array_ensure(array: *mut JanetArray, capacity: i32, growth: i32);
}
extern "C" {
    pub fn janet_array_setcount(array: *mut JanetArray, count: i32);
}
extern "C" {
    pub fn janet_array_push(array: *mut JanetArray, x: Janet);
}
extern "C" {
    pub fn janet_array_pop(array: *mut JanetArray) -> Janet;
}
extern "C" {
    pub fn janet_array_peek(array: *mut JanetArray) -> Janet;
}
extern "C" {
    pub fn janet_buffer(capacity: i32) -> *mut JanetBuffer;
}
extern "C" {
    pub fn janet_buffer_init(
        buffer: *mut JanetBuffer,
        capacity: i32,
    ) -> *mut JanetBuffer;
}
extern "C" {
    pub fn janet_buffer_deinit(buffer: *mut JanetBuffer);
}
extern "C" {
    pub fn janet_buffer_ensure(buffer: *mut JanetBuffer, capacity: i32, growth: i32);
}
extern "C" {
    pub fn janet_buffer_setcount(buffer: *mut JanetBuffer, count: i32);
}
extern "C" {
    pub fn janet_buffer_extra(buffer: *mut JanetBuffer, n: i32);
}
extern "C" {
    pub fn janet_buffer_push_bytes(
        buffer: *mut JanetBuffer,
        string: *const u8,
        len: i32,
    );
}
extern "C" {
    pub fn janet_buffer_push_string(buffer: *mut JanetBuffer, string: JanetString);
}
extern "C" {
    pub fn janet_buffer_push_cstring(
        buffer: *mut JanetBuffer,
        cstring: *const ::libc::c_char,
    );
}
extern "C" {
    pub fn janet_buffer_push_u8(buffer: *mut JanetBuffer, x: u8);
}
extern "C" {
    pub fn janet_buffer_push_u16(buffer: *mut JanetBuffer, x: u16);
}
extern "C" {
    pub fn janet_buffer_push_u32(buffer: *mut JanetBuffer, x: u32);
}
extern "C" {
    pub fn janet_buffer_push_u64(buffer: *mut JanetBuffer, x: u64);
}
extern "C" {
    pub fn janet_tuple_begin(length: i32) -> *mut Janet;
}
extern "C" {
    pub fn janet_tuple_end(tuple: *mut Janet) -> JanetTuple;
}
extern "C" {
    pub fn janet_tuple_n(values: *const Janet, n: i32) -> JanetTuple;
}
extern "C" {
    pub fn janet_string_begin(length: i32) -> *mut u8;
}
extern "C" {
    pub fn janet_string_end(str_: *mut u8) -> JanetString;
}
extern "C" {
    pub fn janet_string(buf: *const u8, len: i32) -> JanetString;
}
extern "C" {
    pub fn janet_cstring(cstring: *const ::libc::c_char) -> JanetString;
}
extern "C" {
    pub fn janet_string_compare(lhs: JanetString, rhs: JanetString) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_string_equal(lhs: JanetString, rhs: JanetString) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_string_equalconst(
        lhs: JanetString,
        rhs: *const u8,
        rlen: i32,
        rhash: i32,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_description(x: Janet) -> JanetString;
}
extern "C" {
    pub fn janet_to_string(x: Janet) -> JanetString;
}
extern "C" {
    pub fn janet_to_string_b(buffer: *mut JanetBuffer, x: Janet);
}
extern "C" {
    pub fn janet_description_b(buffer: *mut JanetBuffer, x: Janet);
}
extern "C" {
    pub fn janet_formatc(format: *const ::libc::c_char, ...) -> JanetString;
}
extern "C" {
    pub fn janet_formatb(
        bufp: *mut JanetBuffer,
        format: *const ::libc::c_char,
        ...
    ) -> *mut JanetBuffer;
}
extern "C" {
    pub fn janet_formatbv(
        bufp: *mut JanetBuffer,
        format: *const ::libc::c_char,
        args: va_list,
    );
}
extern "C" {
    pub fn janet_symbol(str_: *const u8, len: i32) -> JanetSymbol;
}
extern "C" {
    pub fn janet_csymbol(str_: *const ::libc::c_char) -> JanetSymbol;
}
extern "C" {
    pub fn janet_symbol_gen() -> JanetSymbol;
}
extern "C" {
    pub fn janet_struct_begin(count: i32) -> *mut JanetKV;
}
extern "C" {
    pub fn janet_struct_put(st: *mut JanetKV, key: Janet, value: Janet);
}
extern "C" {
    pub fn janet_struct_end(st: *mut JanetKV) -> JanetStruct;
}
extern "C" {
    pub fn janet_struct_get(st: JanetStruct, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_struct_rawget(st: JanetStruct, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_struct_get_ex(
        st: JanetStruct,
        key: Janet,
        which: *mut JanetStruct,
    ) -> Janet;
}
extern "C" {
    pub fn janet_struct_to_table(st: JanetStruct) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_struct_find(st: JanetStruct, key: Janet) -> *const JanetKV;
}
extern "C" {
    pub fn janet_table(capacity: i32) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_table_init(table: *mut JanetTable, capacity: i32) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_table_init_raw(
        table: *mut JanetTable,
        capacity: i32,
    ) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_table_deinit(table: *mut JanetTable);
}
extern "C" {
    pub fn janet_table_get(t: *mut JanetTable, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_table_get_ex(
        t: *mut JanetTable,
        key: Janet,
        which: *mut *mut JanetTable,
    ) -> Janet;
}
extern "C" {
    pub fn janet_table_rawget(t: *mut JanetTable, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_table_remove(t: *mut JanetTable, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_table_put(t: *mut JanetTable, key: Janet, value: Janet);
}
extern "C" {
    pub fn janet_table_to_struct(t: *mut JanetTable) -> JanetStruct;
}
extern "C" {
    pub fn janet_table_merge_table(table: *mut JanetTable, other: *mut JanetTable);
}
extern "C" {
    pub fn janet_table_merge_struct(table: *mut JanetTable, other: JanetStruct);
}
extern "C" {
    pub fn janet_table_find(t: *mut JanetTable, key: Janet) -> *mut JanetKV;
}
extern "C" {
    pub fn janet_table_clone(table: *mut JanetTable) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_table_clear(table: *mut JanetTable);
}
extern "C" {
    pub fn janet_fiber(
        callee: *mut JanetFunction,
        capacity: i32,
        argc: i32,
        argv: *const Janet,
    ) -> *mut JanetFiber;
}
extern "C" {
    pub fn janet_fiber_reset(
        fiber: *mut JanetFiber,
        callee: *mut JanetFunction,
        argc: i32,
        argv: *const Janet,
    ) -> *mut JanetFiber;
}
extern "C" {
    pub fn janet_fiber_status(fiber: *mut JanetFiber) -> JanetFiberStatus;
}
extern "C" {
    pub fn janet_current_fiber() -> *mut JanetFiber;
}
extern "C" {
    pub fn janet_root_fiber() -> *mut JanetFiber;
}
extern "C" {
    pub fn janet_indexed_view(
        seq: Janet,
        data: *mut *const Janet,
        len: *mut i32,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_bytes_view(
        str_: Janet,
        data: *mut *const u8,
        len: *mut i32,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_dictionary_view(
        tab: Janet,
        data: *mut *const JanetKV,
        len: *mut i32,
        cap: *mut i32,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_dictionary_get(data: *const JanetKV, cap: i32, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_dictionary_next(
        kvs: *const JanetKV,
        cap: i32,
        kv: *const JanetKV,
    ) -> *const JanetKV;
}
extern "C" {
    pub fn janet_abstract_begin(
        type_: *const JanetAbstractType,
        size: usize,
    ) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_abstract_end(abstractTemplate: *mut ::libc::c_void) -> JanetAbstract;
}
extern "C" {
    pub fn janet_abstract(type_: *const JanetAbstractType, size: usize) -> JanetAbstract;
}
pub type JanetModule = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut JanetTable),
>;
pub type JanetModconf = ::core::option::Option<
    unsafe extern "C" fn() -> JanetBuildConfig,
>;
extern "C" {
    pub fn janet_native(
        name: *const ::libc::c_char,
        error: *mut JanetString,
    ) -> JanetModule;
}
extern "C" {
    pub fn janet_marshal(
        buf: *mut JanetBuffer,
        x: Janet,
        rreg: *mut JanetTable,
        flags: ::libc::c_int,
    );
}
extern "C" {
    pub fn janet_unmarshal(
        bytes: *const u8,
        len: usize,
        flags: ::libc::c_int,
        reg: *mut JanetTable,
        next: *mut *const u8,
    ) -> Janet;
}
extern "C" {
    pub fn janet_env_lookup(env: *mut JanetTable) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_env_lookup_into(
        renv: *mut JanetTable,
        env: *mut JanetTable,
        prefix: *const ::libc::c_char,
        recurse: ::libc::c_int,
    );
}
extern "C" {
    pub fn janet_mark(x: Janet);
}
extern "C" {
    pub fn janet_sweep();
}
extern "C" {
    pub fn janet_collect();
}
extern "C" {
    pub fn janet_clear_memory();
}
extern "C" {
    pub fn janet_gcroot(root: Janet);
}
extern "C" {
    pub fn janet_gcunroot(root: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_gcunrootall(root: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_gclock() -> ::libc::c_int;
}
extern "C" {
    pub fn janet_gcunlock(handle: ::libc::c_int);
}
extern "C" {
    pub fn janet_gcpressure(s: usize);
}
extern "C" {
    pub fn janet_funcdef_alloc() -> *mut JanetFuncDef;
}
extern "C" {
    pub fn janet_thunk(def: *mut JanetFuncDef) -> *mut JanetFunction;
}
extern "C" {
    pub fn janet_verify(def: *mut JanetFuncDef) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_pretty(
        buffer: *mut JanetBuffer,
        depth: ::libc::c_int,
        flags: ::libc::c_int,
        x: Janet,
    ) -> *mut JanetBuffer;
}
extern "C" {
    pub fn janet_try_init(state: *mut JanetTryState);
}
extern "C" {
    pub fn janet_restore(state: *mut JanetTryState);
}
extern "C" {
    pub fn janet_equals(x: Janet, y: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_hash(x: Janet) -> i32;
}
extern "C" {
    pub fn janet_compare(x: Janet, y: Janet) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_cstrcmp(
        str_: JanetString,
        other: *const ::libc::c_char,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_in(ds: Janet, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_get(ds: Janet, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_next(ds: Janet, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_getindex(ds: Janet, index: i32) -> Janet;
}
extern "C" {
    pub fn janet_length(x: Janet) -> i32;
}
extern "C" {
    pub fn janet_lengthv(x: Janet) -> Janet;
}
extern "C" {
    pub fn janet_put(ds: Janet, key: Janet, value: Janet);
}
extern "C" {
    pub fn janet_putindex(ds: Janet, index: i32, value: Janet);
}
extern "C" {
    pub fn janet_wrap_number_safe(x: f64) -> Janet;
}
extern "C" {
    pub fn janet_keyeq(x: Janet, cstring: *const ::libc::c_char) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_streq(x: Janet, cstring: *const ::libc::c_char) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_symeq(x: Janet, cstring: *const ::libc::c_char) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_sorted_keys(
        dict: *const JanetKV,
        cap: i32,
        index_buffer: *mut i32,
    ) -> i32;
}
extern "C" {
    pub fn janet_init() -> ::libc::c_int;
}
extern "C" {
    pub fn janet_deinit();
}
extern "C" {
    pub fn janet_vm_alloc() -> *mut JanetVM;
}
extern "C" {
    pub fn janet_local_vm() -> *mut JanetVM;
}
extern "C" {
    pub fn janet_vm_free(vm: *mut JanetVM);
}
extern "C" {
    pub fn janet_vm_save(into: *mut JanetVM);
}
extern "C" {
    pub fn janet_vm_load(from: *mut JanetVM);
}
extern "C" {
    pub fn janet_interpreter_interrupt(vm: *mut JanetVM);
}
extern "C" {
    pub fn janet_continue(
        fiber: *mut JanetFiber,
        in_: Janet,
        out: *mut Janet,
    ) -> JanetSignal;
}
extern "C" {
    pub fn janet_continue_signal(
        fiber: *mut JanetFiber,
        in_: Janet,
        out: *mut Janet,
        sig: JanetSignal,
    ) -> JanetSignal;
}
extern "C" {
    pub fn janet_pcall(
        fun: *mut JanetFunction,
        argn: i32,
        argv: *const Janet,
        out: *mut Janet,
        f: *mut *mut JanetFiber,
    ) -> JanetSignal;
}
extern "C" {
    pub fn janet_step(
        fiber: *mut JanetFiber,
        in_: Janet,
        out: *mut Janet,
    ) -> JanetSignal;
}
extern "C" {
    pub fn janet_call(fun: *mut JanetFunction, argc: i32, argv: *const Janet) -> Janet;
}
extern "C" {
    pub fn janet_mcall(
        name: *const ::libc::c_char,
        argc: i32,
        argv: *mut Janet,
    ) -> Janet;
}
extern "C" {
    pub fn janet_stacktrace(fiber: *mut JanetFiber, err: Janet);
}
extern "C" {
    pub fn janet_stacktrace_ext(
        fiber: *mut JanetFiber,
        err: Janet,
        prefix: *const ::libc::c_char,
    );
}
pub type JanetScratchFinalizer = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut ::libc::c_void),
>;
extern "C" {
    pub fn janet_smalloc(size: usize) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_srealloc(mem: *mut ::libc::c_void, size: usize) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_scalloc(nmemb: usize, size: usize) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_sfinalizer(mem: *mut ::libc::c_void, finalizer: JanetScratchFinalizer);
}
extern "C" {
    pub fn janet_sfree(mem: *mut ::libc::c_void);
}
pub const JanetBindingType_JANET_BINDING_NONE: JanetBindingType = 0;
pub const JanetBindingType_JANET_BINDING_DEF: JanetBindingType = 1;
pub const JanetBindingType_JANET_BINDING_VAR: JanetBindingType = 2;
pub const JanetBindingType_JANET_BINDING_MACRO: JanetBindingType = 3;
pub const JanetBindingType_JANET_BINDING_DYNAMIC_DEF: JanetBindingType = 4;
pub const JanetBindingType_JANET_BINDING_DYNAMIC_MACRO: JanetBindingType = 5;
pub type JanetBindingType = ::libc::c_uint;
#[repr(C)]
pub struct JanetBinding {
    pub type_: JanetBindingType,
    pub value: Janet,
    pub deprecation: JanetBinding__bindgen_ty_1,
}
#[automatically_derived]
impl ::core::marker::Copy for JanetBinding {}
#[automatically_derived]
impl ::core::clone::Clone for JanetBinding {
    #[inline]
    fn clone(&self) -> JanetBinding {
        let _: ::core::clone::AssertParamIsClone<JanetBindingType>;
        let _: ::core::clone::AssertParamIsClone<Janet>;
        let _: ::core::clone::AssertParamIsClone<JanetBinding__bindgen_ty_1>;
        *self
    }
}
pub const JanetBinding_JANET_BINDING_DEP_NONE: JanetBinding__bindgen_ty_1 = 0;
pub const JanetBinding_JANET_BINDING_DEP_RELAXED: JanetBinding__bindgen_ty_1 = 1;
pub const JanetBinding_JANET_BINDING_DEP_NORMAL: JanetBinding__bindgen_ty_1 = 2;
pub const JanetBinding_JANET_BINDING_DEP_STRICT: JanetBinding__bindgen_ty_1 = 3;
pub type JanetBinding__bindgen_ty_1 = ::libc::c_uint;
extern "C" {
    pub fn janet_def(
        env: *mut JanetTable,
        name: *const ::libc::c_char,
        val: Janet,
        documentation: *const ::libc::c_char,
    );
}
extern "C" {
    pub fn janet_var(
        env: *mut JanetTable,
        name: *const ::libc::c_char,
        val: Janet,
        documentation: *const ::libc::c_char,
    );
}
extern "C" {
    pub fn janet_cfuns(
        env: *mut JanetTable,
        regprefix: *const ::libc::c_char,
        cfuns: *const JanetReg,
    );
}
extern "C" {
    pub fn janet_cfuns_prefix(
        env: *mut JanetTable,
        regprefix: *const ::libc::c_char,
        cfuns: *const JanetReg,
    );
}
extern "C" {
    pub fn janet_resolve(
        env: *mut JanetTable,
        sym: JanetSymbol,
        out: *mut Janet,
    ) -> JanetBindingType;
}
extern "C" {
    pub fn janet_resolve_ext(env: *mut JanetTable, sym: JanetSymbol) -> JanetBinding;
}
extern "C" {
    pub fn janet_resolve_core(name: *const ::libc::c_char) -> Janet;
}
extern "C" {
    pub fn janet_cfuns_ext(
        env: *mut JanetTable,
        regprefix: *const ::libc::c_char,
        cfuns: *const JanetRegExt,
    );
}
extern "C" {
    pub fn janet_cfuns_ext_prefix(
        env: *mut JanetTable,
        regprefix: *const ::libc::c_char,
        cfuns: *const JanetRegExt,
    );
}
extern "C" {
    pub fn janet_def_sm(
        env: *mut JanetTable,
        name: *const ::libc::c_char,
        val: Janet,
        documentation: *const ::libc::c_char,
        source_file: *const ::libc::c_char,
        source_line: i32,
    );
}
extern "C" {
    pub fn janet_var_sm(
        env: *mut JanetTable,
        name: *const ::libc::c_char,
        val: Janet,
        documentation: *const ::libc::c_char,
        source_file: *const ::libc::c_char,
        source_line: i32,
    );
}
extern "C" {
    pub fn janet_register(name: *const ::libc::c_char, cfun: JanetCFunction);
}
extern "C" {
    pub fn janet_signalv(signal: JanetSignal, message: Janet) -> !;
}
extern "C" {
    pub fn janet_panicv(message: Janet) -> !;
}
extern "C" {
    pub fn janet_panic(message: *const ::libc::c_char) -> !;
}
extern "C" {
    pub fn janet_panics(message: JanetString) -> !;
}
extern "C" {
    pub fn janet_panicf(format: *const ::libc::c_char, ...) -> !;
}
extern "C" {
    pub fn janet_dynprintf(
        name: *const ::libc::c_char,
        dflt_file: *mut FILE,
        format: *const ::libc::c_char,
        ...
    );
}
extern "C" {
    pub fn janet_panic_type(x: Janet, n: i32, expected: ::libc::c_int) -> !;
}
extern "C" {
    pub fn janet_panic_abstract(x: Janet, n: i32, at: *const JanetAbstractType) -> !;
}
extern "C" {
    pub fn janet_arity(arity: i32, min: i32, max: i32);
}
extern "C" {
    pub fn janet_fixarity(arity: i32, fix: i32);
}
extern "C" {
    pub fn janet_getmethod(
        method: JanetKeyword,
        methods: *const JanetMethod,
        out: *mut Janet,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_nextmethod(methods: *const JanetMethod, key: Janet) -> Janet;
}
extern "C" {
    pub fn janet_getnumber(argv: *const Janet, n: i32) -> f64;
}
extern "C" {
    pub fn janet_getarray(argv: *const Janet, n: i32) -> *mut JanetArray;
}
extern "C" {
    pub fn janet_gettuple(argv: *const Janet, n: i32) -> JanetTuple;
}
extern "C" {
    pub fn janet_gettable(argv: *const Janet, n: i32) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_getstruct(argv: *const Janet, n: i32) -> JanetStruct;
}
extern "C" {
    pub fn janet_getstring(argv: *const Janet, n: i32) -> JanetString;
}
extern "C" {
    pub fn janet_getcstring(argv: *const Janet, n: i32) -> *const ::libc::c_char;
}
extern "C" {
    pub fn janet_getsymbol(argv: *const Janet, n: i32) -> JanetSymbol;
}
extern "C" {
    pub fn janet_getkeyword(argv: *const Janet, n: i32) -> JanetKeyword;
}
extern "C" {
    pub fn janet_getbuffer(argv: *const Janet, n: i32) -> *mut JanetBuffer;
}
extern "C" {
    pub fn janet_getfiber(argv: *const Janet, n: i32) -> *mut JanetFiber;
}
extern "C" {
    pub fn janet_getfunction(argv: *const Janet, n: i32) -> *mut JanetFunction;
}
extern "C" {
    pub fn janet_getcfunction(argv: *const Janet, n: i32) -> JanetCFunction;
}
extern "C" {
    pub fn janet_getboolean(argv: *const Janet, n: i32) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_getpointer(argv: *const Janet, n: i32) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_getnat(argv: *const Janet, n: i32) -> i32;
}
extern "C" {
    pub fn janet_getinteger(argv: *const Janet, n: i32) -> i32;
}
extern "C" {
    pub fn janet_getinteger64(argv: *const Janet, n: i32) -> i64;
}
extern "C" {
    pub fn janet_getuinteger64(argv: *const Janet, n: i32) -> u64;
}
extern "C" {
    pub fn janet_getsize(argv: *const Janet, n: i32) -> usize;
}
extern "C" {
    pub fn janet_getindexed(argv: *const Janet, n: i32) -> JanetView;
}
extern "C" {
    pub fn janet_getbytes(argv: *const Janet, n: i32) -> JanetByteView;
}
extern "C" {
    pub fn janet_getdictionary(argv: *const Janet, n: i32) -> JanetDictView;
}
extern "C" {
    pub fn janet_getabstract(
        argv: *const Janet,
        n: i32,
        at: *const JanetAbstractType,
    ) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_getslice(argc: i32, argv: *const Janet) -> JanetRange;
}
extern "C" {
    pub fn janet_gethalfrange(
        argv: *const Janet,
        n: i32,
        length: i32,
        which: *const ::libc::c_char,
    ) -> i32;
}
extern "C" {
    pub fn janet_getargindex(
        argv: *const Janet,
        n: i32,
        length: i32,
        which: *const ::libc::c_char,
    ) -> i32;
}
extern "C" {
    pub fn janet_getflags(
        argv: *const Janet,
        n: i32,
        flags: *const ::libc::c_char,
    ) -> u64;
}
extern "C" {
    pub fn janet_optnumber(argv: *const Janet, argc: i32, n: i32, dflt: f64) -> f64;
}
extern "C" {
    pub fn janet_opttuple(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: JanetTuple,
    ) -> JanetTuple;
}
extern "C" {
    pub fn janet_optstruct(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: JanetStruct,
    ) -> JanetStruct;
}
extern "C" {
    pub fn janet_optstring(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: JanetString,
    ) -> JanetString;
}
extern "C" {
    pub fn janet_optcstring(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: *const ::libc::c_char,
    ) -> *const ::libc::c_char;
}
extern "C" {
    pub fn janet_optsymbol(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: JanetString,
    ) -> JanetSymbol;
}
extern "C" {
    pub fn janet_optkeyword(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: JanetString,
    ) -> JanetKeyword;
}
extern "C" {
    pub fn janet_optfiber(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: *mut JanetFiber,
    ) -> *mut JanetFiber;
}
extern "C" {
    pub fn janet_optfunction(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: *mut JanetFunction,
    ) -> *mut JanetFunction;
}
extern "C" {
    pub fn janet_optcfunction(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: JanetCFunction,
    ) -> JanetCFunction;
}
extern "C" {
    pub fn janet_optboolean(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: ::libc::c_int,
    ) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_optpointer(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt: *mut ::libc::c_void,
    ) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_optnat(argv: *const Janet, argc: i32, n: i32, dflt: i32) -> i32;
}
extern "C" {
    pub fn janet_optinteger(argv: *const Janet, argc: i32, n: i32, dflt: i32) -> i32;
}
extern "C" {
    pub fn janet_optinteger64(argv: *const Janet, argc: i32, n: i32, dflt: i64) -> i64;
}
extern "C" {
    pub fn janet_optsize(argv: *const Janet, argc: i32, n: i32, dflt: usize) -> usize;
}
extern "C" {
    pub fn janet_optabstract(
        argv: *const Janet,
        argc: i32,
        n: i32,
        at: *const JanetAbstractType,
        dflt: JanetAbstract,
    ) -> JanetAbstract;
}
extern "C" {
    pub fn janet_optbuffer(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt_len: i32,
    ) -> *mut JanetBuffer;
}
extern "C" {
    pub fn janet_opttable(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt_len: i32,
    ) -> *mut JanetTable;
}
extern "C" {
    pub fn janet_optarray(
        argv: *const Janet,
        argc: i32,
        n: i32,
        dflt_len: i32,
    ) -> *mut JanetArray;
}
extern "C" {
    pub fn janet_dyn(name: *const ::libc::c_char) -> Janet;
}
extern "C" {
    pub fn janet_setdyn(name: *const ::libc::c_char, value: Janet);
}
extern "C" {
    pub static janet_file_type: JanetAbstractType;
}
extern "C" {
    pub fn janet_makefile(f: *mut FILE, flags: i32) -> Janet;
}
extern "C" {
    pub fn janet_makejfile(f: *mut FILE, flags: i32) -> *mut JanetFile;
}
extern "C" {
    pub fn janet_getfile(argv: *const Janet, n: i32, flags: *mut i32) -> *mut FILE;
}
extern "C" {
    pub fn janet_dynfile(name: *const ::libc::c_char, def: *mut FILE) -> *mut FILE;
}
extern "C" {
    pub fn janet_getjfile(argv: *const Janet, n: i32) -> *mut JanetFile;
}
extern "C" {
    pub fn janet_checkfile(j: Janet) -> JanetAbstract;
}
extern "C" {
    pub fn janet_unwrapfile(j: Janet, flags: *mut i32) -> *mut FILE;
}
extern "C" {
    pub fn janet_file_close(file: *mut JanetFile) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_cryptorand(out: *mut u8, n: usize) -> ::libc::c_int;
}
extern "C" {
    pub fn janet_marshal_size(ctx: *mut JanetMarshalContext, value: usize);
}
extern "C" {
    pub fn janet_marshal_int(ctx: *mut JanetMarshalContext, value: i32);
}
extern "C" {
    pub fn janet_marshal_int64(ctx: *mut JanetMarshalContext, value: i64);
}
extern "C" {
    pub fn janet_marshal_byte(ctx: *mut JanetMarshalContext, value: u8);
}
extern "C" {
    pub fn janet_marshal_bytes(
        ctx: *mut JanetMarshalContext,
        bytes: *const u8,
        len: usize,
    );
}
extern "C" {
    pub fn janet_marshal_janet(ctx: *mut JanetMarshalContext, x: Janet);
}
extern "C" {
    pub fn janet_marshal_abstract(
        ctx: *mut JanetMarshalContext,
        abstract_: JanetAbstract,
    );
}
extern "C" {
    pub fn janet_unmarshal_ensure(ctx: *mut JanetMarshalContext, size: usize);
}
extern "C" {
    pub fn janet_unmarshal_size(ctx: *mut JanetMarshalContext) -> usize;
}
extern "C" {
    pub fn janet_unmarshal_int(ctx: *mut JanetMarshalContext) -> i32;
}
extern "C" {
    pub fn janet_unmarshal_int64(ctx: *mut JanetMarshalContext) -> i64;
}
extern "C" {
    pub fn janet_unmarshal_byte(ctx: *mut JanetMarshalContext) -> u8;
}
extern "C" {
    pub fn janet_unmarshal_bytes(
        ctx: *mut JanetMarshalContext,
        dest: *mut u8,
        len: usize,
    );
}
extern "C" {
    pub fn janet_unmarshal_janet(ctx: *mut JanetMarshalContext) -> Janet;
}
extern "C" {
    pub fn janet_unmarshal_abstract(
        ctx: *mut JanetMarshalContext,
        size: usize,
    ) -> JanetAbstract;
}
extern "C" {
    pub fn janet_unmarshal_abstract_reuse(
        ctx: *mut JanetMarshalContext,
        p: *mut ::libc::c_void,
    );
}
extern "C" {
    pub fn janet_register_abstract_type(at: *const JanetAbstractType);
}
extern "C" {
    pub fn janet_get_abstract_type(key: Janet) -> *const JanetAbstractType;
}
extern "C" {
    pub static janet_peg_type: JanetAbstractType;
}
pub const JanetPegOpcod_RULE_LITERAL: JanetPegOpcod = 0;
pub const JanetPegOpcod_RULE_NCHAR: JanetPegOpcod = 1;
pub const JanetPegOpcod_RULE_NOTNCHAR: JanetPegOpcod = 2;
pub const JanetPegOpcod_RULE_RANGE: JanetPegOpcod = 3;
pub const JanetPegOpcod_RULE_SET: JanetPegOpcod = 4;
pub const JanetPegOpcod_RULE_LOOK: JanetPegOpcod = 5;
pub const JanetPegOpcod_RULE_CHOICE: JanetPegOpcod = 6;
pub const JanetPegOpcod_RULE_SEQUENCE: JanetPegOpcod = 7;
pub const JanetPegOpcod_RULE_IF: JanetPegOpcod = 8;
pub const JanetPegOpcod_RULE_IFNOT: JanetPegOpcod = 9;
pub const JanetPegOpcod_RULE_NOT: JanetPegOpcod = 10;
pub const JanetPegOpcod_RULE_BETWEEN: JanetPegOpcod = 11;
pub const JanetPegOpcod_RULE_GETTAG: JanetPegOpcod = 12;
pub const JanetPegOpcod_RULE_CAPTURE: JanetPegOpcod = 13;
pub const JanetPegOpcod_RULE_POSITION: JanetPegOpcod = 14;
pub const JanetPegOpcod_RULE_ARGUMENT: JanetPegOpcod = 15;
pub const JanetPegOpcod_RULE_CONSTANT: JanetPegOpcod = 16;
pub const JanetPegOpcod_RULE_ACCUMULATE: JanetPegOpcod = 17;
pub const JanetPegOpcod_RULE_GROUP: JanetPegOpcod = 18;
pub const JanetPegOpcod_RULE_REPLACE: JanetPegOpcod = 19;
pub const JanetPegOpcod_RULE_MATCHTIME: JanetPegOpcod = 20;
pub const JanetPegOpcod_RULE_ERROR: JanetPegOpcod = 21;
pub const JanetPegOpcod_RULE_DROP: JanetPegOpcod = 22;
pub const JanetPegOpcod_RULE_BACKMATCH: JanetPegOpcod = 23;
pub const JanetPegOpcod_RULE_TO: JanetPegOpcod = 24;
pub const JanetPegOpcod_RULE_THRU: JanetPegOpcod = 25;
pub const JanetPegOpcod_RULE_LENPREFIX: JanetPegOpcod = 26;
pub const JanetPegOpcod_RULE_READINT: JanetPegOpcod = 27;
pub const JanetPegOpcod_RULE_LINE: JanetPegOpcod = 28;
pub const JanetPegOpcod_RULE_COLUMN: JanetPegOpcod = 29;
pub const JanetPegOpcod_RULE_UNREF: JanetPegOpcod = 30;
pub const JanetPegOpcod_RULE_CAPTURE_NUM: JanetPegOpcod = 31;
pub type JanetPegOpcod = ::libc::c_uint;
#[repr(C)]
pub struct JanetPeg {
    pub bytecode: *mut u32,
    pub constants: *mut Janet,
    pub bytecode_len: usize,
    pub num_constants: u32,
    pub has_backref: ::libc::c_int,
}
#[automatically_derived]
impl ::core::fmt::Debug for JanetPeg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "JanetPeg",
            "bytecode",
            &&self.bytecode,
            "constants",
            &&self.constants,
            "bytecode_len",
            &&self.bytecode_len,
            "num_constants",
            &&self.num_constants,
            "has_backref",
            &&self.has_backref,
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for JanetPeg {}
#[automatically_derived]
impl ::core::clone::Clone for JanetPeg {
    #[inline]
    fn clone(&self) -> JanetPeg {
        let _: ::core::clone::AssertParamIsClone<*mut u32>;
        let _: ::core::clone::AssertParamIsClone<*mut Janet>;
        let _: ::core::clone::AssertParamIsClone<usize>;
        let _: ::core::clone::AssertParamIsClone<u32>;
        let _: ::core::clone::AssertParamIsClone<::libc::c_int>;
        *self
    }
}
extern "C" {
    pub static janet_s64_type: JanetAbstractType;
}
extern "C" {
    pub static janet_u64_type: JanetAbstractType;
}
pub const JanetIntType_JANET_INT_NONE: JanetIntType = 0;
pub const JanetIntType_JANET_INT_S64: JanetIntType = 1;
pub const JanetIntType_JANET_INT_U64: JanetIntType = 2;
pub type JanetIntType = ::libc::c_uint;
extern "C" {
    pub fn janet_is_int(x: Janet) -> JanetIntType;
}
extern "C" {
    pub fn janet_wrap_s64(x: i64) -> Janet;
}
extern "C" {
    pub fn janet_wrap_u64(x: u64) -> Janet;
}
extern "C" {
    pub fn janet_unwrap_s64(x: Janet) -> i64;
}
extern "C" {
    pub fn janet_unwrap_u64(x: Janet) -> u64;
}
extern "C" {
    pub fn janet_malloc(arg1: usize) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_realloc(arg1: *mut ::libc::c_void, arg2: usize) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_calloc(arg1: usize, arg2: usize) -> *mut ::libc::c_void;
}
extern "C" {
    pub fn janet_free(arg1: *mut ::libc::c_void);
}
pub type __builtin_va_list = *mut ::libc::c_char;
impl fmt::Debug for Janet {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(core::any::type_name::<Self>())
    }
}
impl PartialEq<Janet> for Janet {
    #[inline]
    fn eq(&self, other: &Janet) -> bool {
        unsafe { janet_equals(*self, *other) != 0 }
    }
}
impl Eq for Janet {}
impl Hash for Janet {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(unsafe { janet_hash(*self) })
    }
}
impl PartialOrd<Janet> for Janet {
    #[inline]
    fn partial_cmp(&self, other: &Janet) -> Option<Ordering> {
        let res = unsafe { janet_compare(*self, *other) };
        Some(
            match res {
                -1 => Ordering::Less,
                0 => Ordering::Equal,
                1 => Ordering::Greater,
                _ => return None,
            },
        )
    }
}
impl Ord for Janet {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let res = unsafe { janet_compare(*self, *other) };
        match res {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => unsafe { core::hint::unreachable_unchecked() }
        }
    }
}
