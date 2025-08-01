use std::{
    ffi::{c_int, c_void, CStr, CString},
    mem,
    ptr::{self, NonNull},
};

use hamlib_sys::{
    rig_cleanup, rig_get_conf2, rig_get_dcd, rig_get_freq, rig_get_mode, rig_get_ptt, rig_get_vfo,
    rig_init, rig_open, rig_set_conf, rig_set_freq, rig_set_freq_callback, rig_set_mode,
    rig_set_ptt, rig_set_vfo, rig_token_lookup, s_rig,
};

use crate::{
    error::{RigResult, RigResultExt},
    lock::Hamlib,
    types::{DCD, PTT, VFO},
};

pub struct Rig {
    ptr: NonNull<s_rig>,
}

impl Rig {
    /// This assumes you dont want to get the model through `rig_probe()` or
    /// something, but I have doubts about that function, as do the Hamlib authors.
    pub fn new(_lib: &Hamlib, model: u32) -> Option<Self> {
        let rig = unsafe { rig_init(model) };
        Some(Self {
            ptr: NonNull::new(rig)?,
        })
    }

    /// Returns a new `Rig` object through a raw pointer from Hamlib.
    pub unsafe fn from_raw(ptr: *mut s_rig) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Rig ptr should not be null"),
        }
    }

    pub fn open(&mut self, _lib: &Hamlib) -> RigResult<()> {
        RigResult::from_code(unsafe { rig_open(self.ptr.as_ptr()) })
    }

    pub fn set_conf(
        &mut self,
        _lib: &Hamlib,
        tok: i64,
        value: impl Into<CString>,
    ) -> RigResult<()> {
        let s: CString = value.into();
        RigResult::from_code(unsafe { rig_set_conf(self.ptr.as_ptr(), tok, s.as_ptr()) })
    }

    pub fn get_conf(&self, _lib: &Hamlib, tok: i64) -> RigResult<String> {
        let mut buf_len = 64;
        let mut s: Vec<u8> = vec![0; buf_len];
        loop {
            RigResult::from_code(unsafe {
                rig_get_conf2(
                    self.ptr.as_ptr(),
                    tok,
                    s.as_mut_ptr().cast(),
                    buf_len as i32,
                )
            })?;
            if CStr::from_bytes_with_nul(&s).is_ok() {
                break;
            }
            buf_len = buf_len * 2;
            s.resize(buf_len, 0);
        }
        Ok(unsafe { CString::from_vec_unchecked(s).to_string_lossy().to_string() })
    }

    pub fn token_lookup(&self, _lib: &Hamlib, value: impl Into<CString>) -> i64 {
        let s: CString = value.into();
        unsafe { rig_token_lookup(self.ptr.as_ptr(), s.as_ptr()) }
    }

    pub fn set_freq(&mut self, _lib: &Hamlib, vfo: VFO, freq: f64) -> RigResult<()> {
        RigResult::from_code(unsafe { rig_set_freq(self.ptr.as_ptr(), vfo.bits(), freq) })
    }

    pub fn get_freq(&self, _lib: &Hamlib, vfo: VFO) -> RigResult<f64> {
        let mut freq = 0.0;
        RigResult::from_code(unsafe {
            rig_get_freq(self.ptr.as_ptr(), vfo.bits(), ptr::from_mut(&mut freq))
        })?;
        Ok(freq)
    }

    /// Entirely untested and 100% unsound, because my rig does not support this anyways.
    /// The raw callback recieves a pointer to the rig that the freq change event originated from,
    /// but I feel like this should not be given back to avoid multiple objects of the same pointer?
    pub fn set_freq_callback(&mut self, _lib: &Hamlib, closure: impl FnMut(VFO, f64) + Send + 'static) -> RigResult<()>
    {
        type BoxType = Box<dyn FnMut(VFO, f64) + Send>;
        let inner: BoxType = Box::new(closure);
        let outer = Box::into_raw(Box::new(inner));
        unsafe extern "C" fn raw_callback(
            _rig: *mut hamlib_sys::RIG,
            vfo: hamlib_sys::vfo_t,
            freq: hamlib_sys::freq_t,
            ptr: *mut c_void,
        ) -> c_int {
            let outer = unsafe { Box::from_raw(ptr as *mut BoxType) };
            let mut inner = *outer;
            inner(VFO::from_bits_retain(vfo), freq);
            0
        }
        RigResult::from_code(unsafe {
            rig_set_freq_callback(self.ptr.as_ptr(), Some(raw_callback), outer as *mut std::ffi::c_void)
        })
    }

    /// Sets the mode on the given VFO, with mode being a sys::RIG_MODE_*,
    /// and width being represented in Hz.
    pub fn set_mode(&mut self, _lib: &Hamlib, vfo: VFO, mode: u32, width: i64) -> RigResult<()> {
        RigResult::from_code(unsafe {
            rig_set_mode(self.ptr.as_ptr(), vfo.bits(), mode.into(), width)
        })
    }

    /// Returns a tuple representing the current mode (`sys::RIG_MODE_*`), and width in Hz.
    pub fn get_mode(&self, _lib: &Hamlib, vfo: VFO) -> RigResult<(u64, i64)> {
        let mut mode = 0;
        let mut width = 0;
        RigResult::from_code(unsafe {
            rig_get_mode(
                self.ptr.as_ptr(),
                vfo.bits(),
                ptr::from_mut(&mut mode),
                ptr::from_mut(&mut width),
            )
        })?;
        Ok((mode, width))
    }

    pub fn set_vfo(&mut self, _lib: &Hamlib, vfo: VFO) -> RigResult<()> {
        unsafe { mem::transmute(rig_set_vfo(self.ptr.as_ptr(), vfo.bits())) }
    }

    pub fn get_vfo(&self, _lib: &Hamlib) -> RigResult<VFO> {
        let mut vfo = 0;
        RigResult::from_code(unsafe { rig_get_vfo(self.ptr.as_ptr(), ptr::from_mut(&mut vfo)) })?;
        Ok(VFO::from_bits_retain(vfo))
    }

    pub fn set_ptt(&mut self, _lib: &Hamlib, vfo: VFO, ptt: PTT) -> RigResult<()> {
        RigResult::from_code(unsafe { rig_set_ptt(self.ptr.as_ptr(), vfo.bits(), ptt.bits()) })
    }

    pub fn get_ptt(&self, _lib: &Hamlib, vfo: VFO) -> RigResult<PTT> {
        let mut ptt = 0;
        RigResult::from_code(unsafe {
            rig_get_ptt(self.ptr.as_ptr(), vfo.bits(), ptr::from_mut(&mut ptt))
        })?;
        Ok(PTT::from_bits_retain(ptt))
    }

    pub fn get_dcd(&self, _lib: &Hamlib, vfo: VFO) -> RigResult<DCD> {
        let mut dcd = 0;
        RigResult::from_code(unsafe {
            rig_get_dcd(self.ptr.as_ptr(), vfo.bits(), ptr::from_mut(&mut dcd))
        })?;
        Ok(DCD::from_bits_retain(dcd))
    }
}

impl Drop for Rig {
    fn drop(&mut self) {
        unsafe {
            rig_cleanup(self.ptr.as_ptr());
        }
    }
}
