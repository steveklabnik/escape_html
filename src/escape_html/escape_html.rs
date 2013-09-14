use std::cast::transmute;

struct SafeString;
struct UnsafeString;

enum QuestionableString<T> { Str(~str) }

fn mark_unsafe(s: ~str) -> QuestionableString<UnsafeString> {
  Str(s)
}

fn mark_safe(s: QuestionableString<UnsafeString>) -> QuestionableString<SafeString> {
  unsafe { transmute(s) }
}

impl Eq for QuestionableString<UnsafeString> {
    fn eq(&self, other: &QuestionableString<UnsafeString>) -> bool { 
        match (self, other) {
            (&Str(ref s), &Str(ref o)) => o == s
        }
    }
}

impl Eq for QuestionableString<SafeString> {
    fn eq(&self, other: &QuestionableString<SafeString>) -> bool { 
        match (self, other) {
            (&Str(ref s), &Str(ref o)) => o == s
        }
    }
}

fn escape_html<T>(s: QuestionableString<T>) -> ~str {
    match s {
        Str(s) => { 
            let mut out: ~str = ~"";
            out.reserve_at_least(s.len());
            for c in s.iter() {
                match c {
                    '<'  => { out.push_str("&lt;")  }
                    '>'  => { out.push_str("&gt;")  }
                    '&'  => { out.push_str("&amp;") }
                    '\'' => { out.push_str("&#39;") }
                    '"'  => { out.push_str("&#34;") }
                    _    => out.push_char(c),
                }
            }
            out
        }
    }
}

#[test]
fn test_escaping() {
    let input = mark_unsafe(~"<script>\"'&</script>");
    assert_eq!(escape_html(input), ~"&lt;script&gt;&#34;&#39;&amp;&lt;/script&gt;");
}


#[test]
fn test_escaping_lt() {
    let input = mark_unsafe(~"<");
    assert_eq!(escape_html(input), ~"&lt;");
}

#[test]
fn test_escaping_gt() {
    let input = mark_unsafe(~">");
    assert_eq!(escape_html(input), ~"&gt;");
}

#[test]
fn test_escaping_amp() {
    let input = mark_unsafe(~"&");
    assert_eq!(escape_html(input), ~"&amp;");
}

#[test]
fn test_escaping_quote() {
    let input = mark_unsafe(~"'");
    assert_eq!(escape_html(input), ~"&#39;");
}

#[test]
fn test_escaping_double_quote() {
    let input = mark_unsafe(~"\"");
    assert_eq!(escape_html(input), ~"&#34;");
}

#[test]
fn test_mark_safe() {
    let input = mark_unsafe(~"&");
    let sketchy_str : QuestionableString<SafeString> = Str(~"&");
    assert_eq!(mark_safe(input), sketchy_str);
}

