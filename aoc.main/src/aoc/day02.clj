(ns aoc.day02
  (:require [aoc.util :as util]
            [clojure.string :as str]))

(defn parse-line [line]
  (let [[direction amount] (str/split line #" ")]
    [direction (Integer. amount)]))

(defn parse-data [input]
  (->> input
       str/split-lines
       (map parse-line)))

(defn solve-part1 []
  (->> "2"
       util/get-input
       parse-data
       (reduce (fn [[location depth] [direction amount]]
                 (case direction
                   "forward" [(+ location amount) depth]
                   "down" [location (+ depth amount)]
                   "up" [location (- depth amount)]))
               [0 0])
       (apply *)))

(defn solve-part2 []
  (->> "2"
       util/get-input
       parse-data
       (reduce (fn [[location depth aim] [direction amount]]
                 (case direction
                   "forward" [(+ location amount) (+ depth (* aim amount)) aim]
                   "down" [location depth (+ aim amount)]
                   "up" [location depth (- aim amount)]))
               [0 0 0])
       butlast
       (apply *)))