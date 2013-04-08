/*
 * C program to compare the performance of encode/decode.
 * NOTE: assuming little-endian environment to compile.
 */

#include "./lib/modp_b64.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

static inline int
file_length(FILE *fp, size_t *slen)
{
	long len;
	if (fseek(fp, 0, SEEK_END) == -1) return -1;
	if ((len = ftell(fp)) == -1) return -1;
	if (fseek(fp, 0, SEEK_SET) == -1) return -1;
	*slen = (size_t)len;
	return 0;
}

int
main(int argc, char **argv)
{
	FILE *fp;
	char *cmd, *filename, *src, *dst;
	size_t src_len, nconv;

	if (argc < 3)
		return 1;

	cmd = argv[1];
	filename = argv[2];
	src = dst = NULL;

	if (!(fp = fopen(filename, "rb"))) goto err;
	if (file_length(fp, &src_len) == -1) goto err;
	if (!(src = malloc(src_len))) goto err;
	fread(src, src_len, 1, fp); fclose(fp); fp = NULL;

	nconv = 0;
	if (!strcmp("encode", cmd)) {
		dst = malloc(modp_b64_encode_len(src_len));
		nconv = modp_b64_encode(dst, src, src_len);
		if (nconv == -1) goto err;
		else fwrite(dst, nconv, 1, stdout);
	} else if (!strcmp("decode", cmd)) {
		dst = malloc(modp_b64_decode_len(src_len));
		nconv = modp_b64_decode(dst, src, src_len);
		if (nconv == -1) goto err;
		else fwrite(dst, nconv, 1, stdout);
	} else {
		printf("unknown command: %s\n", cmd);
		goto err;
	}

	return 0;
err:
	if (errno) perror("b64");
	if (fp) fclose(fp);
	if (src) free(src);
	if (dst) free(dst);
	if (nconv == -1) puts("failed to encode/decode");
	return 1;
}
