import platform
import os
import pathlib
from shutil import copyfile

# get env

if platform.system() == 'Linux':
    expend_name = "so"
elif platform.system() == 'Darwin':
    expend_name = "dylib"
elif platform.system() == 'Windows':
    expend_name = "dll"
else:
    print('unsupported system type')
    exit()

target_dir = os.environ.get('CARGO_TARGET_DIR')

if target_dir is None:
    target_dir = './target/release/'

# -----

try:
    os.system('cargo build')
    src_path = pathlib.Path(target_dir).joinpath('debug').joinpath(f'rprlib.{expend_name}')
    import_path = pathlib.Path('./test/rprlib.pyd')
    copyfile(src_path, import_path)
except Exception as e:
    print(e)


from rprlib import get_nodes, get_blocks


with open('./test/test.asm') as f:
    test = f.read()

with open('./test/test1.asm') as f:
    test1 = f.read()

print('--------------------print nodes------------------')
print(get_nodes(test))
print('-------------------print blocks------------------')
print(get_blocks(test))
