mod ctx;
mod witx;
mod r#impl;

pub use ctx::WasiSerialCtx;

wasmtime_wiggle::wasmtime_integration!({
    // The wiggle code to integrate with lives here:
    target: witx,
    // This must be the same witx document as used above:
    witx: ["$WASI_ROOT/phases/ephemeral/witx/wasi_ephemeral_serial.witx"],
    // This must be the same ctx type as used for the target:
    ctx: WasiSerialCtx,
    // This macro will emit a struct to represent the instance, with this name and docs:
    modules: {
        wasi_ephemeral_serial => {
          name: WasiSerial,
          docs: "An instantiated instance of the wasi-serial exports.",
          function_override: {}
        }
    },
    // Error to return when caller module is missing memory export:
    missing_memory: { witx::types::Errno::NoPortFound },
});