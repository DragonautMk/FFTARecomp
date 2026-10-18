package com.fftarecomp.port
import org.junit.Assert.assertEquals
import org.junit.Test
class StartupPolicyTest {
    @Test fun firstInstallShowsSetup(){assertEquals(StartupRoute.SETUP,StartupPolicy.route(false,false,false))}
    @Test fun preparedInstallGoesDirectlyToLoading(){assertEquals(StartupRoute.LOADING,StartupPolicy.route(true,true,false))}
    @Test fun existingRomMigratesWithoutShowingSetup(){assertEquals(StartupRoute.LOADING,StartupPolicy.route(false,true,false,false))}
    @Test fun missingRomRecoversToSetup(){assertEquals(StartupRoute.SETUP,StartupPolicy.route(true,false,false))}
    @Test fun failedStoredRomRecoversToSetup(){assertEquals(StartupRoute.SETUP,StartupPolicy.route(false,true,false,true))}
    @Test fun nativeFailureRecoversToSetup(){assertEquals(StartupRoute.SETUP,StartupPolicy.route(true,true,true))}
}
