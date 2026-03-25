// Zig ffi_utils placeholder
// Provides a basic Mutex wrapper for interoperability.
const std = @import("std");

pub const FfiMutex = struct {
    // Zig has builtin support for mutexes via std.Thread.Mutex, but for
    // portability we'll provide a thin wrapper around std.Thread.Mutex
    inner: std.Thread.Mutex,

    pub fn init() FfiMutex { return FfiMutex{ .inner = std.Thread.Mutex.init() }; }

    pub fn lock(self: *FfiMutex) void { self.inner.lock(); }
    pub fn unlock(self: *FfiMutex) void { self.inner.unlock(); }
};
