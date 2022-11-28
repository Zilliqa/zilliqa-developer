load("@bazel_skylib//lib:paths.bzl", "paths")

VuePressInfo = provider(
    doc="Info pertaining to VuePress build.",
    fields=["open_uri"],
)


def _collect_pkg_impl(ctx):
    sandbox = ctx.actions.declare_directory(ctx.label.name)
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

        dest = paths.join(sandbox.path, short_path)
        folders.append(paths.dirname(dest))
        shell_cmds.append("cp  {} {}".format(f.path, dest))

    print("C")

    shell_cmds_prepend = []
    for f in folders:
        cmd = "mkdir -p {}".format(f)
        if cmd not in shell_cmds_prepend:
            shell_cmds_prepend.append(cmd)
    print("D")

    shell_cmds = shell_cmds_prepend + shell_cmds
    ctx.actions.run_shell(
        outputs=[sandbox],
        inputs=ctx.files.srcs,
        mnemonic="VuePressCollect",
        command="; ".join(shell_cmds) + "; echo \"CURRENT DIR: $(pwd)\"",
        progress_message="Collecting VuePress source documents for {}.".format(ctx.label.name),
    )

    print("OUTPUT IN: ", sandbox.path)

    return [
        DefaultInfo(files=depset([sandbox])),
    ]


collect = rule(
    implementation=_collect_pkg_impl,
    doc="VuePress HTML documentation.",
    attrs={
        "srcs": attr.label_list(
            doc="VuePress source and include files.",
            allow_files=True,
            mandatory=True,
            allow_empty=False,
        ),
        "strip_path": attr.string(
            doc="Path to strip from srcs.",
            default="",
            mandatory=False,
        ),
        "files": attr.output(),
    },
)
