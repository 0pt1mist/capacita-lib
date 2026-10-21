
CapacitaOS Architecture and Technical Specification

CapacitaOS is an experimental asynchronous exokernel operating system architecture implemented entirely as a freestanding Rust library (#![no_std]). The system is specifically engineered for the hardware constraints and operational requirements of a mobile cyberdeck powered by a dual-core RISC-V ESP32-P4 microcontroller (400 MHz, 32 MB PSRAM, 16 MB SPI Flash).
The core design centers on strict architectural minimalism and systemic isolation. The kernel does not supervise user processes, fix logic bugs, or manage application errors. High availability and runtime stability are achieved through an aggressive "Let It Crash" lifecycle policy: the core kernel safely multiplexes raw physical resources and provides secure execution boundaries, eliminating abstraction taxes and dynamic validation overhead inside the privileged execution path.

Architectural Principles


1. Neuromorphic Capability Security (Synaptic Access Control)

Instead of relying on standard monolithic file descriptors or heavy page-table structures, CapacitaOS manages access rights via a decentralized, self-cleaning model inspired by synaptic plasticity:
• Actor (Neuron): An isolated execution and logic unit.
• Capability (Synapse): A secure token representing access rights (keys) to an underlying data asset. Capabilities are entirely hidden from the Actor's execution space inside a protected CNode table managed strictly by the kernel. Actors reference their capabilities using an opaque numerical token called a CapSelector (u32).
• Capability Metabolism & Pruning: Access rights degrade or expire over time. Legitimate resource utilization reinforces a capability's saturation index (weight). When a fixed CNode slot table fills up, the kernel executes an aggressive pruning routine: it scans for the least used capability with the lowest weight and overwrites it with the new incoming token. Capabilities granted with a maximum weight (255) gain genetic immunity and are skipped by the pruning subsystem.

2. Exokernel Storage Multiplexing

The kernel rejects conventional file system layers and high-level virtual memory abstractions. Data objects are represented as Engrams—lightweight metadata descriptors that link directly to chains of raw, fixed-size physical blocks (512 bytes each, explicitly aligned with the physical page size of the ESP32-P4's SPI Flash).

3. Asynchronous Non-Blocking IPC & Throttling

Inter-process communication relies on static ring buffers of a fixed size (16 slots) without dynamic memory allocation (alloc). When a recipient's mailbox is saturated, the kernel immediately rejects the transfer, returning an error status to the sender. Persistent spamming triggers a throttling counter (spam_penalty_ticks), stripping the misbehaving Actor of CPU scheduling priority to preserve global system availability.

4. Crash Immunity via Transactional Storage

To safeguard storage integrity under the "Let It Crash" philosophy, the kernel enforces a transaction log system using a Chunk-by-Chunk Shadow Paging pattern. If an Actor faults while modifying data, the kernel instantly rolls back any uncommitted blocks, keeping memory allocations uniform and preventing storage corruption.

Project Structure

The repository is organized as a modular Rust crate managed by a root lib.rs:
├── src/
│   ├── lib.rs          # Root module linking and exporting all components
│   ├── engram.rs       # Metadata descriptors and asset passports
│   ├── storage.rs      # Low-level physical block allocation and FAT-like manager
│   ├── capability.rs   # Synaptic access control logic and C-Node structures
│   ├── actor.rs        # Actor traits and asynchronous mailbox rings
│   └── kernel.rs       # Core execution engine, scheduler, and O(1) syscall routing

Component Specifications


lib.rs

The library entry point that declares the environment attributes and exposes the subsystem modules:
rust
#![no_std]
pub mod engram;
pub mod capability;
pub mod actor;
pub mod storage;
pub mod kernel;
Используйте код с осторожностью.

engram.rs

Defines the Engram structure (~136 bytes), which serves as an asset passport. It manages a fixed array of 32 associative tags (u32) for fast in-RAM filtering, tracks the payload boundary size (total_size), and contains a pointer to the initial allocation segment (first_block_id). It features helper methods to get and set metadata properties.

storage.rs

Manages a physical memory partition sliced into 1,024 blocks (512 bytes each).
• allocate_single_block: Scans the tracking bitmap for unallocated chunks using hardware-accelerated instructions (trailing_zeros) and establishes links inside the static FAT-like block_chain lookup table.
• read_chain: A memory-safe block streaming engine. It guarantees protection against user-space buffer overflows by aborting if the payload boundary exceeds the destination buffer length, and truncates trailing padding junk automatically on read.
• free_chain: Resets targeted bits in the allocation bitmap and clears links to prevent dangling references or ghost storage allocations.

capability.rs

Manages access verification via the Capability structure using active aging mechanics.
• Deduplication: The CNode::insert routine intercepts duplicate tokens targeting the same asset and merges their authorization masks on the fly (keys |= new_cap.keys) while boosting the connection weight.
• Pruning Mitigation: Validates boundaries and evicts the weakest un-immunized capability (weight < 255) if all 16 slots are exhausted.
• check_and_use: Performs lazy evaluation, decrementing the max_ttl of transient tokens and augmenting active link weight via saturating additions (saturating_add), guaranteeing that counter boundaries never overflow.

actor.rs

Handles the asynchronous messaging execution layer.
• The Actor trait enforces a single non-blocking entry point: receive(&mut self, msg: Message).
• The Message enum supports native Ping, Command(u32), and TransferCapability(Capability) variants. It implements #[derive(Clone, Copy)] to facilitate fast array initializations without runtime memory overhead.
• Mailbox wraps a circular message queue that uses wrapping arithmetic (% MAILBOX_SIZE) to adjust head and tail markers without index corruption.

kernel.rs

The privileged management environment that drives the runtime and protects system state.
• Vulnerability Isolation: Encapsulates CNode tables, mailbox rings, and active transaction markers within an isolated ActorControlBlock array hidden from the Actor's execution environment.
• kill_and_reclaim: An atomic system destructor that intercepts Actor faults, releases raw blocks flagged by incomplete transactions (TransactionRecord), and recreates the target Actor with an empty mailbox while preserving its immune capabilities (weight == 255).
• schedule: A cooperative Round-Robin scheduler that cycles through the Actor array. It decrements and honors spam_penalty_ticks bans, routes queued mail via mailbox.pop(), and forwards signals directly to the user-space trait implementation.
• sys_send & sys_read: High-speed system call handlers providing deterministic O(1) execution time. They bypass linear scanning loops through direct array indexing, performing access validation and resource delivery instantly.
