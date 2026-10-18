package com.fftarecomp.port

import android.content.Context
import android.content.Intent
import android.graphics.Rect
import android.os.SystemClock
import android.view.MotionEvent
import android.view.accessibility.AccessibilityNodeInfo
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class TouchSelectionDeviceTest {
    private val instrumentation=InstrumentationRegistry.getInstrumentation()
    private fun find(node:AccessibilityNodeInfo?,label:String):AccessibilityNodeInfo? {
        if(node==null)return null
        if(node.contentDescription?.toString()==label || node.text?.toString()==label)return node
        for(i in 0 until node.childCount)find(node.getChild(i),label)?.let{return it}
        return null
    }
    private fun tap(x:Float,y:Float){
        val time=SystemClock.uptimeMillis()
        val props=MotionEvent.PointerProperties().apply{id=0;toolType=MotionEvent.TOOL_TYPE_FINGER}
        val coords=MotionEvent.PointerCoords().apply{this.x=x;this.y=y;pressure=1f;size=.1f}
        for(action in listOf(MotionEvent.ACTION_DOWN,MotionEvent.ACTION_UP)){
            val event=MotionEvent.obtain(time,SystemClock.uptimeMillis(),action,1,arrayOf(props),arrayOf(coords),
                0,0,1f,1f,0,0,android.view.InputDevice.SOURCE_TOUCHSCREEN,0)
            assertTrue(instrumentation.uiAutomation.injectInputEvent(event,true));event.recycle()
        }
    }
    @Test fun gameClicksExcludeOverlayControlsAndPauseMenu(){
        val context=instrumentation.targetContext
        val prefs=context.getSharedPreferences("touch_layout_v1",Context.MODE_PRIVATE)
        val old=prefs.getBoolean("experimental_clicks",false)
        prefs.edit().putBoolean("experimental_clicks",true).commit()
        val activity=instrumentation.startActivitySync(Intent(context,MainActivity::class.java)
            .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK))
        try{
            val end=SystemClock.uptimeMillis()+90000
            while(!NativeBridge.compiled && SystemClock.uptimeMillis()<end)SystemClock.sleep(100)
            assertTrue("Native engine prepared",NativeBridge.compiled)
            SystemClock.sleep(2000)
            tap(360f,340f) // reveals touch controls on a device with a connected gamepad
            SystemClock.sleep(500)
            val before=GameSurface.deliveredClicks.get()
            tap(360f,340f)
            SystemClock.sleep(500)
            assertEquals("One game tap produces one click",before+1,GameSurface.deliveredClicks.get())
            val button=find(instrumentation.uiAutomation.rootInActiveWindow,"A")?:error("A control missing")
            val rect=Rect();button.getBoundsInScreen(rect)
            val controlsBefore=GameSurface.deliveredClicks.get()
            repeat(3){tap(rect.exactCenterX(),rect.exactCenterY());SystemClock.sleep(180)}
            assertEquals("Touch A must not also click the world",controlsBefore,GameSurface.deliveredClicks.get())
            instrumentation.uiAutomation.performGlobalAction(android.accessibilityservice.AccessibilityService.GLOBAL_ACTION_BACK)
            SystemClock.sleep(500)
            val menuBefore=GameSurface.deliveredClicks.get()
            tap(360f,340f)
            SystemClock.sleep(300)
            assertEquals("Pause UI must not click the world",menuBefore,GameSurface.deliveredClicks.get())
            println("FFTA_TOUCH_DEVICE game=one overlay=excluded pause=excluded")
        }finally{
            instrumentation.runOnMainSync{activity.finish()}
            prefs.edit().putBoolean("experimental_clicks",old).commit()
        }
    }
}
