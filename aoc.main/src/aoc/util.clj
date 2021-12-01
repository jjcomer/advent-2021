(ns aoc.util)

(defn get-input
  [day]
  (slurp (str "../input/2021/day" day ".txt")))