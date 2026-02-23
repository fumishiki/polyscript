;; Compile to Wasm: wat2wasm example.wat -o example.wasm
;; Run:             polyscript wasm example.wasm
(module
  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  ;; "[Wasm] hello\n" @ offset 8
  (data (i32.const 8) "[Wasm] hello\n")
  ;; iovec: ptr=8, len=13  @ offset 0
  (data (i32.const 0) "\08\00\00\00\0d\00\00\00")
  (func (export "_start")
    (drop (call $fd_write
      (i32.const 1)   ;; stdout
      (i32.const 0)   ;; iovec ptr
      (i32.const 1)   ;; iovec count
      (i32.const 20)  ;; nwritten ptr
    ))
  )
)
