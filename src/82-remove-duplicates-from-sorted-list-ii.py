'''82. Remove Duplicates from Sorted List II
https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
'''

from typing import Optional, List

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        head = self.removeHead(head)
        i = head
        while i and i.next:
            j = i.next
            n = 1
            while j.next and j.val == j.next.val:
                j = j.next
                n += 1
            if n > 1:
                i.next = j.next
            else:
                i = i.next
        return head

    def removeHead(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head and head.next and head.val == head.next.val:
            while head.next and head.val == head.next.val:
                head = head.next
            return self.removeHead(head.next)
        return head
