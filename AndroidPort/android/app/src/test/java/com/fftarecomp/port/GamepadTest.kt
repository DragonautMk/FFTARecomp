package com.fftarecomp.port
import android.view.MotionEvent
import org.junit.Assert.*
import org.junit.Test

class GamepadTest {
    @Test fun eventSourcesRecognizeControllersEvenWhenDeviceMetadataIsMissing(){
        assertTrue(ControllerInput.recognizes(android.view.InputDevice.SOURCE_GAMEPAD,0))
        assertTrue(ControllerInput.recognizes(android.view.InputDevice.SOURCE_JOYSTICK,0))
        assertTrue(ControllerInput.recognizes(android.view.InputDevice.SOURCE_KEYBOARD,0x01000511))
        assertFalse(ControllerInput.recognizes(android.view.InputDevice.SOURCE_TOUCHSCREEN,0))
        assertFalse(ControllerInput.recognizes(android.view.InputDevice.SOURCE_KEYBOARD,0))
    }

    @Test fun defaultLayoutMatchesDesktop(){
        val pad=PadBindings()
        assertEquals(PadControl.LT,pad.bindings[PadAction.L])
        assertEquals(PadControl.RT,pad.bindings[PadAction.R])
        assertEquals(PadControl.LB,pad.bindings[PadAction.ZOOM_OUT])
        assertEquals(PadControl.RB,pad.bindings[PadAction.ZOOM_IN])
        assertEquals(PadControl.RS,pad.bindings[PadAction.TOGGLE_FAST])
    }
    @Test fun heldButtonsAndDuplicateAnalogReportsDoNotRepeatActions(){
        val pad=PadBindings()
        pad.assign(PadAction.TOGGLE_FAST,PadControl.LT)
        assertEquals(listOf(PadAction.TOGGLE_FAST to true),pad.update("key",setOf(PadControl.LT)))
        assertTrue(pad.update("key",setOf(PadControl.LT)).isEmpty())
        assertTrue(pad.update("axis",setOf(PadControl.LT)).isEmpty())
        assertTrue(pad.update("key",emptySet()).isEmpty())
        assertEquals(listOf(PadAction.TOGGLE_FAST to false),pad.update("axis",emptySet()))
    }
    @Test fun remappingSwapsOccupiedButtonsAndCanBeReset(){
        val pad=PadBindings()
        pad.assign(PadAction.A,PadControl.B)
        assertEquals(PadControl.B,pad.bindings[PadAction.A])
        assertEquals(PadControl.A,pad.bindings[PadAction.B])
        assertEquals(listOf(PadAction.A to true),pad.update("key",setOf(PadControl.B)))
        pad.defaults()
        assertEquals(PadControl.A,pad.bindings[PadAction.A])
        assertEquals(PadControl.B,pad.bindings[PadAction.B])
    }
    @Test fun triggersHatAndBothSticksAreRecognized(){
        val values=mapOf(MotionEvent.AXIS_LTRIGGER to 1f,MotionEvent.AXIS_HAT_X to -1f,
            MotionEvent.AXIS_Y to 1f,MotionEvent.AXIS_Z to 1f,MotionEvent.AXIS_RZ to -1f)
        assertEquals(setOf(PadControl.LT,PadControl.LEFT,PadControl.LS_DOWN,PadControl.RS_RIGHT,PadControl.RS_UP),
            PadControl.axes{values[it]?:0f})
    }
    @Test fun leftStickAndDpadReleaseIndependently(){
        val pad=PadBindings()
        assertEquals(listOf(PadAction.UP to true),pad.update("axis",setOf(PadControl.LS_UP)))
        assertTrue(pad.update("key",setOf(PadControl.UP)).isEmpty())
        assertTrue(pad.update("axis",emptySet()).isEmpty())
        assertEquals(listOf(PadAction.UP to false),pad.update("key",emptySet()))
    }
}
