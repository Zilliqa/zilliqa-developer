# TODO: Migrate to  ./manage.py generateschema
def _impl(ctx):
    ctx.actions.run(
        inputs = [],
        outputs = [ctx.outputs.api_file],
        executable = ctx.executable.manage_executable,
        progress_message = "generating openapi definition %s" % ctx.label,
        arguments = ["generate_swagger", "-o", ctx.outputs.api_file.path],
    )

export_django_api = rule(
    attrs = {
        "api_file": attr.output(mandatory = True),
        "manage_executable": attr.label(
            executable = True,
            cfg = "exec",
            allow_files = False,
            mandatory = True,
        ),
    },
    implementation = _impl,
)
