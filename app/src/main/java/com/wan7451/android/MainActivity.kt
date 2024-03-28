package com.wan7451.android

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.widget.Toast
import com.wan7451.native.FFICenter
import com.wan7451.rust_android.R

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        Toast.makeText(this, FFICenter.callRustCode("aaaaa"), Toast.LENGTH_LONG).show()


        val result = FFICenter.initHttpClient("http://www.wanandroid.com", "{}")
        Toast.makeText(this, result, Toast.LENGTH_LONG).show()


        val result2 =FFICenter.sendRequest("http://www.wanandroid.com","/article/list/0/json","{}")

        Log.e("wwwww","result2:"+result2)

        // FFICenter.logInit()
    }
}