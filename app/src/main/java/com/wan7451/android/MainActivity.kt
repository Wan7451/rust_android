package com.wan7451.android

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import com.wan7451.native.FFICenter
import com.wan7451.native.StringMessageHandler
import com.wan7451.rust_android.R
import org.json.JSONObject

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        FFICenter.initLog()

        val handler = StringMessageHandler()

        FFICenter.registerMessageHandler(handler)

        val headers = JSONObject()
        headers.put("1", "1")
        headers.put("2", "2")
        headers.put("3", "3")
        headers.put("4", "4")

        FFICenter.initHttpClient("https://www.wanandroid.com", headers.toString())


        FFICenter.sendRequest("https://www.wanandroid.com", "/article/list/0/json", "{}")

    }
}