use bitflags::bitflags;

bitflags! {
pub struct VFO: u32 {
    const RIG_VFO_NONE = 0;
    const RIG_VFO_A = 1;
    const RIG_VFO_B = 2;
    const RIG_VFO_C = 4;
    const RIG_VFO_SUB_A = 2097152;
    const RIG_VFO_SUB_B = 4194304;
    const RIG_VFO_SUB_C = 8;
    const RIG_VFO_MAIN_A = 8388608;
    const RIG_VFO_MAIN_B = 16777216;
    const RIG_VFO_MAIN_C = 16;
    const RIG_VFO_OTHER = 32;
    const RIG_VFO_SUB = 33554432;
    const RIG_VFO_MAIN = 67108864;
    const RIG_VFO_VFO = 134217728;
    const RIG_VFO_MEM = 268435456;
    const RIG_VFO_CURR = 536870912;
    const RIG_VFO_TX_FLAG = 1073741824;
    const RIG_VFO_ALL = 2147483648;
    const RIG_VFO_TX = 1610612736;
    const RIG_VFO_RX = 536870912;
}
}
