package com.wan7451.native

object FFICenter {

    external fun callRustCode(): String

    external fun logInit()

    init {
        System.loadLibrary("rust_library")
    }

}