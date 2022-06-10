#[macro_use]
#[allow(non_upper_case_globals)]

pub mod threading;

extern crate jvm_rs;
extern crate core;

use std::os::raw::{c_void, c_uchar};
use std::ptr::{null_mut};
use jvm_rs::jni::{JavaVM, jint, jlong, JNI_OK};
use jvm_rs::jvmti::{jthread, JVMTI_VERSION_1_0, jvmtiEnv, jvmtiError, jvmtiError_JVMTI_ERROR_NONE, jvmtiEvent, jvmtiEventMode, jvmtiInterface_1_};

macro_rules! error_msg { () => { "Couldn't find {}!" }; }

pub(crate) use error_msg;

#[derive(Clone, Copy)]
pub struct JvmtiInterface {
    pointer: *mut jvmtiEnv,
    interface: jvmtiInterface_1_,
}

#[allow(non_upper_case_globals)]
impl JvmtiInterface {
    pub fn from_vm(jvm: &mut JavaVM) -> Result<JvmtiInterface, jint> {
        let mut env: *mut c_void = null_mut();

        let jvm_invoke = unsafe { **jvm };
        let get_env = jvm_invoke.GetEnv.expect(format!(error_msg!(), "GetEnv").as_str());

        let result = unsafe {
            get_env(jvm, &mut env, JVMTI_VERSION_1_0 as jint)
        };

        if result != JNI_OK as jint {
            Err(result)
        } else {
            Ok(JvmtiInterface::from_ptr(env.cast::<jvmtiEnv>()))
        }
    }

    pub fn from_ptr(ptr: *mut jvmtiEnv) -> JvmtiInterface {
        JvmtiInterface {
            pointer: ptr,
            interface: unsafe { **ptr },
        }
    }

    pub fn set_event_notification_mode(
        self,
        mode: jvmtiEventMode,
        event_type: jvmtiEvent,
        event_thread: &jthread,
    ) -> Result<(), jvmtiError> {
        let error = unsafe {
            self.interface.SetEventNotificationMode
                .expect(format!(error_msg!(), "SetEventNotificationMode").as_str())
                (self.pointer, mode, event_type, *event_thread)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => Ok(()),
            _ => Err(error),
        }
    }

    pub fn allocate(
        self,
        size: jlong,
    ) -> Result<*mut c_uchar, jvmtiError> {
        let mut mem_ptr: *mut c_uchar = null_mut();

        let error = unsafe {
            self.interface.Allocate
                .expect(format!(error_msg!(), "Allocate").as_str())
                (self.pointer, size, &mut mem_ptr)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => Ok(mem_ptr),
            _ => Err(error),
        }
    }

    pub fn deallocate(
        self,
        mem: *mut c_uchar,
    ) -> Result<(), jvmtiError> {
        let error = unsafe {
            self.interface.Deallocate
                .expect(format!(error_msg!(), "Deallocate").as_str())
                (self.pointer, mem)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => Ok(()),
            _ => Err(error),
        }
    }
}
