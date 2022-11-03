"""Bazel rules for generating sources and libraries from openapi schemas
"""

load("@bazel_tools//tools/build_defs/repo:jvm.bzl", "jvm_maven_import_external")

# https://swagger.io/docs/open-source-tools/swagger-codegen/
def openapi_repositories(swagger_codegen_cli_version = "2.4.9", swagger_codegen_cli_sha256 = "8555854798505f6ff924b55a95a7cc23d35b49aa9a62e8463b77da94b89b50b9", prefix = "io_bazel_rules_openapi"):
    jvm_maven_import_external(
        name = prefix + "_io_swagger_swagger_codegen_cli",
        artifact = "io.swagger:swagger-codegen-cli:" + swagger_codegen_cli_version,
        artifact_sha256 = swagger_codegen_cli_sha256,
        server_urls = ["https://repo.maven.apache.org/maven2"],
        licenses = ["notice"],  # Apache 2.0 License
    )
    native.bind(
        name = prefix + "/dependency/openapi-cli",
        actual = "@" + prefix + "_io_swagger_swagger_codegen_cli//jar",
    )

def _comma_separated_pairs(pairs):
    return ",".join([
        "{}={}".format(k, v)
        for k, v in pairs.items()
    ])

def _new_generator_command(ctx, gen_dir, rjars):
    java_path = ctx.attr._jdk[java_common.JavaRuntimeInfo].java_executable_exec_path
    gen_cmd = str(java_path)

    gen_cmd += " -cp {cli_jar}:{jars} io.swagger.codegen.SwaggerCodegen generate -i {spec} -l {language} -o {output}".format(
        java = java_path,
        cli_jar = ctx.file.codegen_cli.path,
        jars = ":".join([j.path for j in rjars.to_list()]),
        spec = ctx.file.spec.path,
        language = ctx.attr.language,
        output = gen_dir,
    )

    gen_cmd += ' -D "{properties}"'.format(
        properties = _comma_separated_pairs(ctx.attr.system_properties),
    )

    additional_properties = dict(ctx.attr.additional_properties)

    # This is needed to ensure reproducible Java output
    if ctx.attr.language == "java" and \
       "hideGenerationTimestamp" not in ctx.attr.additional_properties:
        additional_properties["hideGenerationTimestamp"] = "true"

    gen_cmd += ' --additional-properties "{properties}"'.format(
        properties = _comma_separated_pairs(additional_properties),
    )

    gen_cmd += ' --type-mappings "{mappings}"'.format(
        mappings = _comma_separated_pairs(ctx.attr.type_mappings),
    )

    if ctx.attr.api_package:
        gen_cmd += " --api-package {package}".format(
            package = ctx.attr.api_package,
        )
    if ctx.attr.invoker_package:
        gen_cmd += " --invoker-package {package}".format(
            package = ctx.attr.invoker_package,
        )
    if ctx.attr.model_package:
        gen_cmd += " --model-package {package}".format(
            package = ctx.attr.model_package,
        )

    # fixme: by default, swagger-codegen is rather verbose. this helps with that but can also mask useful error messages
    # when it fails. look into log configuration options. it's a java app so perhaps just a log4j.properties or something
    gen_cmd += " 2>/dev/null"
    return gen_cmd

def _impl(ctx):
    jars = _collect_jars(ctx.attr.deps)
    (cjars, rjars) = (jars.compiletime, jars.runtime)

    output_dir = ctx.actions.declare_directory(ctx.attr.name)
    commands = [
        _new_generator_command(ctx, output_dir.path, rjars),
    ]

    inputs = ctx.files._jdk + [
        ctx.file.codegen_cli,
        ctx.file.spec,
    ] + cjars.to_list() + rjars.to_list()

    ctx.actions.run_shell(
        inputs = inputs,
        outputs = [output_dir, ctx.actions.declare_directory("%s" % (ctx.attr.name))],  #, ctx.outputs.codegen],
        command = " && ".join(commands),
        progress_message = "generating openapi sources %s" % ctx.label,
        arguments = [],
    )

    return [
        DefaultInfo(
            files = depset([output_dir]),
        ),
    ]

# taken from rules_scala
def _collect_jars(targets):
    """Compute the runtime and compile-time dependencies from the given targets"""  # noqa
    compile_jars = depset()
    runtime_jars = depset()
    for target in targets:
        found = False
        if hasattr(target, "scala"):
            if hasattr(target.scala.outputs, "ijar"):
                compile_jars = depset(transitive = [compile_jars, [target.scala.outputs.ijar]])
            compile_jars = depset(transitive = [compile_jars, target.scala.transitive_compile_exports])
            runtime_jars = depset(transitive = [runtime_jars, target.scala.transitive_runtime_deps])
            runtime_jars = depset(transitive = [runtime_jars, target.scala.transitive_runtime_exports])
            found = True
        if hasattr(target, "JavaInfo"):
            # see JavaStarlarkApiProvider.java,
            # this is just the compile-time deps
            # this should be improved in bazel 0.1.5 to get outputs.ijar
            # compile_jars = depset(transitive = [compile_jars, [target.java.outputs.ijar]])
            compile_jars = depset(transitive = [compile_jars, target[JavaInfo].transitive_deps])
            runtime_jars = depset(transitive = [runtime_jars, target[JavaInfo].transitive_runtime_deps])
            found = True
        if not found:
            # support http_file pointed at a jar. http_jar uses ijar,
            # which breaks scala macros
            runtime_jars = depset(transitive = [runtime_jars, target.files])
            compile_jars = depset(transitive = [compile_jars, target.files])

    return struct(compiletime = compile_jars, runtime = runtime_jars)

openapi_gen = rule(
    attrs = {
        "additional_properties": attr.string_dict(),
        "api_package": attr.string(),
        "codegen_cli": attr.label(
            cfg = "exec",
            default = Label("//external:io_bazel_rules_openapi/dependency/openapi-cli"),
            allow_single_file = True,
        ),
        # downstream dependencies
        "deps": attr.label_list(),
        "invoker_package": attr.string(),
        # language to generate
        "language": attr.string(mandatory = True),
        "model_package": attr.string(),
        # openapi spec file
        "spec": attr.label(
            mandatory = True,
            allow_single_file = [".json", ".yaml"],
        ),
        "system_properties": attr.string_dict(),
        "type_mappings": attr.string_dict(),
        "_jdk": attr.label(
            default = Label("@bazel_tools//tools/jdk:current_java_runtime"),
            providers = [java_common.JavaRuntimeInfo],
        ),
    },
    implementation = _impl,
)
