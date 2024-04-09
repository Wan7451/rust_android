package com.wan7451.ffi


interface BridgeMessageHandler {
    fun handleMessage(message: ByteArray)
}

class StringMessageHandler : BridgeMessageHandler {
    override fun handleMessage(message: ByteArray) {
        val str = String(message)
        logLocal(str)
    }

}

