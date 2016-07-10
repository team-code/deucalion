set -e
mkdir sfml_install
wget http://www.sfml-dev.org/files/SFML-2.3.2-sources.zip --no-check-certificate
unzip -q SFML-2.3.2-sources.zip
pushd SFML-2.3.2 && mkdir build && cd build && cmake .. && make
make DESTDIR=/home/travis/build/team-code/deucalion/sfml_install install
popd
wget http://www.sfml-dev.org/files/CSFML-2.3-sources.zip --no-check-certificate
unzip -q CSFML-2.3-sources.zip
pushd CSFML-2.3
mkdir cmake/Modules
cp /home/travis/build/team-code/deucalion/sfml_install/usr/local/share/SFML/cmake/Modules/FindSFML.cmake cmake/Modules
mkdir build && cd build && cmake -DCMAKE_MODULE_PATH=/home/travis/build/team-code/deucalion/CSFML-2.3/cmake/Modules -DSFML_ROOT=/home/travis/build/team-code/deucalion/sfml_install/usr/local .. && make
make DESTDIR=/home/travis/build/team-code/deucalion/sfml_install install
popd
