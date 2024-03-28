use std::result;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JStaticMethodID, JString, JValueGen};
use jni::sys::{jboolean, jmethodID, jstring};

use crate::module::{init, RequestBuilder};

// #[macro_use] extern crate log;
// extern crate android_logger;

mod module;

#[macro_export]
macro_rules! get_jstring {
    ($env:expr; $val:expr) => {
        match $val.is_null() {
            true => Ok("unknown".to_owned()),
            false => match $env.get_string(&$val) {
                Ok(id) => match id.to_str() {
                    Ok(result) => Ok(result.to_owned()),
                    Err(e) => Err(format!("dcs {} to_str error: {}", stringify!($val), e)),
                },
                Err(e) => Err(format!(
                    "dcs {} env.get_string error: {}",
                    stringify!($val),
                    e
                )),
            },
        }
    };
}
#[no_mangle]
extern "C" fn Java_com_wan7451_native_FFICenter_initHttpClient(mut env: JNIEnv, _: JClass, base_url: JString, common_header: JString) -> jstring {
    let base_url = match get_jstring!(env; base_url) {
        Ok(base_url) => base_url,

        Err(e) => {
            return env.new_string(format!("init error:{}", e)).unwrap().into_raw();
        }
    };
    let common_header = match get_jstring!(env; common_header) {
        Ok(common_header) => common_header,

        Err(e) => {
            return env.new_string(format!("init error:{}", e)).unwrap().into_raw();
        }
    };
    match init(&base_url, &common_header) {
        Ok(_) => {
            env.new_string("init success").unwrap().into_raw()
        }
        Err(e) => {
            env.new_string(format!("init error:{}", e)).unwrap().into_raw()
        }
    }
}

#[no_mangle]
extern "C" fn Java_com_wan7451_native_FFICenter_sendRequest(mut env: JNIEnv, _: JClass, base_url: JString, path: JString, params: JString) -> jstring {
    let base_url = match get_jstring!(env; base_url) {
        Ok(base_url) => base_url,

        Err(e) => {
            return env.new_string(format!("init error:{}", e)).unwrap().into_raw();
        }
    };
    let path = match get_jstring!(env; path) {
        Ok(path) => path,

        Err(e) => {
            return env.new_string(format!("init error:{}", e)).unwrap().into_raw();
        }
    };
    let params = match get_jstring!(env; params) {
        Ok(common_header) => common_header,

        Err(e) => {
            return env.new_string(format!("init error:{}", e)).unwrap().into_raw();
        }
    };

    let result = RequestBuilder::new(&base_url).path(&path).params(&params).get(|result| {
        match result {
            Ok(result) => {
                let result = env.new_string(result).unwrap();
                env.call_static_method("com/wan7451/native/FFICenter", "logTest", "(Ljava/lang/String;)V", &[JValueGen::Object(&result)]).unwrap();
            }
            Err(e) => {
                let result = env.new_string(e.to_string()).unwrap();
                env.call_static_method("com/wan7451/native/FFICenter", "logTest", "(Ljava/lang/String;)V", &[JValueGen::Object(&result)]).unwrap();
            }
        }
    });

    match result {
        Ok(result) => {
            env.new_string("success").unwrap().into_raw()
        }
        Err(e) => {
            env.new_string(format!("init error:{}", e)).unwrap().into_raw()
        }
    }
}

#[no_mangle]
extern "C" fn Java_com_wan7451_native_FFICenter_callRustCode(mut env: JNIEnv, _: JClass, input: JString) -> jstring {
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
    use std::collections::HashMap;
    use jni::objects::JValueGen;
    use serde_json::json;

    use crate::module;
    use crate::module::{HttpClient, init, RequestBuilder};

    #[test]
    fn test() {


        init("http://httpbin.org","{}");


        let mut json = json!({});
        json["lang"] = json!("rust");
        json["body"] = json!("json");

        let json = json.to_string();

        let result = RequestBuilder::new("http://httpbin.org").path("/ip").params(&json).get(|result| {
            match result {
                Ok(result) => {
                   println!("{}", result)
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        });



        // module::https::test_request(|str| {
        //     println!("{}", str);
        // });
        //
        // let mut json = json!({});
        // json["1"] = json!("1");
        // json["2"] = json!("2");
        // //let header = module::CommonHeader::new("{\"1\":}");
        // //let log: String = header.unwrap().into();
        // //println!("{}", log);
        //
        // let client = HttpClient::new("wwwww", "{\"1\":}");
        // match client {
        //     Ok(client) => {
        //         println!("success {}", client);
        //     }
        //     Err(err) => {
        //         println!("error {}", err);
        //     }
        // }
    }
}

