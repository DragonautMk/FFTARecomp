# Linux controller update

Linux and Windows are built from the same current desktop source tree. The
Linux package therefore includes the current zoom/widescreen fixes, separated
UI layers and Text bubbles settings, experimental mouse targeting, native
Switch ROM picker and shared save handoff. Windows keeps its working native
XInput implementation.

Linux uses its native evdev/udev controller interfaces through the existing
Gilrs dependency and vendored controller mapping database. It does not load
Microsoft XInput DLLs or require a separate controller executable. Linux must
recognize the controller and permit the desktop session to read its input.

The new Linux router drains device events to refresh the backend cache, then
reads one complete snapshot per connected gamepad per host frame. Remapping,
menu navigation and gameplay use the same snapshot. Repeated and changed events
cannot independently assign or confirm the same press. Capture locks input
until all controls are released and neutral for 20 ms; sticks and analog
triggers use hysteresis. Deliberate input switches the active gamepad, and
disconnecting removes that device's state. Only canonical Xbox button and
stick labels are captured; unknown raw button numbers are not exposed.

Guide opens Pause. Linux also accepts Start+Back when the desktop or Steam
intercepts Guide. Remapping, zoom and fast-forward use the existing PC options.

Validation: four snapshot simulation tests pass (held A, repeated capture and
release bounce, all canonical controls, analog drift/trigger release). The
desktop executable suite passes 25 tests, with one ROM-dependent test ignored.
Windows release and Linux x86_64 release compile successfully. The tar check
verifies ELF architecture, executable permissions, equality to freshly built
artifacts, new feature markers, and absence of ROM/save/Windows DLL payloads.

NOT MEASURED: actual controller input, USB/Bluetooth connection and gameplay
performance on a Linux laptop. Host simulation is not a physical Linux test.

Package: Installer/Linux/FFTAInstaller-linux-x86_64.tar.gz. Extract and run
`chmod +x FFTAInstaller` followed by `./FFTAInstaller`. Requires libasound.so.2
and libudev.so.1. TinyCC is bundled; a system C compiler is not needed.
