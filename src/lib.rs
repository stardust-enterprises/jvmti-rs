pub mod threading;

extern crate jvm_rs;

use std::os::raw::c_void;
use std::ptr::{null_mut};
use jvm_rs::jni::{JavaVM, jint, JNI_OK};
use jvm_rs::jvmti::{jthread, JVMTI_VERSION_1_0, jvmtiEnv, jvmtiError, jvmtiEvent, jvmtiEventMode, jvmtiInterface_1_};

#[derive(Clone, Copy)]
pub struct JvmtiInterface {
    pointer: *mut jvmtiEnv,
    interface: jvmtiInterface_1_,
}

impl JvmtiInterface {
    fn from(ptr: *mut jvmtiEnv) -> JvmtiInterface {
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
    ) -> jvmtiError {
        unsafe {
            self.interface.SetEventNotificationMode
                .expect("Couldn't find SetEventNotificationMode.")
                (self.pointer, mode, event_type, *event_thread)
        }
    }
}

pub fn get_jvmti(jvm: &mut JavaVM) -> Result<JvmtiInterface, u32> {
    let mut env: *mut c_void = null_mut();

    let result = {
        let jvm_invoke = unsafe { **jvm };
        let get_env = jvm_invoke.GetEnv.expect("Couldn't find GetEnv.");

        unsafe {
            get_env(jvm, &mut env, JVMTI_VERSION_1_0 as jint)
        }
    };

    if result != JNI_OK as jint {
        Err(result as u32)
    } else {
        Ok(JvmtiInterface::from(env.cast::<jvmtiEnv>()))
    }
}