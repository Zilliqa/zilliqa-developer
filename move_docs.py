import json
import os

import yaml

try:
    from yaml import CDumper as Dumper
    from yaml import CLoader as Loader
except ImportError:
    from yaml import Dumper, Loader

import glob
import re

print("Hello world")
with open("/tmp/test.json") as fb:
    cfg = json.loads(fb.read())

name_regex = "[^]]+"
url_regex = "[^)]+"

markup_regex = "\[({0})]\(\s*({1})\s*\)".format(name_regex, url_regex)
markup_regex_full = "\[({0})]\(\s*({1})\s*\)".format(name_regex, url_regex)


changes = {}
for item in cfg:

    _, name = item["path"].rsplit("/", 1)
    search_name = name
    if search_name.endswith(".md"):
        search_name = search_name[:-3]
    cand_pat = os.path.join("**/{}.md".format(search_name))
    candidates = [x for x in glob.glob(cand_pat, root_dir="docs", recursive=True)]
    if len(candidates) > 0:
        link_from = item["link_from"]
        if link_from not in changes:
            changes[link_from] = {}

        changes[link_from][name] = candidates[0]

for name, change in changes.items():
    search_name = name.split("://", 1)[1].split("/", 1)[1][:-1]
    cand_pat = os.path.join("**/{}.md".format(search_name))
    candidates = [x for x in glob.glob(cand_pat, root_dir="docs", recursive=True)]
    if len(candidates) == 0:

        print("Open {}".format(name))
        for k, v in change.items():
            print("- ", k, "=>", v)
        continue

    print("Automatic {}".format(name))

    with open(os.path.join("docs", candidates[0]), "r") as fb:
        contents = fb.read()

    replace_cand = []
    for match in re.finditer(markup_regex_full, contents):
        x1, x2 = match.span()
        replace_cand.append(
            {
                "from": x1,
                "to": x2,
                "name": match.group(1),
                "url": match.group(2),
                "full": match.group(),
            }
        )

    replacements = {}
    for k, v in change.items():
        if v.endswith(".md"):
            v = v[:-3]
        for r in replace_cand:

            if k in r["url"]:
                replacements[r["full"]] = "[{}](/{})".format(r["name"], v)

    print("Replacements?")
    for k, v in replacements.items():
        print(k, "=>", v)
        contents = contents.replace(k, v)

        # if match:
        #     print(match)
        #     full, name, url = match
        #     print(full)
        #     print(name)
        #     print(url)
    with open(os.path.join("docs", candidates[0]), "w") as fb:
        fb.write(final)


exit(0)
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
patches = {}
for src, dest in make_directories("new-docs", cfg["nav"], root):
    src = src[len(root) + 1 :]
    dest = dest[len(root) + 1 :]
    # os.system("git mv {} {}".format(src, dest))

    _, src = src.split("/", 1)
    _, dest = dest.split("/", 1)
    print(src, "->", dest)
    patches[src] = dest


def patch_nav(nav, patches):
    ret = []
    for item in nav:
        assert len(item.items()) == 1
        for name, tree in item.items():
            if isinstance(tree, str):
                if tree not in patches:
                    raise BaseException("Could not find patch")
                item[name] = patches[tree]
            else:
                patch_nav(tree, patches)


patch_nav(cfg["nav"], patches)

output = yaml.dump(cfg, Dumper=Dumper)
print(output)
