
CapacitaOS Architecture and Technical Specification

CapacitaOS is an experimental asynchronous exokernel operating system implemented in Rust (#![no_std]). The system is specifically engineered for the hardware constraints and operational requirements of a mobile cyberdeck powered by a dual-core RISC-V ESP32-P4 microcontroller (400 MHz, 32 MB PSRAM, 16 MB SPI Flash).
The core design centers on strict architectural minimalism. The kernel does not supervise user processes or manage application errors. Stability is achieved through a "Let It Crash" lifecycle policy: the runtime isolates and manages raw physical resources safely, avoiding dynamic validation overhead inside the privileged execution path.

Architectural Principles


1. Neuromorphic Capability Security

Instead of using standard file descriptors or heavy page-table structures, CapacitaOS manages access rights via a static, self-cleaning model:
• Actor: An isolated execution unit.
• Capability: A secure token representing access rights to an underlying data object. Capabilities are entirely hidden from the Actor's execution space inside a protected CNode table managed by the kernel. Actors only reference capabilities using an opaque numerical index called a CapSelector (u32).
• Capability Metabolism: Access rights expire or degrade over time. Legitimate resource utilization boosts a capability's saturation index (weight). When the fixed CNode table fills up, the kernel executes an aggressive pruning routine: it scans for the least used capability with the lowest weight and overwrites it with the new incoming token.

2. Exokernel Storage Multiplexing

The kernel rejects conventional file system and virtual memory abstractions. Data objects are represented as Engrams—metadata descriptors that link to chains of raw, fixed-size physical blocks (512 bytes each, aligned with the physical page size of the ESP32-P4's SPI Flash).

3. Asynchronous Non-Blocking IPC

Inter-process communication relies on static ring buffers of a fixed size (16 slots) without dynamic memory allocation (alloc). When a recipient's mailbox is saturated, the kernel immediately rejects the transfer, returning an error status to the sender. Persistent spamming triggers a throttling counter, stripping the misbehaving Actor of CPU scheduling slots.

Project Structure

The project is structured into low-level modules linked together by a root lib.rs:

------------------------
lib.rs          # Root module linking all components

engram.rs       # Metadata descriptors and asset passports

storage.rs      # Low-level block memory manager (FAT-like pool)

capability.rs   # Access control logic and C-Node structures

actor.rs        # Actor traits and asynchronous mailbox rings

------------------------
Component Specifications (Current Codebase)


engram.rs

The Engram structure is a compact data descriptor weighing approximately 136 bytes. It stores an array of 32 associative tags (u32) utilized for fast in-RAM object filtering, the exact data payload size, and the identifier of the first physical block backing the object.

storage.rs

Manages a raw physical storage pool consisting of 1,024 blocks (512 bytes each).
• allocate: Scans the bitmap field for free chunks using hardware-accelerated instructions (trailing_zeros) and tracks linked blocks via a static allocation array (block_chain).
• read_chain: A memory-safe block reader. It guarantees protection against user-space buffer overflows by aborting the operation if the specified payload size exceeds the receiver buffer length, and cuts off block-tail padding noise based on the object's total_size.
• free_chain: Resets bits in the bitmap array and clears FAT linkages to prevent dangling or ghost references.

capability.rs

Defines the Capability structure with active aging fields (weight and max_ttl).
• Deduplication: The CNode::insert method merges overlapping access rights using a bitwise OR operation (keys |= new_cap.keys) to prevent descriptor inflation.
• Pruning: When the slot table hits its limit (maximum 16 capabilities per Actor), the kernel automatically targets and evicts the weakest un-immunized reference.
• check_and_use: Lazily decrements the max_ttl counter of transient capabilities on each invocation and refreshes active capability weight via saturating additions (saturating_add).

actor.rs

Handles the asynchronous runtime primitives.
• The Actor trait exposes a single non-blocking execution entry point: receive(&mut self, msg: Message).
• The Message enumeration supports Ping, Command, and TransferCapability variants. It implements #[derive(Clone, Copy)], allowing fixed-size message arrays to be initialized in #![no_std] without runtime overhead.
• Mailbox implements a circular queue using head and tail index tracking adjusted by wrapping arithmetic (% MAILBOX_SIZE).

Development Roadmap

The next phase of implementation involves assembling the central kernel.rs driver to enforce the following controls:
1. Vulnerability Isolation: Physical encapsulation of CNode tables and active transaction logs within a secure kernel-managed space (ActorControlBlock) to eliminate user-space tampering via the trait's &mut self parameters.
2. Atomic Process Reclamation: A kill_and_reclaim system destructor designed to catch Actor faults, roll back unfinished block-allocation records (TransactionRecord), and regenerate the Actor context with a clean mailbox while retaining only immune system capabilities (weight == 255).
3. Deterministic System Calls: Optimization of the asynchronous IPC layer (sys_send) to bypass linear scanning loops, achieving deterministic O(1) execution time via direct array indexing.
This README.md file outlines the current architecture and state of the repository before the kernel integration.

English isnt my native language, so I gave all my code to ChatGPT and asked it to write a symphony after I told it about all my ideas.
Im novice in rust, but Im trying to create something new, something that I would love to see "Alive"
