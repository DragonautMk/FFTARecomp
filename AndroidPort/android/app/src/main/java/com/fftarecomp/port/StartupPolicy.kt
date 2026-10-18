package com.fftarecomp.port

enum class StartupRoute { SETUP, LOADING }
object StartupPolicy {
    fun route(ready:Boolean,romPresent:Boolean,nativeError:Boolean,recorded:Boolean=true):StartupRoute {
        if(nativeError || !romPresent)return StartupRoute.SETUP
        // Existing installations predate the persisted ready flag. Migrate
        // them directly through Loading instead of flashing the ROM picker.
        return if(ready || !recorded)StartupRoute.LOADING else StartupRoute.SETUP
    }
}
