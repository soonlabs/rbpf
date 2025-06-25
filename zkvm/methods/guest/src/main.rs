use std::sync::Arc;
use risc0_zkvm::guest::env;
use solana_rbpf::{
    elf::Executable, program::BuiltinProgram, verifier::RequisiteVerifier, vm::TestContextObject,
};
use test_utils::create_vm;

fn main() {
    // read the input
    let elf = env::read_frame();
    let executable =
        Executable::<TestContextObject>::from_elf(&elf, Arc::new(BuiltinProgram::new_mock()))
            .unwrap();
    executable.verify::<RequisiteVerifier>().unwrap();
    let mut context_object = TestContextObject::default();
    create_vm!(
        vm,
        &executable,
        &mut context_object,
        stack,
        heap,
        Vec::new(),
        None
    );
    vm.context_object_pointer.remaining = 37;
    let (inx_count, res) = vm.execute_program(&executable, true);
    res.unwrap();

    // write public output to the journal
    env::commit(&inx_count);
}
