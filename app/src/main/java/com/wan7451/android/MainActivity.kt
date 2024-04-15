package com.wan7451.android

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import com.wan7451.ffi.BridgeFFi
import com.wan7451.rust.R
import org.json.JSONObject

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        BridgeFFi.init()

        val headers = JSONObject()
        headers.put("1", "1")
        headers.put("2", "2")
        headers.put("3", "3")
        headers.put("4", "4")

        BridgeFFi.initHttpClient("https://www.wanandroid.com", headers.toString())


        BridgeFFi.sendRequest("https://www.wanandroid.com", "/article/list/0/json", headers.toString())

    }
}