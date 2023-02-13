import glob
import os
import re

url_pattern = "https?:\\/\\/(?:www\\.)?[-a-zA-Z0-9@:%._\\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b(?:[-a-zA-Z0-9()@:%_\\+.~#?&\\/=]*)"
pattern_not_fenced = '([^\("\[])({})([^\)"\]])'.format(url_pattern)

for f in glob.glob("**/*.md", root_dir="old-docs", recursive=True):
    filename = os.path.join("old-docs", f)
    with open(filename, "r") as fb:
        contents = fb.read()

    print("FILE:", filename)
    if "\t" in contents:
        print("x", "Contain tabs!")
        contents = contents.replace("\t", "    ")

    reverse_replace = []
    for m in re.finditer(pattern_not_fenced, contents):
        reverse_replace.append({"span": m.span(), "url": m.group(2).strip()})

    for r in reversed(reverse_replace):
        f, t = r["span"]
        url = r["url"]
        if url.endswith(")"):
            continue

        if url.endswith("."):
            t -= 1
            url = url[:-1]

        new_url = "[{}]({})".format(url, url)
        contents = contents[:f] + new_url + contents[t:]
        print("- ", new_url)

    if (
        "(http://localhost:4201/)" not in contents
        and "[http://localhost:4201/]" not in contents
    ):
        parts = contents.split("http://localhost:4201/")
        contents = "[http://localhost:4201/](http://localhost:4201/)".join(parts)

    with open(filename, "w") as fb:
        fb.write(contents)
