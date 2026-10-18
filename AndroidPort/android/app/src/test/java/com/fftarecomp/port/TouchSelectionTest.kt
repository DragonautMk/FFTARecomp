package com.fftarecomp.port

import org.junit.Assert.*
import org.junit.Test

class TouchSelectionTest {
    @Test fun letterboxBarsAreNotGameClicks(){
        val view=ViewConfig(0,0)
        assertNull(TouchSelection.map(10f,10f,720,720,view))
        val middle=TouchSelection.map(360f,360f,720,720,view)!!
        assertEquals(.5f,middle.first,.001f)
        assertEquals(.5f,middle.second,.001f)
    }
    @Test fun zoomAndFullscreenUseTheDisplayedViewport(){
        for(zoom in 0..6){
            val view=ViewConfig.create(AspectMode.DEVICE,zoom,720,720)
            val middle=TouchSelection.map(360f,360f,720,720,view)!!
            assertEquals(.5f,middle.first,.002f)
            assertEquals(.5f,middle.second,.002f)
        }
        assertNull(TouchSelection.map(Float.NaN,0f,720,720,ViewConfig(0,0)))
    }
}
