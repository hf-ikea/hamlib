use std::{mem, num::NonZeroU32};

use hamlib_sys::{
    rig_errcode_e_RIG_BUSBUSY, rig_errcode_e_RIG_BUSERROR, rig_errcode_e_RIG_EACCESS,
    rig_errcode_e_RIG_EARG, rig_errcode_e_RIG_ECONF, rig_errcode_e_RIG_EDEPRECATED,
    rig_errcode_e_RIG_EDOM, rig_errcode_e_RIG_EEND, rig_errcode_e_RIG_EINTERNAL,
    rig_errcode_e_RIG_EINVAL, rig_errcode_e_RIG_EIO, rig_errcode_e_RIG_ELIMIT,
    rig_errcode_e_RIG_ENAVAIL, rig_errcode_e_RIG_ENIMPL, rig_errcode_e_RIG_ENOMEM,
    rig_errcode_e_RIG_ENTARGET, rig_errcode_e_RIG_EPOWER, rig_errcode_e_RIG_EPROTO,
    rig_errcode_e_RIG_ERJCTED, rig_errcode_e_RIG_ESECURITY, rig_errcode_e_RIG_ETIMEOUT,
    rig_errcode_e_RIG_ETRUNC, rig_errcode_e_RIG_EVFO,
};

pub trait RigResultExt {
    fn from_code(code: i32) -> Self;
}

pub type RigResult<T> = Result<T, RigErrorCode>;

impl RigResultExt for RigResult<()> {
    fn from_code(code: i32) -> Self {
        unsafe { mem::transmute(code) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RigErrorCode(NonZeroU32);

impl RigErrorCode {
    pub fn get(self) -> NonZeroU32 {
        self.0
    }
}

macro_rules! rig_errors {
  (
    $(
      $(#[$meta:meta])*
      $name:ident = $val:expr);+$(;)?
    ) => {
    impl RigErrorCode {
      $(
        $(#[$meta])*
        #[allow(unused)]
        const $name: Self = Self(const { NonZeroU32::new($val).expect("Error code must be nonzero") });
      )+
    }
  }
}

rig_errors! {
    #[doc = "1 invalid parameter"]
    INVALID_PARAMETER = rig_errcode_e_RIG_EINVAL;
    #[doc = "2 invalid configuration (serial,..)"]
    INVALID_CONFIGURATION = rig_errcode_e_RIG_ECONF;
    #[doc = "3 memory shortage"]
    MEMORY_SHORTAGE = rig_errcode_e_RIG_ENOMEM;
    #[doc = "4 function not implemented, but will be"]
    NOT_IMPLEMENTED = rig_errcode_e_RIG_ENIMPL;
    #[doc = "5 communication timed out"]
    CONNECTION_TIMEOUT = rig_errcode_e_RIG_ETIMEOUT;
    #[doc = "6 IO error, including open failed"]
    IO_ERROR = rig_errcode_e_RIG_EIO;
    #[doc = "7 Internal Hamlib error, huh!"]
    HAMLIB_INTERNAL = rig_errcode_e_RIG_EINTERNAL;
    #[doc = "8 Protocol error"]
    PROTOCOL_ERROR = rig_errcode_e_RIG_EPROTO;
    #[doc = "9 Command rejected by the rig"]
    COMMAND_REJECTED = rig_errcode_e_RIG_ERJCTED;
    #[doc = "10 Command performed, but arg truncated"]
    ARG_TRUNCATED = rig_errcode_e_RIG_ETRUNC;
    #[doc = "11 Function not available"]
    UNAVAILABLE = rig_errcode_e_RIG_ENAVAIL;
    #[doc = "12 VFO not targetable"]
    NOT_TARGETABLE = rig_errcode_e_RIG_ENTARGET;
    #[doc = "13 Error talking on the bus"]
    BUS_ERROR = rig_errcode_e_RIG_BUSERROR;
    #[doc = "14 Collision on the bus"]
    BUS_BUSY = rig_errcode_e_RIG_BUSBUSY;
    #[doc = "15 NULL RIG handle or any invalid pointer parameter in get arg"]
    NULL_RIG = rig_errcode_e_RIG_EARG;
    #[doc = "16 Invalid VFO"]
    INVALID_VFO = rig_errcode_e_RIG_EVFO;
    #[doc = "17 Argument out of domain of func"]
    OUT_OF_DOMAIN = rig_errcode_e_RIG_EDOM;
    #[doc = "18 Function deprecated"]
    DEPRECATED = rig_errcode_e_RIG_EDEPRECATED;
    #[doc = "19 Security error"]
    SECURITY_ERROR = rig_errcode_e_RIG_ESECURITY;
    #[doc = "20 Rig not powered on"]
    RIG_POWERED_OFF = rig_errcode_e_RIG_EPOWER;
    #[doc = "21 Limit exceeded"]
    LIMIT_EXCEEDED = rig_errcode_e_RIG_ELIMIT;
    #[doc = "22 Access denied -- e.g. port already in use"]
    ACCESS_DENIED = rig_errcode_e_RIG_EACCESS;
    ERROR_END = rig_errcode_e_RIG_EEND;
}
