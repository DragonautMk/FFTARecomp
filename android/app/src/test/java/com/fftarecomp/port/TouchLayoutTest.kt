package com.fftarecomp.port
import org.junit.Assert.*
import org.junit.Test

class TouchLayoutTest {
    @Test fun draggingIsResolutionIndependentAndKeepsControlsOnScreen(){
        val start=TouchPlacement(.5f,.5f)
        assertEquals(TouchPlacement(.6f,.7f),start.move(100f,100f,1000f,500f))
        assertEquals(start.move(100f,100f,1000f,500f),start.move(200f,200f,2000f,1000f))
        assertEquals(TouchPlacement(1f,0f),start.move(99999f,-99999f,1000f,500f))
        assertEquals(start,start.move(100f,100f,0f,0f))
    }
    @Test fun cancelRestoresPositionsAndHiddenButtonsAndDefaultRestoresInitialLayout(){
        val layout=TouchPositions()
        val original=layout.snapshot()
        layout.positions[TouchControl.A]=TouchPlacement(.1f,.2f,false)
        assertNotEquals(original,layout.snapshot())
        layout.restore(original)
        assertEquals(original,layout.snapshot())
        layout.positions[TouchControl.DPAD]=TouchPlacement(.5f,.2f,false)
        layout.reset()
        assertEquals(original,layout.snapshot())
        assertTrue(layout.positions.getValue(TouchControl.PAUSE).visible)
        assertFalse(layout.positions.getValue(TouchControl.FAST_HOLD).visible)
    }
    @Test fun invalidStoredCoordinatesCannotMakeAControlUnreachable(){
        assertEquals(TouchPlacement(0f,1f),TouchPlacement(Float.NaN,2f).bounded())
    }
}
