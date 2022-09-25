include .env
export

HOST_DIR ?= $(shell pwd)
TEST_REPO ?= https://github.com/LearningOS/rust-os-camp-2022-test.git
CONF_REPO ?= https://github.com/LearningOS/rust-os-camp-2022-conf.git

test: test3 test4 test5 test6 test7 test8

lab1: test3

lab2: test4

lab3: test5

lab4: test6 test7

lab5: test8

clean-os:
	rm -rf os

clean-user:
	rm -rf ci-user

clean-conf:
	rm -rf conf

setup-user:
	@if test -d ci-user; then echo "ci-user 已存在，跳过 clone，你可能希望先执行 make clean-user"; else git clone $(TEST_REPO) ci-user; fi

setup-conf:
	@if test -d conf; then echo "conf 已存在，跳过 clone，你可能希望先执行 make clean-conf"; else git clone $(CONF_REPO) conf; fi

setup: setup-user setup-conf

reset-user: clean-user setup-user

reset-conf: clean-conf setup-conf

reset: clean-os clean-user clean-conf setup-user setup-conf

test1: clean-os
	cp -r os1 os
	cd ci-user && make test CHAPTER=1

test2: clean-os
	cp -r os2 os
	cd ci-user && make test CHAPTER=2

test3: clean-os
	cp -r os3 os
	cd ci-user && make test CHAPTER=3

test4: clean-os
	cp -r os4 os
	cd ci-user && make test CHAPTER=4

test5: clean-os
	cp -r os5 os
	cd ci-user && make test CHAPTER=5

test6: clean-os
	cp -r os6 os
	cd ci-user && make test CHAPTER=6

test7: clean-os
	cp -r os7 os
	cd ci-user && make test CHAPTER=7

test8: clean-os
	cp -r os8 os
	cd ci-user && make test CHAPTER=8

ci-test: setup-user
	$(MAKE) $(FINISHED_LAB)

docker: setup-conf
	HOST_DIR=$(HOST_DIR) $(MAKE) -C conf docker
