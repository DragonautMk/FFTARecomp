# Legacy Windows gamepads

The existing gilrs backend remains available. Windows adds DirectInput 8 as a
fallback when no gilrs pad is selected. In the launcher Controllers page select
the entry starting with DirectInput and bind each GBA control by pressing its
physical button. The live monitor and game runtime use the same mappings.

Raw button names are DInputButton0 through DInputButton127. Numbering is zero
based. DirectInput does not standardize face-button order: the fallback semantic
map uses buttons 0/1/2/3 for South/East/West/North, and may need rebinding.
Stick axes and POV hats are supported, including diagonal POV directions.
Device lists refresh once per second. Losing input returns neutral gamepad state;
the backend tries acquiring the device again and preserves keyboard input.

PowerShell, using the private manual-test ROM/save prepared in this session:

```powershell
.\target\release\recomp.exe controllers
$env:FFTA_GAMEPAD_BACKEND='directinput'
$env:FFTA_WIDESCREEN='32:9'
$env:FFTA_PATCH_BG='1'
$env:FFTA_PATCH_SPR='1'
$env:FFTA_PATCH_VIS='1'
$env:FFTA_PATCH_MOVEMENT='1'
$env:FFTA_OAM_LOG='0'
.\target\release\recomp.exe play 'target/ffta-zoom/manual/FFTA-movement-test.gba'
```

For automatic backend selection remove FFTA_GAMEPAD_BACKEND. Set it to gilrs
to compare with the existing backend. On Linux DirectInput is absent; gilrs
continues using the Linux input stack.

The in-game overlay now has Input -> Keyboard and Gamepad. Entering Keyboard
keeps its existing bindings; entering Gamepad enables the controller backend.
Select a GBA button, press confirm, then press its controller button or stick
direction. Escape cancels capture. Default layout resets only pad bindings.
The default maps GBA A/B to Xbox A/B (South/East), L/R to LB/RB, Start/Select
to Start/Back, and directions to the d-pad and left stick.
Existing saved custom pad bindings are preserved until reset or remapped.

No further physical controller tests are performed at the user's request.
Compilation and menu state-machine checks do not validate hardware input.
