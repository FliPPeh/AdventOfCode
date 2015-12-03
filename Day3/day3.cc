#include <iostream>
#include <map>
#include <tuple>

struct santa {
	int x = 0;
	int y = 0;

	bool move(char d) {
		     if (d == '<') { --x; return true; }
		else if (d == '>') { ++x; return true; }
		else if (d == '^') { --y; return true; }
		else if (d == 'v') { ++y; return true; }
		else { return false; }
	}
};

int main(int argc, char** argv)
{
	int constexpr santa_count = 2; // 0 = santa, 1 = robo santa

	std::map<std::tuple<int, int>, int> houses{{std::make_tuple(0, 0), 0}};
	int c;
	int n = 0;

	santa santas[santa_count];

	while ((c = getc(stdin)) != EOF) {
		if (c == '\n') {
			continue;
		} else if (!santas[n].move(c)) {
			std::cerr << "wot m8: " << c << std::endl;
			std::abort();
		}

		++houses[std::make_tuple(santas[n].x, santas[n].y)];

		n = ++n % santa_count;
	}

	std::cout << "Houses visited: " << houses.size() << std::endl;
}
