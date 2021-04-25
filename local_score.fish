set score 0
cargo run < ./in/0000.txt ^tmp --release
cat tmp | while read line
    set now $line
end
set score (math $score + $now)
for val in (seq 1 9)
  echo $val
  cargo run < ./in/000$val.txt ^tmp --release
  cat tmp | while read line
    set now $line
  end
  set score (math $score + $now)
end

for val in (seq 10 99)
  echo $val
  cargo run < ./in/00$val.txt ^tmp --release
  cat tmp | while read line
    set now $line
  end
  set score (math $score + $now)
end
echo $score