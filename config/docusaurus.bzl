load("@bazel_skylib//lib:paths.bzl", "paths")

DocusaurusInfo = provider(
    doc = "Info pertaining to Docusaurus build.",
    fields = ["open_uri"],
)

def _docusaurus_pkg_impl(ctx):
    sandbox = ctx.actions.declare_directory(ctx.label.name + "_sandbox")
    output_dir = ctx.actions.declare_directory(ctx.label.name + "_html")

    root_dir = paths.dirname(paths.join(sandbox.path, ctx.file.config.short_path))

    # Docusaurus expects the config and index files to be in the root directory with the canonical
    # names.  This possibly renames and relocates the config and index files in the sandbox.
    shell_cmds = [
        "cp {} {}".format(ctx.file.config.path, paths.join(root_dir, "docusaurus.config.js")),
        "cp {} {}".format(ctx.file.sidebars.path, paths.join(root_dir, "sidebars.js")),
    ]

    folders = [root_dir]
    strip_srcs = ctx.attr.strip_path
    for f in ctx.files.srcs:
        short_path = f.short_path

        if short_path.startswith(strip_srcs):
            short_path = short_path[len(strip_srcs):]
            if short_path[0] == "/":
                short_path = short_path[1:]

        dest = paths.join(sandbox.path, short_path)
        folders.append(paths.dirname(dest))
        shell_cmds.append("cp {} {}".format(f.path, dest))

    shell_cmds_prepend = []
    for f in folders:
        cmd = "mkdir -p {}".format(f)
        if cmd not in shell_cmds_prepend:
            shell_cmds_prepend.append(cmd)

    shell_cmds = shell_cmds_prepend + shell_cmds

    ctx.actions.run_shell(
        outputs = [sandbox],
        inputs = ctx.files.config + ctx.files.sidebars + ctx.files.srcs,
        mnemonic = "DocusaurusCollect",
        command = "; ".join(shell_cmds),
        progress_message = "Collecting Docusaurus source documents for {}.".format(ctx.label.name),
    )

    args = ctx.actions.args()
    args.add("build")
    args.add("--config")
    args.add("docusaurus.config.js")

    ctx.actions.run(
        outputs = [output_dir],
        inputs = [sandbox],
        executable = "docusaurus",  #ctx.executable.binary,
        arguments = [args],
        mnemonic = "DocusaurusBuild",
        progress_message = "Building Docusaurus HTML documentation for {}.".format(ctx.label.name),
    )

    return [
        DefaultInfo(files = depset([output_dir])),
        DocusaurusInfo(open_uri = paths.join(output_dir.short_path, "index.html")),
    ]

docusaurus_pkg_gen = rule(
    implementation = _docusaurus_pkg_impl,
    doc = "Docusaurus HTML documentation.",
    attrs = {
        "args": attr.string_list(
            doc = "docusaurus-build argument list.",
        ),
        "config": attr.label(
            doc = "Docusaurus project config file.",
            allow_single_file = True,
            mandatory = True,
        ),
        "sidebars": attr.label(
            doc = "Docusaurus sidebars.",
            allow_single_file = True,
            mandatory = True,
        ),
        "srcs": attr.label_list(
            doc = "Docusaurus source and include files.",
            allow_files = True,
            mandatory = True,
            allow_empty = False,
        ),
        "strip_path": attr.string(
            doc = "Path to strip from srcs.",
            default = "",
            mandatory = False,
        ),
        #        "binary": attr.label(
        #            doc = "docusaurus-build executable.",
        #            executable = True,
        #            mandatory = True,
        #            cfg = "exec",
        #        ),
    },
)

def _docusaurus_view_impl(ctx):
    shell_cmd = ctx.attr.open_cmd.format(ctx.attr.generator[DocusaurusInfo].open_uri)

    script = ctx.actions.declare_file("{}.sh".format(ctx.label.name))
    ctx.actions.write(script, shell_cmd, is_executable = True)

    runfiles = ctx.runfiles(files = ctx.files.generator)

    return [DefaultInfo(executable = script, runfiles = runfiles)]

docusaurus_view = rule(
    implementation = _docusaurus_view_impl,
    doc = "View Docusaurus documentation.",
    attrs = {
        "generator": attr.label(
            doc = "Docusaurus documentation generation target.",
            mandatory = True,
            providers = [DocusaurusInfo],
        ),
        "open_cmd": attr.string(
            doc = "Shell open command for Docusaurus URI.",
            default = "xdg-open {} 1> /dev/null",
        ),
    },
    executable = True,
)

def docusaurus_pkg(name, **kwargs):
    view_args = {"generator": ":" + name}
    if "open_cmd" in kwargs:
        view_args["open_cmd"] = kwargs.pop("open_cmd")

    print(kwargs)
    docusaurus_pkg_gen(name = name, **kwargs)
    #docusaurus_view(name = name + ".view", **view_args)
