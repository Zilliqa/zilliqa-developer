"""
Executes a python script that gets the volatile & stable workspace status
files as an input.
The result is a C++ header & source file containing the git hash & git dirty flag.
"""

def _impl(ctx):
    # The list of arguments we pass to the script.
    # volatile status file: ctx.version_file
    # stable status file: ctx.info_file
    args = [
        "--output",
        ctx.outputs.output.path,
        "--template",
        ctx.file.template.path,
        "--volatile_file",
        ctx.version_file.path,
        "--stable_file",
        ctx.info_file.path,
        "--true_value",
        ctx.attr.true_value,
        "--false_value",
        ctx.attr.false_value,
    ]

    # Action to call the script.
    ctx.actions.run(
        inputs = [ctx.version_file, ctx.info_file, ctx.file.template],
        outputs = [ctx.outputs.output],
        arguments = args,
        progress_message = "Adding Git Hash to %s" % ctx.outputs.output.short_path,
        executable = ctx.executable._gen_tool,
    )

    if ctx.outputs.output.path.endswith(".py"):
        return [
            PyInfo(
                transitive_sources = depset([ctx.outputs.output]),
            ),
        ]

    return [
        DefaultInfo(
            files = depset([ctx.outputs.output]),
        ),
    ]

generate_version_library = rule(
    implementation = _impl,
    attrs = {
        "false_value": attr.string(default = "false"),
        "output": attr.output(mandatory = True),
        "template": attr.label(
            allow_single_file = [".tpl"],
            mandatory = True,
        ),
        "true_value": attr.string(default = "true"),
        "_gen_tool": attr.label(
            executable = True,
            cfg = "exec",
            allow_files = True,
            default = Label("//config:version-lib-generator"),
        ),
    },
)
