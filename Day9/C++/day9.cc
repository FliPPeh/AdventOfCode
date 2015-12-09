#include <algorithm>
#include <vector>
#include <unordered_set>
#include <unordered_map>
#include <string>
#include <tuple>
#include <limits>

#include <cstdio>
#include <cstring>

namespace std {
	template <typename T, typename U>
	struct hash<std::tuple<T, U>> {
		std::size_t operator()(std::tuple<T, U> const& k) const
		{
			auto a = std::get<0>(k);
			auto b = std::get<1>(k);

			return std::hash<T>()(a) ^ std::hash<U>()(b);
		}
	};
};

namespace {
void remove_rn(char *s)
{
	size_t const len = std::strlen(s);
	size_t const l1 = len - 1;
	size_t const l2 = len - 2;

	if ((s[l1] == '\n') || (s[l1] == '\r')) { s[l1] = '\0'; }
	if ((s[l2] == '\n') || (s[l2] == '\r')) { s[l2] = '\0'; }
}
};

int main(int argc, char **argv)
{
	std::vector<std::string> places{};
	std::unordered_set<std::string> known{};
	std::unordered_map<std::tuple<std::string, std::string>, int> dists{};

	auto insert_place = [&] (std::string const& p) {
		if (known.find(p) == std::end(known)) {
			places.push_back(p);
			known.insert(p);
		}
	};

	do {
		char buf[512] = {0};

		if (std::fgets(buf, sizeof(buf), stdin)) {
			char a[32], b[32];
			int d;

			remove_rn(buf);

			if (std::sscanf(buf, "%32s to %32s = %d", &a, &b, &d) != 3) {
				std::fprintf(stderr, "aaaaaaaaa: %s", buf);
				return 1;
			}

			insert_place(a);
			insert_place(b);

			dists.insert(std::make_pair(std::make_tuple(a, b), d));
			dists.insert(std::make_pair(std::make_tuple(b, a), d));

			std::printf("'%s' -> '%s': %+d\n", a, b, d);
		}
	} while (!std::feof(stdin));

	for (auto const& dist : dists) {
		std::printf("%s -> %s = %d\n",
			std::get<0>(dist.first).c_str(),
			std::get<1>(dist.first).c_str(),
			dist.second);
	}

	std::sort(std::begin(places), std::end(places));

	int mi = std::numeric_limits<int>::max();
	int ma = 0;

	do {
		std::printf("Trying: ");

		for (auto const& p : places) {
			std::printf("%s, ", p.c_str());
		}

		std::printf("\n");

		int d = 0;
		for (int i = 0; i < places.size() - 1; ++i) {
			d += dists[std::make_tuple(places[i], places[i + 1])];
		}

		if (d > ma) { ma = d; }
		if (d < mi) { mi = d; }

	} while (std::next_permutation(std::begin(places), std::end(places)));

	std::printf("Shortest: %d  Longest: %d\n", mi, ma);
}
