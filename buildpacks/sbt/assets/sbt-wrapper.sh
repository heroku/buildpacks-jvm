#!/usr/bin/env bash

case $(ulimit -u) in
16384) # PM Dyno
  maxSbtHeap="2000"
  ;;
32768) # PL Dyno
  maxSbtHeap="5220"
  ;;
*)
  maxSbtHeap="768"
  ;;
esac

sbt-extras \
  -J-Xmx${maxSbtHeap}M \
  -J-Xms${maxSbtHeap}M \
  -J-XX:+UseCompressedOops \
  "$@"
