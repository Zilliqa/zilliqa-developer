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
from pathlib import Path

print("Hello world")
with open("/tmp/test.json") as fb:
    cfg = json.loads(fb.read())

name_regex = "[^]]+"
url_regex = "[^)]+"

markup_regex = "\[({0})]\(\s*({1})\s*\)".format(name_regex, url_regex)
markup_regex_full = "\[({0})]\(\s*({1})\s*\)".format(name_regex, url_regex)

global_replacements = {}
for x in cfg:
    orig_fullpath = x["path"]
    path, name = orig_fullpath.rsplit("/", 1)

    if "%" in name:
        name = name.split("%", 1)[0]

    if "'" in name:
        name = name.split("%", 1)[0]

    if name == "":
        continue

    fullpath = os.path.join(path, name)

    if orig_fullpath != fullpath:
        global_replacements["({})".format(orig_fullpath)] = "({})".format(fullpath)

    cand_pat = "**/{}".format(name)
    cands = [x for x in glob.glob(cand_pat, root_dir="old-docs", recursive=True)]
    print("Looing for", name)
    if len(cands):
        print("Found missing file", name)
        src = os.path.join("old-docs", cands[0])
        dest = os.path.join("docs", cands[0])
        dest_folder, _ = dest.rsplit("/", 1)
        try:
            os.makedirs(dest_folder)
        except:
            pass
        print(src, "=>", dest)
        os.system("git mv {} {}".format(src, dest))

# exit(0)
for f in glob.glob("**/*.md", root_dir="docs", recursive=True):
    print("FILE:", f)
    filename = os.path.abspath(os.path.join("docs", f))
    dirname = os.path.abspath(os.path.dirname(filename))
    with open(filename, "r") as fb:
        contents = fb.read()

    for k, v in global_replacements.items():
        print("GREP", k, "=>", v)
        contents = contents.replace(k, v)

    replace_cand = []
    replacements = {}
    for match in re.finditer(markup_regex_full, contents):
        anchor = None
        x1, x2 = match.span()
        url = match.group(2)
        replace_cand.append(
            {
                "from": x1,
                "to": x2,
                "name": match.group(1),
                "url": url,
                "full": match.group(),
            }
        )

        if url.startswith("http://") or url.startswith("https://"):
            continue

        anchor = None
        if "#" in url:
            url, anchor = url.split("#", 1)

        url = url.strip()
        if url.strip() == "":
            continue
        print("MATCH:", url)

        fullpath = os.path.abspath(os.path.join(dirname, url))
        if url[0] == "/":
            fullpath = os.path.abspath(os.path.join("docs", url[1:]))

        print("-", fullpath)
        print("-", os.path.exists(fullpath))
        print("-", os.path.exists(fullpath + ".md"))
        if os.path.exists(fullpath + ".md"):
            fullpath += ".md"

        if os.path.exists(fullpath):
            relpath = os.path.relpath(fullpath, dirname)
            # if relpath.endswith(".md"):
            #     relpath = relpath[:-3]
            if anchor:
                relpath = "{}#{}".format(relpath, anchor)
            old = match.group()
            new = "[{}]({})".format(match.group(1), relpath)
            print("=>", new)
            if old != new:
                replacements[old] = new
            continue

        if "/" in url:
            _, url = url.rsplit("/", 1)

        if url.endswith(".md"):
            url = url[:-3]

        if url == "":
            continue

        if "." not in url:
            url += ".md"

        cand_pat = "**/{}".format(url)
        cands = [x for x in glob.glob(cand_pat, root_dir="docs", recursive=True)]
        print("Searching for", cand_pat, "=>", cands)
        if len(cands) == 0:
            print("- No candidates for", match.group(2))
        else:
            link_file = os.path.abspath(os.path.join("docs", cands[0]))
            relpath = os.path.relpath(link_file, os.path.dirname(filename))
            # relpath = relpath[:-3]
            if match.group(2) != relpath:
                old = match.group()
            if anchor:
                relpath = "{}#{}".format(relpath, anchor)
            old = match.group()
            new = "[{}]({})".format(match.group(1), relpath)
            print("=>", new)
            if old != new:
                replacements[old] = new

    # print(filename)
    for k, v in replacements.items():
        print("REP", k, "=>", v)
        contents = contents.replace(k, v)

    with open(filename, "w") as fb:
        fb.write(contents)
    continue

    # if match:
    #     print(match)
    #     full, name, url = match
    #     print(full)
    #     print(name)
    #     print(url)
    # with open(os.path.join("docs", candidates[0]), "w") as fb:
    #     fb.write(contents)

exit(0)
changes = {}
for item in cfg:

    _, name = item["path"].rsplit("/", 1)
    search_name = name
    if search_name.endswith(".md"):
        search_name = search_name[:-3]

    candidates = [x for x in glob.glob(cand_pat, root_dir="docs", recursive=True)]
    if len(candidates) > 0:
        link_from = item["link_from"]
        if link_from not in changes:
            changes[link_from] = {}

        changes[link_from][name] = candidates[0]
    else:
        print("NOT FOUND:", search_name)
print("-------")

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

    filename = os.path.join("docs", candidates[0])
    with open(filename, "r") as fb:
        contents = fb.read()

    replace_cand = []
    replacements = {}
    for match in re.finditer(markup_regex_full, contents):
        x1, x2 = match.span()
        url = match.group(2)
        replace_cand.append(
            {
                "from": x1,
                "to": x2,
                "name": match.group(1),
                "url": url,
                "full": match.group(),
            }
        )

        if "/" in url:
            _, url = url.split("/", 1)
        if url.endswith(".md"):
            url = url[:-3]

        cand_pat = "**/{}.md".format(url)
        cands = [x for x in glob.glob(cand_pat, root_dir="docs", recursive=True)]
        if len(cands) > 0:
            link_file = os.path.join("docs", cands[0])
            relpath = os.path.relpath(link_file, os.path.dirname(filename))
            relpath = relpath[:-3]
            if match.group(2) != relpath:
                old = match.group()
                new = "[{}]({})".format(match.group(1), relpath)
                replacements[old] = new

    print("")
    print(filename)
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
        fb.write(contents)


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
