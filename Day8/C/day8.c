#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int count_string(
		char const *s,
		int *len_repr,
		int *len_str,
		int *len_encrepr,
		int *len_encstr)
{
	*len_repr = 0;
	*len_str = 0;
	*len_encrepr = 2;
	*len_encstr = 0;

	while (*s) {
		switch (*s) {
			case '"':
				*len_repr += 1;
				*len_encrepr += 2;
				break;

			case '\\': {
				++s;

				*len_str += 1;

				switch (*s) {
					case '\\':
					case '\"':
						*len_repr += 2;
						*len_encrepr += 4;
						*len_encstr += 1;

						break;

					case 'x':
						*len_repr += 4;
						*len_encrepr += 5;
						*len_encstr += 4;

						s += 2;

						break;
				}

				break;
			}

			default:
				++*len_repr;
				++*len_str;
				++*len_encrepr;
				++*len_encstr;

				break;
		}

		++s;
	}

	return 0;
}

void remove_rn(char *s)
{
	size_t const len = strlen(s);
	size_t const l1 = len - 1;
	size_t const l2 = len - 2;

	if ((s[l1] == '\n') || (s[l1] == '\r')) { s[l1] = '\0'; }
	if ((s[l2] == '\n') || (s[l2] == '\r')) { s[l2] = '\0'; }
}

int main(int argc, char **argv)
{
	FILE *f = stdin;

	int total_repr = 0;
	int total_str = 0;
	int total_something = 0;

	do {
		char buf[512] = {0};

		if (fgets(buf, sizeof(buf), f)) {
			int r, s;
			int er, es;

			remove_rn(buf);
			count_string(buf, &r, &s, &er, &es);
			printf("-> '%s' = %d, %d : %d, %d\n", buf, r, s, er, es);

			total_repr += r;
			total_str += s;
			total_something += er - r;
		}
	} while (!feof(f));

	printf("Result: %d\n", total_repr - total_str);
	printf("Something: %d\n", total_something);
}
