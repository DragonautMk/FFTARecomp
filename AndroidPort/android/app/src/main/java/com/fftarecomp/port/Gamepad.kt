package com.fftarecomp.port
import android.view.KeyEvent
import android.view.MotionEvent

enum class PadControl(val label:String,val key:Int=0) {
    A("A",96),B("B",97),X("X",99),Y("Y",100),
    LB("LB",102),RB("RB",103),LT("LT",104),RT("RT",105),
    BACK("Back",109),START("Start",108),GUIDE("Guide",110),
    LS("LS click",106),RS("RS click",107),
    UP("D-pad up",19),DOWN("D-pad down",20),LEFT("D-pad left",21),RIGHT("D-pad right",22),
    LS_UP("LS up"),LS_DOWN("LS down"),LS_LEFT("LS left"),LS_RIGHT("LS right"),
    RS_UP("RS up"),RS_DOWN("RS down"),RS_LEFT("RS left"),RS_RIGHT("RS right");
    companion object {
        fun fromKey(key:Int)=entries.firstOrNull{it.key!=0 && it.key==key}
        fun axes(value:(Int)->Float):Set<PadControl> = buildSet {
            fun stick(x:Float,y:Float,left:PadControl,right:PadControl,up:PadControl,down:PadControl){
                if(x< -0.5f)add(left);if(x>0.5f)add(right)
                if(y< -0.5f)add(up);if(y>0.5f)add(down)
            }
            stick(value(MotionEvent.AXIS_HAT_X),value(MotionEvent.AXIS_HAT_Y),LEFT,RIGHT,UP,DOWN)
            stick(value(MotionEvent.AXIS_X),value(MotionEvent.AXIS_Y),LS_LEFT,LS_RIGHT,LS_UP,LS_DOWN)
            stick(value(MotionEvent.AXIS_Z),value(MotionEvent.AXIS_RZ),RS_LEFT,RS_RIGHT,RS_UP,RS_DOWN)
            if(maxOf(value(MotionEvent.AXIS_LTRIGGER),value(MotionEvent.AXIS_BRAKE))>0.5f)add(LT)
            if(maxOf(value(MotionEvent.AXIS_RTRIGGER),value(MotionEvent.AXIS_GAS))>0.5f)add(RT)
        }
    }
}
enum class PadAction(val label:String,val default:PadControl,val mask:Int=0){
    A("A",PadControl.A,1),B("B",PadControl.B,2),
    L("L",PadControl.LT,512),R("R",PadControl.RT,256),
    START("Start",PadControl.START,8),SELECT("Select",PadControl.BACK,4),
    UP("Up",PadControl.UP,64),DOWN("Down",PadControl.DOWN,128),
    LEFT("Left",PadControl.LEFT,32),RIGHT("Right",PadControl.RIGHT,16),
    ZOOM_IN("Zoom in (+)",PadControl.RB),ZOOM_OUT("Zoom out (-)",PadControl.LB),
    FAST("Fast forward",PadControl.LS),TOGGLE_FAST("Toggle fast forward",PadControl.RS),
    PAUSE("Pause",PadControl.GUIDE)
}
/** Merge keys and analog axes so dual LT key+axis reports fire only once. */
class PadBindings {
    val bindings=PadAction.entries.associateWith{it.default}.toMutableMap()
    private val sources=mutableMapOf<String,Set<PadControl>>()
    private var previous=emptySet<PadAction>()
    var leftStickMovement=true
    fun assign(action:PadAction,button:PadControl){
        val old=bindings.getValue(action)
        bindings.entries.firstOrNull{it.key!=action && it.value==button}?.let{bindings[it.key]=old}
        bindings[action]=button;clear()
    }
    fun defaults(){bindings.clear();bindings.putAll(PadAction.entries.associateWith{it.default});leftStickMovement=true;clear()}
    fun clear(){sources.clear();previous=emptySet()}
    fun update(source:String,held:Set<PadControl>):List<Pair<PadAction,Boolean>>{
        if(held.isEmpty())sources.remove(source)else sources[source]=held
        val physical=sources.values.flatten().toSet()
        val current=bindings.filter{(action,button)->
            button in physical || (leftStickMovement && button==action.default && when(action){
                PadAction.UP->PadControl.LS_UP in physical
                PadAction.DOWN->PadControl.LS_DOWN in physical
                PadAction.LEFT->PadControl.LS_LEFT in physical
                PadAction.RIGHT->PadControl.LS_RIGHT in physical
                else->false
            })
        }.keys
        val changes=PadAction.entries.filter{(it in current)!=(it in previous)}.map{it to (it in current)}
        previous=current.toSet();return changes
    }
}

object ControllerInput {
    fun recognizes(eventSources:Int,deviceSources:Int):Boolean{
        val sources=eventSources or deviceSources
        return sources and android.view.InputDevice.SOURCE_GAMEPAD==android.view.InputDevice.SOURCE_GAMEPAD ||
            sources and android.view.InputDevice.SOURCE_JOYSTICK==android.view.InputDevice.SOURCE_JOYSTICK
    }
}
