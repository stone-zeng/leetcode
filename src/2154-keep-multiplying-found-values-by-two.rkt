; 2154. Keep Multiplying Found Values by Two
; https://leetcode.com/problems/keep-multiplying-found-values-by-two/

#lang racket

(define/contract (find-final-value nums original)
  (-> (listof exact-integer?) exact-integer? exact-integer?)
  (find-value (list->set nums) original)
)

(define (find-value nums original)
  (if (set-member? nums original)
    (find-value nums (* original 2))
    original
  )
)

(find-final-value (list 5 3 6 1 12) 3)
(find-final-value (list 2 7 9) 4)
