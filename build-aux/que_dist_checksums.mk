# Output both a file that can be attatched to releases and also write STDOUT
# for the sake of CI build logs so they can be audited as matching what is
# eventually posted. The list of files checksummed is a glob (even though we
# know an exact pattern) to avoid errors for formats not generated.
checksum_dist = \
	shopt -s nullglob ; \
	$(SHA256SUM) $(distdir)*.{tar.{gz,bz2,lz,xz,zst},zip} |\
		$(TEE) $(distdir).sha256.txt

# Since the checksums file isn't an artifact produced by the default source dist
# creation process, we have to clean it up ourselves so distcheck can see that
# everything round-tripped cleanly.
distclean-local:
	rm -f $(distdir).sha256.txt

# Append checksum operation to function that runs after compressing dist archives
am__post_remove_distdir = $(am__remove_distdir); $(checksum_dist)

# vim: ft=automake
