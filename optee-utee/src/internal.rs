use crate::{Attribute, Error, ObjHandle, Result, TransientObject, Parameters};
use optee_utee_sys as raw;
use std::{mem, ptr};

/// An abstraction of the logical connection between a Trusted Application (TA)
/// and another TA
pub struct Context;

pub struct Session {
    handle: raw::TEE_TASessionHandle
}

//pub struct Params {
    //pub types: [u32; 4],
    //pub params: [raw::TEE_Param; 4]
//}

impl Session {
    pub fn new(uuid_time_low: u32, uuid_time_mid: u16, uuid_time_hi_and_version: u16, uuid_clock_seq_and_node: [u8;8] ) -> Result<Self> {
        let mut handle:raw::TEE_TASessionHandle = ptr::null_mut();
        let mut return_origin: u32 = 0;
        let uuid = raw::TEE_UUID {
            timeLow: uuid_time_low,
            timeMid: uuid_time_mid,
            timeHiAndVersion: uuid_time_hi_and_version,
            clockSeqAndNode: uuid_clock_seq_and_node,
        };
        unsafe {
            match raw::TEE_OpenTASession(&uuid, // destination
                                        0, // cancellationRequestTimeout
                                        0, // paramTypes
                                        ptr::null_mut(), // params
                                        &mut handle, // session
                                        &mut return_origin,) {
                raw::TEE_SUCCESS => Ok(Session {
                                        handle: handle
                                    }),
                code => {
                    trace_println!("optee-utee::internal::Context::open_session TEE_OpenTASession failed:{:?}", code);
                    Err(Error::from_raw_error(code))
                }
            }
        }
    }

    pub fn invoke_command(
            &mut self,
            command_id: u32,
            params: &mut Parameters,
    ) -> Result<()> {
        let types = raw::TEE_PARAM_TYPES(params.0.param_type as u32,
                                         params.1.param_type as u32,
                                         params.2.param_type as u32,
                                         params.3.param_type as u32);
        let mut return_origin: u32 = 0;
        let mut tee_params: [raw::TEE_Param; 4] = unsafe {
            [*params.0.raw, *params.1.raw, *params.2.raw, *params.3.raw]
        };
        unsafe {
                match raw::TEE_InvokeTACommand(
                    self.handle,
                    0, // cancellationRequestTimeout
                    command_id, //commndID
                    types, // paramTypes
                    tee_params.as_mut_ptr(), //params.params.as_mut_ptr(), // params
                    &mut return_origin, // returnOrigin
                ) {
                raw::TEE_SUCCESS => Ok(()),
                code => {
                    trace_println!("optee-utee::internal::Session::invoke_command TEE_InvokeTACommand failed: {:?}", code);
                    Err(Error::from_raw_error(code))
                }
            }
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { raw::TEE_CloseTASession(self.handle) };
    }
}

