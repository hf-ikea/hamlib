use core::slice;
use std::{
    ffi::CStr,
    mem::{self},
    ptr,
    sync::OnceLock,
};

use hamlib_sys::{RIG_COMBO_MAX, confparams, frontend_cfg_params};

use crate::lock::Hamlib;

pub static FRONTEND_CFG_PARAMS: OnceLock<Vec<ConfParams>> = OnceLock::new();

pub fn init_params(_lib: &Hamlib) {
    let mut len = 0;
    unsafe {
        loop {
            let read = ptr::read(&(*frontend_cfg_params.add(size_of::<confparams>() * len)).token);
            if read == 0 {
                break;
            };
            len += 1;
        }
    }
    FRONTEND_CFG_PARAMS.get_or_init(|| unsafe {
        slice::from_raw_parts(frontend_cfg_params, len)
            .iter()
            .map(|m| ConfParams::from_raw(m))
            .collect()
    });
}

#[repr(C)]
pub struct ConfParams {
    #[doc = "Conf param token ID"]
    pub token: i64,
    #[doc = "Param name, no spaces allowed"]
    pub name: String,
    #[doc = "Human readable label"]
    pub label: String,
    #[doc = "Hint on the parameter"]
    pub tooltip: String,
    #[doc = "Default value"]
    pub dflt: String,
    #[doc = "Type of the parameter"]
    pub type_: RigParamType,

    pub numeric: Option<Numeric>,
    pub integer: Option<Integer>,
    pub combo: Option<Vec<String>>,
}

impl ConfParams {
    pub fn from_raw(raw: &confparams) -> Self {
        let params = *raw;
        let token = params.token;
        let name = unsafe { CStr::from_ptr(params.name) }
            .to_string_lossy()
            .to_string();
        let label = unsafe { CStr::from_ptr(params.label) }
            .to_string_lossy()
            .to_string();
        let tooltip = unsafe { CStr::from_ptr(params.tooltip) }
            .to_string_lossy()
            .to_string();
        let dflt = unsafe { CStr::from_ptr(params.dflt) }
            .to_string_lossy()
            .to_string();
        let type_ = unsafe { mem::transmute(params.type_) };
        let mut numeric = None;
        let mut integer = None;
        let mut combo = None;
        match type_ {
            RigParamType::Combo => {
                let mut c = Vec::with_capacity(RIG_COMBO_MAX as usize);
                for ptr in unsafe { params.u.c.combostr } {
                    c.push(unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string());
                }
                combo = Some(c)
            }
            RigParamType::Numeric => numeric = Some(unsafe { mem::transmute(params.u.n) }),
            RigParamType::Checkbutton => todo!(),
            RigParamType::Integer => integer = Some(unsafe { mem::transmute(params.u.n) }),
            _ => {}
        }
        ConfParams {
            token,
            name,
            label,
            tooltip,
            dflt,
            type_,
            numeric,
            integer,
            combo,
        }
    }
}

#[repr(C)]
pub enum RigParamType {
    String,
    Combo,
    Numeric,
    Checkbutton,
    Button,
    Binary,
    Integer,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Numeric {
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Integer {
    pub min: i32,
    pub max: i32,
    pub step: i32,
}
