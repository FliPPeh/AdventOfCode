#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef STEPS
#	define STEPS 5
#endif

void showgrid(char **grid, size_t w, size_t h)
{
	size_t i, j;

	for (i = 0; i < h; ++i) {
		for (j = 0; j < w; ++j) {
			printf("%c", grid[i][j]);
		}

		printf("\n");
	}
}

#ifdef PART2
int is_stuck(char **grid, size_t x, size_t y, size_t w, size_t h)
{
	size_t stuck[][2] = {{0, 0}, {0, h - 1}, {w - 1, 0}, {w - 1, h - 1}};
	size_t i;

	for (i = 0; i < 4; ++i) {
		if ((x == stuck[i][0]) && (y == stuck[i][1])) {
			return 1;
		}
	}

	return 0;
}
#endif

int count_neighbors(char **grid, size_t x, size_t y, size_t w, size_t h)
{
	int i, j;

	int count = 0;

	for (i = -1; i <= 1; ++i) {
		for (j = -1; j <= 1; ++j) {
			if (((i == 0) && (j == 0))
					|| ((((int)y + i) < 0) || (((int)y + i) >= h))
					|| ((((int)x + j) < 0) || (((int)x + j) >= w))) {

				continue; /* don't go over the edges */
			}

			count += grid[y + i][x + j] == '#';
		}
	}

	return count;
}

void step(char **grid, char **res, size_t w, size_t h)
{
	size_t i, j;

	for (i = 0; i < h; ++i) {
		for (j = 0; j < w; ++j) {
			int n = count_neighbors(grid, j, i, w, h);

			if (((grid[i][j] == '#') && ((n == 2) || (n == 3))) || (n == 3)
#ifdef PART2
				|| is_stuck(grid, j, i, w, h)
#endif
			) {
				res[i][j] = '#';
			} else {
				res[i][j] = '.';
			}
		}
	}
}

int main(int argc, char **argv)
{
	size_t i, j;
	int lightcount = 0;

	char **grid = NULL;
	char **grid2 = NULL;

	size_t grid_height = 0;
	size_t grid_width = 0;

	do {
		char buf[1024];
		char *nl = NULL;

		if ((fgets(buf, sizeof(buf), stdin)) == NULL) {
			break;
		}

		if ((nl = strchr(buf, '\n')) != NULL) {
			*nl = '\0';
		}

		if (grid_width && (strlen(buf) != grid_width)) {
			fprintf(stderr, "invalid input: inconsistent grid width\n");

			free(grid);
			return 1;
		} else if (!grid_width) {
			grid_width = strlen(buf);
		}

		grid = realloc(grid, sizeof(*grid) * (grid_height + 1));
		grid[grid_height] = calloc(grid_width, sizeof(char));

		grid2 = realloc(grid2, sizeof(*grid2) * (grid_height + 1));
		grid2[grid_height] = calloc(grid_width, sizeof(char));

		strncpy(grid[grid_height], buf, grid_width);

		++grid_height;
	} while (!feof(stdin));

#ifdef PART2
	for (i = 0; i < grid_height; ++i) {
		for (j = 0; j < grid_width; ++j) {
			if (is_stuck(grid, j, i, grid_width, grid_height)) {
				grid[i][j] = '#';
			}
		}
	}
#endif

	printf("Init:\n");
	showgrid(grid, grid_width, grid_height);
	printf("\n");

	for (i = 0; i < STEPS; ++i) {
		char **temp = grid;

		printf("Step %d:\n", i + 1);
		step(grid, grid2, grid_width, grid_height);
		showgrid(grid2, grid_width, grid_height);
		printf("\n");

		grid = grid2;
		grid2 = temp;
	}

	for (i = 0; i < grid_height; ++i) {
		for (j = 0; j < grid_width; ++j) {
			if (grid[i][j] == '#') {
				++lightcount;
			}
		}
	}

	printf("Number of lights turned on: %d\n", lightcount);

	for (i = 0; i < grid_height; ++i) {
		free(grid[i]);
		free(grid2[i]);
	}

	free(grid);
	free(grid2);
	return 0;
}
