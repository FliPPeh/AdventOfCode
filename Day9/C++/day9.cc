#include <iostream>
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
void remove_rn(std::string& s)
{
	size_t const len = s.size();
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
		std::string line;
		std::getline(std::cin, line);

		if (!std::cin.good()) {
			break;
		}

		remove_rn(line);

		char a[32], b[32];
		int d;

		if (std::sscanf(line.c_str(), "%32s to %32s = %d", &a, &b, &d) != 3) {
			std::cerr << "aaaaaaaaa: " << line << std::endl;
			return 1;
		}

		insert_place(a);
		insert_place(b);

		dists.insert(std::make_pair(std::make_tuple(a, b), d));
		dists.insert(std::make_pair(std::make_tuple(b, a), d));
	} while (std::cin.good());

	for (auto const& dist : dists) {
		std::cout
			<< std::get<0>(dist.first)
			<< " -> "
			<< std::get<1>(dist.first)
			<< " = "
			<< dist.second
			<< std::endl;
	}

	std::sort(std::begin(places), std::end(places));

	int mi = std::numeric_limits<int>::max();
	int ma = 0;

	do {
		std::cout << "Trying: ";

		for (auto const& p : places) {
			std::cout << p << " ";
		}

		std::cout << std::endl;

		int d = 0;

		for (int i = 0; i < places.size() - 1; ++i) {
			d += dists[std::make_tuple(places[i], places[i + 1])];
		}

		if (d > ma) { ma = d; }
		if (d < mi) { mi = d; }

	} while (std::next_permutation(std::begin(places), std::end(places)));

	std::cout << "Shortest: " << mi << std::endl;
	std::cout << "Longest : " << ma << std::endl;
}
