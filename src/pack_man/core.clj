(ns pack-man.core
  (:require [clojure.java.io :refer [file reader]]
            [clojure.string :refer [join]])
  (:import [java.io PrintWriter])
  (:gen-class))

(defn pack
  "Packs the given Postscript string seq for easier search and replace"
  [coll]
  (lazy-seq
   (let [[header body] (split-with #(not (.startsWith % "%%Page: ")) coll)]
     (loop [result (vec header)
            acc    nil
            body   body]
       (if (seq body)
         (let [s  (first body)
               xs (rest body)]
           (cond
            (re-matches #"^\[.*\].+$|^\[/.+$" s)
            (recur (conj result s) nil xs)

            (or (.startsWith s "(") (and (.startsWith s "[") (nil? acc)))
            (recur result [s] xs)

            (and acc (re-matches #"^[^\[]+].+$" s))
            (recur (conj result (.replaceAll (join " " (conj acc s)) "\\\\ " ""))
                   nil
                   xs)

            acc
            (recur result (conj acc s) xs)

            :default
            (recur (conj result s) acc xs)))
         result)))))

(defn -main [& [src dest]]
  (let [src (file src)
        dest (file (if dest dest (str "packed-" (.getName src))))]
    (with-open [fin   (reader src)
                fout  (PrintWriter. dest)]
      (doseq [line (-> fin reader line-seq pack)]
        (.println fout line)))))
