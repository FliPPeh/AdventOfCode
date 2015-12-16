#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum aunt_property_func {
	F_EQ,
	F_GT,
	F_LT
};

enum aunt_property {
	K_CHILDREN,
	K_CATS,
	K_SAMOYEDS,
	K_POMERANIANS,
	K_AKITAS,
	K_VIZSLAS,
	K_GOLDFISH,
	K_TREES,
	K_CARS,
	K_PERFUMES,
	K_MAX
};

static char const *property_names[] = {
	"children",
	"cats",
	"samoyeds",
	"pomeranians",
	"akitas",
	"vizlas",
	"goldfish",
	"trees",
	"cars",
	"perfumes"
};

int main(int argc, char **argv)
{
	int target_properties[] = { 3, 7, 2, 3, 0, 0, 5, 3, 2, 1 };
	int target_funcs[] =
		{ F_EQ, F_GT, F_EQ, F_LT, F_EQ, F_EQ, F_LT, F_GT, F_EQ, F_EQ };

	int i = 0;

	do {
		char key[32];
		int val;
		int properties[K_MAX] = { -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 };
		int j;

		if (fscanf(stdin, "Sue %*d: ") != 0) {
			break;
		}

		while (fscanf(stdin, "%32[^:]: %d, ", key, &val) == 2) {
			enum aunt_property p;

			     if (!strcmp(key, "children"))    { p = K_CHILDREN;    }
			else if (!strcmp(key, "cats"))        { p = K_CATS;        }
			else if (!strcmp(key, "samoyeds"))    { p = K_SAMOYEDS;    }
			else if (!strcmp(key, "pomeranians")) { p = K_POMERANIANS; }
			else if (!strcmp(key, "akitas"))      { p = K_AKITAS;      }
			else if (!strcmp(key, "vizslas"))     { p = K_VIZSLAS;     }
			else if (!strcmp(key, "goldfish"))    { p = K_GOLDFISH;    }
			else if (!strcmp(key, "trees"))       { p = K_TREES;       }
			else if (!strcmp(key, "cars"))        { p = K_CARS;        }
			else if (!strcmp(key, "perfumes"))    { p = K_PERFUMES;    }
			else {
				fprintf(stderr, "%s: bad key: %s\n", argv[0], key);
				return 1;
			}

			properties[p] = val;
		}

		++i;

		for (j = 0; j < K_MAX; ++j) {
			if (properties[j] >= 0) {
				int p;

				switch (target_funcs[j]) {
				case F_EQ: p = properties[j] == target_properties[j]; break;
				case F_GT: p = properties[j] >  target_properties[j]; break;
				case F_LT: p = properties[j] <  target_properties[j]; break;
				}

				if (!p) {
					goto next;
				}
			}
		}

		printf("Could it be %d?\n", i);
		printf(" => ");

		for (j = 0; j < K_MAX; ++j) {
			if (properties[j] >= 0) {
				printf("%d/%d %s   ",
					properties[j], target_properties[j], property_names[j]);
			}
		}

		printf("\n");
next:
		;
	} while (!feof(stdin));

	return 0;
}
