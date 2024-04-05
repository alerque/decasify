AC_PREREQ([2.69])
AC_INIT([decasify], [m4_esyscmd(build-aux/git-version-gen .tarball-version)], [caleb@alerque.com])
AC_CONFIG_AUX_DIR([build-aux])
AC_CONFIG_MACRO_DIR([build-aux])
AM_INIT_AUTOMAKE([foreign tar-pax dist-zstd dist-zip no-dist-gzip color-tests])
AM_SILENT_RULES([yes])
AM_MAINTAINER_MODE

QUE_TRANSFORM_PACKAGE_NAME

QUE_GIT_VERSION
AM_COND_IF([MAINTAINER_MODE], [QUE_DIST_CHECKSUMS])

AC_PROG_AWK

QUE_RUST_BOILERPLATE

AC_CONFIG_FILES([Makefile])

AC_OUTPUT
