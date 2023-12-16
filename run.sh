#!/bin/bash
source example/env/bin/activate
maturin develop
( cd example ; python3 ./main.py )
