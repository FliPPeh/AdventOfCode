#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define AMOUNT 100

struct ingredient {
	char name[256];

	int capacity;
	int durability;
	int flavor;
	int texture;
	int calories;
};

#define MAX(a, b) ((a < b) ? b : a)

int main(int argc, char **argv)
{
	struct ingredient *ingredients = NULL;
	size_t ningredients = 0;
	long i;

	do {
		struct ingredient i;

		if (fscanf(stdin, "%[^:]: capacity %d, durability %d, "
					              "flavor %d, texture %d, calories %d\n",
				i.name,
				&i.capacity,
				&i.durability,
				&i.flavor,
				&i.texture,
				&i.calories) != 6) {
			break;
		}

		ingredients = realloc(ingredients,
			sizeof(*ingredients) * (ningredients + 1));
		memcpy(&ingredients[ningredients++], &i, sizeof(i));

	} while (!feof(stdin));

	for (i = 0; i < ningredients; ++i) {
		printf("> %s; c:%d, d:%d, f:%d, t:%d, C:%d\n",
				ingredients[i].name,
				ingredients[i].capacity,
				ingredients[i].durability,
				ingredients[i].flavor,
				ingredients[i].texture,
				ingredients[i].calories);
	}

	printf("%d ingredients -> trying %ld combinations...\n",
			ningredients,
			(long)pow(AMOUNT, ningredients));

	int *fs = malloc(sizeof(*fs) * ningredients);
	int m = 0;
	int m500 = 0;

	for (i = 0; i < (long)pow(AMOUNT, ningredients); ++i) {
		int j;
		int f = 1;
		long s = 0;
		struct ingredient it = {};
		long is = 0;

		for (j = 0; j < ningredients; ++j) {
			fs[j] = i/f % AMOUNT;
			f *= AMOUNT;

			if ((s += fs[j]) > AMOUNT) {
				goto out;
			}
		}

		if (s != AMOUNT) {
			goto out;
		}

		for (j = 0; j < ningredients; ++j) {
			it.capacity += ingredients[j].capacity * fs[j];
			it.durability += ingredients[j].durability * fs[j];
			it.flavor += ingredients[j].flavor * fs[j];
			it.texture += ingredients[j].texture * fs[j];
			it.calories += ingredients[j].calories * fs[j];
		}

		is = MAX(0, it.capacity)
			* MAX(0, it.durability)
			* MAX(0, it.flavor)
			* MAX(0, it.texture);

		if ((it.calories == 500) && (is > m500)) {
			m500 = is;
		}

		if (is > m) {
			m = is;
		}
out:
		;;
	}

	printf("%d, %d\n", m, m500);

	return 0;
}
