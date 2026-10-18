package com.fftarecomp.port

import android.content.Intent
import android.os.SystemClock
import android.view.InputDevice
import android.view.KeyEvent
import android.view.MotionEvent
import android.view.accessibility.AccessibilityNodeInfo
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith

/** Synthetic events use the actual integrated controller's Android device id.
 * This verifies app/framework routing, not a human pressing its physical switches. */
@RunWith(AndroidJUnit4::class)
class GamepadDetectionTest {
    private val instrumentation=InstrumentationRegistry.getInstrumentation()
    private fun contains(node:AccessibilityNodeInfo?,text:String):Boolean{
        if(node==null)return false
        if(node.text?.toString()==text)return true
        return (0 until node.childCount).any{contains(node.getChild(it),text)}
    }
    private fun waitFor(test:()->Boolean){
        val end=SystemClock.uptimeMillis()+30000
        while(!test()){
            check(SystemClock.uptimeMillis()<end){"Input overlay did not reach expected state"}
            SystemClock.sleep(100)
        }
    }
    private fun touch(){
        val props=MotionEvent.PointerProperties().apply{id=0;toolType=MotionEvent.TOOL_TYPE_FINGER}
        val coords=MotionEvent.PointerCoords().apply{x=360f;y=360f;pressure=1f;size=.1f}
        val time=SystemClock.uptimeMillis()
        for(action in listOf(MotionEvent.ACTION_DOWN,MotionEvent.ACTION_UP)){
            val event=MotionEvent.obtain(time,SystemClock.uptimeMillis(),action,1,arrayOf(props),arrayOf(coords),
                0,0,1f,1f,0,0,InputDevice.SOURCE_TOUCHSCREEN,0)
            assertTrue(instrumentation.uiAutomation.injectInputEvent(event,true));event.recycle()
        }
    }
    @Test fun integratedControllerAndTouchSwitchOverlayAutomatically(){
        val device=InputDevice.getDeviceIds().toList().mapNotNull{InputDevice.getDevice(it)}
            .firstOrNull{ControllerInput.recognizes(0,it.sources)}
            ?:error("A real connected gamepad is required")
        assertEquals("retrogame_joypad",device.name)
        val intent=Intent(instrumentation.targetContext,MainActivity::class.java)
            .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK)
            .putExtra("ffta_benchmark",true).putExtra("ffta_benchmark_frames",600)
        val activity=instrumentation.startActivitySync(intent)
        try{
            waitFor{NativeBridge.compiled}
            SystemClock.sleep(1000)
            assertFalse(contains(instrumentation.uiAutomation.rootInActiveWindow,"Pause"))
            // Include C, which has no gameplay binding: any controller key hides touch UI.
            for(key in listOf(KeyEvent.KEYCODE_BUTTON_C,KeyEvent.KEYCODE_BUTTON_A,
                KeyEvent.KEYCODE_BUTTON_X,KeyEvent.KEYCODE_BUTTON_R1,KeyEvent.KEYCODE_DPAD_LEFT)){
                touch()
                waitFor{contains(instrumentation.uiAutomation.rootInActiveWindow,"Pause")}
                val time=SystemClock.uptimeMillis()
                for(action in listOf(KeyEvent.ACTION_DOWN,KeyEvent.ACTION_UP)){
                    val event=KeyEvent(time,SystemClock.uptimeMillis(),action,key,0,0,device.id,0,0,InputDevice.SOURCE_GAMEPAD)
                    assertTrue(instrumentation.uiAutomation.injectInputEvent(event,true))
                }
                waitFor{!contains(instrumentation.uiAutomation.rootInActiveWindow,"Pause")}
            }
            touch()
            waitFor{contains(instrumentation.uiAutomation.rootInActiveWindow,"Pause")}
        }finally{instrumentation.runOnMainSync{activity.finish()}}
    }
}
