import io
import os
import shutil
from pathlib import Path
from urllib.request import urlopen
from tempfile import TemporaryDirectory
from zipfile import ZipFile
import hashlib


URL_PATH = "https://gekkio.fi/files/mooneye-test-suite/mts-20240926-1737-443f6e1/mts-20240926-1737-443f6e1.zip"
THIS_FILE_DIR = Path(__file__).parent
ROOT_TEST_FOLDER = THIS_FILE_DIR / ".." / ".."
MOONEYE_ROM_FOLDER = ROOT_TEST_FOLDER / "emulator" / "tests" / "mts"
EXPECTED_HASH = "5cf50314cd3d42ec8f423b0a71b9ab6dbfad2386d8d88f03687af1b5b5baba71"

def main():
    if os.path.exists(MOONEYE_ROM_FOLDER):
        print("Folder already exists, skipping download. Folder path: {}".format(MOONEYE_ROM_FOLDER))
        return

    print("Downloading ROM file: {}".format(URL_PATH))
    with urlopen(URL_PATH) as f,  TemporaryDirectory() as tmp_dir:
        data = f.read()
        digest = hashlib.sha256(data).hexdigest()
        if digest != EXPECTED_HASH:
            raise RuntimeError("Expected {} digest but got {}.".format(EXPECTED_HASH, digest))

        tmp_extract_path = Path(tmp_dir) / "extracted"
        with ZipFile(io.BytesIO(data)) as zf:
            zf.extractall(tmp_extract_path)

        folders = os.listdir(tmp_extract_path)
        if len(folders) != 1:
            raise RuntimeError("Expected 1 folder but got {}".format(folders))

        shutil.move(tmp_extract_path / folders[0], MOONEYE_ROM_FOLDER)


if __name__ == "__main__":
    main()
