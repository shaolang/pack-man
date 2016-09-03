(ns pack-man.core-test
  (require [midje.sweet :refer :all]
           [pack-man.core :refer :all]))

(fact
  (pack ["prelude" "[leave" "them" "intact]" "%%Page: x x" "[0" "1] Td"])
  =>    ["prelude" "[leave" "them" "intact]" "%%Page: x x" "[0 1] Td"]

  (pack ["whatever" "%%Page: y y" "(a postscript string)" "[12" "11] TJ"])
  =>    ["whatever" "%%Page: y y" "(a postscript string) [12 11] TJ"]

  (pack ["%%Page: x x" "%%EndPageSetup" "[] 0 d" "1 i" "/DeviceGray {} cs"
         "[123" "234] Td"])
  =>    ["%%Page: x x" "%%EndPageSetup" "[] 0 d" "1 i" "/DeviceGray {} cs"
         "[123 234] Td"]

  (pack ["%%Page: x x" "[/Indexed <" " 000" ">] something"])
  =>    ["%%Page: x x" "[/Indexed <" " 000" ">] something"]

  (pack ["%%Page: xyz xyz" "(a post\\" "script \\" "string)" "[0" "1] TJ"])
  =>    ["%%Page: xyz xyz" "(a postscript string) [0 1] TJ"]

  (pack ["%%Page: x x" "0 0 cm"])
  =>    ["%%Page: x x" "0 0 cm"])
