// 389. Find the Difference
// https://leetcode.com/problems/find-the-difference/

#include <cstddef>
#include <print>
#include <string>
#include <utility>

using namespace std;

class Solution {
  public:
    char findTheDifference(string s, string t) {
        if (s.empty())
            return t[0];

        for (const auto i : s)
            ++counter_s[i - 'a'];
        for (const auto i : t)
            ++counter_t[i - 'a'];

        auto k = 0;
        for (auto i = 0; i != CHAR_TYPES; ++i) {
            if (counter_t[i] - counter_s[i] == 1)
                return 'a' + k;
            ++k;
        }

        unreachable();
    }

  private:
    static constexpr auto CHAR_TYPES = 26;
    size_t counter_s[26];
    size_t counter_t[26];
};

int main() {
    println("{}", Solution().findTheDifference("abcd", "abcde"));
    println("{}", Solution().findTheDifference("", "y"));
}
