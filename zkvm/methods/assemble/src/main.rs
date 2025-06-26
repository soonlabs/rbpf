use std::sync::Arc;
#[allow(unused_imports)]
use risc0_zkvm::guest::env;
use solana_rbpf::{ebpf, assembler::assemble, program::BuiltinProgram, verifier::RequisiteVerifier, vm::TestContextObject};
use test_utils::create_vm;
use solana_rbpf::memory_region::MemoryRegion;
use solana_rbpf::program::FunctionRegistry;
use solana_rbpf::vm::Config;

fn main() {
    execute_program(
        "
    ldxb r0, [r1]
    add r1, 1
    mov r0, r1
    and r0, 0xFFFFFF
    jlt r0, 0x20000, -5
    exit",
        Config::default(),
        655361,
        &mut [0; 0x20000],
    )
}

fn execute_program(
    assembly: &str,
    config: Config,
    instruction_meter: u64,
    mem: &mut [u8],
) {
    let executable = assemble::<TestContextObject>(
        assembly,
        Arc::new(BuiltinProgram::new_loader(
            config,
            FunctionRegistry::default(),
        )),
    )
        .unwrap();
    executable.verify::<RequisiteVerifier>().unwrap();
    let mut context_object = TestContextObject::default();
    let mem_region = MemoryRegion::new_writable(mem, ebpf::MM_INPUT_START);
    create_vm!(
        vm,
        &executable,
        &mut context_object,
        stack,
        heap,
        vec![mem_region],
        None
    );
    vm.context_object_pointer.remaining = instruction_meter;
    let (instruction_count_interpreter, result) = vm.execute_program(&executable, true);
    result.unwrap();
    assert_eq!(instruction_count_interpreter, instruction_meter);
}