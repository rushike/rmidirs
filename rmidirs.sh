#! /bin/bash

if [ $1 == "doc" ];
then
  cargo doc --open
else
  echo "Pass Argument"
fi