#!/bin/sh
for i in `find src -type f -name '*.rs'`
	do rustfmt --write-mode=overwrite $i
done
