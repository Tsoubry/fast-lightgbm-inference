#!/bin/bash

CORES=6

PID_WORKERS=( )
for ((i = 1; i <=$CORES; i++));do
  locust -f main.py --worker & PID_WORKERS+=( $! )
done