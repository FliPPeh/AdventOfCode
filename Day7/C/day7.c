#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFSIZE 256

char *read_file(FILE *fp)
{
	char *res = NULL;
	size_t bufsiz = 0;
	size_t offset = 0;

	do {
		char buffer[BUFSIZE] = {0};
		size_t r;

		if (!bufsiz || !(bufsiz - offset)) {
			bufsiz += BUFSIZE;
			res = realloc(res, bufsiz * sizeof(char));
		}

		r = fread(res + offset, 1, bufsiz - offset, fp);
		offset += r;

	} while (!feof(fp));

	fclose(fp);

	return res;
}

enum wire_instruction {
	WI_CONST,
	WI_MOV,
	WI_AND,
	WI_OR,
	WI_LSHIFT,
	WI_RSHIFT,
	WI_NOT
};

struct wire {
	char const *name;
	int value;

	enum wire_instruction type;

	union {
		int k;

		struct {
			char const *a;
			char const *b;
		} binops;

		char const *unop;
	} t;
};

struct circuit {
	struct wire *wires;
	size_t num_wires;
};

char *read_token(char **input, char **start, char **end)
{
	while (**input && isspace(**input)) { ++*input; }
	*start = *input;

	while (**input && !isspace(**input)) { ++*input; }
	*end = *input;

	if (**input) {
		**input = '\0';

		if (*end) {
			*input = *end + 1;
		}

		return *start;
	} else {
		return NULL;
	}
}

char *parse_wire(char *input, struct wire *w)
{
	char *start = NULL;
	char *end = NULL;

	if (!read_token(&input, &start, &end)) {
		return NULL;
	}

	if (!strncmp(start, "NOT", end - start)) {
		/* NOT instruction: NOT x -> y */
		w->type = WI_NOT;

		/* read a */
		if (!(w->t.unop = read_token(&input, &start, &end))) {
			return NULL;
		}

		/* read -> */
		if (!read_token(&input, &start, &end) || strcmp(start, "->")) {
			return NULL;
		}

		/* read name */
		if (!(w->name = read_token(&input, &start, &end))) {
			return NULL;
		}

		return input;
	} else {
		/* x OP y -> z  ///  x -> z*/
		char const *first = start;

		/* read x */
		if (!read_token(&input, &start, &end)) {
			return NULL;
		}

		/* read op */
		if (!strcmp(start, "AND")) {
			w->type = WI_AND;
		} else if (!strcmp(start, "OR")) {
			w->type = WI_OR;
		} else if (!strcmp(start, "LSHIFT")) {
			w->type = WI_LSHIFT;
		} else if (!strcmp(start, "RSHIFT")) {
			w->type = WI_RSHIFT;
		} else if (!strcmp(start, "->")) {
			/* special case! */
			w->type = WI_MOV;
			w->t.unop = first;

			if (!(w->name = read_token(&input, &start, &end))) {
				return NULL;
			}

			return input;
		} else {
			fprintf(stderr, "invalid operation: '%s'\n", start);

			exit(1);
		}

		if (!read_token(&input, &start, &end)) {
			return NULL;
		}

		w->t.binops.a = first;
		w->t.binops.b = start;

		if (!read_token(&input, &start, &end) || strcmp(start, "->")) {
			return NULL;
		}

		if (!(w->name = read_token(&input, &start, &end))) {
			return NULL;
		}

		return input;
	}
}

int calculate(struct circuit const *c, char const *w);

int eval(struct circuit const *c, char const *t)
{
	int isnum = 1;
	int i;

	for (i = 0; i < strlen(t); ++i) {
		if (!isdigit(t[i])) {
			isnum = 0;
			break;
		}
	}

	if (isnum) {
		return atoi(t);
	} else {
		return calculate(c, t);
	}
}

int calculate(struct circuit const *c, char const *w)
{
	int i;
	int r = -1;

	struct wire *wi = NULL;

	/* find wire */
	for (i = 0; i < c->num_wires; ++i) {
		if (!strcmp(c->wires[i].name, w)) {
			wi = &c->wires[i];

			break;
		}
	}

	if (!wi) {
		return -1;
	}

	if (wi->type == WI_CONST) {
		return wi->t.k;
	} else if (wi->type == WI_MOV) {
		r = eval(c, wi->t.unop);
	} else if (wi->type == WI_AND) {
		r = eval(c, wi->t.binops.a) & eval(c, wi->t.binops.b);
	} else if (wi->type == WI_OR) {
		r = eval(c, wi->t.binops.a) | eval(c, wi->t.binops.b);
	} else if (wi->type == WI_LSHIFT) {
		r = eval(c, wi->t.binops.a) << eval(c, wi->t.binops.b);
	} else if (wi->type == WI_RSHIFT) {
		r = eval(c, wi->t.binops.a) >> eval(c, wi->t.binops.b);
	} else if (wi->type == WI_NOT) {
		r = ~((unsigned)eval(c, wi->t.unop));
	}

	wi->type = WI_CONST;
	wi->t.k = r;

	return r;
}

int main(int argc, char **argv)
{
	int i;
	char *file = read_file(stdin);

	struct circuit c = {
		.wires = NULL,
		.num_wires = 0
	};

	char *filep = file;
	struct wire w;

	while ((filep = parse_wire(filep, &w)) != NULL) {
		c.wires = realloc(c.wires, sizeof(c.wires[0]) * (c.num_wires + 1));

		memcpy(&c.wires[c.num_wires++], &w, sizeof(w));
	}

	printf("Wire a: %d\n", calculate(&c, "a"));

	free(c.wires);
	free(file);
}
