package com.fftarecomp.port

data class ScreenTap(val x:Float,val y:Float,val back:Boolean=false)

/** Inverse of the actual GL viewport, including letterboxing and full-screen crop. */
object TouchSelection {
    fun map(x:Float,y:Float,width:Int,height:Int,view:ViewConfig):Pair<Float,Float>? {
        if(width<=0 || height<=0 || !x.isFinite() || !y.isFinite())return null
        val scale=if(view.fillScreen)maxOf(width.toFloat()/view.width,height.toFloat()/view.height)
            else minOf(width.toFloat()/view.width,height.toFloat()/view.height)
        val w=(view.width*scale).toInt();val h=(view.height*scale).toInt()
        val nx=(x-(width-w)/2)/w;val ny=(y-(height-h)/2)/h
        return if(nx>=0f && nx<1f && ny>=0f && ny<1f)nx to ny else null
    }
}
