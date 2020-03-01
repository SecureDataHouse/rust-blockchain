use std::mem;
use std::intrinsics::transmute;

/*pub fn i64Tou8(v: i64) -> &'static [u8] {
    unsafe {
        let i64Ptr: *const i64 = &v as *const i64;
        let i8Ptr: *const i8 = i64Ptr as *const i8;
        return i8Ptr as *const u8;
    }
}*/
