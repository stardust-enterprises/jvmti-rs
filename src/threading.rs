#[allow(non_upper_case_globals)]

use std::ptr::null_mut;
use jvm_rs::jni::jobject;
use jvm_rs::jvmti::{jthread, jvmtiError, jvmtiError_JVMTI_ERROR_NONE, jvmtiThreadInfo};
use crate::{error_msg, JvmtiInterface};

/**
 * JVMTI Threading implementations.
 */
impl JvmtiInterface {
    pub fn get_all_threads(
        self
    ) -> Result<Vec<jthread>, jvmtiError> {
        let mut count = 0;
        let mut threads: *mut jthread = null_mut();

        let error = unsafe {
            self.interface.GetAllThreads
                .expect(format!(error_msg!(), "GetAllThreads").as_str())
                (self.pointer, &mut count, &mut threads)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => {
                let mut vec = Vec::new();
                for pos in 0..count {
                    let thread = unsafe { *threads.offset(pos as _) };
                    vec.push(thread);
                }
                Ok(vec)
            },
            _ => Err(error),
        }
    }

    pub fn suspend_thread(
        self,
        thread: &jthread,
    ) -> Result<(), jvmtiError> {
        let error = unsafe {
            self.interface.SuspendThread
                .expect(format!(error_msg!(), "SuspendThread").as_str())
                (self.pointer, *thread)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => Ok(()),
            _ => Err(error),
        }
    }

    pub fn resume_thread(
        self,
        thread: &jthread,
    ) -> Result<(), jvmtiError> {
        let error = unsafe {
            self.interface.ResumeThread
                .expect(format!(error_msg!(), "ResumeThread").as_str())
                (self.pointer, *thread)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => Ok(()),
            _ => Err(error),
        }
    }

    pub fn stop_thread(
        self,
        thread: &jthread,
        exception: &jobject,
    ) -> Result<(), jvmtiError> {
        let error = unsafe {
            self.interface.StopThread
                .expect(format!(error_msg!(), "StopThread").as_str())
                (self.pointer, *thread, *exception)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => Ok(()),
            _ => Err(error),
        }
    }

    pub fn interrupt_thread(
        self,
        thread: &jthread,
    ) -> Result<(), jvmtiError> {
        let error = unsafe {
            self.interface.InterruptThread
                .expect(format!(error_msg!(), "InterruptThread").as_str())
                (self.pointer, *thread)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => Ok(()),
            _ => Err(error),
        }
    }

    pub fn get_thread_info(
        self,
        thread: &jthread,
    ) -> Result<jvmtiThreadInfo, jvmtiError> {
        let mut info: jvmtiThreadInfo = jvmtiThreadInfo {
            name: null_mut(),
            priority: 0,
            is_daemon: 0,
            thread_group: null_mut(),
            context_class_loader: null_mut(),
        };

        let error = unsafe {
            self.interface.GetThreadInfo
                .expect(format!(error_msg!(), "GetThreadInfo").as_str())
                (self.pointer, *thread, &mut info)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => Ok(info),
            _ => Err(error),
        }
    }

    pub fn get_owned_monitor_info(
        self,
        thread: &jthread,
    ) -> Result<Vec<jobject>, jvmtiError> {
        let mut count = 0;
        let mut monitors: *mut jobject = null_mut();

        let error = unsafe {
            self.interface.GetOwnedMonitorInfo
                .expect(format!(error_msg!(), "GetOwnedMonitorInfo").as_str())
                (self.pointer, *thread, &mut count, &mut monitors)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => {
                let mut vec = Vec::new();
                for pos in 0..count {
                    let monitor = unsafe { *monitors.offset(pos as _) };
                    vec.push(monitor);
                }
                Ok(vec)
            },
            _ => Err(error),
        }
    }

    pub fn get_current_contended_monitor(
        self,
        thread: &jthread,
    ) -> Result<jobject, jvmtiError> {
        let mut monitor: jobject = null_mut();

        let error = unsafe {
            self.interface.GetCurrentContendedMonitor
                .expect(format!(error_msg!(), "GetCurrentContendedMonitor").as_str())
                (self.pointer, *thread, &mut monitor)
        };

        match error {
            jvmtiError_JVMTI_ERROR_NONE => Ok(monitor),
            _ => Err(error),
        }
    }
}
