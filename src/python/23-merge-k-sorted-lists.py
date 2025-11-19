'''23. Merge k Sorted Lists
https://leetcode.com/problems/merge-k-sorted-lists/
'''

from typing import List, Optional

class ListNode:
    '''Definition for singly-linked list.'''
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        length = len(lists)
        if length == 0:
            return None
        if length == 1:
            return lists[0]
        if length == 2:
            return self.mergeTwoLists(lists[0], lists[1])
        half = length // 2
        return self.mergeTwoLists(
            self.mergeKLists(lists[:half]),
            self.mergeKLists(lists[half:]),
        )

    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]):
        if not list1:
            return list2
        if not list2:
            return list1
        if list1.val < list2.val:
            return ListNode(list1.val, self.mergeTwoLists(list1.next, list2))
        else:
            return ListNode(list2.val, self.mergeTwoLists(list2.next, list1))
