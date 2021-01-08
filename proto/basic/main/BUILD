load("@io_bazel_rules_rust//proto:proto.bzl", "rust_proto_library")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

package(default_visibility = ["//proto:__subpackages__"])

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    visibility = ["//proto/helloworld:__subpackages__"],
    deps = [
        "//proto:common_proto_rust",
        "//proto/basic/lib:common_lib",
    ],
)
