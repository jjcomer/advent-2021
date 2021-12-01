(ns aoc.day01
  (:require [aoc.util :as util]
            [clojure.string :as str]))

(defn parse-data [input]
  (map #(Integer. %) (str/split-lines input)))

(defn solve-part1 []
  (let [input (-> "1" util/get-input parse-data)]
    (->> input
         (partition 2 1)
         (filter (fn [[x y]] (< x y)))
         count)))

(defn solve-part2 []
  (let [input (-> "1" util/get-input parse-data)]
    (->> input
         (partition 4 1)
         (filter (fn [[a b c d]] (< (+ a b c) (+ b c d))))
         count)))