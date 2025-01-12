#! /bin/sh

set -eux

version="1.26.0"

if ! test -d ./janet/.git
then
  rm -rf ./janet
  git clone https://github.com/janet-lang/janet
fi
cd ./janet
git checkout master
git pull
git checkout "v${version}"
git clean -fxd
make
mkdir -p ../csrc
cp build/c/janet.c ../csrc/
cp build/janet.h ../csrc/
# cp src/conf/janetconf.h ../csrc/
