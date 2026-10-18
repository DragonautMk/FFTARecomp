# Experimental mouse: EUR language selector

Enable with `FFTA_MOUSE=1`. Disabled by default. Only the exact verified EUR ROM
is accepted; USA and other scenes keep keyboard/gamepad behavior unchanged.

## Measured selector

A synthetic D-pad run from frame 0 visits all five languages. The selection byte
at 0200F3D6 takes values 0,1,2,3,4,0. Its owning structure is obtained through the
pointer at 0201C3D4; the controller follows this pointer rather than writing a
fixed RAM address. Thumb instruction 0813FFCA writes the selection:
`strb r5, [r0, #0x2]`. The four short verification bytes there are 85 70 39 68.
The observed LR at that instruction was 08149325; this is not claimed as the
selector's original caller.

DISPCNT=1140, BG0CNT=3000. Five groups each contain a flag and two text OBJ pieces.
Text tiles start at 050,05C,068,074,080. Their active palette changes from bank0
to bank1. The two-piece quill uses tiles 08C/090, palette3; its Y changes by18
per language. All five groups and both quill pieces must match to enable hits.

## Semantic layer

The invisible intermediate layer holds five target IDs and native-coordinate
rectangles. It uses the existing inverse world viewport/letterbox transform,
including aspect ratio, physical window size and zoom. It does not duplicate,
rescale or re-render the original words/flags. It does not use OCR or colors.

Hover selects a language directly: at the next emulated frame boundary, write
only the validated selector structure's selection byte. The original game then
updates text palette and quill in two frames, with no intervening language.
No directional buttons, ROM patch or replacement draw function is needed.
The cached exact EUR ROM hash and the short original instruction bytes must
match. The current complete menu signature and pointer bounds are rechecked.
Left click selects the requested target then sends one normal A press after the
redraw delay. Physical keyboard/gamepad input wins. Pause, focus loss, pointer
leaving the window and scene loss cancel pending automation.

RECOMP_RECORD_INPUT includes direct transitions as `FRAME mouse:INDEX` alongside
ordinary active-low key masks in the existing `gba-input v1` script. Current
frames/runc/verify replay these events through the same guarded state commit;
an unsupported ROM or mismatched scene makes replay fail explicitly. Historical
key-only scripts retain their behavior. Cursor stays visible in the selector.

## Validation

- 208 suite tests passed, 5 ignored. Tests include all25 source/destination pairs,
  no directional keys, one selection-byte write, instruction mismatch rejection,
  cancellation, manual input priority, one A edge, and replay event parsing.
- Real EUR controller run reaches4,2,0,3,1 directly. Each RAM selection changes
  immediately. OAM palette and quill switch to the target within two frames;
  assertions exclude every intermediate language. Click confirms once.
- Five reached-language PPM files match byte for byte on interpreter and native
  runs at304/308/312/316/320 frames. Full370-frame replay matches interpreter
  and two native runs. Final hash d17fc597c1fb9325.
- USA/EUR extra=0 framebuffer and OAM remain identical for600 frames, without
  mouse requests. Mouse selection intentionally changes output when requested.
- Physical mouse interaction after the direct-jump change is NOT MEASURED yet.
- Other game menus, gameplay clicks, touch gestures and mouse menu configuration
  remain outside this prototype.

Windows executable: target/release/recomp-mouse-direct.exe.
Diagnostic example: cargo run --release -p gba-core --example ffta_language_probe
with EUR ROM path, private output prefix and --mouse (no save loaded or written).

## Reusable grid integration

The five language targets now publish into InteractionGrid (8x8 logical cells).
The generic layer tracks the current cell and resolves EventId(context,option),
with exact bounds, layer precedence, modal blockers and hover entry/exit/click.
Only entering another event or clicking queues an action; crossing cells of one
option does not repeat selection. Geometry is reused until viewport/targets
change. See ffta_interaction_grid.md. New Windows executable:
target/release/recomp-mouse-grid.exe; enable with the same FFTA_MOUSE=1 flag.
