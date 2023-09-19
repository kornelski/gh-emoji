# GitHub emoji for Rust

Full, up-to-date database of [GitHub emoji](https://github.com/github/gemoji) which have Unicode equivalents. Pre-generated and hashed at compile time for fast lookup.

Useful when rendering [GitLab](https://gitlab.com/gitlab-org/gitlab-ce/blob/master/doc/user/markdown.md#emoji)/[GitHub-flavored  Markdown](https://github.github.com/gfm/), although this crate does not parse any Markdown itself.

Used by [lib.rs website](https://lib.rs/crates/gh-emoji).

## Example usage

```rust
let emoji = gh_emoji::get("smile");
assert_eq!(emoji, Some("😄"));
```

```rust
let replacer = gh_emoji::Replacer::new();
let text = replacer.replace_all(":crocodile:, see you in a while!");
```
