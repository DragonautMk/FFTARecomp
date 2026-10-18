package com.fftarecomp.port
import android.app.Activity
import android.content.Context
import java.io.File
/** Debug-only deterministic replay. Never uses the user's normal save file. */
class GameBenchmark private constructor(context:Context,val frames:Int,val paced:Boolean,val view:ViewConfig){
    private val points:List<Pair<Long,Int>>
    private val report=File(context.cacheDir,"benchmark.csv")
    private val checkpoints=File(context.cacheDir,"benchmark-hashes.txt")
    private val camera=File(context.cacheDir,"benchmark-camera.csv").bufferedWriter()
    private val times=ArrayList<Long>(120)
    private var cursor=0
    var frame=0;private set
    init{
        val lines=File(context.filesDir,"benchmark.input").readLines()
        require(lines.firstOrNull()?.trim()=="gba-input v1"){"Invalid benchmark input"}
        points=lines.drop(1).filter{it.isNotBlank()}.map{
            val fields=it.trim().split(Regex("\\s+"))
            fields[0].toLong() to fields[1].toInt(16)
        }.sortedBy{it.first}
        require(points.isNotEmpty()){"Benchmark input contains no events"}
        camera.write("frame,hofs,vofs,keys,frame_ms\n")
        report.writeText("frame,mean_ms,p95_ms,max_ms\n")
        checkpoints.writeText("Debug replay; isolated benchmark.sav\n")
    }
    fun keys():Int{
        while(cursor<points.size && points[cursor].first<=frame)cursor++
        return if(cursor==0)0x3ff else points[cursor-1].second
    }
    fun record(nanos:Long){
        val position=NativeBridge.gbaCamera()
        camera.write(listOf(frame,if(position<0)-1 else position and 511,
            if(position<0)-1 else (position ushr 9) and 511,keys(),nanos/1e6).joinToString(",")+"\n")
        times.add(nanos);frame++
        if(times.size>=120||frame==frames){
            val sorted=times.sorted()
            report.appendText(listOf(frame,times.average()/1e6,sorted[((sorted.size-1)*.95).toInt()]/1e6,sorted.last()/1e6).joinToString(",")+"\n")
            times.clear();camera.flush()
        }
        if(frame==600||frame==frames-1200||frame==frames-600||frame==frames){
            checkpoints.appendText("Replay frame "+frame+"\n"+NativeBridge.gbaDiagnostics())
        }
        if(frame==frames){camera.close();checkpoints.appendText("COMPLETE\n")}
    }
    companion object{
        fun enabled(context:Context)=BuildConfig.DEBUG &&
            (context as? Activity)?.intent?.getBooleanExtra("ffta_benchmark",false)==true
        fun create(context:Context):GameBenchmark?{
            if(!enabled(context))return null
            val intent=(context as Activity).intent
            NativeBridge.gbaParallelZoom(intent.getBooleanExtra("ffta_benchmark_parallel",true))
            return GameBenchmark(context,intent.getIntExtra("ffta_benchmark_frames",9600).coerceIn(600,30000),
                intent.getBooleanExtra("ffta_benchmark_clock",false),
                ViewConfig(intent.getIntExtra("ffta_benchmark_extra_x",8).coerceIn(0,648),
                    intent.getIntExtra("ffta_benchmark_extra_y",48).coerceIn(0,320)))
        }
    }
}
