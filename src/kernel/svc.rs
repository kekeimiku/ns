#![allow(unused, non_camel_case_types, non_snake_case)]

use core::{
    arch::global_asm,
    ffi::{c_char, c_double, c_float, c_void},
};

global_asm!(include_str!("svc.s"));

/// Pseudo handle for the current process.
pub const CUR_PROCESS_HANDLE: u32 = 0xFFFF8001;

/// Pseudo handle for the current thread.
pub const CUR_THREAD_HANDLE: u32 = 0xFFFF8000;

/// Maximum number of objects that can be waited on by \ref svcWaitSynchronization (Horizon kernel limitation).
pub const MAX_WAIT_OBJECTS: i32 = 0x40;

type HANDLE = u32; //< Kernel object handle.

#[repr(C)]
struct MemoryInfo {
    addr: u64,            //< Base address.
    size: u64,            //< Size.
    state: u32,           //< Memory type (see lower 8 bits of \ref MemoryState).
    attr: u32,            //< Memory attributes (see \ref MemoryAttribute).
    perm: u32,            //< Memory permissions (see \ref Permission).
    ipc_refcount: u32,    //< IPC reference count.
    device_refcount: u32, //< Device reference count.
    padding: u32,         //< Padding.
}

/// Context of a scheduled thread.
#[repr(C)]
struct LastThreadContext {
    fp: u64, //< Frame Pointer for the thread.
    sp: u64, //< Stack Pointer for the thread.
    lr: u64, //< Link Register for the thread.
    pc: u64, //< Program Counter for the thread.
}

#[repr(C)]
enum LimitableResource {
    LimitableResource_Memory = 0,           //<How much memory can a process map.
    LimitableResource_Threads = 1,          //<How many threads can a process spawn.
    LimitableResource_Events = 2,           //<How many events can a process have.
    LimitableResource_TransferMemories = 3, //<How many transfer memories can a process make.
    LimitableResource_Sessions = 4,         //<How many sessions can a process own.
}

/// Thread Activity.
#[repr(C)]
enum ThreadActivity {
    ThreadActivity_Runnable = 0, //< Thread can run.
    ThreadActivity_Paused = 1,   //< Thread is paused.
}

/// Armv8 CPU register.
#[repr(C)]
pub union CpuRegister {
    x: u64, //< 64-bit AArch64 register view.
    w: u32, //< 32-bit AArch64 register view.
    r: u32, //< AArch32 register view.
}

/// Armv8 NEON register.
#[repr(C)]
pub union FpuRegister {
    v: u128,     //< 128-bit vector view.
    d: c_double, //< 64-bit double-precision view.
    s: c_float,  //< 32-bit single-precision view.
}

#[repr(C)]
pub struct ThreadContext {
    /// The general-purpose CPU registers
    pub gpu_gprs: [CpuRegister; 29],
    /// The FP register
    pub fp: u64,
    /// The LR register
    pub lr: u64,
    /// The SP register
    pub sp: u64,
    /// The PC register
    pub pc: CpuRegister,
    /// The PSR value
    pub psr: u32,
    /// The general-purpose FPU registers
    pub fpu_gprs: [FpuRegister; 32],
    /// The FPCR value
    pub fpcr: u32,
    /// The FPSR value
    pub fpsr: u32,
    /// The TPIDR value
    pub tpidr: u64,
}

/// Code memory mapping operations
#[repr(C)]
enum CodeMapOperation {
    CodeMapOperation_MapOwner = 0,   //< Map owner.
    CodeMapOperation_MapSlave = 1,   //< Map slave.
    CodeMapOperation_UnmapOwner = 2, //< Unmap owner.
    CodeMapOperation_UnmapSlave = 3, //< Unmap slave.
}

/// Process Activity.
#[repr(C)]
enum ProcessActivity {
    ProcessActivity_Runnable = 0, //< Process can run.
    ProcessActivity_Paused = 1,   //< Process is paused.
}

/// Physical memory information structure.
#[repr(C)]
struct PhysicalMemoryInfo {
    physical_address: u64, //< Physical address.
    virtual_address: u64,  //< Virtual address.
    size: u64,             //< Size.
}

/// Debug Thread Parameters.
#[repr(C)]
enum DebugThreadParam {
    DebugThreadParam_ActualPriority = 0,
    DebugThreadParam_State = 1,
    DebugThreadParam_IdealCore = 2,
    DebugThreadParam_CurrentCore = 3,
    DebugThreadParam_CoreMask = 4,
}

/// Process Information.
#[repr(C)]
enum ProcessInfoType {
    ProcessInfoType_ProcessState = 0, //<What state is a process in.
}

/// Secure monitor arguments.
#[repr(C, packed)]
struct SecmonArgs {
    X: [u64; 8], //< Values of X0 through X7.
}

#[allow(improper_ctypes)]
extern "C" {
    fn svcSetHeapSize(out_addr: *mut *mut c_void, size: u64) -> u32;
    fn svcSetMemoryPermission(addr: *mut c_void, size: u64, perm: u32) -> u32;
    fn svcSetMemoryAttribute(addr: *mut c_void, size: u64, val0: u32, val1: u32) -> u32;
    fn svcMapMemory(dst_addr: *mut c_void, src_addr: *mut c_void, size: u64) -> u32;
    fn svcUnmapMemory(dst_addr: *mut c_void, src_addr: *mut c_void, size: u64) -> u32;
    fn svcQueryMemory(meminfo_ptr: *mut MemoryInfo, pageinfo: *mut u32, addr: u64) -> u32;
    fn svcExitProcess() -> !;
    fn svcCreateThread(
        out: *mut HANDLE,
        entry: *mut c_void,
        arg: *mut c_void,
        stack_top: *mut c_void,
        prio: i32,
        cpuid: i32,
    ) -> u32;
    fn svcStartThread(handle: HANDLE) -> u32;
    fn svcExitThread() -> !;
    fn svcSleepThread(nano: i64);
    fn svcGetThreadPriority(priority: *mut i32, handle: u32) -> u32;
    fn svcSetThreadPriority(handle: HANDLE, priority: u32) -> u32;
    fn svcGetThreadCoreMask(preferred_core: *mut i32, affinity_mask: *mut u64, handle: u32) -> u32;
    fn svcSetThreadCoreMask(handle: HANDLE, preferred_core: i32, affinity_mask: u32) -> u32;
    fn svcGetCurrentProcessorNumber() -> u32;
    fn svcSignalEvent(handle: HANDLE) -> u32;
    fn svcClearEvent(handle: HANDLE) -> u32;
    fn svcMapSharedMemory(handle: HANDLE, addr: *mut c_void, size: usize, perm: u32) -> u32;
    fn svcUnmapSharedMemory(handle: HANDLE, addr: *mut c_void, size: usize) -> u32;
    fn svcCreateTransferMemory(out: *mut HANDLE, addr: *mut c_void, size: usize, perm: u32) -> u32;
    fn svcCloseHandle(handle: HANDLE) -> u32;
    fn svcResetSignal(handle: HANDLE) -> u32;
    fn svcWaitSynchronization(index: *mut i32, handles: *const HANDLE, handle_count: i32, timeout: u64);
    fn svcCancelSynchronization(thread: HANDLE) -> u32;
    fn svcArbitrateLock(wait_tag: u32, tag_location: *mut u32, self_tag: u32) -> u32;
    fn svcArbitrateUnlock(tag_location: *mut u32) -> u32;
    fn svcWaitProcessWideKeyAtomic(key: *mut u32, tag_location: *mut u32, self_tag: u32, timeout: u64)
        -> u32;
    fn svcSignalProcessWideKey(key: *mut u32, mun: i32);
    fn svcGetSystemTick() -> u64;
    fn svcConnectToNamedPort(session: *mut HANDLE, name: *const c_char) -> u32;
    fn svcSendSyncRequestLight(session: HANDLE) -> u32;
    fn svcSendSyncRequest(session: HANDLE) -> u32;
    fn svcSendSyncRequestWithUserBuffer(usr_buffer: *mut c_void, size: u64, session: HANDLE) -> u32;
    fn svcSendAsyncRequestWithUserBuffer(
        handle: *mut HANDLE,
        use_buffer: *mut c_void,
        size: u64,
        session: HANDLE,
    ) -> u32;
    fn svcGetProcessId(process_id: *mut u64, handle: HANDLE) -> u32;
    fn svcGetThreadId(thread_id: *mut u64, handle: HANDLE) -> u32;
    fn svcBreak(break_reason: u32, address: usize, size: usize) -> u32;
    fn svcOutputDebugString(str: *const c_char, size: u64);
    fn svcReturnFromException(res: u32) -> !;
    fn svcGetInfo(out: *mut u64, id0: u32, handle: HANDLE, id1: u64) -> u32;
    fn svcFlushEntireDataCache();
    fn svcFlushDataCache(address: *mut c_void, size: usize) -> u32;
    fn svcMapPhysicalMemory(address: *mut c_void, size: u64) -> u32;
    fn svcUnmapPhysicalMemory(address: *mut c_void, size: u64) -> u32;
    fn svcGetDebugFutureThreadInfo(
        out_context: *mut LastThreadContext,
        out_thread_id: *mut u64,
        debug: HANDLE,
        ns: i64,
    ) -> u32;
    fn svcGetLastThreadInfo(
        out_context: *mut LastThreadContext,
        out_tls_address: *mut u64,
        out_flags: *mut u32,
    ) -> u32;
    fn svcGetResourceLimitLimitValue(out: *mut i64, reslimit_h: HANDLE, which: LimitableResource) -> u32;
    fn svcGetResourceLimitCurrentValue(out: *mut i64, reslimit_h: HANDLE, which: LimitableResource) -> u32;
    fn svcSetThreadActivity(thread: HANDLE, paused: ThreadActivity) -> u32;
    fn svcGetThreadContext3(ctx: *mut ThreadContext, thread: HANDLE) -> u32;
    fn svcWaitForAddress(address: *mut c_void, arb_type: u32, value: i32, timeout: i64) -> u32;
    fn svcSignalToAddress(address: *mut c_void, signal_type: u32, value: i32, count: i32) -> u32;
    fn svcSynchronizePreemptionState();
    fn svcGetResourceLimitPeakValue(out: *mut i64, reslimit_h: HANDLE, which: LimitableResource) -> u32;
    fn svcCreateIoPool(out_handle: *mut HANDLE, pool_type: u32) -> u32;
    fn svcCreateIoRegion(
        out_handle: *mut HANDLE,
        io_pool_h: HANDLE,
        physical_address: u64,
        size: u64,
        memory_mapping: u32,
        perm: u32,
    ) -> u32;
    fn svcDumpInfo(dump_info_type: u32, arg0: u64);
    fn svcKernelDebug(kern_debug_type: u32, arg0: u64, arg1: u64, arg2: u64);
    fn svcChangeKernelTraceState(kern_trace_state: u32);
    fn svcCreateSession(server_handle: *mut HANDLE, client_handle: *mut HANDLE, unk0: u32, unk1: u64) -> u32;
    fn svcAcceptSession(session_handle: *mut HANDLE, port_handle: HANDLE) -> u32;
    fn svcReplyAndReceiveLight(handle: HANDLE) -> u32;
    fn svcReplyAndReceive(
        index: *mut i32,
        handles: *const HANDLE,
        handle_count: i32,
        reply_target: HANDLE,
        timeout: u64,
    ) -> u32;
    fn svcReplyAndReceiveWithUserBuffer(
        index: *mut i32,
        usr_buffer: *mut c_void,
        size: u64,
        handles: *const HANDLE,
        handle_count: i32,
        reply_target: HANDLE,
        timeout: u64,
    ) -> u32;
    fn svcCreateEvent(server_handle: *mut HANDLE, client_handle: *mut HANDLE) -> u32;
    fn svcMapIoRegion(io_region_h: HANDLE, address: *mut c_void, size: u64, perm: u32) -> u32;
    fn svcUnmapIoRegion(io_region_h: HANDLE, address: *mut c_void, size: u64) -> u32;
    fn svcMapPhysicalMemoryUnsafe(address: *mut c_void, size: u64) -> u32;
    fn svcUnmapPhysicalMemoryUnsafe(address: *mut c_void, size: u64) -> u32;
    fn svcSetUnsafeLimit(size: u64) -> u32;
    fn svcCreateCodeMemory(code_handle: *mut HANDLE, src_addr: *mut c_void, size: u64) -> u32;
    fn svcControlCodeMemory(
        code_handle: HANDLE,
        op: CodeMapOperation,
        dst_addr: *mut c_void,
        size: u64,
        perm: u64,
    ) -> u32;
    fn svcSleepSystem();
    fn svcReadWriteRegister(out_val: *mut u32, reg_addr: u64, rw_mask: u32, in_val: u32) -> u32;
    fn svcSetProcessActivity(process: HANDLE, paused: ProcessActivity) -> u32;
    fn svcCreateSharedMemory(out: *mut HANDLE, size: usize, local_perm: u32, other_perm: u32) -> u32;
    fn svcMapTransferMemory(tmem_handle: HANDLE, addr: *mut c_void, size: usize, perm: u32) -> u32;
    fn svcUnmapTransferMemory(tmem_handle: HANDLE, addr: *mut c_void, size: usize) -> u32;
    fn svcCreateInterruptEvent(handle: *mut HANDLE, irq_num: u64, flag: u32) -> u32;
    fn svcQueryPhysicalAddress(out: *mut PhysicalMemoryInfo, virtaddr: u64) -> u32;
    fn svcQueryIoMapping(virtaddr: *mut u64, out_size: *mut u64, physaddr: u64, size: u64) -> u32;
    fn svcLegacyQueryIoMapping(virtaddr: *mut u64, physaddr: u64, size: u64) -> u32;
    fn svcCreateDeviceAddressSpace(handle: *mut HANDLE, dev_addr: u64, dev_size: u64) -> u32;
    fn svcAttachDeviceAddressSpace(device: u64, handle: HANDLE) -> u32;
    fn svcDetachDeviceAddressSpace(device: u64, handle: HANDLE) -> u32;
    fn svcMapDeviceAddressSpaceByForce(
        handle: HANDLE,
        proc_handle: HANDLE,
        map_addr: u64,
        dev_size: u64,
        dev_addr: u64,
        perm: u32,
    ) -> u32;
    fn svcMapDeviceAddressSpaceAligned(
        handle: HANDLE,
        proc_handle: HANDLE,
        map_addr: u64,
        dev_size: u64,
        dev_addr: u64,
        perm: u32,
    ) -> u32;
    fn svcMapDeviceAddressSpace(
        out_mapped_size: *mut u64,
        handle: HANDLE,
        proc_handle: HANDLE,
        map_addr: u64,
        dev_size: u64,
        dev_addr: u64,
        perm: u32,
    ) -> u32;
    fn svcUnmapDeviceAddressSpace(
        handle: HANDLE,
        proc_handle: HANDLE,
        map_addr: u64,
        map_size: u64,
        dev_addr: u64,
    ) -> u32;
    fn svcInvalidateProcessDataCache(process: HANDLE, address: usize, size: usize) -> u32;
    fn svcStoreProcessDataCache(process: HANDLE, address: usize, size: usize) -> u32;
    fn svcFlushProcessDataCache(process: HANDLE, address: usize, size: usize) -> u32;
    fn svcDebugActiveProcess(debug: *mut HANDLE, process_id: u64) -> u32;
    fn svcBreakDebugProcess(debug: HANDLE) -> u32;
    fn svcTerminateDebugProcess(debug: HANDLE) -> u32;
    fn svcGetDebugEvent(event_out: *mut c_void, debug: HANDLE) -> u32;
    fn svcContinueDebugEvent(debug: HANDLE, flags: u32, tid_list: *mut u64, num_tids: u32) -> u32;
    fn svcLegacyContinueDebugEvent(debug: HANDLE, flags: u32, thread_id: u64) -> u32;
    fn svcGetDebugThreadContext(ctx: *mut ThreadContext, debug: HANDLE, thread_id: u64, flags: u32) -> u32;
    fn svcSetDebugThreadContext(debug: HANDLE, thread_id: u64, ctx: *const ThreadContext, flags: u32) -> u32;
    fn svcGetProcessList(num_out: *mut i32, pids_out: *mut u64, max_pids: u32) -> u32;
    fn svcGetThreadList(num_out: *mut i32, tids_out: *mut u64, max_tids: u32, debug: HANDLE) -> u32;
    fn svcQueryDebugProcessMemory(meminfo_ptr: *mut MemoryInfo, pageinfo: *mut u32, debug: HANDLE, addr: u64);
    fn svcReadDebugProcessMemory(buffer: *mut c_void, debug: HANDLE, addr: u64, size: u64) -> u32;
    fn svcWriteDebugProcessMemory(debug: HANDLE, buffer: *const c_void, addr: u64, size: u64) -> u32;
    fn svcSetHardwareBreakPoint(which: u32, flags: u64, value: u64) -> u32;
    fn svcGetDebugThreadParam(
        out_64: *mut u64,
        out_32: *mut u32,
        debug: HANDLE,
        thread_id: u64,
        parm: DebugThreadParam,
    ) -> u32;
    fn svcGetSystemInfo(out: *mut u64, id0: u64, handle: HANDLE, id1: u64) -> u32;
    fn svcCreatePort(
        portServer: *mut HANDLE,
        portClient: *mut HANDLE,
        max_sessions: i32,
        is_light: bool,
        name: *const c_char,
    ) -> u32;
    fn svcManageNamedPort(portServer: *mut HANDLE, name: *const c_char, maxSessions: i32) -> u32;
    fn svcConnectToPort(session: *mut HANDLE, port: HANDLE) -> u32;
    fn svcSetProcessMemoryPermission(proc: HANDLE, addr: u64, size: u64, perm: u32) -> u32;
    fn svcMapProcessMemory(dst: *mut c_void, proc: HANDLE, src: u64, size: u64) -> u32;
    fn svcUnmapProcessMemory(dst: *mut c_void, proc: HANDLE, src: u64, size: u64) -> u32;
    fn svcQueryProcessMemory(
        meminfo_ptr: *mut MemoryInfo,
        pageinfo: *mut u32,
        proc: HANDLE,
        addr: u64,
    ) -> u32;
    fn svcMapProcessCodeMemory(proc: HANDLE, dst: u64, src: u64, size: u64) -> u32;
    fn svcUnmapProcessCodeMemory(proc: HANDLE, dst: u64, src: u64, size: u64) -> u32;
    fn svcCreateProcess(out: *mut HANDLE, proc_info: *const c_void, caps: *const u32, cap_num: u64) -> u32;
    fn svcStartProcess(proc: HANDLE, main_prio: i32, default_cpu: i32, stack_size: u32) -> u32;
    fn svcTerminateProcess(proc: HANDLE) -> u32;
    fn svcGetProcessInfo(out: *mut i64, proc: HANDLE, which: ProcessInfoType) -> u32;
    fn svcCreateResourceLimit(out: *mut HANDLE) -> u32;
    fn svcSetResourceLimitLimitValue(reslimit: HANDLE, which: LimitableResource, value: u64) -> u32;
    fn svcCallSecureMonitor(regs: *mut SecmonArgs);
}
