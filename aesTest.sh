#!/bin/bash

echo "TEST: AES-$4"
echo "$1 <=> $2"
retval=0
printf "Encryption: "
answer=$($(dirname "$0")/main -e $1 $3 $4)

if [ "$answer" == "$2" ]; then
	echo "Passed!"
else
	echo "Failed."
	retval=1
fi

printf "Decryption: "
answer=$($(dirname "$0")/main -d $2 $3 $4)

if [ "$answer" == "$1" ]; then
	echo "Passed!"
else
	echo "Failed."
	retval=1
fi
echo
exit $retval
