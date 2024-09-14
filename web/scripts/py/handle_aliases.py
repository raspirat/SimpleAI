import json
import subprocess
from pathlib import Path
from multipledispatch import dispatch
import tempfile
import os
from shutil import copy, rmtree, copytree
from argparse import ArgumentParser, Namespace
from typing import List, Dict, Tuple, Set

from colorama import Fore, Back, Style, init

init(autoreset=True)


class AliasTsFilePaths:
    def __init__(self, path, js_path, root_dir, tmp_dir, out_dir):
        self.path = Path(path).absolute()
        self.js_path = Path(js_path)
        self.rel_dir = self.path.relative_to(root_dir).parent
        self.to_root = Path(str(os.path.relpath(root_dir, path.parent)))
        self.ts_file_name = self.path.name.replace(".alias.ts", ".ts")
        self.js_file_name = self.ts_file_name.replace(".ts", ".js")
        self.no_ext_file_name = self.ts_file_name.replace(".ts", "")
        self.rel_no_ext = self.rel_dir.joinpath(self.no_ext_file_name)
        self.rel_ts_path = self.rel_dir.joinpath(self.ts_file_name)
        self.rel_js_dir = self.js_path.relative_to("/")
        self.rel_js_path = self.rel_js_dir.joinpath(self.js_file_name)
        self.repl_js_path = self.js_path.joinpath(self.js_file_name)
        self.tmp_ts_path = tmp_dir.joinpath(self.rel_ts_path)
        self.tmp_js_path = tmp_dir.joinpath(self.rel_dir.joinpath(self.js_file_name))
        self.dest_path = out_dir.joinpath(self.rel_js_path)
        self._make_dirs()

    def _make_dirs(self):
        self.tmp_ts_path.parent.mkdir(exist_ok=True, parents=True)
        self.dest_path.parent.mkdir(exist_ok=True, parents=True)


class Alias:
    def __init__(self, name, symbol):
        self.name = name
        if Path(self.name).name == "index":
            self.name = str(Path(self.name).parent)
        self.symbol = symbol

        self.alias = self.symbol + self.name


class AliasTsFile:
    def __init__(self, alias: Alias, paths: AliasTsFilePaths):
        self.alias = alias
        self.paths = paths
        self.replaces: Dict[int, (int, int, str)] = {}

    def replace_ts(self, line: str, number: int, alias) -> str:
        a = alias.alias.alias
        r = str(os.path.join(self.paths.to_root, alias.paths.rel_no_ext))
        s = line.find(a)
        self.replaces[number] = (s, s + len(r), alias.paths.repl_js_path)
        return line.replace(a, r)

    def replace_js(self):
        with (self.paths.tmp_js_path.open('r') as f_in, self.paths.dest_path.open('w') as f_out):
            lines = f_in.readlines()
            for line_num, rpl in self.replaces.items():
                s, e, r = rpl
                l: str = lines[line_num]
                st: str = l[:s]
                en: str = l[e:]
                lines[line_num] = st + str(r) + en
            f_out.writelines(lines)


class AliasHandler:
    def __init__(self, base_path: Path = Path.cwd(), resolve_file_paths: bool = False):
        # const vars
        self.base_dir: Path = base_path.absolute()
        self.resolve_file_paths: bool = resolve_file_paths
        self.resolve_file_paths_list: List[str] = ["index.js", "main.js", "index.mjs", "main.mjs"]
        # vars
        self.tmp_dir: Path = Path()
        self.ts_config: Dict[str, any] = {}
        self.options: List[str] = []
        self.root_dir: Path = Path()
        self.out_dir: Path = Path()
        # aliases
        self.symbol: str = "@"
        self.aliases: Set[AliasTsFile] = set()

    def _assign(self) -> None:
        self.ts_config = self._get_ts_config(self.base_dir)
        self.tmp_dir = self._get_tmp_path(self.base_dir)
        # comp options
        comp_options = self.ts_config.get("compilerOptions")
        self.options = self._get_options(comp_options)
        self.root_dir: Path = self.base_dir.joinpath(comp_options.get("rootDir"))
        self.out_dir: Path = self.base_dir.joinpath(comp_options.get("outDir"))
        # aliases
        self.symbol = self.ts_config.get("alias_symbol") or self.symbol
        self.aliases = self._get_aliases(self.ts_config.get("aliases"))

    def _cleanup(self):
        self.tmp_dir.rmdir()

    @staticmethod
    def _get_ts_config(base_path: Path) -> Dict[str, any]:
        with open(base_path.joinpath("tsconfig.json"), 'r') as f:
            return json.load(f)

    @staticmethod
    def _get_tmp_path(base_path) -> Path:
        tmp_path = Path(tempfile.mkdtemp())
        node_modules: Path = base_path.joinpath("node_modules")
        node_modules_dest: Path = tmp_path.joinpath("node_modules")
        node_modules_dest.symlink_to(node_modules, target_is_directory=True)
        return tmp_path

    @staticmethod
    def _get_options(comp_options, without: List[str] = None) -> List[str]:
        if without is None:  # default param
            without = ["rootDir", "outDir"]

        l: List[str] = []
        for k, v in comp_options.items():
            if without.count(k) > 0:
                continue
            l.append("--" + str(k))
            l.append(str(v).replace("True", "true").replace("False", "false"))
        return l

    def _get_alias_files(self, aliases: Dict[str, str]):
        for a, p in aliases.items():
            po: Path = self.root_dir.joinpath(Path(p["ts"]))
            js: Path = Path(p["js"])
            if po.is_dir():
                for f in po.rglob("*.ts"):
                    add = f.parent.relative_to(po)
                    ar = str(Path(a).joinpath(add).joinpath(f.stem)).replace(".alias", "")
                    yield f, ar, js.joinpath(add)
            else:
                yield po, a, js.parent

    def _get_alias_from(self, f, a, js):
        return AliasTsFile(
            Alias(a, self.symbol),
            AliasTsFilePaths(f, js, self.root_dir, self.tmp_dir, self.out_dir)
        )

    def _get_aliases(self, aliases: Dict[str, any]) -> Set[AliasTsFile]:
        s: Set[AliasTsFile] = set()
        for f, a, js in self._get_alias_files(aliases):
            s.add(self._get_alias_from(f, a, js))
        return s

    @staticmethod
    def _get_matched_count(a: AliasTsFile, line: str):
        la: str = line.replace(a.alias.alias, "")
        return len(line) - len(la)

    def _get_largest_match_list(self, line: str):
        l: List[(int, AliasTsFile)] = []
        for a in self.aliases:
            mc = self._get_matched_count(a, line)
            if mc > 0:
                l.append((mc, a))
        return sorted(l, key=lambda x: x[0])

    def _get_largest_match(self, line: str) -> AliasTsFile:
        lml = self._get_largest_match_list(line)
        if lml:
            return lml.pop()[1]

    def _to_tmp(self, a):
        with (a.paths.path.open('r') as f_in, a.paths.tmp_ts_path.open('w') as f_out):
            for ln, l in enumerate(f_in):
                lm = self._get_largest_match(l)
                if lm:
                    l = a.replace_ts(l, ln, lm)
                f_out.write(l)

    def _compile(self):
        print(Fore.YELLOW + "Compiling...")
        command = ["pnpm", "tsc", *self.tmp_dir.rglob("*.ts"), *self.options]
        process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        while True:
            output = process.stdout.readline()
            if output == "" and process.poll() is not None:
                break
            if output:
                print(output.strip())
        stderr = process.communicate()[1]

        if stderr:
            print(Fore.RED + "Error:", stderr.strip())

        if process.returncode == 0:
            print(Fore.GREEN + "Command executed successfully!")
        else:
            print(Fore.RED + f"Command failed with return code {process.returncode}")

    def handle_aliases(self):
        self._assign()
        for a in self.aliases:
            self._to_tmp(a)
        self._compile()
        for a in self.aliases:
            a.replace_js()
        # self._cleanup()


# class AliasHandler:
#     def __init__(self, base_path: Path = Path.cwd(), res_f_paths: bool = False):
#         self.base_path = base_path.absolute()
#         self.res_f_paths = res_f_paths
#         self.tsconfig = self._get_ts_config()
#
#     def _get_ts_config(self):
#         with open(self.base_path.joinpath("tsconfig.json"), 'r') as f:
#             return json.load(f)
#
#     @dispatch(dict)
#     def _get_comp_options(self, tsconfig):
#         return tsconfig.get("compilerOptions", {})
#
#     @dispatch(Path)
#     def _get_comp_options(self):
#         return self._get_comp_options(self._get_ts_config())
#
#     @staticmethod
#     def _get_matched_count(a: str, line: str):
#         la: str = line.replace(a, "")
#         return len(line) - len(la)
#
#     def _get_largest_match_list(self, line: str):
#         l: List[Tuple[int, Tuple[any, any]]] = []
#         for a, p in self.aliases.items():
#             mc = self._get_matched_count(a, line)
#             if mc > 0:
#                 l.append((mc, (a, p)))
#         return sorted(l)
#
#     def _get_largest_match(self, line: str):
#         lml = self._get_largest_match_list(line)
#         if lml:
#             return lml.pop()[1]
#
#     def _cpy(self, f: Path, t: Path):
#         if f.is_file():
#             copy(f, t)
#         elif f.is_dir():
#             copytree(f, t)
#
#     def _copy_asset_to_tmp(self, ts: Path):
#         self._cpy(self.root_dir.joinpath(ts), self.tmp_path.joinpath(ts))
#
#     def _copy_asset_to_dest(self, ts: Path):
#         dest: Path = self.out_dir.joinpath(ts)
#         rmtree(dest, ignore_errors=True)
#         self._cpy(self.tmp_path.joinpath(ts), dest)
#
#     def _copy_asset(self, ts: Path):
#         self._copy_asset_to_tmp(ts)
#         self._copy_asset_to_dest(ts)
#
#     def _handle_non_alias_ts_file(self, file):
#         rel_to_root = file.relative_to(self.root_dir)
#         dest = self.tmp_path.joinpath(rel_to_root)
#         copy(file, dest)
#
#     def _get_ts_files(self):
#         for alias, paths in self.aliases.items():
#             if "asset" in paths and paths["asset"]:
#                 self._copy_asset(paths["ts"])
#                 continue
#
#             ts_path: Path = self.root_dir.joinpath(paths["ts"])
#
#             for file in ts_path.rglob("*.ts"):
#                 if str(file.name).endswith(".alias.ts"):
#                     yield
#                 else:
#                     self._handle_non_alias_ts_file(file)
#
#     def _to_tmp_ts(self):
#         for alias, paths in self.aliases.items():
#
#             ts_path: Path = self.root_dir.joinpath(paths["ts"])
#             paths["fd"] = Path()
#             paths["rp"] = []
#             ts_dir_path = ts_path
#
#             if str(ts_path.name).endswith(".alias.ts"):
#                 ts_dir_path = ts_path.parent
#                 self._write_ts_file(ts_path, ts_dir_path, paths)
#                 continue
#
#             for file in ts_path.rglob("*.ts"):
#                 if str(file.name).endswith(".alias.ts"):
#                     subdirs = file.parent.relative_to(ts_path)
#                     if not str(subdirs) == ".":
#                         ts_dir_path = file.parent
#                     self._write_ts_file(file, ts_dir_path, paths)
#                 else:
#                     rel_to_root = file.relative_to(self.root_dir)
#                     dest = self.tmp_path.joinpath(rel_to_root)
#                     copy(file, dest)
#                 # if not str(file.name).endswith(".alias.ts"):
#                 #     rel_to_root = file.relative_to(self.root_dir)
#                 #     dest = self.tmp_path.joinpath(rel_to_root)
#                 #     copy(file, dest)
#
#             # for file_path in ts_dir_path.glob("*.alias.ts"):
#             #     self._write_ts_file(file_path, ts_dir_path, paths)
#
#     def _write_ts_file(self, file_path, ts_dir_path, paths):
#         # Generate *.ts filename
#         ts_filename = file_path.name.replace(".alias.ts", ".ts")
#         tmp_ts_path = ts_dir_path.joinpath(ts_filename).relative_to(self.root_dir)
#         ts_output_path = self.tmp_path.joinpath(tmp_ts_path)
#         files_dir = ts_output_path.parent
#         paths["fd"] = files_dir
#         files_dir.mkdir(parents=True, exist_ok=True)
#         # Replace alias paths in *.alias.ts and output *.ts file
#         with file_path.open('r') as f_in, ts_output_path.open('w') as f_out:
#             for line in f_in:
#                 lm = self._get_largest_match(line)
#                 if lm:
#                     a, p = lm
#                     rp = str(
#                         os.path.relpath(
#                             p["ts"],
#                             file_path.parent.relative_to(self.root_dir)
#                         )
#                     ).replace(".alias.ts", "")
#                     if not rp.startswith("../"):
#                         rp = "./" + rp
#                     paths["rp"].append((rp, p["js"]))
#                     line = line.replace(a, rp)
#                     line = line.rstrip("\n").rstrip() + "//!tsc_replaced" + "\n"
#                 f_out.write(line)
#
#     def _compile_tmp_ts(self):
#
#         print(Fore.YELLOW + "Compiling...")
#
#         command = ["pnpm", "tsc", *self.tmp_path.rglob("*.ts"), *self.options]
#
#         # Start the subprocess
#         process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
#
#         # Read the output line by line
#         while True:
#             output = process.stdout.readline()
#             if output == "" and process.poll() is not None:
#                 break
#             if output:
#                 print(output.strip())
#
#         # Check for any remaining errors after the process finishes
#         stderr = process.communicate()[1]
#         if stderr:
#             print(Fore.RED + "Error:", stderr.strip())
#
#         # Check the return code
#         if process.returncode == 0:
#             print(Fore.GREEN + "Command executed successfully!")
#         else:
#             print(Fore.RED + f"Command failed with return code {process.returncode}")
#
#         self._alias_process(self._create_dirs)
#         self._alias_process(self._replace_imports)
#
#     @staticmethod
#     def _create_dirs(ts_comp_path, real_output_path: Path, paths):
#         real_output_path.parent.mkdir(parents=True, exist_ok=True)
#         real_output_path.touch(exist_ok=True)
#
#     def _replace_line(self, line, pat, rep) -> str:
#         return line.replace(pat, rep).replace("//!tsc_replaced", "")
#
#     def _replace_imports(self, ts_comp_path: Path, real_output_path: Path, paths):
#         # Replace import paths in *.js file with js alias
#         with ts_comp_path.open('r') as f_in, real_output_path.open('w') as f_out:
#             for line in f_in:
#                 for pat, rep in paths["rp"]:
#                     if line.find("//!tsc_replaced") != -1:
#                         if self.res_f_paths:
#                             rep_p: Path = Path(rep)
#                             rep_rlp: Path = self._get_rlp(rep_p, self.out_dir, rep_p)
#                             if rep_rlp.is_dir():
#                                 self.res_f_paths_l.append(rep_p.name + ".js")
#                                 for e in self.res_f_paths_l:
#                                     jn = rep_rlp.joinpath(e)
#                                     if jn.exists():
#                                         rep = str(rep_p.joinpath(e))
#                         # if line.find(self._with_dq(pat)) != -1:
#                         #     line = self._replace_line(line, self._with_dq(pat), self._with_dq(rep))
#                         # elif line.find(self._with_sq(pat)) != -1:
#                         #     line = self._replace_line(line, self._with_sq(pat), self._with_sq(rep))
#                         line = self._replace_line(line, pat, rep)
#                         paths["rp"].pop(0)
#                 f_out.write(line)
#
#     def _alias_process(self, func):
#         for alias, paths in self.aliases.items():
#             if "asset" in paths:
#                 continue
#
#             js_path: Path = self.out_dir.joinpath(paths["js"])
#
#             files_dir: Path = paths["fd"]
#             for file_path in files_dir.glob("*.ts"):
#                 # Generate *.js filename
#                 js_dir_path = js_path
#                 if str(js_path.name).endswith(".js"):
#                     js_dir_path = js_path.parent
#                 js_filename = file_path.name.replace(".ts", ".js")
#                 ts_comp_path = file_path.parent.joinpath(js_filename)
#                 js_output_path = js_dir_path.joinpath(js_filename)
#                 real_output_path = self._get_rlp(js_output_path, self.out_dir, js_output_path)
#                 func(ts_comp_path, real_output_path, paths)
#
#     @staticmethod
#     def _get_rlp(ip: Path, od: Path, alt: Path = None) -> Path:
#         if str(ip).startswith("/"):
#             return od.joinpath(str(ip)[1:])
#         return alt
#
#     @staticmethod
#     def _with_sq(string: str) -> str:
#         return "'" + string + "'"
#
#     @staticmethod
#     def _with_dq(string: str) -> str:
#         return '"' + string + '"'
#
#     def _get_options(self, without: List[str] = None):
#         if without is None:
#             without = ["rootDir", "outDir"]
#         l: List[str] = []
#         for k, v in self.comp_options.items():
#             if without.count(k) > 0:
#                 continue
#             l.append("--" + str(k))
#             l.append(str(v).replace("True", "true").replace("False", "false"))
#         return l
#
#     def handle_aliases(self):
#         with tempfile.TemporaryDirectory() as tmp_dir:
#             self.tmp_path = Path(tmp_dir)
#             node_modules: Path = self.base_path.joinpath("node_modules")
#             node_modules_dest: Path = self.tmp_path.joinpath("node_modules")
#             node_modules_dest.symlink_to(node_modules, target_is_directory=True)
#             self._to_tmp_ts()
#             self._compile_tmp_ts()


def main():
    parser: ArgumentParser = ArgumentParser()
    parser.add_argument("-b, --basePath", dest="b", type=Path, default=Path.cwd())
    parser.add_argument("-r, --resolveFullPaths", dest="r", default=False, action="store_true")
    result: Namespace = parser.parse_args()
    alias_handler: AliasHandler = AliasHandler(result.b, result.r)
    alias_handler.handle_aliases()


if __name__ == '__main__':
    main()
