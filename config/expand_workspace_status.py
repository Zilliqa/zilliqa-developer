import argparse
from string import Template


def main():

    parser = argparse.ArgumentParser(
        description="Bake a git hash into a header & source."
    )
    parser.add_argument("--output", required=True, help="output file")

    parser.add_argument("--template", required=True, help="template  file")
    parser.add_argument("--true_value", required=True, help="value that denotes true")
    parser.add_argument("--false_value", required=True, help="value that denotes false")

    parser.add_argument(
        "--volatile_file", required=True, help="file containing the volatile variables"
    )
    parser.add_argument(
        "--stable_file", required=True, help="file containing the stable variables"
    )

    args = parser.parse_args()

    with open(args.stable_file, "r") as fb:
        stable_values = [
            x.strip().split(" ", 1) for x in fb.readlines() if " " in x.strip()
        ]

    with open(args.volatile_file, "r") as fb:
        volatile_values = [
            x.strip().split(" ", 1) for x in fb.readlines() if " " in x.strip()
        ]

    values = dict(stable_values + volatile_values)

    with open(args.template, "r") as fb:
        template = Template(fb.read())

    with open(args.output, "w") as f:
        f.write(template.substitute(**values))


if __name__ == "__main__":
    main()
