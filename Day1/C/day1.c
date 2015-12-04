#include <stdlib.h>
#include <stdio.h>

int main(int argc, char **argv)
{
	int curfloor = 0;
	int c;
	int basement = -1;
	int i = 0;

	while ((c = getchar()) != EOF) {
		++i;

		if (c == '(') {
			++curfloor;
		} else if (c == ')') {
			--curfloor;

			if ((curfloor < 0) && (basement < 0)) {
				basement = i;
			}
		} else if (c != '\n') {
			printf("wat m8\n");
			abort();
		}
	}

	printf("On floor: %d\n", curfloor);

	if (basement >= 0) {
		printf("Entered baement at: %d\n", basement);
	}
}
