import json
import subprocess
from pathlib import Path
from multipledispatch import dispatch
import tempfile
import os
from shutil import copy, rmtree, copytree
from argparse import ArgumentParser, Namespace
from typing import List, Dict, Tuple

from colorama import Fore, Back, Style, init

init(autoreset=True)


class AliasHandler:
    def __init__(self, base_path: Path = Path.cwd(), res_f_paths: bool = False):
        ## DEFINE ##
        self.base_path: Path
        self.tmp_path: Path
        self.root_dir: Path
        self.out_dir: Path
        self.aliases: Dict[any, any]
        self.res_f_paths: bool
        self.tsconfig = Dict[any, any]
        self.comp_options = Dict[any, any]
        self.options = Dict[any, any]
        self.res_f_paths_l: List[str]

        ## ASSIGN ##
        self.base_path = base_path.absolute()
        self.res_f_paths = res_f_paths
        self.tsconfig = self._get_ts_config()
        self.comp_options = self._get_comp_options(self.tsconfig)
        self.root_dir: Path = self.base_path.joinpath(self.comp_options.get("rootDir"))
        self.out_dir: Path = self.base_path.joinpath(self.comp_options.get("outDir"))
        self.aliases = self.tsconfig.get("aliases", {})
        self.res_f_paths_l = ["index.js", "main.js", "index.mjs", "main.mjs"]
        self.options = self._get_options()

    def _get_ts_config(self):
        with open(self.base_path.joinpath("tsconfig.json"), 'r') as f:
            return json.load(f)

    @dispatch(dict)
    def _get_comp_options(self, tsconfig):
        return tsconfig.get("compilerOptions", {})

    @dispatch(Path)
    def _get_comp_options(self):
        return self._get_comp_options(self._get_ts_config())

    @staticmethod
    def _get_matched_count(a: str, line: str):
        la: str = line.replace(a, "")
        return len(line) - len(la)

    def _get_largest_match_list(self, line: str):
        l: List[Tuple[int, Tuple[any, any]]] = []
        for a, p in self.aliases.items():
            mc = self._get_matched_count(a, line)
            if mc > 0:
                l.append((mc, (a, p)))
        return sorted(l)

    def _get_largest_match(self, line: str):
        lml = self._get_largest_match_list(line)
        if lml:
            return lml.pop()[1]

    def _cpy(self, f: Path, t: Path):
        if f.is_file():
            copy(f, t)
        elif f.is_dir():
            copytree(f, t)

    def _copy_asset_to_tmp(self, ts: Path):
        self._cpy(self.root_dir.joinpath(ts), self.tmp_path.joinpath(ts))

    def _copy_asset_to_dest(self, ts: Path):
        dest: Path = self.out_dir.joinpath(ts)
        rmtree(dest, ignore_errors=True)
        self._cpy(self.tmp_path.joinpath(ts), dest)

    def _copy_asset(self, ts: Path):
        self._copy_asset_to_tmp(ts)
        self._copy_asset_to_dest(ts)

    def _to_tmp_ts(self):
        for alias, paths in self.aliases.items():

            if "asset" in paths and paths["asset"]:
                self._copy_asset(paths["ts"])
                continue

            ts_path: Path = self.root_dir.joinpath(paths["ts"])
            paths["fd"] = Path()
            paths["rp"] = []
            ts_dir_path = ts_path
            if str(ts_path.name).endswith(".ts"):
                ts_dir_path = ts_path.parent
                self._write_ts_file(ts_path, ts_dir_path, paths)

            for file in ts_dir_path.rglob("*.ts"):
                if not str(file.name).endswith(".alias.ts"):
                    rel_to_root = file.relative_to(self.root_dir)
                    dest = self.tmp_path.joinpath(rel_to_root)
                    copy(file, dest)

            for file_path in ts_dir_path.glob("*.alias.ts"):
                self._write_ts_file(file_path, ts_dir_path, paths)

    def _write_ts_file(self, file_path, ts_dir_path, paths):
        # Generate *.ts filename
        ts_filename = file_path.name.replace(".alias.ts", ".ts")
        tmp_ts_path = ts_dir_path.joinpath(ts_filename).relative_to(self.root_dir)
        ts_output_path = self.tmp_path.joinpath(tmp_ts_path)
        files_dir = ts_output_path.parent
        paths["fd"] = files_dir
        files_dir.mkdir(parents=True, exist_ok=True)
        # Replace alias paths in *.alias.ts and output *.ts file
        with file_path.open('r') as f_in, ts_output_path.open('w') as f_out:
            for line in f_in:
                lm = self._get_largest_match(line)
                if lm:
                    a, p = lm
                    rp = str(
                        os.path.relpath(
                            p["ts"],
                            file_path.parent.relative_to(self.root_dir)
                        )
                    ).replace(".alias.ts", "")
                    if not rp.startswith("../"):
                        rp = "./" + rp
                    paths["rp"].append((rp, p["js"]))
                    line = line.replace(a, rp)
                    line = line.rstrip("\n").rstrip() + "//!tsc_replaced" + "\n"
                f_out.write(line)

    def _compile_tmp_ts(self):

        print(Fore.YELLOW + "Compiling...")

        command = ["pnpm", "tsc", *self.tmp_path.rglob("*.ts"), *self.options]

        # Start the subprocess
        process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

        # Read the output line by line
        while True:
            output = process.stdout.readline()
            if output == "" and process.poll() is not None:
                break
            if output:
                print(output.strip())

        # Check for any remaining errors after the process finishes
        stderr = process.communicate()[1]
        if stderr:
            print(Fore.RED + "Error:", stderr.strip())

        # Check the return code
        if process.returncode == 0:
            print(Fore.GREEN + "Command executed successfully!")
        else:
            print(Fore.RED + f"Command failed with return code {process.returncode}")

        self._alias_process(self._create_dirs)
        self._alias_process(self._replace_imports)

    @staticmethod
    def _create_dirs(ts_comp_path, real_output_path: Path, paths):
        real_output_path.parent.mkdir(parents=True, exist_ok=True)
        real_output_path.touch(exist_ok=True)

    def _replace_line(self, line, pat, rep) -> str:
        return line.replace(pat, rep).replace("//!tsc_replaced", "")

    def _replace_imports(self, ts_comp_path: Path, real_output_path: Path, paths):
        # Replace import paths in *.js file with js alias
        with ts_comp_path.open('r') as f_in, real_output_path.open('w') as f_out:
            for line in f_in:
                for pat, rep in paths["rp"]:
                    if line.find("//!tsc_replaced") != -1:
                        if self.res_f_paths:
                            rep_p: Path = Path(rep)
                            rep_rlp: Path = self._get_rlp(rep_p, self.out_dir, rep_p)
                            if rep_rlp.is_dir():
                                self.res_f_paths_l.append(rep_p.name + ".js")
                                for e in self.res_f_paths_l:
                                    jn = rep_rlp.joinpath(e)
                                    if jn.exists():
                                        rep = str(rep_p.joinpath(e))
                        # if line.find(self._with_dq(pat)) != -1:
                        #     line = self._replace_line(line, self._with_dq(pat), self._with_dq(rep))
                        # elif line.find(self._with_sq(pat)) != -1:
                        #     line = self._replace_line(line, self._with_sq(pat), self._with_sq(rep))
                        line = self._replace_line(line, pat, rep)
                        paths["rp"].pop(0)
                f_out.write(line)

    def _alias_process(self, func):
        for alias, paths in self.aliases.items():
            if "asset" in paths:
                continue

            js_path: Path = self.out_dir.joinpath(paths["js"])

            files_dir: Path = paths["fd"]
            for file_path in files_dir.glob("*.ts"):
                # Generate *.js filename
                js_dir_path = js_path
                if str(js_path.name).endswith(".js"):
                    js_dir_path = js_path.parent
                js_filename = file_path.name.replace(".ts", ".js")
                ts_comp_path = file_path.parent.joinpath(js_filename)
                js_output_path = js_dir_path.joinpath(js_filename)
                real_output_path = self._get_rlp(js_output_path, self.out_dir, js_output_path)
                func(ts_comp_path, real_output_path, paths)

    @staticmethod
    def _get_rlp(ip: Path, od: Path, alt: Path = None) -> Path:
        if str(ip).startswith("/"):
            return od.joinpath(str(ip)[1:])
        return alt

    @staticmethod
    def _with_sq(string: str) -> str:
        return "'" + string + "'"

    @staticmethod
    def _with_dq(string: str) -> str:
        return '"' + string + '"'

    def _get_options(self, without: List[str] = None):
        if without is None:
            without = ["rootDir", "outDir"]
        l: List[str] = []
        for k, v in self.comp_options.items():
            if without.count(k) > 0:
                continue
            l.append("--" + str(k))
            l.append(str(v).replace("True", "true").replace("False", "false"))
        return l

    def handle_aliases(self):
        with tempfile.TemporaryDirectory() as tmp_dir:
            self.tmp_path = Path(tmp_dir)
            node_modules: Path = self.base_path.joinpath("node_modules")
            node_modules_dest: Path = self.tmp_path.joinpath("node_modules")
            node_modules_dest.symlink_to(node_modules, target_is_directory=True)
            self._to_tmp_ts()
            self._compile_tmp_ts()


def main():
    parser: ArgumentParser = ArgumentParser()
    parser.add_argument("-b, --basePath", dest="b", type=Path, default=Path.cwd())
    parser.add_argument("-r, --resolveFullPaths", dest="r", default=False, action="store_true")
    result: Namespace = parser.parse_args()
    alias_handler: AliasHandler = AliasHandler(result.b, result.r)
    alias_handler.handle_aliases()


if __name__ == '__main__':
    main()
