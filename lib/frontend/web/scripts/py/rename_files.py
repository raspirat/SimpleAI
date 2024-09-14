from pathlib import Path
from shutil import move

start_path = Path.cwd()

def rename_files(pdn: str, repl: str = '') -> None:
    for obj in start_path.rglob(pdn):
        if obj.is_dir():
            for child in obj.iterdir():
                new_child_name = child.parent / child.name.replace(repl, '');
                if child.is_file():
                    child.rename(new_child_name)
                move(new_child_name, obj.parent)
            obj.rmdir()

rename_files('ts', '.alias')
rename_files('html')
rename_files('css')