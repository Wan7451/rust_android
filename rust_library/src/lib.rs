use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use crate::module::{init, init_once, RequestBuilder};

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
            }
            Err(e) => {
            }
        }
    });

    match result {
        Ok(result) => {
            env.new_string(format!("success:{}", result)).unwrap().into_raw()
        }
        Err(e) => {
            env.new_string(format!("init error:{}", e)).unwrap().into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_com_wan7451_native_FFICenter_initLog(_: JNIEnv, _: JClass) {
    init_once();
}


#[cfg(test)]
mod test {
    use std::result;

    use log::error;

    use crate::module::{init, init_once, RequestBuilder};

    #[test]
    fn test() {
        init_once();
        let header = serde_json::json!({ "1":"2"});
        let base_url = "https://www.wanandroid.com";
        let path = "/article/list/0/json";
        let params = "{}";
        let result = init(base_url, header.to_string().as_str());
        let result = RequestBuilder::new(base_url).path(path).params(params).get(|result| {
            match result {
                Ok(result) => {
                }
                Err(e) => {
                }
            }
        });
    }
}

