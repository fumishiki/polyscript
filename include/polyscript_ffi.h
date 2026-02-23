// polyscript FFI convention — all C/C++ symbols callable via `polyscript cpp` must export:
//
//   extern "C" int run(int argc, const char** argv);
//
// This header is used by build.rs / bindgen to generate compile-time Rust FFI bindings.

#ifdef __cplusplus
extern "C" {
#endif

int run(int argc, const char** argv);

#ifdef __cplusplus
}
#endif
