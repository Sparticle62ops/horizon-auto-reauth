# auto-reauth (Horizon port)

A Rust port of [Auto Reauth](https://github.com/Basicprogrammer10/auto-reauth)
(Basicprogrammer10, MIT): automatically refreshes an expiring Microsoft
session before a server kick, so a long play session does not end at the
token's expiry. Built on Horizon's native mod API.

## Scope (the mod's full logic core, 1:1)

Upstream read at master e5b0f59 (2026-10-19). The 1:1 core lives in
`src/runtime.rs`, behind the mod API wiring in `src/lib.rs`:

- The chat command grammar (`/autoreauth login | logout | status |
  reauth | auto <on|off>` and the `/reauth` alias), the usage line and
  the action enum.
- The config file `config/auto-reauth/config.json` with the original's
  shape ({"auto": bool, "debug": bool, "account": {...}|null,
  "last_error": str|null}), its defaults and its save/load.
- Form encoding and decoding, the authorize URL with a random state, the
  code-exchange and refresh-token bodies, and the token parsing that
  keeps the old refresh token.
- The Xbox/XSTS/Minecraft chain: the request bodies, the XErr mapping
  (no Xbox profile, child account), the identity header, the login and
  profile parsers and the dashed-uuid writer.
- The status words (Online/Invalid/Offline/Unknown), their colours and
  chat lines, and the JWT expiry check with its base64url decoder.

11 tests green.

## Host seams

Upstream's process-level IO is the mod's `HostSeam` (the Microsoft HTTP
chain, the browser open and the localhost callback listener on port
9090), and the `/autoreauth` chat commands wait on a chat-command hook
in the mod API. Both wait on carrier hooks in the SDK; the grammar and
parsing behind them carry 1:1 with tests.

## Licence

MIT, like upstream. Upstream (c) Basicprogrammer10.
