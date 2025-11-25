use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

#[link(name = "monster_ffi")]
extern "C" {
    fn monster_verify_element(element: u64) -> bool;
    fn monster_verify_hecke(value: i32) -> bool;
    fn monster_verify_modular(elements: *const u64, count: usize) -> bool;
    fn monster_create_trait(name: *const c_char, element: u64, hecke: i32) -> *mut c_void;
    fn monster_free_trait(trait_ptr: *mut c_void);
}

pub struct MonsterTrait {
    ptr: *mut c_void,
}

impl MonsterTrait {
    pub fn new(name: &str, element: u64, hecke: i32) -> Result<Self, Box<dyn std::error::Error>> {
        let c_name = CString::new(name)?;
        let ptr = unsafe { monster_create_trait(c_name.as_ptr(), element, hecke) };
        
        if ptr.is_null() {
            return Err("Failed to create Monster trait".into());
        }
        
        Ok(MonsterTrait { ptr })
    }
}

impl Drop for MonsterTrait {
    fn drop(&mut self) {
        unsafe { monster_free_trait(self.ptr) };
    }
}

pub fn verify_monster_element(element: u64) -> bool {
    unsafe { monster_verify_element(element) }
}

pub fn verify_hecke_eigenvalue(value: i32) -> bool {
    unsafe { monster_verify_hecke(value) }
}

pub fn verify_modular_constraint(elements: &[u64]) -> bool {
    unsafe { monster_verify_modular(elements.as_ptr(), elements.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monster_verification() {
        assert!(verify_monster_element(196882));
        assert!(!verify_monster_element(196883));
        
        assert!(verify_hecke_eigenvalue(196883));
        assert!(verify_hecke_eigenvalue(-5472));
        assert!(!verify_hecke_eigenvalue(0));
        
        let elements = vec![24, 48, 72];
        assert!(verify_modular_constraint(&elements));
    }
}
