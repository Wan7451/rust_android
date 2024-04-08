package com.wan7451.native

object FFICenter {

    external fun initHttpClient(baseUrl: String, commonHeader: String): String

    external fun sendRequest(baseUrl: String, path: String, params: String): String

    external fun initLog()

    init {
        System.loadLibrary("rust_library")
    }
}