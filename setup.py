import os
import platform
import subprocess

rust_nightly_version = "nightly-2018-11-09"
bonsai_runtime_jar = "target/runtime-1.0-SNAPSHOT.jar"
bonsai_runtime_src = "runtime/"
bonsai_libstd_src = "libstd/"
bonsai_libstd_jar = "target/libstd-1.0-SNAPSHOT.jar"

ending_message = "\nSuccesfully installed bonsai (and its standard library) and bonsai runtime.\n"

install_rust_cmd = ["rustup", "override", "set", rust_nightly_version]
list_target_rustup_cmd = ["rustup", "target", "list"]
clone_bonsai_cmd = ["git", "clone", "https://github.com/ptal/bonsai.git"]
install_bonsai_cmd = ["cargo", "install", "--path", ".", "--force"]
try_bonsai = ["bonsai", "-h"]

def mvn_install_cmd(groupId, artifactId, version, file):
  return ["mvn", "install:install-file",
    "-DgroupId={}".format(groupId),
    "-DartifactId={}".format(artifactId),
    "-Dversion={}".format(version),
    "-Dpackaging=jar",
    "-Dfile={}".format(file),
    "-quiet",
    "--fail-fast"]

mvn_package_cmd = ["mvn", "package", "-quiet", "--fail-fast"]

install_bonsai_runtime_cmd = mvn_install_cmd("bonsai", "runtime", "1.0", bonsai_runtime_jar)
install_bonsai_libstd_cmd = mvn_install_cmd("bonsai", "libstd", "1.0", bonsai_libstd_jar)

def install_rust():
  try:
    install_rust_nightly()
  except OSError as e:
    if e.errno == os.errno.ENOENT:
      install_rustup()
    else:
      print("`rustup` call failed.")
      raise

def install_rustup():
  print("Please install `rustup` with the following command and come back:\n")
  print("  curl https://sh.rustup.rs -sSf | sh\n")
  exit()

def install_rust_nightly():
  print("Installing rust compiler (nightly channel)...")
  subprocess.run(install_rust_cmd)
  print("rust compiler (nightly channel) has been installed.")

class cd:
  """Context manager for changing the current working directory"""
  def __init__(self, newPath):
    self.newPath = os.path.expanduser(newPath)

  def __enter__(self):
    self.savedPath = os.getcwd()
    os.chdir(self.newPath)

  def __exit__(self, etype, value, traceback):
    os.chdir(self.savedPath)

def rustup_target():
  targets = subprocess.run(list_target_rustup_cmd, stdout=subprocess.PIPE, universal_newlines=True).stdout
  for target in targets.splitlines():
    if target.endswith("(default)"):
      target = target.split(' ')[0]
      return rust_nightly_version + "-" + target
  input('Unknown target directory (enter the name of the target directory, it is in `~/.multirust/toolchains/`): ')

def install_bonsai():
  print("Installing bonsai compiler...")
  subprocess.run(install_bonsai_cmd, stdout=subprocess.DEVNULL).check_returncode()
  print("`bonsai` compiler has been installed.")

def bonsai_is_installed():
  try:
    subprocess.run(try_bonsai, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    True
  except OSError:
    False

def install_bonsai_libstd():
  print("Installing Bonsai standard libary...")
  with cd(bonsai_libstd_src):
    subprocess.run(mvn_package_cmd).check_returncode()
    subprocess.run(install_bonsai_libstd_cmd).check_returncode()
    print("`Bonsai libstd` has been installed.")

def install_bonsai_runtime():
  print("Installing Bonsai runtime Java library...")
  with cd(bonsai_runtime_src):
    subprocess.run(mvn_package_cmd).check_returncode()
    subprocess.run(install_bonsai_runtime_cmd).check_returncode()
    print("`Bonsai runtime` has been installed.")

def installing_chain():
  install_rust()
  install_bonsai()
  install_bonsai_runtime()
  install_bonsai_libstd()
  print(ending_message)

installing_chain()
