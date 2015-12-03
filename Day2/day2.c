#include <stdio.h>
#include <stdlib.h>

inline static int min(int a, int b)
{
	if (a < b) { return a; } else { return b; }
}

inline static int *minp(int *a, int *b)
{
	if (*a < *b) { return a; } else { return b; }
}

inline static int *maxp(int *a, int *b)
{
	if (*a > *b) { return a; } else { return b; }
}

int main(int argc, char **argv)
{
	long total_wrap = 0;
	long total_ribb = 0;

	while (!feof(stdin)) {
		int l, w, h;

		if (fscanf(stdin, "%dx%dx%d\n", &l, &w, &h) != 3) {
			fprintf(stderr, "m8 wat\n");
			abort();
		}

		int *lo = minp(minp(&l, &w), &h);
		int *hi = maxp(maxp(&l, &w), &h);
		int *mi = (&l != lo) && (&l != hi)
			? &l
			: ((&w != lo) && (&w != hi))
			? &w
			: &h;

		total_wrap += 2*l*w + 2*w*h + 2*h*l + min(min(l*w, w*h), h*l);
		total_ribb += l*w*h + ((*lo + *lo) + (*mi + *mi));
	}

	printf("Total wrap: %ld'\n", total_wrap);
	printf("Total ribbon: %ld'\n", total_ribb);
}
