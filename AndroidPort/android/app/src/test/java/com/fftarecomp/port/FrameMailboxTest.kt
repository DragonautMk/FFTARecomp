package com.fftarecomp.port

import org.junit.Assert.*
import org.junit.Test

class FrameMailboxTest {
    @Test fun uploadKeepsExclusiveOwnershipWhileProducerPublishes() {
        val mailbox=FrameMailbox<Any>()
        val a=Any();val b=Any();val c=Any()
        assertNull(mailbox.publish(a))
        val uploading=mailbox.acquire()
        assertSame(a,uploading)
        assertNull(mailbox.publish(b))
        // A slow GPU cannot block the producer; it replaces the pending frame.
        assertSame(b,mailbox.publish(c))
        assertSame(a,uploading)
        mailbox.release(uploading!!)
        assertSame(c,mailbox.acquire())
        assertSame(a,mailbox.publish(b))
    }
    @Test fun replayingTheLastTextureDoesNotAcquireAnInUseBuffer() {
        val mailbox=FrameMailbox<Any>();val a=Any();val b=Any()
        mailbox.publish(a)
        assertSame(a,mailbox.acquire())
        assertNull(mailbox.acquire())
        mailbox.release(a)
        assertSame(a,mailbox.publish(b))
        assertSame(b,mailbox.acquire())
    }
}
