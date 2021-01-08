load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "rs_simple_hello_world",
    srcs = ["src/main.rs"],
    deps = ["//rs/simple/hello_lib:rs_simple_hello_lib"],
)
