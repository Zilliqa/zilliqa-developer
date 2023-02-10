import os

import yaml

try:
    from yaml import CDumper as Dumper
    from yaml import CLoader as Loader
except ImportError:
    from yaml import Dumper, Loader

print("Hello world")

with open("products/developer-portal/mkdocs.yml") as fb:
    cfg = yaml.load(fb.read(), Loader=Loader)


def make_directories(name, nav, root_dir):
    dirname = name.lower().strip().replace(" ", "-")
    if not os.path.exists(dirname):
        os.mkdir(dirname)
    os.chdir(dirname)

    ret = []
    for item in nav:
        assert len(item.items()) == 1
        for name, tree in item.items():
            if isinstance(tree, str):
                src = os.path.join(root_dir, "docs", tree)
                _, filename = src.rsplit("/", 1)
                dst = os.path.join(os.getcwd(), filename)
                ret.append((src, dst))
            else:
                ret += make_directories(name, tree, root_dir)
    os.chdir("..")
    return ret


root = os.getcwd()
for src, dest in make_directories("new-docs", cfg["nav"], root):
    src = src[len(root) + 1 :]
    dest = dest[len(root) + 1 :]
    os.system("git mv {} {}".format(src, dest))
    print(src, "->", dest)
