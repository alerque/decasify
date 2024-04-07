AC_PREREQ([2.69])
AC_INIT([decasify], [m4_esyscmd(build-aux/git-version-gen .tarball-version)], [caleb@alerque.com])
AC_CONFIG_AUX_DIR([build-aux])
AC_CONFIG_MACRO_DIR([build-aux])
AM_INIT_AUTOMAKE([foreign tar-pax dist-zstd dist-zip no-dist-gzip color-tests])
AM_SILENT_RULES([yes])

QUE_TRANSFORM_PACKAGE_NAME
QUE_DEVELOPER_MODE

QUE_GIT_VERSION
QUE_DIST_CHECKSUMS

AC_PROG_AWK

QUE_RUST_BOILERPLATE

AC_CONFIG_FILES([Makefile])

AC_OUTPUT
