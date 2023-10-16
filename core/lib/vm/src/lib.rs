// #![deny(unreachable_pub)]
// #![deny(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_imports)]

pub use old_vm::{
    history_recorder::{HistoryDisabled, HistoryEnabled, HistoryMode},
    memory::SimpleMemory,
};

pub use oracles::storage::StorageOracle;

pub use tracers::{
    utils::VmExecutionStopReason, CallTracer, StorageInvocations, ValidationError,
    ValidationTracer, ValidationTracerParams,
};

pub use types::internals::ZkSyncVmState;
pub use utils::transaction_encoding::TransactionVmExt;

pub use bootloader_state::BootloaderState;

pub use crate::vm::Vm;

mod bootloader_state;
mod implementation;
mod old_vm;
mod oracles;
mod tracers;
mod types;
mod vm;

pub mod constants;
pub mod utils;

#[cfg(test)]
mod tests;
