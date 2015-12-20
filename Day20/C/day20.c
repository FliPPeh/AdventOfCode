#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_HOUSES 10000000

int main(int argc, char **argv)
{
	long n = 36000000L;
	long *houses = calloc(MAX_HOUSES, sizeof(*houses));
	int i;

	if (argc > 1) {
		n = atol(argv[1]);
	}

	for (i = 1; i < MAX_HOUSES; ++i) {
		int j;
#ifdef PART2
		int elfcnt = 50;

		for (j = i; elfcnt-- && j <= MAX_HOUSES; j += i) {
			houses[j-1] += i * 11;
		}
#else
		for (j = i; j <= MAX_HOUSES; j += i) {
			houses[j-1] += i * 10;
		}
#endif
	}

end:
	for (i = 1; i < MAX_HOUSES; ++i) {
		if (houses[i - 1] >= n) {
			printf("First house >= %ld: %d\n", n, i);
			break;
		}
	}

	return 0;
}
