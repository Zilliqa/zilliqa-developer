load("@bazel_skylib//lib:paths.bzl", "paths")

CollectedFilesInfo = provider(
    doc = "Info pertaining to CollectedFiles build.",
    fields = ["open_uri"],
)

def _collect_files_impl(ctx):
    sandbox = ctx.actions.declare_directory(ctx.label.name + "_sandbox")

    root_dir = sandbox.path
    folders = [root_dir]
    strip_srcs = ctx.attr.strip_path
    shell_cmds = []
    for f in ctx.files.srcs:
        short_path = f.short_path

        if short_path.startswith(strip_srcs):
            short_path = short_path[len(strip_srcs):]
            if short_path[0] == "/":
                short_path = short_path[1:]

        dest = paths.join(root_dir, short_path)
        folders.append(paths.dirname(dest))
        shell_cmds.append("cp -L {} {}".format(f.path, dest))

    shell_cmds_prepend = []
    for f in folders:
        cmd = "mkdir -p {}".format(f)
        if cmd not in shell_cmds_prepend:
            shell_cmds_prepend.append(cmd)

    shell_cmds = shell_cmds_prepend + shell_cmds
    ctx.actions.run_shell(
        outputs = [sandbox],
        inputs = ctx.files.config + ctx.files.sidebars + ctx.files.srcs,
        mnemonic = "CollectedFilesCollect",
        command = "; ".join(shell_cmds) + "; echo \"CURRENT DIR: $(pwd)\"",
        progress_message = "Collecting CollectedFiles source documents for {}.".format(ctx.label.name),
    )

    return [
        DefaultInfo(files = depset([sandbox])),
    ]

collect_files_gen = rule(
    implementation = _collect_files_impl,
    doc = "CollectedFiles HTML documentation.",
    attrs = {
        "args": attr.string_list(
            doc = "docusaurus-build argument list.",
        ),
        "files": attr.output(),
        "srcs": attr.label_list(
            doc = "CollectedFiles source and include files.",
            allow_files = True,
            mandatory = True,
            allow_empty = False,
        ),
        "strip_path": attr.string(
            doc = "Path to strip from srcs.",
            default = "",
            mandatory = False,
        ),
    },
)

def collect_files(name, **kwargs):
    view_args = {"generator": ":" + name}
    if "open_cmd" in kwargs:
        view_args["open_cmd"] = kwargs.pop("open_cmd")

    collect_files_gen(name = name, **kwargs)
    #docusaurus_view(name = name + ".view", **view_args)
