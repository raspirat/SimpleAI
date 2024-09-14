from collections.abc import Generator
from pathlib import Path
from shutil import copy, move, copytree, rmtree
from argparse import ArgumentParser, Namespace
from sys import argv
from typing import Tuple, List
from itertools import product


# def _rmtree(path: Path):
#     for p in path.iterdir():
#         if p.is_dir():
#             _rmtree(p)
#         else:
#             p.unlink()
#     path.rmdir()


def _copy(in_dir: Path,
          out_dir: Path,
          sub_dir: Path,
          except_patterns=None,
          include_patterns=None,
          files_only=False):
    if include_patterns is None:
        include_patterns = ["*"]
    if except_patterns is None:
        except_patterns = [" "]
    for content in sub_dir.iterdir():
        for ex_pattern, in_pattern in product(except_patterns, include_patterns):
            if not content.match(ex_pattern) and content.match(in_pattern):
                rel_path: Path = content.relative_to(in_dir)
                dest_path: Path = out_dir.joinpath(rel_path)
                if content.is_file():
                    dest_path.parent.mkdir(exist_ok=True, parents=True)
                    copy(
                        content,
                        dest_path
                    )
                    continue
                elif content.is_dir() and not files_only:
                    copytree(
                        content,
                        dest_path,
                        dirs_exist_ok=True
                    )
                    continue
            if content.is_dir():
                _copy(in_dir, out_dir, content, except_patterns, include_patterns, files_only)


def _rmove(in_dir: Path, out_dir: Path, rename_patterns=None):
    if rename_patterns is None:
        rename_patterns = [(" ", " ")]
    out_dir.mkdir(exist_ok=True)
    for content in in_dir.iterdir():
        rel_path: Path = content.relative_to(in_dir)
        for pattern in rename_patterns:
            if content.match(pattern[0]):
                rel_path = pattern[1]
        dest_path: Path = out_dir.joinpath(rel_path)
        if content.is_file():
            move(content, dest_path)
        else:
            _rmove(content, dest_path, rename_patterns)


def _rremove_empty_folders(dir: Path):
    for content in dir.iterdir():
        if content.is_dir():
            if not any(content.iterdir()):
                content.rmdir()
            else:
                _rremove_empty_folders(content)


def copy_files(in_dir: Path,
               out_dir: Path,
               except_patterns: List[str],
               include_patterns: List[str],
               rename_patterns: List[str],
               rm_out_dir: bool = False,
               files_only: bool = False):
    if not in_dir.is_dir():
        raise Exception("[Error]: Your specified in-directory is not a valid directory.")
    if not out_dir.exists():
        out_dir.mkdir(parents=True)
    if not out_dir.is_dir():
        raise Exception("[Error]: Your specified out-directory is not a valid directory.")
    if rm_out_dir:
        rmtree(out_dir)

    in_dir = in_dir.absolute()
    out_dir = out_dir.absolute()
    _copy(in_dir, out_dir, in_dir, except_patterns, include_patterns, files_only)

    if rename_patterns:
        rename_patterns_split: List[Tuple[str, Path]] = [(" ", " ")]
        for pattern in rename_patterns:
            k, v = pattern.split(":", 1)
            pat: str = k
            pth: Path = v
            rename_patterns_split.append((pat, pth))
        _rmove(out_dir, out_dir, rename_patterns_split)
        _rremove_empty_folders(out_dir)


def main():
    parser: ArgumentParser = ArgumentParser()
    parser.add_argument("-i, --inDir", dest="i", type=Path, required=True)
    parser.add_argument("-o, --outDir", dest="o", type=Path, required=True)
    parser.add_argument("-e, --exceptPatterns", dest="e", nargs="+", type=str, default=" ")
    parser.add_argument("-c, --includePatterns", dest="c", nargs="+", type=str, default="*")
    parser.add_argument("-p, --renameParents", dest="p", nargs="+", type=str)
    parser.add_argument("-r, --rmOutDir", dest="r", action="store_true")
    parser.add_argument("-f, --filesOnly", dest="f", action="store_true")
    result: Namespace = parser.parse_args(argv[1:])
    copy_files(result.i, result.o, result.e, result.c, result.p, result.r, result.f)


if __name__ == "__main__":
    main()
