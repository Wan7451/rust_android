package com.wan7451.ffi

import android.util.Log

private const val TAG = "wwww"

fun logLocal(message: String) {
    if (BuildConfig.DEBUG) {
        Log.e("${TAG}:${Thread.currentThread().name} ", message)
    }
}

fun logRemote(message: String) {
    logLocal(message)
    //日志
}