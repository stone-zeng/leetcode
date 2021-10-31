// 430. Flatten a Multilevel Doubly Linked List
// https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/

// Definition for a Node.
class Node {
public:
    int val;
    Node* prev;
    Node* next;
    Node* child;
};

class Solution {
public:
    Node* flatten(Node* head) {
        if (!head) return nullptr;
        auto p = head;
        while (p) {
            if (p->child) {
                auto c = p->child;
                while (c->next) c = c->next;
                if (p->next) p->next->prev = c;
                c->next = p->next;
                p->next = p->child;
                p->child->prev = p;
                p->child = nullptr;
            }
            p = p->next;
        }
        return head;
    }
};
