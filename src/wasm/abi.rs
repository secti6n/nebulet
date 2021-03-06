use abi;
pub use super::abi_types::AbiFunction;
use hashmap_core::HashMap;

// TODO: Verify function signatures so we don't
// throw bad data at functions and crash everything.
abi_map! {
    ABI_MAP,
    // testing
    exit: { // eventually will exit maybe, right now is just for testing
        params: [I64],
        returns: I64,
        abi::test::output_test,
    },
    // actual abis
    wasm_compile: {
        params: [I32, I32],
        returns: I64,
        abi::process::wasm_compile,
    },
    process_create: {
        params: [I32, I32],
        returns: I64,
        abi::process::process_create,
    },
    process_start: {
        params: [I32],
        returns: I64,
        abi::process::process_start,
    },
    // ipc
    channel_create: {
        params: [I32, I32],
        returns: I64,
        abi::ipc::channel_create,
    },
    channel_write: {
        params: [I32, I32, I32],
        returns: I64,
        abi::ipc::channel_write,
    },
    channel_read: {
        params: [I32, I32, I32, I32],
        returns: I64,
        abi::ipc::channel_read,
    },
    // I/O
    print: {
        params: [I32, I32],
        returns: VOID,
        abi::io::print,
    },

    // driver ABIs
    physical_map: {
        params: [I64, I32],
        returns: I64,
        abi::driver::physical_map,
    },
}

abi_map! {
    INTRINSIC_MAP,
    grow_memory: {
        params: [I32],
        returns: I32,
        abi::intrinsics::grow_memory,
    },
    current_memory: {
        params: [],
        returns: I32,
        abi::intrinsics::current_memory,
    },
}
