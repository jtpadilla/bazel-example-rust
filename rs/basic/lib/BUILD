load("@io_bazel_rules_rust//proto:proto.bzl", "rust_proto_library")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_library(
    name = "basic_main_lib",
    srcs = ["src/lib.rs"],
    visibility = ["//rs:__subpackages__"],
    deps = ["//proto:common_proto_rust"],
)
