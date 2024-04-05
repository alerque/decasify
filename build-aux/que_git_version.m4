AC_DEFUN([QUE_GIT_VERSION], [

        AC_PROG_AWK
        AC_PROG_GREP
        QUE_PROGVAR([cmp])

        QUE_TRANSFORM_PACKAGE_NAME

        AC_REQUIRE([AX_AM_MACROS])
        AX_ADD_AM_MACRO([dnl
$(cat build-aux/que_git_version.mk)
])dnl

])
