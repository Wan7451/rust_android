use std::sync::OnceLock;

use jni::{JavaVM, JNIEnv};
use jni::objects::{GlobalRef, JClass, JObject, JValue};

use crate::module::error::Result;

pub(crate) static JNI_PROXY: OnceLock<JniProxy> = OnceLock::new();


#[no_mangle]
extern "C" fn Java_com_wan7451_ffi_FFICenter_registerMessageHandler(mut env: JNIEnv, _: JClass, callback: JObject) {
    JNI_PROXY.get_or_init(|| {
        let jvm = env.get_java_vm().unwrap();
        let global = env.new_global_ref(callback).unwrap();
        JniProxy::new(jvm, global)
    });
}


pub struct JniProxy {
    jvm: JavaVM,
    global_ref: GlobalRef,
}

impl JniProxy {
    pub fn new(jvm: JavaVM, global_ref: GlobalRef) -> Self {
        Self {
            jvm,
            global_ref,
        }
    }

    pub(crate) fn invoke(&self, bytes_slice: &[u8]) -> Result<()> {
        let mut jni_env = self.jvm.attach_current_thread()?;
        let java_bytes = jni_env.byte_array_from_slice(bytes_slice)?;
        let java_value = JValue::Object(&java_bytes);
        jni_env.call_method(&self.global_ref, "handleMessage", "([B)V", &[java_value])?;
        Ok(())
    }
}