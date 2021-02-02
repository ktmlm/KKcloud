#!/usr/bin/env bash

#################################################
#### Ensure we are in the right path. ###########
#################################################
if [[ 0 -eq $(echo $0 | grep -c '^/') ]]; then
    # relative path
    EXEC_PATH=$(dirname "`pwd`/$0")
else
    # absolute path
    EXEC_PATH=$(dirname "$0")
fi

EXEC_PATH=$(echo ${EXEC_PATH} | sed 's@/\./@/@g' | sed 's@/\.*$@@')
cd $EXEC_PATH || exit 1
#################################################

OS=$(uname -s)
if [[ $OS == "Linux" ]]; then
    CMD="sed -ri"
elif [[ $OS == "Darwin" || $OS == "FreeBSD" ]]; then
    CMD="sed -rI"
else
    echo -e "\033[31;01mUnsupported platform!\[033[00m"
    exit 1
fi

for file in $(find .. -type f \
    -name "*.rs" \
    -o -name "*.c" \
    -o -name "*.h" \
    -o -name "*.sh" \
    -o -name "*.toml" \
    -o -name "*.json" \
    -o -name "*.md"\
    -o -name "rc.local"\
    | grep -v "$(basename $0)" \
    | grep -v 'target/' \
    | grep -v 'postgres'); do

    $CMD 's/　/ /g' $file
    $CMD 's/！/!/g' $file
    $CMD 's/（/(/g' $file
    $CMD 's/）/)/g' $file

    $CMD 's/：/: /g' $file
    $CMD 's/， */, /g' $file
    $CMD 's/。 */. /g' $file
    $CMD 's/、 +/、/g' $file

    $CMD 's/, +/, /g' $file
    $CMD 's/\. +/. /g' $file

    $CMD 's/\t/    /g' $file
    $CMD 's/ +$//g' $file
done

cargo fmt --all
