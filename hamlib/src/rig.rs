use std::{
    ffi::CString,
    mem,
    ptr::{self, NonNull},
};

use hamlib_sys::{
    rig_cleanup, rig_get_freq, rig_init, rig_open, rig_set_conf, rig_set_freq, rig_set_vfo, s_rig,
};

use crate::{
    error::{RigResult, RigResultExt},
    vfo::VFO,
};

pub struct Rig {
    ptr: NonNull<s_rig>,
}

impl Rig {
    pub fn new(model: u32) -> Option<Self> {
        let rig = unsafe { rig_init(model) };
        Some(Self {
            ptr: NonNull::new(rig)?,
        })
    }

    pub unsafe fn from_raw(ptr: *mut s_rig) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Rig ptr should not be null"),
        }
    }

    pub fn open(&mut self) -> RigResult<()> {
        RigResult::from_code(unsafe { rig_open(self.ptr.as_ptr()) })
    }

    pub fn set_conf(&mut self, tok: i64, value: impl Into<CString>) -> RigResult<()> {
        let s: CString = value.into();
        RigResult::from_code(unsafe { rig_set_conf(self.ptr.as_ptr(), tok, s.as_ptr()) })
    }

    pub fn set_vfo(&mut self, vfo: VFO) -> RigResult<()> {
        unsafe { mem::transmute(rig_set_vfo(self.ptr.as_ptr(), vfo.bits())) }
    }

    pub fn set_freq(&mut self, vfo: VFO, freq: f64) -> RigResult<()> {
        RigResult::from_code(unsafe { rig_set_freq(self.ptr.as_ptr(), vfo.bits(), freq) })
    }

    pub fn get_freq(&self, vfo: VFO) -> RigResult<f64> {
        let mut freq = 0.0;
        RigResult::from_code(unsafe {
            rig_get_freq(self.ptr.as_ptr(), vfo.bits(), ptr::from_mut(&mut freq))
        })?;
        Ok(freq)
    }
}

impl Drop for Rig {
    fn drop(&mut self) {
        unsafe {
            rig_cleanup(self.ptr.as_ptr());
        }
    }
}
