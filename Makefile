EXTENSION     = snowid
EXTVERSION    = $(shell grep "^default_version" snowid.control | sed -r "s/default_version[^']+'([^']+).*/\1/")
DATA          = $(wildcard sql/*--*.sql)
TESTS         = $(wildcard test/sql/*.sql)
REGRESS       = $(patsubst test/sql/%.sql,%,$(TESTS))
REGRESS_OPTS ?= --inputdir=test
EXTRA_CLEAN   = $(EXTENSION)-$(EXTVERSION).zip sql/$(EXTENSION)--$(EXTVERSION).sql META.json Trunk.toml
PG_CONFIG   ?= pg_config

# pg_isolation_regress available in v14 and higher.
ifeq ($(shell test $$($(PG_CONFIG) --version | awk '{print $$2}' | awk 'BEGIN { FS = "." }; { print $$1 }' | sed -E 's/[^0-9]//g') -ge 14; echo $$?),0)
ISOLATION   = $(patsubst test/specs/%.spec,%,$(wildcard test/specs/*.spec))
ISOLATION_OPTS = $(REGRESS_OPTS)
endif

PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)

all: sql/$(EXTENSION)--$(EXTVERSION).sql Trunk.toml

sql/$(EXTENSION)--$(EXTVERSION).sql: sql/$(EXTENSION).sql
	cp $< $@