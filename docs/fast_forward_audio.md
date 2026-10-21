# Desktop Fast audio

Fast Forward has three audio choices: `fast` consumes at the selected 2x/4x/8x
multiplier, `silent` discards guest audio, and `normal` plays the accelerated
guest's faithful PCM at 1x from a temporary file. The latter can fall behind
the video; it is not pitch-preserving time stretch. Its backlog limit remains
an open item in [BUG-03](development/todo_bug_fix.md).

## Discarding an old audio timeline

Fast activation/deactivation, multiplier changes and audio-mode changes now
invalidate both the producer queues and the callback's signal history. Opening
the pause menu does the same, discarding any Normal temporary backlog. The
session Fast latch is preserved and the effective speed is evaluated again on
resume. Returning to 1x primes the ordinary rings again.

The device-loss watchdog also replaces pending output, including the Normal
queue, before attempting reconnection. Load/Undo continues to rebuild the stream
and goes through the same queue invalidation; neither uses audio from the
previous guest timeline.

`AudioStreams::discard_pending` increments a host epoch under the existing
mutex. At its next callback, the consumer resets the faithful DAC/fractional
position, PSG/HLE sinc histories, FIFO impulses/integrator/event position, DC
filters and crossfade state. It reuses filter tables and storage, and ramps new
audio in through the existing approximately 20 ms volume slew. No allocation,
I/O, logging, extra lock or clock is added to the callback reset. With no
invalidation, the DSP and queue-consumption policy are unchanged.

This removes stale **application-side** audio. Samples already submitted to the
device/backend remain subject to its buffering; the host epoch cannot retract
them. The new audio starts at the current producer position; it is not replayed
from the discarded queues. Guest mix, timers, snapshot format and native
translation ABI are unchanged.

## Verification and remaining diagnosis

A synthetic test first reproduced output after all producer queues were
emptied (faithful peak 0.19975911 on a normalized f32 output). With consumer
invalidation, all five paths (faithful, PSG, HLE, FIFO and Normal) produce exact
zero after discard at multipliers 1/2/4/8. A second regression compares fresh
audio after successive transitions against an independently initialized
callback: exact samples and remaining queues at 44.1/48 kHz, including
Faithful/Enhanced/HLE/Normal/Silent, observation on/off and multiple resets
between callbacks.

Native release runtime validation passed 68 tests (4 optional ignored), then
the added fresh-history regression passed separately. The final Windows GNU
runtime suite passed 70 tests under Wine (4 optional ignored). The native
candidate build succeeded and the prior canonical runtime was restored by
hash; portable exports were not replaced.

Actual Linux CPAL 44.1 kHz callbacks were exercised in an idle battle with
Xvfb/llvmpipe and native translation, separately for Faithful and Enhanced.
Hold/release, toggle, pause/resume and clean window close passed; inspected
captures show Fast remains latched after pause and returns to approximately
59.7 INT/EXT when disabled. The original slot bytes were unchanged. The test
used volume zero: this checks the real callback and lifecycle, not audible
quality. An obsolete launcher assertion in the helper failed after Faithful
had already exited successfully; its completed result was retained and only
the unrun Enhanced case followed the helper correction.

Those runs also reproduce a distinct consumption mismatch: at requested 4x,
captures show only 121.7/128.2 INT and 30.4/32.1 EXT. Faithful/Enhanced recorded
497/499 Fast callbacks, all with faithful source holds; about 47% of PSG/HLE
requests were empty. Enhanced's configured PSG/HLE targets were affected too.
Neither run reported producer drops or callbacks exceeding their period.
Normal-playing intervals had zero faithful holds (207/205 callbacks), and
paused mixed queues stayed empty. This supports underproduction relative to
the nominal 4x consumer on this CPU-rendered setup, rather than a growing
backlog in the reproduced case. It does not establish the user's hardware
behavior or a measured audio-to-video latency. Logs, inputs and captures are
local under ignored `target/fast-audio-reset/`.

The reported case is **Fast audio at 4x**. Clearing stale history alone does not
establish that a prolonged audio/video synchronization issue is resolved. The
callback still consumes at the requested multiplier even if guest production
cannot keep up. Sustained source starvation, device latency, audible transition
quality and native Windows hardware need separate measurements. Normal's
unbounded file and blocking read path remain unchanged in this fix.
