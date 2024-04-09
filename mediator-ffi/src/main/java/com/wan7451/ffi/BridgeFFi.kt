package com.wan7451.ffi

object BridgeFFi {


    init {
        System.loadLibrary("megazord")
    }

    fun init() {
        try {
            FFICenter.initLog()
            FFICenter.registerMessageHandler(StringMessageHandler())
        } catch (e: Exception) {
            if (BuildConfig.DEBUG) {
                e.printStackTrace()
            }
        }
    }

    fun initHttpClient(baseUrl: String, commonHeader: String) {
        try {
            FFICenter.initHttpClient(baseUrl, commonHeader)
        } catch (e: Exception) {
            if (BuildConfig.DEBUG) {
                e.printStackTrace()
            }
        }
    }

    fun sendRequest(baseUrl: String, path: String, params: String) {
        try {
            FFICenter.sendRequest(baseUrl, path, params)
        } catch (e: Exception) {
            if (BuildConfig.DEBUG) {
                e.printStackTrace()
            }
        }
    }

}