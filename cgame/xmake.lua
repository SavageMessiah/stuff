add_rules("mode.debug", "mode.release")

add_requires("raylib", { system = false })

target("cgame")
set_kind("binary")
add_files("src/main.c")
add_packages("raylib")
set_languages("c99")
set_toolchains("clang")
set_warnings("all", "error")
