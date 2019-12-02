#ifndef LEETCODE_UTIL_H
#define LEETCODE_UTIL_H

#include <string>
#include <vector>

namespace leetcode {

// Definition for singly-linked list.
template <class T>
struct _ListNode {
    T val;
    _ListNode * next;
    _ListNode(int x) : val(x), next(nullptr) {}
};

typedef _ListNode<int> ListNode;

template <class T>
inline std::string to_string(T val) {
    return std::to_string(val);
}

inline std::string to_string(std::string val) {
    return val;
}

inline std::string to_string(char c) {
    std::string s(1, c); 
    return s;
}

inline std::string to_string(bool b) {
    return b ? "True" : "False";
}

template <class T>
inline std::string to_string(
        std::vector<T> v,
        std::string joiner=",",
        std::pair<std::string, std::string> braces={"[", "]"}) {
    if (v.empty()) return braces.first + braces.second;
    std::string s;
    for (auto iter = v.begin(); iter < v.end() - 1; ++iter)
        s += to_string(*iter) + joiner;
    return braces.first + s + to_string(v.back()) + braces.second;
}

template <class T>
inline std::string to_string(
        _ListNode<T> * head,
        std::string joiner="->",
        std::pair<std::string, std::string> braces={"(", ")"}) {
    std::vector<T> v;
    while (head) {
        v.push_back(head->val);
        head = head->next;
    }
    return to_string(v, joiner, braces);
}

}

#endif
