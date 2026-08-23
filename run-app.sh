#!/bin/bash

export WEBKIT_DISABLE_DMABUF_RENDERER=1
export GDK_BACKEND=x11

./src-tauri/target/release/pungplay
