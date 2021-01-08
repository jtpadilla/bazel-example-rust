load("@io_bazel_rules_rust//proto:proto.bzl", "rust_proto_library")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

package(default_visibility = ["//proto:__subpackages__"])

rust_library(
    name = "common_lib",
    srcs = ["src/lib.rs"],
    deps = ["//proto:common_proto_rust"],
)
