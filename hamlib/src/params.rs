use core::slice;
use std::{ptr, sync::OnceLock};

use hamlib_sys::{confparams, frontend_cfg_params, RIG_COMBO_MAX};

use crate::lock::Hamlib;

pub static FRONTEND_CFG_PARAMS: OnceLock<&'static [ConfParams]> = OnceLock::new();

pub fn init_params(_lib: Hamlib) {
    let mut len = 0;
    unsafe {
        loop {
            let read = ptr::read(& (*frontend_cfg_params.add(size_of::<confparams>() * len)).token);
            if read == 0 { break };
            len += 1;
        }
    }
    FRONTEND_CFG_PARAMS.get_or_init(|| {
        unsafe {
            slice::from_raw_parts(frontend_cfg_params.cast(), len)
        }
    });
}

#[repr(C)]
pub struct ConfParams {
    #[doc = "< Conf param token ID"]
    pub token: u32,
    #[doc = "< Param name, no spaces allowed"]
    pub name: String,
    #[doc = "< Human readable label"]
    pub label: String,
    #[doc = "< Hint on the parameter"]
    pub tooltip: String,
    #[doc = "< Default value"]
    pub dflt: String,
    #[doc = "< Type of the parameter"]
    pub type_: RigParamType,
    #[doc = "< Type union"]
    pub u: ConfParamsUnion,
}

#[repr(C)]
pub union ConfParamsUnion {
    pub n: Numeric,
    pub c: std::mem::ManuallyDrop<[String; RIG_COMBO_MAX as usize]>
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Numeric {
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

#[repr(C)]
pub enum RigParamType {
    String,
    Combo,
    Numeric,
    Checkbutton,
    Button,
    Binary,
    Integer
}
