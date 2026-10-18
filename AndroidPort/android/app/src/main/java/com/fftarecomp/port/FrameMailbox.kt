package com.fftarecomp.port

/** Latest complete frame, plus one reusable spare. acquire transfers exclusive ownership
 * to the renderer; release must run only after texture uploads have finished.
 * No GL or simulation work may run while this object's monitor is held. */
internal class FrameMailbox<T> {
    private var pending:T?=null
    private var spare:T?=null
    @Synchronized fun publish(frame:T):T? {
        val reusable=pending ?: spare.also { spare=null }
        pending=frame
        return reusable
    }
    @Synchronized fun acquire():T?=pending.also { pending=null }
    @Synchronized fun release(frame:T) { spare=frame }
}
