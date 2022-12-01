"""
Module to workspace status variables in template files
"""

def _impl(ctx):
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

expand_workspace_status = rule(
    implementation = _impl,
    attrs = {
        "false_value": attr.string(default = "false"),
        "output": attr.output(mandatory = True),
        "template": attr.label(
            allow_single_file = True,
            mandatory = True,
        ),
        "true_value": attr.string(default = "true"),
        "_gen_tool": attr.label(
            executable = True,
            cfg = "exec",
            allow_files = True,
            default = Label("//config:expand_workspace_status"),
        ),
    },
)
