# jvmti-rs

Interact with the JVM Tooling Interface with idiomatic Rust.

## why?

This work is based on [x4e's jvm-rs][jvm-rs] project, which is feature-rich but
doesn't offer the greatest syntax for idiomatic Rust programming.

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
