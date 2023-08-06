const std = @import("std");

const allocator = std.heap.wasm_allocator;

extern fn consoleLog(ptr: [*]const u8) void;

export fn sayHello() void {
    // nice try
    consoleLog("HELLO ZIG!!!!");
}

export fn mallocu8(length: usize) ?[*]u8 {
    const buff = allocator.alloc(u8, length) catch return null;
    return buff.ptr;
}

export fn freeu8(buf: [*]u8, length: usize) void {
    allocator.free(buf[0..length]);
}

export fn reverseString(str: [*]const u8, size: u32) u64 {
    var code_points = std.ArrayList([]const u8).initCapacity(allocator, size) catch return 0;
    defer code_points.deinit();
    const view = std.unicode.Utf8View.init(str[0..size]) catch return 0;
    var iter = view.iterator();
    var num_code_points: u32 = 0;
    while (iter.nextCodepointSlice()) |cps| {
        code_points.insert(num_code_points, cps) catch break;
        num_code_points += 1;
    }

    var reversed = std.ArrayList(u8).initCapacity(allocator, size) catch return 0;
    defer reversed.deinit();
    var i = num_code_points;
    var j: usize = 0;
    while (i > 0) {
        i -= 1;
        const cp = code_points.items[i];
        for (cp) |b| {
            reversed.insert(j, b) catch break;
            j += 1;
        }
    }
    const reversed_slice = reversed.toOwnedSlice() catch return 0;
    const msg = std.fmt.allocPrint(allocator, "?{s}", .{reversed_slice}) catch return 0;
    const p: u64 = @intFromPtr(msg.ptr);
    return p << 32 | msg.len;
}
