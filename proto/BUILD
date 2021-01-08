load("@rules_proto//proto:defs.bzl", "proto_library")
load("@io_bazel_rules_rust//proto:proto.bzl", "rust_proto_library")

package(default_visibility = ["//:__subpackages__"])

proto_library(
    name = "common",
    srcs = ["common.proto"],
)

rust_proto_library(
    name = "common_proto_rust",
    deps = [":common"],
)
