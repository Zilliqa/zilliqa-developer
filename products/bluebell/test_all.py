import glob
import subprocess
import sys

success = 0
failed = 0
total = 0
for f in glob.glob(f"{sys.argv[1]}/*.scilla"):
    _, fname = f.rsplit("/", 1)
    process = subprocess.run(
        ["./target/release/bluebell", f], capture_output=True, text=True
    )
    dots = "." * (80 - len(fname))
    sys.stdout.write(f"Testing '{fname}' {dots} ")
    sys.stdout.flush()
    total += 1
    if process.returncode != 0:
        sys.stdout.write("[ FAILED ]\n")
        sys.stdout.flush()

        print("Output:")
        print(process.stdout)
        print("")
        success += 1
    else:
        sys.stdout.write("[   OK   ]\n")
        sys.stdout.flush()
        failed += 1


print(f"{success} successful and {failed} failed out of {total}")
