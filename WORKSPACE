load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

git_repository(
    name = "io_bazel_rules_rust",
    commit = "df18ddbece5b68f86e63414ea4b50d691923039a",
    remote = "https://github.com/bazelbuild/rules_rust.git",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories(
    edition = "2018",
    version = "1.48.0",
)

load("@io_bazel_rules_rust//proto:repositories.bzl", "rust_proto_repositories")

rust_proto_repositories()
