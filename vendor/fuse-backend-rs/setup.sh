#!/bin/bash

docker run --rm -ti \
	--volume $(pwd):/fuse-backend \
	--workdir /fuse-backend \
	--privileged \
	--security-opt seccomp=unconfined \
	rustvmm/dev:v15 /bin/bash
