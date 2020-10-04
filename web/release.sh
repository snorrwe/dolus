#!/bin/bash

set -e

echo "Release command starting"

/dolus/diesel migration run

echo "Release command finished"
