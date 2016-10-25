import os
import platform
import subprocess

rust_nightly_version = "nightly-2016-10-21"
sugarcubes_jar = "/tmp/SugarCubesv4.0.0a5.jar"
chococubes_jar = "target/ChocoCubes-1.0-SNAPSHOT.jar"
chococubes_src = "ChocoCubes/"

startup_script = "unknown"
lib_path = "unknown"
ending_message = "\nSuccesfully installed bonsai, SugarCubes and ChocoCubes.\n"

if platform.system() == 'Darwin':
  startup_script = "~/.profile"
  lib_path = "DYLD_LIBRARY_PATH"
elif platform.system() == 'Linux':
  startup_script = "~/.bashrc"
  lib_path = "LD_LIBRARY_PATH"

install_rustup_cmd = ["curl", "https://sh.rustup.rs", "-sSf"]
install_rust_cmd = ["rustup", "override", "add", rust_nightly_version]
list_target_rustup_cmd = ["rustup", "target", "list"]
clone_bonsai_cmd = ["git", "clone", "https://github.com/ptal/bonsai.git"]
install_bonsai_cmd = ["cargo", "install"]
try_bonsai = ["bonsai", "-h"]

download_sugarcubes_cmd = ["curl",
  "http://jeanferdysusini.free.fr/v4.0/SugarCubesv4.0.0a5.jar",
  "-o", sugarcubes_jar]

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

install_sugarcubes_cmd = mvn_install_cmd("inria.meije.rc", "SugarCubes", "4.0.0a5", sugarcubes_jar)
install_chococubes_cmd = mvn_install_cmd("bonsai", "ChocoCubes", "1.0", chococubes_jar)

def install_rust():
  try:
    install_rust_nightly()
  except OSError as e:
    if e.errno == os.errno.ENOENT:
      install_rustup()
      install_rust_nightly()
    else:
      print("`rustup` call failed.")
      raise

def install_rustup():
  print("Installing rustup now...")
  curl_script = subprocess.Popen(install_rustup_cmd, stdout=subprocess.PIPE)
  subprocess.run(['sh'], stdin=curl_script.stdout, stdout=subprocess.DEVNULL).check_returncode()
  curl_script.wait()
  print("`rustup` has been installed.")

def install_rust_nightly():
  print("Installing rust compiler (nightly channel)...")
  subprocess.run(install_rust_cmd, stdout=subprocess.DEVNULL)
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
      return target.split(' ')[0]
  input('Unknown target directory (enter the name of the target directory, it is in `~/.multirust/toolchains/`): ')

def rustlib_export():
  target = rustup_target()
  return "export {0}=${0}:~/.multirust/toolchains/{1}/lib".format(lib_path, target)

def add_export_bug():
  global ending_message
  export_line = rustlib_export()
  print("\nDue to a bug, the following should be appended to `{}`:\n".format(startup_script))
  print("  ", export_line)
  answer = input("\nWould you like to proceed (you can set it up manually after the installation) [Y/n]? ").lower()
  if answer.startswith('y') or answer == "":
    with open(os.path.expanduser(startup_script), 'a') as file:
      file.write("\n" + export_line)
      ending_message += "Do not forget to reload your profile with `source {}`\n".format(startup_script)

def install_bonsai():
  try:
    subprocess.run(try_bonsai, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
  except OSError:
    print("Installing bonsai compiler...")
    subprocess.run(install_bonsai_cmd, stdout=subprocess.DEVNULL).check_returncode()
    add_export_bug()
    print("`bonsai` compiler has been installed.")

def install_sugarcubes():
  print("Installing SugarCubes Java libary...")
  subprocess.run(download_sugarcubes_cmd, stdout=subprocess.DEVNULL)
  try:
    subprocess.run(install_sugarcubes_cmd).check_returncode()
    print("`SugarCubes` has been installed.")
  except OSError as e:
    if e.errno == os.errno.ENOENT:
      print("Please install Maven (see maven.apache.org/)")
      exit()

def install_chococubes():
  print("Installing ChocoCubes Java library...")
  with cd(chococubes_src):
    subprocess.run(mvn_package_cmd).check_returncode()
    subprocess.run(install_chococubes_cmd).check_returncode()
    print("`ChocoCubes` has been installed.")

def installing_chain():
  install_rust()
  install_bonsai()
  install_sugarcubes()
  install_chococubes()
  print(ending_message)

installing_chain()
