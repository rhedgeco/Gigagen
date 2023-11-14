import shutil
import pathlib
import sys

libname = "gigagen"
projname = "Gigagen"
script_dir = pathlib.Path(__file__).parent.resolve()


def main():
    # get linux library src
    linux_lib_src_path = script_dir.parent.joinpath(
        "target", "x86_64-unknown-linux-gnu", "release", f"lib{libname}.so"
    )
    if not linux_lib_src_path.exists():
        print(f"library '{linux_lib_src_path}' does not exist", file=sys.stderr)
        sys.exit(1)

    # get windows library src
    windows_lib_src_path = script_dir.parent.joinpath(
        "target", "x86_64-pc-windows-gnu", "release", f"{libname}.dll"
    )
    if not windows_lib_src_path.exists():
        print(f"library '{windows_lib_src_path}' does not exist", file=sys.stderr)
        sys.exit(1)

    # get binding file src
    cs_src_path = script_dir.parent.joinpath("bindings", f"Native.cs")
    if not cs_src_path.exists():
        print(f"binding '{cs_src_path}' does not exist", file=sys.stderr)
        sys.exit(1)

    # create dst paths
    project_dir = script_dir.parent.parent.joinpath("Assets", projname)
    plugins_dir = project_dir.joinpath("Plugins")
    linux_lib_dst_path = plugins_dir.joinpath("x86_64", f"lib{libname}.so")
    windows_lib_dst_path = plugins_dir.joinpath("x86_64", f"{libname}.dll")
    cs_dst_path = project_dir.joinpath("Scripts", f"Native.cs")

    # remove all current plugins if they exist
    if plugins_dir.exists():
        shutil.rmtree(plugins_dir)

    # create necessary directories
    linux_lib_dst_path.parent.mkdir(parents=True, exist_ok=True)
    windows_lib_dst_path.parent.mkdir(parents=True, exist_ok=True)
    cs_dst_path.parent.mkdir(parents=True, exist_ok=True)

    # copy all library and binding files
    shutil.copyfile(linux_lib_src_path, linux_lib_dst_path)
    shutil.copyfile(windows_lib_src_path, windows_lib_dst_path)
    shutil.copyfile(cs_src_path, cs_dst_path)


if __name__ == "__main__":
    main()
