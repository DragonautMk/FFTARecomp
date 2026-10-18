package com.fftarecomp.port

import android.content.Context

/** Routing hint only: never substitutes the ROM/hash and native-cache checks. */
class StartupState(context:Context) {
    private val preferences=context.getSharedPreferences("startup",Context.MODE_PRIVATE)
    val recorded:Boolean get()=preferences.contains("rom_ready")
    val ready:Boolean get()=preferences.getBoolean("rom_ready",false)
    fun setReady(ready:Boolean){check(preferences.edit().putBoolean("rom_ready",ready).commit()){"Cannot save startup state"}}
}
