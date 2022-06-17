# Fedora Dev Setup

# Install Commands

Required:
```bash
sudo dnf update 
sudo dnf install cmake gcc g++ gdb python3-pip python3-venv python3-dev 
pip install --user conan
```
The path to the conan script will need to be added to PATH

Optional: nicer terminal
```bash
sudo dnf install guake gnome-tweaks
sudo dnf install fish util-linux-user powerline-fonts
```