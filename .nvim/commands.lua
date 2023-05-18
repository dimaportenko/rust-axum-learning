return {
  ['run'] = 'cargo run',
  ['run:watch'] = 'cargo watch -q -c -w src/ -x run',
  ['test:watch'] = 'cargo watch -q -c -w tests/ -x "test -- --nocapture"',
}
