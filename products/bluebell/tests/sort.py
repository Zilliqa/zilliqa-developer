import glob
import os

data = []

for f in glob.glob("./data/formatter/formatted/*.scilla"):
    with open(f, "r") as fb:
        d = fb.read()
        data.append((len(d), f))


i = 0
for _, f in sorted(data):
    path, name = f.rsplit("/", 1)

    try:
        number, remainder = name.split("_", 1)
        if len(number) > 3:
            remainder = name
        else:
            int(number)
            remainder = remainder
    except:
        remainder = name

    i += 1
    number = str(i).rjust(3).replace(" ", "0")
    newname = "{}_{}".format(number, remainder)
    f2 = os.path.join(path, newname)
    print(f, f2)
    if f != f2:
        os.system("mv {} {}".format(f, f2))
