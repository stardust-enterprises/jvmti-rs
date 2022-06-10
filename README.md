# jvmti-rs

Interact with the JVM Tooling Interface with idiomatic Rust.

## why?

This work is based on [x4e's jvm-rs][jvm-rs] project, which is feature-rich but
doesn't offer the greatest syntax for idiomatic Rust programming.

### comparaisons


<details>
<summary>With jvm-rs</summary>
<br>

```rs
#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(jvm: *mut JavaVM, _res: &mut c_void) -> c_int {
    // Getting jvmti instance
    let mut ptr: *mut c_void = null_mut();
    let result = (*(*jvm)).GetEnv.unwrap()(jvm, &mut ptr, JVMTI_VERSION_1_2 as jint);
    if result != JNI_OK as jint {
        panic!("Couldn't get JVMTI!");
    }

    let jvmti = ptr.cast::<jvmtiEnv>();
    
    // Getting loaded classes
    let mut class_count: jint = 0;
    let mut classes_ptr: *mut jclass = null_mut();
    let error = (*(*jvmti)).GetLoadedClasses.unwrap()(jvmti, &mut class_count, &mut classes_ptr);
    if error != jvmtiError_JVMTI_ERROR_NONE {
        panic!("Cound't get classes!");
    }

    // transform classes_ptr to array
    // actually do stuff...
}
```

</details>

<details>
<summary>With jvmti-rs</summary>
<br>

```rs
#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(jvm: *mut JavaVM, _res: &mut c_void) -> c_int {
    // Getting jvmti instance
    let jvmti = JvmtiInterface::from_vm(jvm, JVMTI_VERSION_1_2);

    // Getting loaded classes
    let result = jvmti.get_loaded_classes();

    match result {
        Ok(classes) => {
            for class in classes {
                // Do stuff
            }
        },
        Err(e) => panic!(e);
    }
}
```

</details>

## how to use

Everything revolves around the `jvmti_rs::JvmtiInterface` struct;
you'd need a reference to a `jvm_rs::jni::JavaVM` to construct it:

```rs
let mut java_vm: JavaVM = get_current_vm();

let jvmti = JvmtiInterface::from_vm(&mut java_vm);
```

From there you have access to all the JVMTI calls that are available from
[jvm-rs][jvm-rs] without the verbose syntax.

## license
This project is licensed under the [ISC License][blob-license]

<!-- Links -->

[jvm-rs]: https://github.com/x4e/jvm-rs "jvm-rs github page"

[blob-license]: https://github.com/stardust-enterprises/jvmti-rs/blob/trunk/LICENSE
