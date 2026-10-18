package com.fftarecomp.port

import android.content.Context
import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioTrack
import android.opengl.GLES20.*
import android.opengl.GLSurfaceView
import android.os.Process
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.FloatBuffer
import java.util.concurrent.atomic.AtomicInteger
import java.util.concurrent.locks.LockSupport
import javax.microedition.khronos.egl.EGLConfig
import javax.microedition.khronos.opengles.GL10

/** Simulation and audio run together off the UI thread; GL only uploads completed frames. */
class GameSurface(context: Context, val onError: (String) -> Unit) : GLSurfaceView(context) {
    val buttons = AtomicInteger(0)
    @Volatile var inputSource: AtomicInteger = buttons
    @Volatile var gamePaused = false
    @Volatile var appActive = true
    @Volatile var viewRequest = ViewRequest(AspectMode.DEVICE,0)
    @Volatile var speed = 1
    @Volatile var audioMode=FastAudio.NORMAL
    @Volatile var soundEnabled = true
    @Volatile var volume = 0.8f
    @Volatile var performance = "Starting..."
    @Volatile private var running = false
    private var worker: Thread? = null
    private val frames = FrameRenderer(onError)

    init {
        setEGLContextClientVersion(2)
        preserveEGLContextOnPause = true
        setRenderer(frames)
        renderMode = RENDERMODE_WHEN_DIRTY
    }

    override fun onAttachedToWindow() {
        super.onAttachedToWindow()
        running = true
        worker = Thread(::runGame, "FFTA-game").apply { start() }
    }

    override fun onDetachedFromWindow() {
        running = false
        buttons.set(0)
        worker?.interrupt()
        worker?.join(2000)
        worker = null
        super.onDetachedFromWindow()
    }

    private fun runGame() {
        Process.setThreadPriority(Process.THREAD_PRIORITY_DISPLAY)
        val benchmark=GameBenchmark.create(context)
        val sampleBuffer = ShortArray(4096)
        // Small bounded queue absorbs AudioTrack partial writes without accumulating latency.
        val queued = ShortArray(16384)
        var queuedCount = 0
        var back: ByteBuffer? = null
        val track = try {
            val minimum = AudioTrack.getMinBufferSize(48000, AudioFormat.CHANNEL_OUT_STEREO,
                AudioFormat.ENCODING_PCM_16BIT).coerceAtLeast(4096)
            AudioTrack.Builder()
                .setAudioAttributes(AudioAttributes.Builder().setUsage(AudioAttributes.USAGE_GAME)
                    .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC).build())
                .setAudioFormat(AudioFormat.Builder().setSampleRate(48000)
                    .setChannelMask(AudioFormat.CHANNEL_OUT_STEREO)
                    .setEncoding(AudioFormat.ENCODING_PCM_16BIT).build())
                .setBufferSizeInBytes(minimum * 2).build()
        } catch (error: Exception) { post { onError("Audio initialization: ${error.message}") }; null }
        var audible = false
        var lastAudioKey=-1
        var deadline = System.nanoTime()
        var count = 0
        var sampleStart = deadline
        var simulationNanos = 0L
        var wasPaused = false
        try {
            while (running) {
                if (gamePaused || !appActive) {
                    if (!wasPaused) {
                        NativeBridge.gbaSave()
                        buttons.set(0)
                        track?.pause(); track?.flush()
                        queuedCount = 0
                        audible = false
                        wasPaused = true
                    }
                    LockSupport.parkNanos(10_000_000)
                    deadline = System.nanoTime()
                    sampleStart = deadline; count = 0; simulationNanos = 0
                    continue
                }
                wasPaused = false
                val request=viewRequest
                val config = benchmark?.view ?: ViewConfig.create(request.aspect,request.zoom,frames.screenWidth,frames.screenHeight)
                val bytes = config.width * config.height * 4
                if (back == null || back.capacity() != bytes) {
                    back = ByteBuffer.allocateDirect(bytes).order(ByteOrder.nativeOrder())
                }
                val multiplier = if(benchmark!=null)1 else speed.coerceIn(1, 8)
                val mode=audioMode
                val sound = soundEnabled && (multiplier==1 || mode!=FastAudio.SILENT) && track != null
                val audioKey=if(!sound)0 else if(multiplier==1)1 else 10+mode.ordinal*10+multiplier
                if(audioKey!=lastAudioKey) {
                    track?.pause();track?.flush();queuedCount=0;audible=false;lastAudioKey=audioKey
                }
                val start = System.nanoTime()
                val samples = NativeBridge.gbaFrame(benchmark?.keys() ?: (inputSource.get().inv() and 0x3ff),
                    config.extraX, config.extraY, multiplier, sound, if(mode==FastAudio.NORMAL)2 else 1, back, sampleBuffer)
                val frameNanos=System.nanoTime()-start
                simulationNanos += frameNanos
                benchmark?.record(frameNanos)
                if (samples < 0) throw IllegalStateException(NativeBridge.gbaStatus())
                back = frames.publish(back, config)
                requestRender()
                if (sound) {
                    if (!audible) { track?.play(); audible = true }
                    track?.setVolume(volume.coerceIn(0f, 1f))
                    if (queuedCount + samples > queued.size) queuedCount = 0
                    sampleBuffer.copyInto(queued, queuedCount, 0, samples)
                    queuedCount += samples
                    val written = track?.write(queued, 0, queuedCount, AudioTrack.WRITE_NON_BLOCKING) ?: 0
                    if (written > 0) {
                        queued.copyInto(queued, 0, written, queuedCount)
                        queuedCount -= written
                    }
                } else if (audible) {
                    track?.pause(); track?.flush(); queuedCount = 0; audible = false
                }
                if(benchmark!=null && benchmark.frame>=benchmark.frames){gamePaused=true;continue}
                count++
                if (count >= 120) {
                    val elapsed = System.nanoTime() - sampleStart
                    performance = "Game loop: %.1f FPS\nSimulation: %.2f ms/frame\nSpeed: %dx\nAudio underruns: %d\n".format(
                        count * 1e9 / elapsed, simulationNanos / 1e6 / count / multiplier,
                        multiplier, track?.underrunCount ?: 0)
                    NativeBridge.gbaSave()
                    if(BuildConfig.DEBUG) {
                        java.io.File(context.cacheDir,"performance.txt").writeText(NativeBridge.gbaDiagnostics()+performance)
                    }
                    count = 0; simulationNanos = 0; sampleStart = System.nanoTime()
                }
                // Actual GBA cadence: 280896 cycles / 16777216 Hz = 59.7275 FPS.
                if(benchmark!=null && !benchmark.paced)continue
                deadline += 16_742_706L
                val now = System.nanoTime()
                if (now - deadline > 4 * 16_742_706L) deadline = now
                while (running && System.nanoTime() < deadline) {
                    LockSupport.parkNanos((deadline-System.nanoTime()).coerceAtMost(2_000_000L))
                }
            }
        } catch (error: Throwable) {
            gamePaused = true
            post { onError(error.stackTraceToString()) }
        } finally {
            try { NativeBridge.gbaSave() } catch (_: Throwable) {}
            try { track?.pause(); track?.flush(); track?.release() } catch (_: Exception) {}
        }
    }
}

private class FrameRenderer(private val onError: (String) -> Unit) : GLSurfaceView.Renderer {
    private val frameLock = Any()
    private var front: ByteBuffer? = null
    private var config = ViewConfig(0, 0)
    @Volatile var screenWidth = 2280
        private set
    @Volatile var screenHeight = 1080
        private set
    private var textureWidth = 0
    private var textureHeight = 0
    private var texture = 0
    private var program = 0
    private var position = 0
    private var coordinates = 0
    private val vertices = floats(floatArrayOf(-1f,-1f, 1f,-1f, -1f,1f, 1f,1f))
    private val uv = floats(floatArrayOf(0f,1f, 1f,1f, 0f,0f, 1f,0f))

    fun publish(buffer: ByteBuffer, view: ViewConfig): ByteBuffer? = synchronized(frameLock) {
        val old = front
        front = buffer
        config = view
        old
    }

    override fun onSurfaceCreated(gl: GL10?, egl: EGLConfig?) {
        try {
            val vertex = shader(GL_VERTEX_SHADER,
                "attribute vec2 position; attribute vec2 uv; varying vec2 tex; void main(){ gl_Position=vec4(position,0.0,1.0);tex=uv; }")
            val fragment = shader(GL_FRAGMENT_SHADER,
                "precision mediump float; varying vec2 tex; uniform sampler2D image; void main(){ gl_FragColor=texture2D(image,tex).bgra; }")
            program = glCreateProgram()
            glAttachShader(program,vertex); glAttachShader(program,fragment); glLinkProgram(program)
            val linked = IntArray(1); glGetProgramiv(program,GL_LINK_STATUS,linked,0)
            check(linked[0] != 0) { glGetProgramInfoLog(program) }
            glDeleteShader(vertex); glDeleteShader(fragment)
            position = glGetAttribLocation(program,"position")
            coordinates = glGetAttribLocation(program,"uv")
            val textures = IntArray(1); glGenTextures(1,textures,0); texture = textures[0]
            glBindTexture(GL_TEXTURE_2D,texture)
            glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MIN_FILTER,GL_NEAREST)
            glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_MAG_FILTER,GL_NEAREST)
            glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_S,GL_CLAMP_TO_EDGE)
            glTexParameteri(GL_TEXTURE_2D,GL_TEXTURE_WRAP_T,GL_CLAMP_TO_EDGE)
            textureWidth = 0; textureHeight = 0
        } catch (error: Throwable) { onError("OpenGL: ${error.message}") }
    }

    override fun onSurfaceChanged(gl: GL10?, width: Int, height: Int) {
        screenWidth = width; screenHeight = height
    }

    override fun onDrawFrame(gl: GL10?) {
        glViewport(0,0,screenWidth,screenHeight)
        glClearColor(0.025f,0.035f,0.03f,1f); glClear(GL_COLOR_BUFFER_BIT)
        if (program == 0) return
        val view = synchronized(frameLock) {
            val buffer = front ?: return
            buffer.position(0)
            glActiveTexture(GL_TEXTURE0); glBindTexture(GL_TEXTURE_2D,texture)
            if (textureWidth != config.width || textureHeight != config.height) {
                glTexImage2D(GL_TEXTURE_2D,0,GL_RGBA,config.width,config.height,0,GL_RGBA,GL_UNSIGNED_BYTE,buffer)
                textureWidth = config.width; textureHeight = config.height
            } else {
                glTexSubImage2D(GL_TEXTURE_2D,0,0,0,config.width,config.height,GL_RGBA,GL_UNSIGNED_BYTE,buffer)
            }
            config
        }
        val scale = if(view.fillScreen)maxOf(screenWidth.toFloat()/view.width, screenHeight.toFloat()/view.height)
            else minOf(screenWidth.toFloat()/view.width, screenHeight.toFloat()/view.height)
        val width = (view.width*scale).toInt(); val height=(view.height*scale).toInt()
        glViewport((screenWidth-width)/2,(screenHeight-height)/2,width,height)
        glUseProgram(program); glUniform1i(glGetUniformLocation(program,"image"),0)
        glEnableVertexAttribArray(position); glEnableVertexAttribArray(coordinates)
        vertices.position(0); uv.position(0)
        glVertexAttribPointer(position,2,GL_FLOAT,false,0,vertices)
        glVertexAttribPointer(coordinates,2,GL_FLOAT,false,0,uv)
        glDrawArrays(GL_TRIANGLE_STRIP,0,4)
    }
    private fun shader(type: Int, source: String): Int {
        val shader = glCreateShader(type); glShaderSource(shader,source); glCompileShader(shader)
        val status=IntArray(1);glGetShaderiv(shader,GL_COMPILE_STATUS,status,0)
        check(status[0] != 0) { glGetShaderInfoLog(shader) }
        return shader
    }
    private fun floats(values: FloatArray): FloatBuffer = ByteBuffer.allocateDirect(values.size*4)
        .order(ByteOrder.nativeOrder()).asFloatBuffer().apply { put(values);position(0) }
}
