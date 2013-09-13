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
#[should_fail]
fn test_escaping() {
    let input = mark_unsafe(~"<script>\"'&</script>");
    assert_eq!(escape_html(input), ~"&lt;script&gt;&#34;&#39;&amp;&lt;/script&gt;");
}


#[test]
#[should_fail]
fn test_escaping_lt() {
    let input = mark_unsafe(~"<");
    assert_eq!(escape_html(input), ~"&lt;");
}

#[test]
#[should_fail]
fn test_escaping_gt() {
    let input = mark_unsafe(~">");
    assert_eq!(escape_html(input), ~"&gt;");
}

#[test]
#[should_fail]
fn test_escaping_amp() {
    let input = mark_unsafe(~"&");
    assert_eq!(escape_html(input), ~"&amp;");
}

#[test]
#[should_fail]
fn test_escaping_quote() {
    let input = mark_unsafe(~"'");
    assert_eq!(escape_html(input), ~"&#39;");
}

#[test]
#[should_fail]
fn test_escaping_double_quote() {
    let input = mark_unsafe(~"\"");
    assert_eq!(escape_html(input), ~"&#34;");
}

