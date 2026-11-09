extern crate alloc;

use core::alloc::Layout;
use kernel::{
    SomeDataStruct,
    ResultPointer,
};

#[no_mangle]
pub unsafe extern "C" fn ns_malloc(
    size: u32, alignment: u32,
) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::alloc(layout)
}
