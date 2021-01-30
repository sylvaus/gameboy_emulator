# Script to generate conan files in the build folders of CLion
for folder in "$PWD"/../+(cmake-build-*)/ ; do
    echo "Running conan install in $folder"
    conan install .. -if  "$folder"
done 
