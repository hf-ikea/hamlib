use bitflags::bitflags;
use hamlib_sys::{
    RIG_VFO_A, RIG_VFO_ALL, RIG_VFO_B, RIG_VFO_CURR, RIG_VFO_MAIN, RIG_VFO_MAIN_A, RIG_VFO_MAIN_B,
    RIG_VFO_MAIN_C, RIG_VFO_MEM, RIG_VFO_NONE, RIG_VFO_OTHER, RIG_VFO_RX, RIG_VFO_SUB,
    RIG_VFO_SUB_A, RIG_VFO_SUB_B, RIG_VFO_SUB_C, RIG_VFO_TX, RIG_VFO_TX_FLAG, RIG_VFO_VFO,
    dcd_e_RIG_DCD_OFF, dcd_e_RIG_DCD_ON, ptt_type_t_RIG_PTT_CM108, ptt_type_t_RIG_PTT_GPIO,
    ptt_type_t_RIG_PTT_GPION, ptt_type_t_RIG_PTT_NONE, ptt_type_t_RIG_PTT_PARALLEL,
    ptt_type_t_RIG_PTT_RIG, ptt_type_t_RIG_PTT_RIG_MICDATA, ptt_type_t_RIG_PTT_SERIAL_DTR,
    ptt_type_t_RIG_PTT_SERIAL_RTS,
};

bitflags! {
#[derive(Debug)]
pub struct VFO: u32 {
    const RIG_VFO_NONE = RIG_VFO_NONE;
    const RIG_VFO_A = RIG_VFO_A;
    const RIG_VFO_B = RIG_VFO_B;
    const RIG_VFO_C = RIG_VFO_A;
    const RIG_VFO_SUB_A = RIG_VFO_SUB_A;
    const RIG_VFO_SUB_B = RIG_VFO_SUB_B;
    const RIG_VFO_SUB_C = RIG_VFO_SUB_C;
    const RIG_VFO_MAIN_A = RIG_VFO_MAIN_A;
    const RIG_VFO_MAIN_B = RIG_VFO_MAIN_B;
    const RIG_VFO_MAIN_C = RIG_VFO_MAIN_C;
    const RIG_VFO_OTHER = RIG_VFO_OTHER;
    const RIG_VFO_SUB = RIG_VFO_SUB;
    const RIG_VFO_MAIN = RIG_VFO_MAIN;
    const RIG_VFO_VFO = RIG_VFO_VFO;
    const RIG_VFO_MEM = RIG_VFO_MEM;
    const RIG_VFO_CURR = RIG_VFO_CURR;
    const RIG_VFO_TX_FLAG = RIG_VFO_TX_FLAG;
    const RIG_VFO_ALL = RIG_VFO_ALL;
    const RIG_VFO_TX = RIG_VFO_TX;
    const RIG_VFO_RX = RIG_VFO_RX;
}
}

bitflags! {
#[derive(Debug)]
pub struct PTT: u32 {
    const RIG_PTT_NONE = ptt_type_t_RIG_PTT_NONE;
    const RIG_PTT_RIG = ptt_type_t_RIG_PTT_RIG;
    const RIG_PTT_SERIAL_DTR = ptt_type_t_RIG_PTT_SERIAL_DTR;
    const RIG_PTT_SERIAL_RTS = ptt_type_t_RIG_PTT_SERIAL_RTS;
    const RIG_PTT_PARALLEL = ptt_type_t_RIG_PTT_PARALLEL;
    const RIG_PTT_RIG_MICDATA = ptt_type_t_RIG_PTT_RIG_MICDATA;
    const RIG_PTT_CM108 = ptt_type_t_RIG_PTT_CM108;
    const RIG_PTT_GPIO = ptt_type_t_RIG_PTT_GPIO;
    const RIG_PTT_GPION = ptt_type_t_RIG_PTT_GPION;
}
}

bitflags! {
pub struct DCD: u32 {
    const RIG_DCD_OFF = dcd_e_RIG_DCD_OFF;
    const RIG_DCD_ON = dcd_e_RIG_DCD_ON;
}
}

bitflags! {
pub struct Setting: u64 {
    const RIG_FUNC_NONE = hamlib_sys::RIG_FUNC_NONE as u64;
    const RIG_FUNC_FAGC = hamlib_sys::RIG_FUNC_FAGC as u64;
    const RIG_FUNC_NB = hamlib_sys::RIG_FUNC_NB as u64;
    const RIG_FUNC_COMP = hamlib_sys::RIG_FUNC_COMP as u64;
    const RIG_FUNC_VOX = hamlib_sys::RIG_FUNC_VOX as u64;
    const RIG_FUNC_TONE = hamlib_sys::RIG_FUNC_TONE as u64;
    const RIG_FUNC_TSQL = hamlib_sys::RIG_FUNC_TSQL as u64;
    const RIG_FUNC_SBKIN = hamlib_sys::RIG_FUNC_SBKIN as u64;
    const RIG_FUNC_FBKIN = hamlib_sys::RIG_FUNC_FBKIN as u64;
    const RIG_FUNC_ANF = hamlib_sys::RIG_FUNC_ANF as u64;
    const RIG_FUNC_NR = hamlib_sys::RIG_FUNC_NR as u64;
    const RIG_FUNC_AIP = hamlib_sys::RIG_FUNC_AIP as u64;
    const RIG_FUNC_APF = hamlib_sys::RIG_FUNC_APF as u64;
    const RIG_FUNC_MON = hamlib_sys::RIG_FUNC_MON as u64;
    const RIG_FUNC_MN = hamlib_sys::RIG_FUNC_MN as u64;
    const RIG_FUNC_RF = hamlib_sys::RIG_FUNC_RF as u64;
    const RIG_FUNC_ARO = hamlib_sys::RIG_FUNC_ARO as u64;
    const RIG_FUNC_LOCK = hamlib_sys::RIG_FUNC_LOCK as u64;
    const RIG_FUNC_MUTE = hamlib_sys::RIG_FUNC_MUTE as u64;
    const RIG_FUNC_VSC = hamlib_sys::RIG_FUNC_VSC as u64;
    const RIG_FUNC_REV = hamlib_sys::RIG_FUNC_REV as u64;
    const RIG_FUNC_SQL = hamlib_sys::RIG_FUNC_SQL as u64;
    const RIG_FUNC_ABM = hamlib_sys::RIG_FUNC_ABM as u64;
    const RIG_FUNC_BC = hamlib_sys::RIG_FUNC_BC as u64;
    const RIG_FUNC_MBC = hamlib_sys::RIG_FUNC_MBC as u64;
    const RIG_FUNC_RIT = hamlib_sys::RIG_FUNC_RIT as u64;
    const RIG_FUNC_AFC = hamlib_sys::RIG_FUNC_AFC as u64;
    const RIG_FUNC_SATMODE = hamlib_sys::RIG_FUNC_SATMODE as u64;
    const RIG_FUNC_SCOPE = hamlib_sys::RIG_FUNC_SCOPE as u64;
    const RIG_FUNC_RESUME = hamlib_sys::RIG_FUNC_RESUME as u64;
    const RIG_FUNC_TBURST = hamlib_sys::RIG_FUNC_TBURST as u64;
    const RIG_FUNC_TUNER = hamlib_sys::RIG_FUNC_TUNER as u64;
    const RIG_FUNC_XIT = hamlib_sys::RIG_FUNC_XIT as u64;
    const RIG_FUNC_NB2 = hamlib_sys::RIG_FUNC_NB2;
    const RIG_FUNC_CSQL = hamlib_sys::RIG_FUNC_CSQL;
    const RIG_FUNC_AFLT = hamlib_sys::RIG_FUNC_AFLT;
    const RIG_FUNC_ANL = hamlib_sys::RIG_FUNC_ANL;
    const RIG_FUNC_BC2 = hamlib_sys::RIG_FUNC_BC2;
    const RIG_FUNC_DUAL_WATCH = hamlib_sys::RIG_FUNC_DUAL_WATCH;
    const RIG_FUNC_DIVERSITY = hamlib_sys::RIG_FUNC_DIVERSITY;
    const RIG_FUNC_DSQL = hamlib_sys::RIG_FUNC_DSQL;
    const RIG_FUNC_SCEN = hamlib_sys::RIG_FUNC_SCEN;
    const RIG_FUNC_SLICE = hamlib_sys::RIG_FUNC_SLICE;
    const RIG_FUNC_TRANSCEIVE = hamlib_sys::RIG_FUNC_TRANSCEIVE;
    const RIG_FUNC_SPECTRUM = hamlib_sys::RIG_FUNC_SPECTRUM;
    const RIG_FUNC_SPECTRUM_HOLD = hamlib_sys::RIG_FUNC_SPECTRUM_HOLD;
    const RIG_FUNC_SEND_MORSE = hamlib_sys::RIG_FUNC_SEND_MORSE;
    const RIG_FUNC_SEND_VOICE_MEM = hamlib_sys::RIG_FUNC_SEND_VOICE_MEM;
    const RIG_FUNC_OVF_STATUS = hamlib_sys::RIG_FUNC_OVF_STATUS;
    const RIG_FUNC_SYNC = hamlib_sys::RIG_FUNC_SYNC;
    const RIG_FUNC_BIT49 = hamlib_sys::RIG_FUNC_BIT49;
    const RIG_FUNC_BIT50 = hamlib_sys::RIG_FUNC_BIT50;
    const RIG_FUNC_BIT51 = hamlib_sys::RIG_FUNC_BIT51;
    const RIG_FUNC_BIT52 = hamlib_sys::RIG_FUNC_BIT52;
    const RIG_FUNC_BIT53 = hamlib_sys::RIG_FUNC_BIT53;
    const RIG_FUNC_BIT54 = hamlib_sys::RIG_FUNC_BIT54;
    const RIG_FUNC_BIT55 = hamlib_sys::RIG_FUNC_BIT55;
    const RIG_FUNC_BIT56 = hamlib_sys::RIG_FUNC_BIT56;
    const RIG_FUNC_BIT57 = hamlib_sys::RIG_FUNC_BIT57;
    const RIG_FUNC_BIT58 = hamlib_sys::RIG_FUNC_BIT58;
    const RIG_FUNC_BIT59 = hamlib_sys::RIG_FUNC_BIT59;
    const RIG_FUNC_BIT60 = hamlib_sys::RIG_FUNC_BIT60;
    const RIG_FUNC_BIT61 = hamlib_sys::RIG_FUNC_BIT61;
    const RIG_FUNC_BIT62 = hamlib_sys::RIG_FUNC_BIT62;
    const RIG_FUNC_BIT63 = 1 << 63;
}
}
