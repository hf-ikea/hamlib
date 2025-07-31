#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

unsafe extern "C" {
    pub static frontend_cfg_params: *const confparams;
}
