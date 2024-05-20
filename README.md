An implementation of the RESP protocol in Rust. Inspired by https://codingchallenges.fyi/challenges/challenge-redis/

This is meant as a learning exercise since I have been meaning to learn Rust for a while. Expect bugs and sub-par best practices while i am figuring things out.

TODO:

- [x] implement RESP V2 deserialization
- [x] implement RESP V2 serialization
- [ ] redis lite server
- [ ] SET command
- [ ] concurrent clients
- [x] automated tests
- [ ] EX, PX EAXT PXAT expiry options for SET command
- [ ] extend command support
- [ ] CI/CD support for releases (github actions)
- [ ] benchmarking
- [ ] implment RESP v3 deserialization
- [ ] implement RESP v3 serialization
- [ ] add support for more commands (https://redis.io/commands/)

