AC_PREREQ([2.69])
AC_INIT([decasify], [m4_esyscmd(build-aux/git-version-gen .tarball-version)], [caleb@alerque.com])
AC_CONFIG_AUX_DIR([build-aux])
AC_CONFIG_MACRO_DIR([build-aux])
AM_INIT_AUTOMAKE([foreign tar-pax dist-xz dist-zip no-dist-gzip color-tests])
AM_SILENT_RULES([yes])

AX_GIT_VERSION
AX_TRANSFORM_PACKAGE_NAME

AC_PROG_AWK

AX_RUST_BOILERPLATE

AC_CONFIG_FILES([Makefile])

AC_OUTPUT
