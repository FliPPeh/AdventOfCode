/*
 * Compile with -lssl -lcrypto (OpenSSL)
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <openssl/md5.h>

char *md5(char const *str, int len, char *out/*[33]*/)
{
	int n;
	unsigned char digest[16];

	MD5_CTX c;
	MD5_Init(&c);

	while (len > 0) {
		MD5_Update(&c, str, len > 512 ? 512 : len);
		len -= 512;
		str += 512;
	}

	MD5_Final(digest, &c);

	for (n = 0; n < 16; ++n) {
		snprintf(out + n*2, 33, "%02x", (unsigned)digest[n]);
	}

	return out;
}

char *make_input(char const *key, int n, char *out, size_t outsiz)
{
	snprintf(out, outsiz, "%s%d", key, n);

	return out;
}

#define ZEROS "00000000000000000000000000000000"

int main(int argc, char **argv)
{
	int i = 0;
	int n = 5;
	char keyout[256];

	if (argc < 2) {
		fprintf(stderr, "usage: %s <key> [n]\n", argv[0]);
		return 1;
	}

	if (argc > 2) {
		n = atoi(argv[2]);
	}

	for (;;) {
		char hash[33];
		char const *input = make_input(argv[1], i++, keyout, sizeof(keyout));

		if (!strncmp(md5(input, strlen(input), hash), ZEROS, n)) {
			printf("Ho ho ho: %s = %s\n", input, hash);
		}
	}
}
