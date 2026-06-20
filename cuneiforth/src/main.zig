const rl = @import("raylib");
const std = @import("std");

const VM = @import("vm.zig");

fn readKey() ?u32 {
    return switch (rl.getKeyPressed()) {
        .backspace => 8,
        .tab => 9,
        .enter => 13,
        .delete => 127,
        else => {
            const c = rl.getCharPressed();
            if (c != 0) {
                return @intCast(c);
            }
            return null;
        },
    };
}

fn render(vm: *VM, img: *rl.Image) void {
    const offset = 1 << 20;
    for (0..684) |y| {
        for (0..512) |x| {
            const pixel: rl.Color = if (vm.mem[offset + (y * 512) + x] == 0)
                .black
            else
                .white;
            rl.imageDrawPixel(img, @intCast(x), @intCast(y), pixel);
        }
    }
}

fn run(io: std.Io) anyerror!void {
    var vm: VM = .{
        .pc = 0,
        .mem = @splat(0),
        .state = .Run,
    };

    try vm.loadProg(io, "resources/disk.img");

    const screenWidth = 512;
    const screenHeight = 684;

    rl.initWindow(screenWidth * 2, screenHeight * 2, "cuneiforth");
    defer rl.closeWindow(); // Close window and OpenGL context

    rl.setTargetFPS(60); // Set our game to run at 60 frames-per-second

    var image = rl.genImageColor(screenWidth, screenHeight, .white);
    defer image.unload();

    var tex = try rl.loadTextureFromImage(image);
    defer tex.unload();

    while (!rl.windowShouldClose()) { // Detect window close button or ESC key
        vm.run();
        if (vm.state == .Render) {
            // std.debug.print("render\n", .{});
            render(&vm, &image);
            rl.updateTexture(tex, image.data);
            vm.resumeAfterRender();
        }

        if (vm.state == .Read) {
            if (readKey()) |k| {
                // std.debug.print("key {d}\n", .{k});
                vm.resumeWithKey(k);
            }
        }

        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(.white);

        rl.drawTextureEx(tex, .{ .x = 0, .y = 0 }, 0, 2.0, .white);
    }
}

fn docs(io: std.Io) anyerror!void {
    var file = try std.Io.Dir.cwd().openFile(io, "resources/disk.img", .{});
    defer file.close(io);

    var imgData: [43776]u8 = @splat(0);
    _ = try file.readPositionalAll(io, imgData[0..], 0);

    var img = rl.genImageColor(684, 512, .black);
    defer img.unload();

    var x: i32 = 0;
    var y: i32 = 0;
    for (imgData) |b| {
        var bit: u8 = 1 << 7;
        while (bit != 0) {
            if ((b & bit) != 0) {
                rl.imageDrawPixel(&img, x, y, .white);
            }
            x += 1;
            if (x == 684) {
                y += 1;
                x = 0;
            }
            bit >>= 1;
        }
    }

    _ = rl.exportImage(img, "docs.png");
}

pub fn main(init: std.process.Init) anyerror!void {
    const io = init.io;
    const args = try init.minimal.args.toSlice(init.arena.allocator());

    if (args.len == 1 or std.mem.eql(u8, args[1], "run")) {
        try run(io);
    } else if (std.mem.eql(u8, args[1], "docs")) {
        try docs(io);
    }
}
