use hamlib_sys::vfo_t;

const RIG_VFO_A: vfo_t = rig_vfo_n(0);
const RIG_VFO_B: vfo_t = rig_vfo_n(1);
const RIG_VFO_C: vfo_t = rig_vfo_n(2);
const RIG_VFO_CURR: vfo_t = rig_vfo_n(29);

pub const fn rig_vfo_n(n: i32) -> vfo_t {
    1 << n
}

#[cfg(test)]
mod tests {
    use std::{mem::MaybeUninit, ptr, slice::from_raw_parts};

    use hamlib_sys::{hamlib_port, rig_debug_level_e_RIG_DEBUG_NONE, rig_errcode_e_RIG_OK, rig_get_freq, rig_get_mode, rig_init, rig_load_all_backends, rig_open, rig_port_e_RIG_PORT_SERIAL, rig_probe, rig_set_debug, rig_set_freq, rig_set_vfo, rig_type_t_RIG_FLAG_RECEIVER, rig_type_t_RIG_FLAG_TRANSMITTER, serial_handshake_e_RIG_HANDSHAKE_NONE, serial_parity_e_RIG_PARITY_NONE, RIG, RIG_MODEL_NONE, RIG_VFO_NONE};

    use crate::{rig_vfo_n, RIG_VFO_B, RIG_VFO_CURR};

    #[test]
    fn sandbox() {
        unsafe {
            rig_set_debug(rig_debug_level_e_RIG_DEBUG_NONE);
            let mut myport = MaybeUninit::<hamlib_port>::zeroed().assume_init();
            
            myport.type_.rig = rig_type_t_RIG_FLAG_RECEIVER | rig_type_t_RIG_FLAG_TRANSMITTER;
            myport.parm.serial.rate = 19200;
            myport.parm.serial.data_bits = 8;
            myport.parm.serial.stop_bits = 1;
            myport.parm.serial.parity = serial_parity_e_RIG_PARITY_NONE;
            myport.parm.serial.handshake = serial_handshake_e_RIG_HANDSHAKE_NONE;
            
            let port_cstr = c"/dev/serial/by-id/usb-Silicon_Labs_CP2102_USB_to_UART_Bridge_Controller_IC-7200_0202084-if00-port0";
            let port_len = port_cstr.count_bytes().min(myport.pathname.len());
            myport.pathname[..port_len].copy_from_slice(from_raw_parts(port_cstr.as_ptr(), port_len));
            rig_load_all_backends();
            //let myrig_model = rig_probe(std::ptr::from_mut(&mut myport));
            // https://github.com/Hamlib/Hamlib/blob/master/include/hamlib/riglist.h#L320
            let myrig_model = 3061;
            let my_rig = rig_init(myrig_model);

            if my_rig.is_null() {
                panic!("um")
            }

            let mut retcode = rig_open(my_rig);

            if retcode != rig_errcode_e_RIG_OK as i32 {
                panic!("{}", retcode)
            }

            retcode = rig_set_vfo(my_rig, RIG_VFO_B);

            if retcode != rig_errcode_e_RIG_OK as i32 {
                panic!("um")
            }

            retcode = rig_set_freq(my_rig, rig_vfo_n(29), 21235175.0);

            if retcode != rig_errcode_e_RIG_OK as i32 {
                panic!("um")
            }

            let mut freq = 0.0;

            rig_get_freq(my_rig, RIG_VFO_CURR, ptr::from_mut(&mut freq));
            dbg!(freq);
        }
    }
}
