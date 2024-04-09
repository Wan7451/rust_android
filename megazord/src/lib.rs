use jni::JNIEnv;
use jni::objects::{JClass, JString};

use crate::ffi::JNI_PROXY;
use crate::module::{init, init_once, RequestBuilder};

mod module;
mod ffi;

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
                Ok(result) => {}
                Err(e) => {}
            }
        });
    }
}

