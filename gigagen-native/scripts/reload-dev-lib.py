import random
import shutil
import pathlib
import sys
import argparse

libname = "gigagen"
projname = "Gigagen"
script_dir = pathlib.Path(__file__).parent.resolve()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("platform", type=str)
    return parser.parse_args()


def main():
    args = parse_args()
    random_id = str(random.randrange(0, 99999)).rjust(5, "0")
    if args.platform == "linux":
        lib_src_name = f"lib{libname}.so"
        lib_dst_name = f"lib{libname}-{random_id}.so"
    elif args.platform == "windows":
        lib_src_name = f"{libname}.dll"
        lib_dst_name = f"{libname}-{random_id}.dll"
    else:
        print(f"unknown platform '{args.platform}'", file=sys.stderr)
        sys.exit(1)

    lib_src_path = script_dir.joinpath("..", "target", "release", lib_src_name)
    if not lib_src_path.exists():
        print(f"library '{lib_src_path}' does not exist", file=sys.stderr)
        sys.exit(1)

    cs_src_path = script_dir.parent.joinpath("bindings", f"Native.cs")
    if not cs_src_path.exists():
        print(f"binding '{lib_src_path}' does not exist", file=sys.stderr)
        sys.exit(1)

    lib_dst_path = script_dir.parent.parent.joinpath(
        "Assets",
        projname,
        "Plugins",
        "_dev",
        lib_dst_name,
    )

    cs_dst_path = script_dir.parent.parent.joinpath(
        "Assets", projname, "Scripts", f"Native.cs"
    )

    # remove all current plugins and write dev plugin
    if lib_dst_path.parent.parent.exists():
        shutil.rmtree(lib_dst_path.parent.parent)
    lib_dst_path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(lib_src_path, lib_dst_path)

    # replace plugin name with new one and write cs binding file
    with open(cs_src_path, "r") as f:
        binding = f.read()
        binding = binding.replace(
            f'__DllName = "{libname}"', f'__DllName = "{libname}-{random_id}"'
        )
    with open(cs_dst_path, "w") as f:
        f.write(binding)


if __name__ == "__main__":
    main()
