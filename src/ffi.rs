use super::libc::c_char;
use super::libc::c_int;
use std::ffi::CStr;

/// Execute a dice roll based on the given notation
#[no_mangle]
pub unsafe extern "C" fn roll_dice(notation: *const c_char) -> c_int
{
    let notation = CStr::from_ptr(notation);

    let result = super::roll_dice::<i32>(notation.to_str().unwrap());

    let result = result.unwrap_or(-1);
    result
}