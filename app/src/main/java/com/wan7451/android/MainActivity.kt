package com.wan7451.android

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.Toast
import com.wan7451.native.FFICenter
import com.wan7451.rust_android.R

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        Toast.makeText(this, FFICenter.callRustCode("aaaaa"), Toast.LENGTH_LONG).show()


       // FFICenter.logInit()
    }
}