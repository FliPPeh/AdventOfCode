#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define EGGNOG 150

static int lt(void const *a, void const *b)
{
	return *(int *)a < *(int *)b;
}

int main(int argc, char **argv)
{
	int *containers = NULL;
	int *set = NULL;

	int c1 = 0;
	int c2 = 0;
	int target = 0;

	size_t i;
	size_t num_containers = 0;

	do {
		int n;

		if (fscanf(stdin, "%d\n", &n) != 1) {
			break;
		}

		containers = realloc(containers,
				sizeof(*containers) * (num_containers + 1));
		containers[num_containers++] = n;
	} while (!feof(stdin));

	qsort(containers, num_containers, sizeof(int), lt);

#ifdef VERBOSE
	printf("change{");
	for (i = 0; i < num_containers; ++i) {
		printf("%d", containers[i]);

		if (i < (num_containers - 1)) {
			printf(", ");
		}
	}
	printf("} => %d\n", EGGNOG);
#endif

	set = calloc(num_containers, sizeof(int));

	for (i = 1; i < (1 << num_containers); ++i) {
		int setc = 0;
		int s = 0;
		size_t j;

		/* Split up counter i's bits into set[] and sum up */
		for (j = 0; j < num_containers; ++j) {
			if ((set[j] = !!(i & (1 << j)))) {
				++setc;
				if ((s += containers[j]) > EGGNOG) {
					goto nope;
				}
			}
		}

		if (s == EGGNOG) {
#ifdef VERBOSE
			printf("choice{");
			for (j = 0; j < num_containers; ++j) {
				if (set[j]) {
					printf("%d", containers[j]);

					if (j < (num_containers - 1)) {
						printf(", ");
					}
				}
			}
			printf("} = %d\n", EGGNOG);
#endif
			/* First match we find will also be the minimum match */
			if (!c1) {
				target = setc;
			}

			++c1;

			/* If minimum match, increment counter */
			if (setc == target) {
				++c2;
			}
		}
nope:
		;
	}

	printf("Combinations: %d\nMinimum: %d\n", c1, c2);
}
