use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

// Opaque pointer for the Factory context
pub type FactoryContext = *mut c_void;

// Function signatures for the ABI-compatible FactoryBlock
pub type FactoryBlockGetName = extern "C" fn(block_ptr: *mut c_void) -> *const c_char;
pub type FactoryBlockGetCost = extern "C" fn(block_ptr: *mut c_void) -> u32;
pub type FactoryBlockExecute = extern "C" fn(
    block_ptr: *mut c_void,
    factory_ctx: FactoryContext,
    crate_path: *const c_char,
) -> bool; // bool for success/failure

// A struct to hold the function pointers for a dynamically loaded FactoryBlock
// This is what rusttycoon will receive and store.
#[repr(C)]
pub struct AbiFactoryBlock {
    pub block_ptr: *mut c_void, // Pointer to the actual Rust FactoryBlock instance
    pub get_name: FactoryBlockGetName,
    pub get_cost: FactoryBlockGetCost,
    pub execute: FactoryBlockExecute,
}

// Helper functions for blocks to create C-strings from Rust strings
pub fn to_c_string(s: &str) -> *const c_char {
    CString::new(s).expect("CString::new failed").into_raw()
}

// Helper functions for blocks to convert C-strings to Rust strings (for internal use)
pub fn from_c_string<'a>(ptr: *const c_char) -> &'a str {
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .expect("CStr::to_str failed")
}
