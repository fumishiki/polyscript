const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    var args = std.process.args();
    _ = args.next(); // skip program name
    try stdout.writeAll("[Zig] args:");
    while (args.next()) |arg| {
        try stdout.print(" {s}", .{arg});
    }
    try stdout.writeByte('\n');
}
