package com.fftarecomp.port

import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioTrack
import android.os.Process
import java.util.concurrent.locks.LockSupport

/** Desktop-style x1 audio consumer, independent of fast-forward simulation. */
class NormalAudioOutput(private val onError:(String)->Unit):AutoCloseable {
    @Volatile var active=false
    @Volatile var volume=0.8f
    @Volatile var writtenSamples=0L; private set
    @Volatile var playedPairs=0L; private set
    @Volatile var underruns=0; private set
    @Volatile private var running=true
    private val worker=Thread(::run,"FFTA-normal-audio").apply{start()}
    private fun run(){
        Process.setThreadPriority(Process.THREAD_PRIORITY_AUDIO)
        val samples=ShortArray(2048)
        var track:AudioTrack?=null
        var playing=false
        var previousVolume=-1f
        try {
            while(running){
                if(!active){
                    if(playing){track?.pause();track?.flush();playing=false}
                    LockSupport.parkNanos(2_000_000);continue
                }
                val count=NativeBridge.gbaPullNormalAudio(samples)
                check(count>=0){NativeBridge.gbaStatus()}
                if(count==0){LockSupport.parkNanos(2_000_000);continue}
                if(track==null){
                    val minimum=AudioTrack.getMinBufferSize(48000,AudioFormat.CHANNEL_OUT_STEREO,AudioFormat.ENCODING_PCM_16BIT).coerceAtLeast(4096)
                    track=AudioTrack.Builder().setAudioAttributes(AudioAttributes.Builder()
                        .setUsage(AudioAttributes.USAGE_GAME).setContentType(AudioAttributes.CONTENT_TYPE_MUSIC).build())
                        .setAudioFormat(AudioFormat.Builder().setSampleRate(48000)
                            .setChannelMask(AudioFormat.CHANNEL_OUT_STEREO).setEncoding(AudioFormat.ENCODING_PCM_16BIT).build())
                        .setBufferSizeInBytes(minimum*2).build()
                }
                val level=volume.coerceIn(0f,1f)
                if(level!=previousVolume){track.setVolume(level);previousVolume=level}
                var offset=0
                while(offset<count && active && running){
                    val written=track.write(samples,offset,count-offset,AudioTrack.WRITE_BLOCKING)
                    check(written>=0){"Normal audio output error: $written"}
                    if(written==0){LockSupport.parkNanos(1_000_000);continue}
                    offset+=written
                    writtenSamples+=written
                    if(!playing){track.play();playing=true}
                    playedPairs=track.playbackHeadPosition.toLong() and 0xffffffffL
                    underruns=track.underrunCount
                }
            }
        } catch(error:Throwable){if(running)onError("Normal audio: ${error.message}")}
        finally{try{track?.pause();track?.flush();track?.release()}catch(_:Exception){}}
    }
    override fun close(){
        active=false
        running=false
        worker.interrupt()

        // GameSurface interrupts its simulation thread while detaching. That
        // thread calls close() from finally with its interrupt flag already
        // set; Thread.join then throws immediately unless we temporarily clear
        // and restore the flag. Never let audio shutdown crash the app.
        val caller=Thread.currentThread()
        var interrupted=Thread.interrupted()
        val deadline=System.nanoTime()+1_000_000_000L
        try {
            while(worker.isAlive) {
                val remaining=deadline-System.nanoTime()
                if(remaining<=0L)break
                val millis=remaining/1_000_000L
                val nanos=(remaining%1_000_000L).toInt()
                try { worker.join(millis,nanos) }
                catch(_:InterruptedException) {
                    interrupted=true
                    worker.interrupt()
                }
            }
        } finally {
            if(interrupted)caller.interrupt()
        }
    }
}
