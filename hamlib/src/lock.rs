use std::sync::{
    Mutex, MutexGuard, OnceLock,
    atomic::{AtomicBool, Ordering},
};

use hamlib_sys::{
    amp_load_all_backends, rig_load_all_backends, rig_set_debug, rig_set_debug_time_stamp,
    rot_load_all_backends,
};

use crate::{
    LogLevel,
    error::{RigResult, RigResultExt},
};

static HAMLIB_INIT: AtomicBool = AtomicBool::new(false);

/// ZST marker object, bootleg Mutex
pub struct Hamlib;
impl Hamlib {
    pub fn new() -> Option<Self> {
        if HAMLIB_INIT
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            Some(Hamlib)
        } else {
            None
        }
    }
    /// This function /must/ be called, but only once
    pub unsafe fn init_hamlib() {
        HAMLIB_STATE.get_or_init(|| {
            Mutex::new(State {
                rig_loaded: false,
                rot_loaded: false,
                amp_loaded: false,
            })
        });
    }
}

impl Drop for Hamlib {
    fn drop(&mut self) {
        HAMLIB_INIT.swap(false, Ordering::SeqCst);
    }
}

static HAMLIB_STATE: OnceLock<Mutex<State>> = OnceLock::new();

struct State {
    rig_loaded: bool,
    rot_loaded: bool,
    amp_loaded: bool,
}

fn hamlib_state_lock() -> MutexGuard<'static, State> {
    HAMLIB_STATE
        .get()
        .expect("HAMLIB static not initalized. See lock::init_hamlib() for details")
        .lock()
        .expect("Could not lock HAMLIB Mutex")
}

pub fn load_rig_backends(_lib: &Hamlib) -> RigResult<()> {
    let mut l = hamlib_state_lock();
    if l.rig_loaded {
        return Ok(());
    }
    match RigResult::from_code(unsafe { rig_load_all_backends() }) {
        Ok(_) => {
            let _ = std::mem::replace(&mut l.rig_loaded, true);
            Ok(())
        }
        Err(v) => return Err(v),
    }
}

pub fn load_rot_backends(_lib: &Hamlib) -> RigResult<()> {
    let mut l = hamlib_state_lock();
    if l.rot_loaded {
        return Ok(());
    }
    match RigResult::from_code(unsafe { rot_load_all_backends() }) {
        Ok(_) => {
            let _ = std::mem::replace(&mut l.rot_loaded, true);
            Ok(())
        }
        Err(v) => return Err(v),
    }
}

pub fn load_amp_backends(_lib: &Hamlib) -> RigResult<()> {
    let mut l = hamlib_state_lock();
    if l.amp_loaded {
        return Ok(());
    }
    match RigResult::from_code(unsafe { amp_load_all_backends() }) {
        Ok(_) => {
            let _ = std::mem::replace(&mut l.amp_loaded, true);
            Ok(())
        }
        Err(v) => return Err(v),
    }
}

pub fn set_log_level(_lib: &Hamlib, level: LogLevel) {
    let _l = hamlib_state_lock();
    unsafe { rig_set_debug(level as u32) }
}

/// Enables or disables timestamps in debug messages
pub fn set_log_timestamps(_lib: &Hamlib, yes: bool) {
    let _l = hamlib_state_lock();
    unsafe { rig_set_debug_time_stamp(yes as i32) }
}
