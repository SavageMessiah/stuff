const State = enum { Run, Render, Read, End };
pc: u32 = 0,
mem: [1 << 21]u32 = @splat(0),
state: State = .Run,

const Self = @This();

fn init(mem: []const u32) Self {
    var vm: Self = .{};
    @memcpy(vm.mem[0..mem.len], mem);
    return vm;
}

pub fn loadProg(vm: *Self, io: std.Io, path: []const u8) !void {
    var file = try std.Io.Dir.cwd().openFile(io, path, .{});
    defer file.close(io);

    const doc_len = 43776;
    const stat = try file.stat(io);
    const prog_len = (stat.size - doc_len) / 4;

    var buf: [1024]u8 = undefined;
    var reader = file.reader(io, buf[0..]);
    _ = try std.Io.Reader.discard(&reader.interface, std.Io.Limit.limited(doc_len));

    try std.Io.Reader.readSliceEndian(&reader.interface, u32, vm.mem[0..prog_len], .big);
}

pub fn resumeWithKey(vm: *Self, key: u32) void {
    if (vm.state != .Read) {
        return;
    }
    const addr = vm.mem[vm.pc + 1];
    vm.mem[addr] = key;
    vm.pc += 4;
    vm.state = .Run;
}

pub fn resumeAfterRender(vm: *Self) void {
    if (vm.state != .Render) {
        return;
    }
    vm.pc += 4;
    vm.state = .Run;
}

inline fn set(vm: *Self, idx: u32, val: u32) void {
    vm.mem[idx] = val;
    //std.debug.print("set mem[{}] to {}\n", .{ idx, val });
}

fn runOne(vm: *Self) void {
    const op = vm.mem[vm.pc];
    const a = vm.mem[vm.pc + 1];
    const b = vm.mem[vm.pc + 2];
    const c = vm.mem[vm.pc + 3];

    //std.debug.print("pc: {d} op: {d} a: {d} -> {d} b: {d} -> {d} c: {d} -> {d}\n", .{ vm.pc, op, a, vm.mem[a], b, vm.mem[b], c, vm.mem[c] });

    switch (op) {
        1 => vm.pc = vm.mem[a],
        2 => if (vm.mem[b] == 0) {
            vm.pc = vm.mem[a];
        } else {
            vm.pc += 4;
        },
        3 => vm.set(a, vm.pc),
        4 => vm.set(a, vm.mem[b]),
        5 => vm.set(a, vm.mem[vm.mem[b]]),
        6 => vm.set(vm.mem[b], vm.mem[a]),
        7 => vm.set(a, vm.mem[b] +% vm.mem[c]),
        8 => vm.set(a, vm.mem[b] -% vm.mem[c]),
        9 => vm.set(a, vm.mem[b] *% vm.mem[c]),
        10 => vm.set(a, vm.mem[b] / vm.mem[c]),
        11 => vm.set(a, vm.mem[b] % vm.mem[c]),
        12 => vm.set(a, @intFromBool(vm.mem[b] < vm.mem[c])),
        13 => vm.set(a, ~(vm.mem[b] & vm.mem[c])),
        14 => {
            vm.state = .Render;
            return;
        },
        15 => {
            vm.state = .Read;
            return;
        },
        else => {
            vm.state = .End;
            return;
        },
    }

    if (op != 1 and op != 2) {
        vm.pc += 4;
    }
}

pub fn run(vm: *Self) void {
    if (vm.state != .Run) {
        return;
    }

    //var i: u8 = 0;
    while (vm.state == .Run) {
        vm.runOne();
        //i += 1;
        // if (i > 100) {
        //     vm.state = .End;
        // }
    }
}

const std = @import("std");
const expect = std.testing.expect;

test "ops" {
    const T = struct {
        name: []const u8,
        mem: []const u32,
        state: Self.State = .Run,
        pc: u32,
        result: ?[]const u32 = null,
    };
    const tests = [_]T{ .{ .name = "set pc", .mem = &.{
        1, 4, 0, 0,
        2, 0,
    }, .pc = 2 }, .{ .name = "jz 0", .mem = &.{
        2, 6, 5, 0,
        0, 0, 8, 0,
        0, 0, 0, 0,
    }, .pc = 8 }, .{ .name = "jz 1", .mem = &.{
        2, 6, 5, 0,
        0, 1, 4, 0,
        0, 0, 0, 0,
    }, .pc = 4 }, .{ .name = "read", .mem = &.{
        15,
        0,
        0,
        0,
    }, .pc = 0, .state = .Read, .result = null } };

    for (tests) |t| {
        std.debug.print("test {s}\n", .{t.name});
        var vm = Self.init(t.mem);
        vm.runOne();
        std.debug.print("vm.pc {d}\n", .{vm.pc});
        try expect(vm.pc == t.pc);
        try expect(vm.state == t.state);
    }
}

test "prog" {
    var vm = Self.init(&.{
        1, 10, 0, 0,
        1, 11, 0, 0,
        0, 0,  4, 8,
    });
    vm.run();
    std.debug.print("vm.pc {d}\n", .{vm.pc});
    try expect(vm.pc == 8);
    try expect(vm.state == .End);
}
