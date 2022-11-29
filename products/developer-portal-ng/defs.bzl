load("@bazel_skylib//lib:paths.bzl", "paths")

MkDocsInfo = provider(
    doc = "Info pertaining to MkDocs build.",
    fields = ["open_uri"],
)

def _mkdocs_html_impl(ctx):
    sandbox = ctx.actions.declare_directory(ctx.label.name + "_sandbox")
    output_dir = ctx.actions.declare_directory(ctx.label.name + "_html")
    rel_outdir = paths.join("..", ctx.label.name + "_html")
    root_dir = sandbox.path
    folders = [root_dir]
    strip_srcs = ctx.attr.strip_path
    shell_cmds = [
        "cp {} {}".format(ctx.file.config.path, paths.join(root_dir, "mkdocs.yml")),
    ]

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
        inputs = ctx.files.srcs,
        mnemonic = "CollectedFilesCollect",
        command = "; ".join(shell_cmds),
        progress_message = "Collecting CollectedFiles source documents for {}.".format(ctx.label.name),
    )

    args = ctx.actions.args()
    args.add("build")
    args.add("-d")
    args.add(rel_outdir)
    args.add("-f")
    args.add(paths.join(root_dir, "mkdocs.yml"))

    ctx.actions.run(
        outputs = [output_dir],
        inputs = [sandbox],
        executable = ctx.executable._mkdocs_build,
        arguments = [args],
        mnemonic = "MkDocsBuild",
        progress_message = "Building MkDocs HTML documentation for {}.".format(ctx.label.name),
    )

    return [
        DefaultInfo(files = depset([output_dir])),
        MkDocsInfo(open_uri = paths.join(output_dir.short_path, "index.html")),
    ]

mkdocs_html_gen = rule(
    implementation = _mkdocs_html_impl,
    doc = "MkDocs HTML documentation.",
    attrs = {
        "args": attr.string_list(
            doc = "mkdocs-build argument list.",
        ),
        "config": attr.label(
            doc = "MkDocs project config file.",
            allow_single_file = True,
            mandatory = True,
        ),
        "srcs": attr.label_list(
            doc = "MkDocs source and include files.",
            allow_files = True,
            mandatory = True,
            allow_empty = False,
        ),
        "strip_path": attr.string(
            doc = "Path to strip from srcs.",
            default = "",
            mandatory = False,
        ),
        "_mkdocs_build": attr.label(
            doc = "mkdocs-build wrapper.",
            default = Label("//products/developer-portal-ng/tools:mkdocs_wrapper"),
            executable = True,
            cfg = "exec",
        ),
    },
)

def _mkdocs_view_impl(ctx):
    shell_cmd = ctx.attr.open_cmd.format(ctx.attr.generator[MkDocsInfo].open_uri)

    script = ctx.actions.declare_file("{}.sh".format(ctx.label.name))
    ctx.actions.write(script, shell_cmd, is_executable = True)

    runfiles = ctx.runfiles(files = ctx.files.generator)

    return [DefaultInfo(executable = script, runfiles = runfiles)]

mkdocs_view = rule(
    implementation = _mkdocs_view_impl,
    doc = "View MkDocs documentation.",
    attrs = {
        "generator": attr.label(
            doc = "MkDocs documentation generation target.",
            mandatory = True,
            providers = [MkDocsInfo],
        ),
        "open_cmd": attr.string(
            doc = "Shell open command for MkDocs URI.",
            default = "xdg-open {} 1> /dev/null",
        ),
    },
    executable = True,
)

def mkdocs_html(name, **kwargs):
    view_args = {"generator": ":" + name}
    if "open_cmd" in kwargs:
        view_args["open_cmd"] = kwargs.pop("open_cmd")

    mkdocs_html_gen(name = name, **kwargs)
    mkdocs_view(name = name + ".view", **view_args)
