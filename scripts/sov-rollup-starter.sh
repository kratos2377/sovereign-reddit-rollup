#!/usr/bin/env bash
trap 'jobs -p | xargs -r kill' EXIT
echo 'Running: '\''cd crates/rollup/'\'''
cd crates/rollup/
if [ $? -ne 0 ]; then
    echo "Expected exit code 0, got $?"
    exit 1
fi

echo 'Running: '\''make clean-db'\'''
make clean-db
if [ $? -ne 0 ]; then
    echo "Expected exit code 0, got $?"
    exit 1
fi

echo 'Running: '\''cargo run --bin node'\'''
output=$(mktemp)
cargo run --bin node &> $output &
background_process_pid=$!
echo "Waiting for process with PID: $background_process_pid"
until grep -q -i rest_address $output
do       
  if ! ps $background_process_pid > /dev/null 
  then
    echo "The background process died died" >&2
    exit 1
  fi
  echo -n "."
  sleep 5
done

echo 'Running: '\''make test-create-token'\'''
make test-create-token
if [ $? -ne 0 ]; then
    echo "Expected exit code 0, got $?"
    exit 1
fi

echo 'Running: '\''curl -sS http://127.0.0.1:12346/ledger/txs/0x6bbe9b5976fb6015c4522f9d4359585344598ca95447f71e3b34dbdecd43af03 | jq'\'''
curl -sS http://127.0.0.1:12346/ledger/txs/0x6bbe9b5976fb6015c4522f9d4359585344598ca95447f71e3b34dbdecd43af03 | jq
if [ $? -ne 0 ]; then
    echo "Expected exit code 0, got $?"
    exit 1
fi

echo 'Running: '\''curl -sS http://127.0.0.1:12346/ledger/txs/0x6bbe9b5976fb6015c4522f9d4359585344598ca95447f71e3b34dbdecd43af03/events | jq'\'''
curl -sS http://127.0.0.1:12346/ledger/txs/0x6bbe9b5976fb6015c4522f9d4359585344598ca95447f71e3b34dbdecd43af03/events | jq
if [ $? -ne 0 ]; then
    echo "Expected exit code 0, got $?"
    exit 1
fi

echo 'Running: '\''curl -Ss http://127.0.0.1:12346/modules/bank/tokens/token_17732u2vyp35dl6lkgjrdqs4mtuzt7rmy02hq9nqct8wq3g74rqyqz0rt2x/total-supply | jq -c -M'\'''
output=$(curl -Ss http://127.0.0.1:12346/modules/bank/tokens/token_17732u2vyp35dl6lkgjrdqs4mtuzt7rmy02hq9nqct8wq3g74rqyqz0rt2x/total-supply | jq -c -M)
expected='{"data":{"amount":"1000000","token_id":"token_17732u2vyp35dl6lkgjrdqs4mtuzt7rmy02hq9nqct8wq3g74rqyqz0rt2x"},"meta":{}}
'
# Either of the two must be a substring of the other. This kinda protects us
# against whitespace differences, trimming, etc.
if ! [[ $output == *"$expected"* || $expected == *"$output"* ]]; then
    echo "'$expected' not found in text:"
    echo "'$output'"
    exit 1
fi

if [ $? -ne 0 ]; then
    echo "Expected exit code 0, got $?"
    exit 1
fi

echo 'Running: '\''curl -Ss http://127.0.0.1:12346/modules/bank/tokens/token_17732u2vyp35dl6lkgjrdqs4mtuzt7rmy02hq9nqct8wq3g74rqyqz0rt2x/total-supply | jq -c -M'\'''
output=$(curl -Ss http://127.0.0.1:12346/modules/bank/tokens/token_17732u2vyp35dl6lkgjrdqs4mtuzt7rmy02hq9nqct8wq3g74rqyqz0rt2x/total-supply | jq -c -M)
expected='{"data":{"amount":"1000000","token_id":"token_17732u2vyp35dl6lkgjrdqs4mtuzt7rmy02hq9nqct8wq3g74rqyqz0rt2x"},"meta":{}}
'
# Either of the two must be a substring of the other. This kinda protects us
# against whitespace differences, trimming, etc.
if ! [[ $output == *"$expected"* || $expected == *"$output"* ]]; then
    echo "'$expected' not found in text:"
    echo "'$output'"
    exit 1
fi

if [ $? -ne 0 ]; then
    echo "Expected exit code 0, got $?"
    exit 1
fi

echo "All tests passed!"; exit 0
