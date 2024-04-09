package com.wan7451.native

import android.util.Log
import androidx.annotation.Keep

/**
 * 外部函数接口 Foreign Function Interface: FFI 是 "Foreign Function Interface" 的缩写，
 *
 * 它是一种编程技术或机制，允许一种编程语言能够调用另一种编程语言编写的函数或库。
 * 通过 FFI，开发者可以在自己的代码中无缝地使用其他语言提供的功能，而无需重新实现这些功能或者编写特定的适配器层。
 * 这种互操作性有助于不同语言之间的代码复用、系统集成以及利用特定语言的性能优势。
 * FFI 通常涉及以下方面：
 * 绑定生成：创建一种方式来描述外部函数的接口（参数类型、返回值类型、调用约定等），以便目标语言能够正确地调用它们。
 * 数据类型转换：在不同语言间传递数据时，需要确保类型兼容性，并可能需要进行隐式或显式的类型转换。
 * 内存管理：处理不同语言的内存管理模型差异，如垃圾回收与手动内存分配的协调。
 * 错误处理：确保跨语言调用中的异常或错误状态能够被正确捕获和处理。
 */

@Keep
object FFICenter {

    external fun initLog()

    @Throws(Throwable::class)
    external fun initHttpClient(baseUrl: String, commonHeader: String)

    @Throws(Throwable::class)
    external fun sendRequest(baseUrl: String, path: String, params: String)

    @Throws(Throwable::class)
    external fun registerMessageHandler(handler: Any)

    init {
        System.loadLibrary("megazord")
    }

    //声明静态方法 方法名字 test
    @JvmStatic
    fun logTest(srt: String) {
        Log.e("wwwww", srt)
    }
}

class StringMessageHandler : BridgeMessageHandler {
    override fun handleMessage(message: ByteArray) {
        val str = String(message)
        Log.e("StringMessageHandler", str)
    }

}

interface BridgeMessageHandler {
    fun handleMessage(message: ByteArray)
}