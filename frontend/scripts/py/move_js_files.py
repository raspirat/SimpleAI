import os
import shutil
import sys

def move_files(src_dir, dest_dir):
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            src_file_path = os.path.join(root, file)
            relative_path = os.path.relpath(root, src_dir)
            folders = relative_path.split("/")
            dest_folder = dest_dir
            for folder in folders:
                if folder == "ts":
                    folder = "js"
                dest_folder = os.path.join(dest_folder, folder)
            dest_file_path = os.path.join(dest_folder, file)
            os.makedirs(dest_folder, exist_ok=True)
            shutil.move(src_file_path, dest_file_path)

def main():
    if len(sys.argv) != 3:
        print("Usage: python move_files.py <src_dir> <dest_dir>")
        sys.exit(1)

    src_dir = sys.argv[1]
    dest_dir = sys.argv[2]

    move_files(src_dir, dest_dir)

    # Optionally remove the source directory after moving files
    shutil.rmtree(src_dir)

if __name__ == "__main__":
    main()
