/*
 * Compile with -lssl -lcrypto (OpenSSL) -std=c++11
 */
#include <cstdio>
#include <cstdlib>
#include <cstring>

#include <thread>
#include <atomic>
#include <vector>
#include <iostream>

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
		std::snprintf(out + n*2, 33, "%02x", (unsigned)digest[n]);
	}

	return out;
}

char *make_input(char const *key, int n, char *out, size_t outsiz)
{
	std::snprintf(out, outsiz, "%s%d", key, n);

	return out;
}

#define ZEROS "00000000000000000000000000000000"

int main(int argc, char **argv)
{
	int n = 5;

	if (argc < 2) {
		std::cerr << "usage: " << argv[0] << " <key> [n]" << std::endl;
		return 1;
	}

	if (argc > 2) {
		n = std::atoi(argv[2]);
	}

	std::atomic_bool found{false};

	std::vector<std::thread> threads;
	unsigned const nthreads = std::thread::hardware_concurrency();

	std::cout << "Running with " << nthreads << " threads..." << std::endl;

	for (int i = 0; i < nthreads; ++i) {
		threads.emplace_back([&, n, i] {
			char hash[33];
			char keyout[256];

			for (int j = i; !found; j += nthreads) {
				char const *input =
					make_input(argv[1], j, keyout, sizeof(keyout));

				if (!std::strncmp(md5(input, strlen(input), hash), ZEROS, n)) {
					std::cout << "Ho ho ho from thread " << i << ": "
						<< input << " = " << hash << std::endl;
					found = true;

					return;
				}
			}
		});
	}

	while (!found) {
		std::this_thread::yield();
	}

	for (auto& t : threads) {
		t.join();
	}
}
