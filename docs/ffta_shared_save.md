# Shared save when switching ROM

Switch ROM saves the current progress, passes its exact save path to the new
process, and remembers that path in the user data directory under
`FFTARecomp/shared-save-path.txt`. USA and Europe therefore use the same file,
even across ROM renames and directories. Original save files are not renamed,
deleted or copied. Normal gameplay continues writing the selected save.

At startup `FFTA_SAVE_PATH` overrides the remembered path. Without either,
the original basename save is preferred. If absent, one `.sav` beside the ROM
is accepted regardless of its basename; several files open a save picker to
avoid guessing. Cancelling creates the normal basename save. Only choose FFTA
saves; arbitrary saves of other games are not validated or converted.

Automated test uses temporary synthetic files and checks discovery of arbitrary
names and preservation of multiple files. Actual cross-region progress was
previously confirmed by the user; the new desktop handoff awaits manual testing.
