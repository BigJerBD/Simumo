#!/bin/bash

# this script is used to start  the Simumo then the visualiser after
# this is copied in the build when the system is built
# note :: this might be replaced with a python script in the future


source venv/bin/activate
./simulator/simulator $*
deactivate