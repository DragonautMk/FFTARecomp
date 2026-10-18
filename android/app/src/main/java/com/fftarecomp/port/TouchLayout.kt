package com.fftarecomp.port

import android.content.Context
import androidx.compose.runtime.*

enum class TouchControl(val label:String,val width:Int,val height:Int,val x:Float,val y:Float,
    val mask:Int=0,val visibleByDefault:Boolean=true){
    PAUSE("Pause",86,40,0f,0f),
    FAST_TOGGLE("Toggle FF",72,40,.80f,0f),
    ZOOM_OUT("-",52,40,.91f,0f),ZOOM_IN("+",52,40,1f,0f),
    L("L",72,36,0f,.16f,512),R("R",72,36,1f,.16f,256),
    DPAD("D-pad",132,132,0f,1f),
    SELECT("Select",66,36,.44f,1f,4),START("Start",66,36,.55f,1f,8),
    B("B",58,58,.87f,.98f,2),A("A",64,64,1f,.90f,1),
    FAST_HOLD("Hold FF",72,40,.68f,0f,visibleByDefault=false)
}
data class TouchPlacement(val x:Float,val y:Float,val visible:Boolean=true){
    fun bounded()=copy(x=if(x.isFinite())x.coerceIn(0f,1f)else 0f,
        y=if(y.isFinite())y.coerceIn(0f,1f)else 0f)
    fun move(dx:Float,dy:Float,availableWidth:Float,availableHeight:Float)=copy(
        x=x+if(availableWidth>0f)dx/availableWidth else 0f,
        y=y+if(availableHeight>0f)dy/availableHeight else 0f).bounded()
}
class TouchPositions {
    val positions=mutableStateMapOf<TouchControl,TouchPlacement>()
    init{reset()}
    fun reset(){positions.clear();TouchControl.entries.forEach{positions[it]=TouchPlacement(it.x,it.y,it.visibleByDefault)}}
    fun snapshot():Map<TouchControl,TouchPlacement> = positions.toMap()
    fun restore(snapshot:Map<TouchControl,TouchPlacement>){
        reset();snapshot.forEach{(key,value)->positions[key]=value.bounded()}
    }
}
@Stable
class TouchLayout(context:Context){
    private val prefs=context.getSharedPreferences("touch_layout_v1",Context.MODE_PRIVATE)
    val layout=TouchPositions().apply{
        TouchControl.entries.forEach{control->
            positions[control]=TouchPlacement(prefs.getFloat(control.name+"_x",control.x),
                prefs.getFloat(control.name+"_y",control.y),
                prefs.getBoolean(control.name+"_visible",control.visibleByDefault)).bounded()
        }
    }
    var vibration by mutableStateOf(prefs.getBoolean("vibration",false))
    fun save(){
        prefs.edit().apply{
            putBoolean("vibration",vibration)
            layout.positions.forEach{(control,pose)->
                putFloat(control.name+"_x",pose.x);putFloat(control.name+"_y",pose.y)
                putBoolean(control.name+"_visible",pose.visible)
            }
        }.apply()
    }
}
