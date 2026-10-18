/* FFTARecomp desktop adapter, added 2026-10-16.
 * Project-owned code; distributed under the project's MIT license.
 * ABI version, not a binary fingerprint: compatible modified TinyCC builds
 * are intentionally accepted. TinyCC itself retains its LGPL license. */
#ifdef _WIN32
__declspec(dllexport)
#else
__attribute__((visibility("default")))
#endif
int ffta_tcc_abi(void) { return 1; }
