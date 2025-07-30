pub mod token;

#[cfg(test)]
mod tests {
    use std::ptr;

    use hamlib_sys::{rig_cleanup, rig_debug_level_e_RIG_DEBUG_NONE, rig_errcode_e_RIG_OK, rig_get_freq, rig_init, rig_load_all_backends, rig_open, rig_set_conf, rig_set_debug, rig_set_freq, rig_set_vfo, RIG_MODEL_IC7200, RIG_VFO_B, RIG_VFO_CURR};

    use crate::token::TOK_PATHNAME;

    #[test]
    fn sandbox() {
        unsafe {
            rig_set_debug(rig_debug_level_e_RIG_DEBUG_NONE);
            rig_load_all_backends();

            let myrig_model = RIG_MODEL_IC7200;
            let my_rig = rig_init(myrig_model);

            if my_rig.is_null() {
                panic!("um")
            }

            let port_cstr = c"/dev/serial/by-id/usb-Silicon_Labs_CP2102_USB_to_UART_Bridge_Controller_IC-7200_0202084-if00-port0";
            rig_set_conf(my_rig, TOK_PATHNAME, port_cstr.as_ptr());

            let mut retcode = rig_open(my_rig);

            if retcode != rig_errcode_e_RIG_OK as i32 {
                panic!("{}", retcode)
            }

            retcode = rig_set_vfo(my_rig, RIG_VFO_B);

            if retcode != rig_errcode_e_RIG_OK as i32 {
                panic!("um")
            }

            retcode = rig_set_freq(my_rig, RIG_VFO_CURR, 21235175.0);

            if retcode != rig_errcode_e_RIG_OK as i32 {
                panic!("um")
            }

            let mut freq = 0.0;

            rig_get_freq(my_rig, RIG_VFO_CURR, ptr::from_mut(&mut freq));
            dbg!(freq);
            rig_cleanup(my_rig);
        }
    }
}
