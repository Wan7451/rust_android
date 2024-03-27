package com.wan7451.native

import android.content.Context
import android.util.Log
import android.widget.Toast

object FFICenter {

    external fun callRustCode(str: String): String

    external fun logInit()

    init {
        System.loadLibrary("rust_library")
    }


    //声明静态方法 方法名字 test
    @JvmStatic
    fun logTest(srt: String) {
        Log.e("wwwww", srt)
    }
}