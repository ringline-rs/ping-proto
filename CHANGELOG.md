# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `DEFAULT_MAX_LINE_LEN` (64 KiB, matching Redis's `PROTO_INLINE_MAX_SIZE`) and
  `Response::parse_with_max_line_len`.

### Fixed

- Response parsing is now bounded. The RESP-style `+...` and `-...` forms
  reported `Incomplete` for an unterminated line no matter how large the buffer
  grew, so a peer that never sent a newline could make a caller buffer without
  limit. Both now return `Invalid` past `DEFAULT_MAX_LINE_LEN`.

### Documentation

- `Request::encode` and `Response::encode` document that they panic when the
  destination buffer is too small.

## [0.0.1] - 2026-02-21

### Added

- Initial release extracted from crucible workspace
- PING request encoding
- PONG response parsing (ASCII and RESP-style)
- Error response parsing
