(defproject pack-man "0.1.0"
  :description "Packs Postscript files for easier deciphering"
  :url "https://github.com/shaolang/pack-man"
  :license {:name "MIT License"
            :url "https://mit-license.org"}
  :dependencies [[org.clojure/clojure "1.8.0"]]
  :main ^:skip-aot pack-man.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}
             :dev {:dependencies [[midje "1.8.3"]]}})
