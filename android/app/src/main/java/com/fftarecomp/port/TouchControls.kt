package com.fftarecomp.port

import android.view.HapticFeedbackConstants
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.gestures.awaitEachGesture
import androidx.compose.foundation.gestures.awaitFirstDown
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.gestures.waitForUpOrCancellation
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.LocalView
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.window.Dialog
import kotlin.math.roundToInt

private val TouchInk=Color(0xFF302519)
private val TouchPaper=Color(0xFFF6EBD0)
private val TouchGold=Color(0xFFB68B35)

@Composable
fun TouchscreenSettings(settings:GameSettings,onEdit:()->Unit){
    OutlinedButton(onClick=onEdit,modifier=Modifier.fillMaxWidth()){Text("Edit layout")}
    Row(verticalAlignment=Alignment.CenterVertically){
        Switch(settings.controls,{settings.controls=it;settings.save()});Text("Touch controls")
    }
    Row(verticalAlignment=Alignment.CenterVertically){
        Switch(settings.touch.vibration,{settings.touch.vibration=it;settings.touch.save()})
        Text("Vibration feedback")
    }
    Text("Opacity")
    Slider(settings.opacity,{settings.opacity=it},valueRange=0.2f..1f,onValueChangeFinished={settings.save()})
    Text("Size")
    Slider(settings.controlSize,{settings.controlSize=it},valueRange=0.75f..1.25f,onValueChangeFinished={settings.save()})
    Text("Move controls and choose which buttons appear in Edit layout. Android Back always opens Pause.",fontSize=12.sp)
}

@Composable
fun TouchOverlay(settings:GameSettings,input:GameInput,editing:Boolean,onPause:()->Unit,
    onSave:()->Unit={},onCancel:()->Unit={}){
    val density=LocalDensity.current
    val view=LocalView.current
    val haptic by rememberUpdatedState(settings.touch.vibration)
    val feedback:()->Unit={if(haptic)view.performHapticFeedback(HapticFeedbackConstants.VIRTUAL_KEY);Unit}
    var selected by remember{mutableStateOf<TouchControl?>(null)}
    var showButtons by remember{mutableStateOf(false)}
    var panelY by remember{mutableFloatStateOf(0f)}
    Box(Modifier.fillMaxSize()){
        if(editing)Box(Modifier.fillMaxSize().background(Color(0x4014231C)))
        BoxWithConstraints(Modifier.fillMaxSize().safeDrawingPadding().padding(8.dp)){
            val canvasWidth=with(density){maxWidth.toPx()}
            val canvasHeight=with(density){maxHeight.toPx()}
            TouchControl.entries.forEach{control->
                val pose=settings.touch.layout.positions.getValue(control)
                val utility=control in listOf(TouchControl.PAUSE,TouchControl.FAST_TOGGLE,TouchControl.ZOOM_IN,TouchControl.ZOOM_OUT)
                if(pose.visible&&(editing||utility||settings.controls)){
                    val scale=if(control==TouchControl.A||control==TouchControl.B||control==TouchControl.DPAD)settings.controlSize else 1f
                    val w=(control.width*scale).dp;val h=(control.height*scale).dp
                    val availableX=(canvasWidth-with(density){w.toPx()}).coerceAtLeast(0f)
                    val availableY=(canvasHeight-with(density){h.toPx()}).coerceAtLeast(0f)
                    val draggable=if(editing)Modifier.pointerInput(control,availableX,availableY){
                        detectDragGestures(onDragStart={selected=control},onDrag={change,amount->
                            change.consume()
                            val current=settings.touch.layout.positions.getValue(control)
                            settings.touch.layout.positions[control]=current.move(amount.x,amount.y,availableX,availableY)
                        })
                    }else Modifier
                    val modifier=Modifier.offset{IntOffset((pose.x*availableX).roundToInt(),(pose.y*availableY).roundToInt())}
                        .size(w,h).then(draggable)
                        .then(if(editing&&selected==control)Modifier.border(2.dp,TouchGold,RoundedCornerShape(12.dp))else Modifier)
                        .semantics{contentDescription=if(editing)"Move "+control.label else control.label}
                    val label=when(control){
                        TouchControl.FAST_TOGGLE->if(settings.fastForward)settings.multiplier.toString()+"x ON" else ">>"
                        TouchControl.FAST_HOLD->"Hold >>"
                        else->control.label
                    }
                    if(control==TouchControl.DPAD){
                        TouchDirectionPad(modifier,settings,input,editing,feedback)
                    }else{
                        val onPressed:(Boolean)->Unit={down->
                            when{
                                control.mask!=0->input.set("touch:"+control.name,if(down)control.mask else 0)
                                control==TouchControl.FAST_HOLD->settings.holdFast=down
                                down->when(control){
                                    TouchControl.PAUSE->onPause()
                                    TouchControl.FAST_TOGGLE->settings.fastForward=!settings.fastForward
                                    TouchControl.ZOOM_IN->settings.zoomBy(-1)
                                    TouchControl.ZOOM_OUT->settings.zoomBy(1)
                                    else->Unit
                                }
                            }
                        }
                        DisposableEffect(control,editing){
                            onDispose{
                                if(control.mask!=0)input.set("touch:"+control.name,0)
                                if(control==TouchControl.FAST_HOLD)settings.holdFast=false
                            }
                        }
                        TouchKey(label,modifier,settings.opacity,editing,feedback,onPressed)
                    }
                }
            }
            if(editing){
                Column(Modifier.align(Alignment.Center).offset{IntOffset(0,panelY.roundToInt())}.widthIn(max=440.dp)
                    .background(TouchPaper,RoundedCornerShape(12.dp)).border(1.dp,TouchGold,RoundedCornerShape(12.dp))
                    .padding(8.dp),horizontalAlignment=Alignment.CenterHorizontally){
                    Text("Edit touchscreen",Modifier.pointerInput(canvasHeight){
                        detectDragGestures{change,amount->
                            change.consume()
                            val limit=(canvasHeight/2-with(density){64.dp.toPx()}).coerceAtLeast(0f)
                            panelY=(panelY+amount.y).coerceIn(-limit,limit)
                        }
                    },color=TouchInk,fontWeight=FontWeight.Bold,fontSize=16.sp)
                    Text("Paused - drag controls or this title",color=TouchInk,fontSize=11.sp)
                    Row(horizontalArrangement=Arrangement.spacedBy(4.dp)){
                        TextButton(onClick={showButtons=true}){Text("Buttons")}
                        TextButton(onClick={settings.touch.layout.reset()}){Text("Default")}
                        TextButton(onClick=onCancel){Text("Cancel")}
                        Button(onClick=onSave,contentPadding=PaddingValues(horizontal=12.dp)){Text("Save")}
                    }
                }
            }
        }
        if(showButtons){
            Dialog(onDismissRequest={showButtons=false}){
                Column(Modifier.heightIn(max=320.dp).background(TouchPaper,RoundedCornerShape(16.dp))
                    .padding(16.dp)){
                    Text("Visible buttons",fontSize=21.sp,color=TouchInk)
                    Column(Modifier.weight(1f,fill=false).verticalScroll(rememberScrollState())){
                    TouchControl.entries.forEach{control->
                        val pose=settings.touch.layout.positions.getValue(control)
                        Row(verticalAlignment=Alignment.CenterVertically){
                            Checkbox(pose.visible,{settings.touch.layout.positions[control]=pose.copy(visible=it)})
                            Text(control.label,color=TouchInk)
                        }
                    }
                    Row(verticalAlignment=Alignment.CenterVertically){
                        Switch(settings.touch.vibration,{settings.touch.vibration=it})
                        Text("Vibration feedback",color=TouchInk)
                    }
                    }
                    Button(onClick={showButtons=false},modifier=Modifier.fillMaxWidth()){Text("Done")}
                }
            }
        }
    }
}

@Composable
private fun TouchKey(label:String,modifier:Modifier,opacity:Float,editing:Boolean,
    feedback:()->Unit,onPressed:(Boolean)->Unit){
    val press by rememberUpdatedState(onPressed)
    val buzz by rememberUpdatedState(feedback)
    val interaction=if(editing)Modifier else Modifier.pointerInput(Unit){
        awaitEachGesture{
            awaitFirstDown(requireUnconsumed=false)
            buzz();press(true)
            try{waitForUpOrCancellation()}finally{press(false)}
        }
    }
    Box(modifier.background(TouchInk.copy(alpha=opacity),RoundedCornerShape(18.dp))
        .border(1.dp,TouchPaper.copy(alpha=.6f),RoundedCornerShape(18.dp)).then(interaction),
        contentAlignment=Alignment.Center){
        Text(label,color=TouchPaper,fontSize=if(label=="A"||label=="B")22.sp else 12.sp,fontWeight=FontWeight.Bold)
    }
}
@Composable
private fun TouchDirectionPad(modifier:Modifier,settings:GameSettings,input:GameInput,editing:Boolean,feedback:()->Unit){
    val buzz by rememberUpdatedState(feedback)
    DisposableEffect(input,editing){onDispose{input.set("dpad",0)}}
    val interaction=if(editing)Modifier else Modifier.pointerInput(Unit){
        var previous=0
        try{
            awaitPointerEventScope{
                while(true){
                    val event=awaitPointerEvent()
                    val pointer=event.changes.firstOrNull{it.pressed}
                    var mask=0
                    if(pointer!=null){
                        if(pointer.position.x<size.width/3f)mask=mask or 32
                        if(pointer.position.x>size.width*2/3f)mask=mask or 16
                        if(pointer.position.y<size.height/3f)mask=mask or 64
                        if(pointer.position.y>size.height*2/3f)mask=mask or 128
                    }
                    if(mask!=0&&mask!=previous)buzz()
                    previous=mask;input.set("dpad",mask)
                    event.changes.forEach{it.consume()}
                }
            }
        }finally{input.set("dpad",0)}
    }
    Box(modifier.then(interaction)){
        Canvas(Modifier.fillMaxSize()){
            val color=TouchInk.copy(alpha=settings.opacity)
            drawRoundRect(color,Offset(size.width/3,0f),Size(size.width/3,size.height),androidx.compose.ui.geometry.CornerRadius(12f))
            drawRoundRect(color,Offset(0f,size.height/3),Size(size.width,size.height/3),androidx.compose.ui.geometry.CornerRadius(12f))
        }
        Text("\u25b2",Modifier.align(Alignment.TopCenter).padding(top=8.dp),color=TouchPaper)
        Text("\u25bc",Modifier.align(Alignment.BottomCenter).padding(bottom=8.dp),color=TouchPaper)
        Text("\u25c0",Modifier.align(Alignment.CenterStart).padding(start=8.dp),color=TouchPaper)
        Text("\u25b6",Modifier.align(Alignment.CenterEnd).padding(end=8.dp),color=TouchPaper)
    }
}
