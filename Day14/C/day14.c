#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum reindeer_state {
	FLYING,
	RESTING
};

struct reindeer {
	char const *name;

	int speed;
	int flying_time;
	int resting_time;

	int distance;
	int points;

	enum reindeer_state state;

	int n; // Flying time left or resting time left
};

int reindeer_comp_dist(void const *a, void const *b)
{
	return ((struct reindeer *)a)->distance > ((struct reindeer *)b)->distance;
}

int reindeer_comp_points(void const *a, void const *b)
{
	return ((struct reindeer *)a)->points > ((struct reindeer *)b)->points;
}

int main(int argc, char **argv)
{
	struct reindeer *reindeers = NULL;
	size_t num_reindeers = 0;
	int n = 1000;
	int i;

	if (argc >= 1) {
		n = atoi(argv[1]);
	}

	do {
		char name[128];
		int kms, t, r;
		struct reindeer *d;

		if (fscanf(stdin, "%128s can fly %d km/s for %d seconds, "
					      "but then must rest for %d seconds.",
						name, &kms, &t, &r) != 4) {
			break;
		}

		reindeers = realloc(reindeers,
				sizeof(struct reindeer) * (num_reindeers + 1));
		d = &reindeers[num_reindeers++];

		d->name = strncpy(malloc(strlen(name) + 1), name, strlen(name) + 1);
		d->speed = kms;
		d->flying_time = t;
		d->resting_time = r;
		d->distance = 0;
		d->points = 0;

		d->state = FLYING;
		d->n = d->flying_time;
	} while (!feof(stdin));

	for (i = 0; i < n; ++i) {
		int j;
		int lead = 0;

		for (j = 0; j < num_reindeers; ++j) {
			struct reindeer *d = &reindeers[j];

			if (d->state == FLYING) {
				if (!d->n--) {
					d->state = RESTING;
					d->n = d->resting_time - 1;
				} else {
					d->distance += d->speed;
				}
			} else {
				if (!d->n--) {
					d->state = FLYING;
					d->n = d->flying_time - 1;
					d->distance += d->speed;
				}
			}

			if (d->distance > lead) {
				lead = d->distance;
			}
		}

		for (j = 0; j < num_reindeers; ++j) {
			struct reindeer *d = &reindeers[j];

			if (d->distance == lead) {
				++d->points;
			}
		}
	}

	qsort(reindeers, num_reindeers, sizeof(struct reindeer),
			reindeer_comp_dist);

	printf("By distance:\n---------\n");
	for (i = 0; i < num_reindeers; ++i) {
		struct reindeer *d = &reindeers[i];

		printf("%s (%d points) has travelled %d km\n",
				d->name, d->points, d->distance);
	}

	printf("---------\n");

	qsort(reindeers, num_reindeers, sizeof(struct reindeer),
			reindeer_comp_points);

	printf("By points:\n---------\n");
	for (i = 0; i < num_reindeers; ++i) {
		struct reindeer *d = &reindeers[i];

		printf("%s (%d points) has travelled %d km\n",
				d->name, d->points, d->distance);
	}

	free(reindeers);
}
