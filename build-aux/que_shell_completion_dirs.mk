COMPLETIONS_OUT_DIR = completions

# We're substituting this ourselves
ifneq (@_BASH_COMPLETION@,no)
bashcompletiondir = $(BASH_COMPLETION_DIR)
nodist_bashcompletion_DATA = $(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME)
CLEANFILES += $(nodist_bashcompletion_DATA)
endif

ifneq (@_ZSH_COMPLETION@,no)
zshcompletiondir = $(ZSH_COMPLETION_DIR)
nodist_zshcompletion_DATA = $(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME)
CLEANFILES += $(nodist_zshcompletion_DATA)
endif

# vim: ft=automake
