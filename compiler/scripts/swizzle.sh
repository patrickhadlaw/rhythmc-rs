#!/bin/bash

N="2"
TESTS="false"
XYZW=(x y z w)
RGBA=(r g b a)
STPQ=(s t p q)
COUNTER=0

print_usage() {
  echo "$package - generate vector swizzle methods"
  echo " "
  echo "$package [options]"
  echo " "
  echo "options:"
  echo "-h    show help"
  echo "-n    the dimension of vector type to swizzle"
  echo "-t    enable to generate the tests"
}

while getopts 'n:th' flag; do
  case "${flag}" in
    n) N=$OPTARG ;;
    t) TESTS='true' ;;
    h) print_usage
      exit 0 ;;
    *) print_usage
      exit 1 ;;
  esac
done

function swizzle () {
  local basis=("$@")
  local vals=(0 0 0 0)
  for ((level=1;level<=N;level++)); do
    local test_assign=""
    for ((i=0;i<N**level;i++)); do
      local base_n="$(echo "ibase=10; obase=$N; $i" | bc)"
      local padded=`printf "%0${level}d\n" $base_n`
      local setter="true"
      if echo "$padded" | grep -q '\(.\).*\1'; then
        setter="false"
      fi
      local name=""
      local get_inner=""
      local set_inner=""
      local assign=""
      for ((j=0;j<level;j++)); do
        local idx=${padded:j:1}
        name="${name}${basis[$idx]}"
        if [ $setter == "true" ]; then
          ((COUNTER=COUNTER+1))
          vals[$idx]=$COUNTER
        fi
        if [ $j == "0" ]; then
          get_inner="self.$idx.clone()"
          set_inner="self.$idx = value.$j;"
          assign="${vals[$idx]}"
        else
          get_inner="${get_inner}, self.$idx.clone()"
          set_inner="${set_inner} self.$idx = value.$j;"
          assign="${assign}, ${vals[$idx]}"
        fi
      done
      if [ $TESTS == "true" ]; then
        if [ $level == "1" ]; then
          echo "v$N.set_$name($COUNTER);"
          echo "assert_eq!(v$N.$name(), $COUNTER);"
        else
          if [ $setter == "true" ]; then
            echo "v$N.set_$name(V$level($assign));"
          fi
          echo "assert_eq!(v$N.$name(), V$level($assign));"
        fi
      else
        if [ $level == "1" ]; then
          echo "pub fn $name(&self) -> T { $get_inner }"
          echo "pub fn set_$name(&mut self, value: T) { self.${padded:0:1} = value; }"
        else
          echo "pub fn $name(&self) -> V$level<T> { V$level($get_inner) }"
          if [ $setter == "true" ]; then
            echo "pub fn set_$name(&mut self, value: V$level<T>) { $set_inner }"
          fi
        fi
      fi
    done
  done
}

swizzle "${XYZW[@]}"
swizzle "${RGBA[@]}"
swizzle "${STPQ[@]}"
