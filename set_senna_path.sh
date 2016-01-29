#!/usr/bin/sh

echo "/// Generated automatically at build time" > src/sennapath.rs
echo "pub const SENNA_PATH : &'static str = \"$1\";" >> src/sennapath.rs

