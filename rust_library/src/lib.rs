use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jstring;

#[macro_use] extern crate log;
extern crate android_logger;

mod module;

#[no_mangle]
extern fn Java_com_wan7451_native_FFICenter_callRustCode(env: JNIEnv, _: JObject) -> jstring {
    env.new_string("Hello from rust!").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_wan7451_native_FFICenter_logInit(env: JNIEnv, _: JObject) {
    use crate::module::logs;
    logs::init_once();
    logs::test_record_builder();
}



#[cfg(test)]
mod test {
    use crate::module::https;

    #[test]
    fn test() {
        https::test();
        println!("Hello from rust test!")
    }
}
