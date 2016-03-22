//! TIScript Virtual Machine Runtime.

#![allow(non_camel_case_types, non_snake_case)]

use sctypes::{LPVOID, UINT64};

MAKE_HANDLE!(HVM, _HVM);
pub type tiscript_value = UINT64;

#[repr(C)]
pub struct tiscript_native_interface
{
	create_vm: LPVOID,
}
