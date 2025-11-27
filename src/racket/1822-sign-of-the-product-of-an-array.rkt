; 1822. Sign of the Product of an Array
; https://leetcode.com/problems/sign-of-the-product-of-an-array/

#lang racket

(define/contract (array-sign nums)
  (-> (listof exact-integer?) exact-integer?)
  (if (member 0 nums)
      0
      (- 1 (* 2 (remainder (count negative? nums) 2)))))

(array-sign (list -1 -2 -3 -4 3 2 1))
(array-sign (list 1 5 0 2 -3))
(array-sign (list -1 1 -1 1 -1))
