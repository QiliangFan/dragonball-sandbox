#!/bin/bash

docker run --rm -ti \
	--volume $(pwd):/dbs-uhttp \
	--workdir /dbs-uhttp \
	--privileged \
	--security-opt seccomp=unconfined \
	rustvmm/dev:v15 /bin/bash
