include .env
export

HOST_DIR ?= $(shell pwd)

test: test3 test4 test5 test6 test7 test8

lab1: test3

lab2: test4

lab3: test5

lab4: test6 test7

lab5: test8

setup: clean-os clean-user
	git submodule update --init

clean-os:
	rm -rf os

clean-user:
	git submodule deinit -f -- ci-user
	rm -rf ci-user

test1: setup
	cp -r os1 os
	cd ci-user && make test CHAPTER=1

test2: setup
	cp -r os2 os
	cd ci-user && make test CHAPTER=2

test3: setup
	cp -r os3 os
	cd ci-user && make test CHAPTER=3

test4: setup
	cp -r os4 os
	cd ci-user && make test CHAPTER=4

test5: setup
	cp -r os5 os
	cd ci-user && make test CHAPTER=5

test6: setup
	cp -r os6 os
	cd ci-user && make test CHAPTER=6

test7: setup
	cp -r os7 os
	cd ci-user && make test CHAPTER=7

test8: setup
	cp -r os8 os
	cd ci-user && make test CHAPTER=8

ci-test: setup
	$(MAKE) $(FINISHED_LAB)

docker:
	HOST_DIR=$(HOST_DIR) $(MAKE) -C conf docker
