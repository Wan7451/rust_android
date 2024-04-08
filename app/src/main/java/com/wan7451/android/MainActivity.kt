package com.wan7451.android

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.widget.Toast
import com.wan7451.native.FFICenter
import com.wan7451.rust_android.R
import org.json.JSONObject

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        FFICenter.initLog()

        val headers = JSONObject()
        headers.put("1", "1")
        headers.put("2", "2")
        headers.put("3", "3")
        headers.put("4", "4")

        val result = FFICenter.initHttpClient("https://www.wanandroid.com", headers.toString())
        Toast.makeText(this, result, Toast.LENGTH_LONG).show()


        val result2 =FFICenter.sendRequest("https://www.wanandroid.com","/article/list/0/json","{}")

    }
}