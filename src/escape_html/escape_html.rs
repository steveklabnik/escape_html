use std::cast::transmute;

struct SafeString;
struct UnsafeString;

enum QuestionableString<T> { Str(~str) }

fn mark_unsafe(s: ~str) -> QuestionableString<UnsafeString> {
  Str(s)
}

fn escape_html<T>(s: QuestionableString<T>) -> ~str {
  match s {
    Str(s) => { s }
  }
}

/* For the future
fn mark_safe(s: QuestionableString<UnsafeString>) -> QuestionableString<SafeString> {
  unsafe { transmute(s) }
}
*/

#[test]
fn test_escaping() {
    let input = mark_unsafe(~"<script>");
    assert_eq!(escape_html(input), ~"<script>");
}
