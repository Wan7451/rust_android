use jni::JNIEnv;
use jni::objects::{JClass, JMethodID, JObject, JStaticMethodID, JString, JValue, JValueGen};
use jni::sys::{jclass, jmethodID, jstring};

// #[macro_use] extern crate log;
// extern crate android_logger;

mod module;

#[no_mangle]
extern "C" fn Java_com_wan7451_native_FFICenter_callRustCode(mut env: JNIEnv, class: JClass, input: JString) -> jstring {
    // let str = match env.get_string(&input) {
    //     Ok(s) => s.into(),
    //     Err(s) => {
    //         env.throw(s).unwrap();
    //     }
    // };

    let str: String = env.get_string(&input).unwrap().into();
    let out = env.new_string(format!("hello,{}", str)).unwrap();
    //let method_id = find_static_method_id(env, "com/wan7451/native/FFICenter", "logTest", "(Ljava/lang/String;)V");


    module::https::test_request(|str| {
        let result = env.new_string(str).unwrap();
        env.call_static_method("com/wan7451/native/FFICenter", "logTest", "(Ljava/lang/String;)V", &[JValueGen::Object(&result)]).unwrap();
    });
    out.into_raw()
}


extern fn Java_com_wan7451_native_FFICenter_logTest(env: JNIEnv, _: JObject, input: jstring) -> jstring {
    /* let a :String = env.new_string(input).unwrap_or(JString::from("")).into();
     return input;*/
    return env.new_string("Hello from rust!").unwrap().into_raw();
}


fn find_static_method_id(mut env: JNIEnv, clazz: &str, method_name: &str, signature: &str) -> JStaticMethodID {
    let method_id = env.get_static_method_id(clazz, method_name, signature).unwrap();
    method_id
}

fn find_method_id(mut env: JNIEnv, clazz: &str, method_name: &str, signature: &str) -> jmethodID {
    let method_id = env.get_method_id(clazz, method_name, signature).unwrap();
    method_id.into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_wan7451_native_FFICenter_logInit(env: JNIEnv, _: JObject) {
    use crate::module::logs;
    logs::init_once();
    logs::test_record_builder();
}


#[cfg(test)]
mod test {
    use serde_json::json;
    use crate::module;
    use crate::module::HttpClient;

    #[test]
    fn test() {
        module::https::test_request(|str| {
            println!("{}", str);
        });

        let mut json = json!({});
        json["1"] = json!("1");
        json["2"] = json!("2");
        //let header = module::CommonHeader::new("{\"1\":}");
        //let log: String = header.unwrap().into();
        //println!("{}", log);

        let client = HttpClient::new("wwwww", "{\"1\":}");
        match client {
            Ok(client) => {
                println!("success {}", client);
            },
            Err(err) => {
                println!("error {}", err);
            }
        }
    }
}
