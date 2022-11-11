load("@bazel_skylib//lib:paths.bzl", "paths")

DocusaurusInfo = provider(
    doc = "Info pertaining to Docusaurus build.",
    fields = ["open_uri"],
)

def _docusaurus_html_impl(ctx):
    sandbox = ctx.actions.declare_directory(ctx.label.name + "_sandbox")
    output_dir = ctx.actions.declare_directory(ctx.label.name + "_html")

    root_dir = paths.dirname(paths.join(sandbox.path, ctx.file.config.short_path))

    # Docusaurus expects the config and index files to be in the root directory with the canonical
    # names.  This possibly renames and relocates the config and index files in the sandbox.
    shell_cmds = [
        "mkdir -p {}".format(root_dir),
        "cp {} {}".format(ctx.file.config.path, paths.join(root_dir, "conf.py")),
        "cp {} {}".format(ctx.file.index.path, paths.join(root_dir, "index.rst")),
    ]

    for f in ctx.files.srcs:
        dest = paths.join(sandbox.path, f.short_path)
        shell_cmds.append("mkdir -p {}; cp {} {}".format(paths.dirname(dest), f.path, dest))

    ctx.actions.run_shell(
        outputs = [sandbox],
        inputs = ctx.files.config + ctx.files.index + ctx.files.srcs,
        mnemonic = "DocusaurusCollect",
        command = "; ".join(shell_cmds),
        progress_message = "Collecting Docusaurus source documents for {}.".format(ctx.label.name),
    )

    args = ctx.actions.args()
    args.add("-b", "html")
    args.add("-q")
    args.add_all(ctx.attr.args)
    args.add(root_dir)
    args.add(output_dir.path)

    ctx.actions.run(
        outputs = [output_dir],
        inputs = [sandbox],
        executable = ctx.executable._docusaurus_build,
        arguments = [args],
        mnemonic = "DocusaurusBuild",
        progress_message = "Building Docusaurus HTML documentation for {}.".format(ctx.label.name),
    )

    return [
        DefaultInfo(files = depset([output_dir])),
        DocusaurusInfo(open_uri = paths.join(output_dir.short_path, "index.html")),
    ]

docusaurus_html_gen = rule(
    implementation = _docusaurus_html_impl,
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
        "docs": attr.label_list(
            doc = "Docusaurus docsfiles.",
            allow_files = True,
            mandatory = True,
            allow_empty = False,
        ),
        "index": attr.label(
            doc = "Docusaurus project index.",
            allow_single_file = True,
            mandatory = True,
        ),
        "srcs": attr.label_list(
            doc = "Docusaurus source and include files.",
            allow_files = True,
            mandatory = True,
            allow_empty = False,
        ),
        "_docusaurus_build": attr.label(
            doc = "docusaurus-build wrapper.",
            default = Label("@rules_docusaurus//docusaurus/tools:docusaurus_build_wrapper"),
            executable = True,
            cfg = "exec",
        ),
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

def docusaurus_html(name, **kwargs):
    view_args = {"generator": ":" + name}
    if "open_cmd" in kwargs:
        view_args["open_cmd"] = kwargs.pop("open_cmd")

    print(kwargs)
    #docusaurus_html_gen(name = name, **kwargs)
    #docusaurus_view(name = name + ".view", **view_args)
