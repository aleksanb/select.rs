#![feature(plugin)]
#![plugin(speculate)]

pub use std::collections::HashMap;

extern crate select;
pub use select::document::Document;
pub use select::node;

speculate! {
    describe "node" {
        before {
            let document = Document::from("<html><head></head><body id=something>\
                                           foo<bar>baz<quux class=another-thing>\
                                           <!--comment-->");

            let html = document.nth(0).unwrap();
            let head = document.nth(1).unwrap();
            let body = document.nth(2).unwrap();
            let foo = document.nth(3).unwrap();
            let bar = document.nth(4).unwrap();
            let baz = document.nth(5).unwrap();
            let quux = document.nth(6).unwrap();
            let comment = document.nth(7).unwrap();
        }

        test "Node::name()" {
            assert_eq!(html.name(), Some("html"));
            assert_eq!(head.name(), Some("head"));
            assert_eq!(body.name(), Some("body"));
            assert_eq!(foo.name(), None);
            assert_eq!(bar.name(), Some("bar"));
            assert_eq!(baz.name(), None);
            assert_eq!(quux.name(), Some("quux"));
        }

        test "Node::attr()" {
            assert_eq!(html.attr("id"), None);
            assert_eq!(head.attr("id"), None);
            assert_eq!(body.attr("id"), Some("something"));
            assert_eq!(body.attr("class"), None);
            assert_eq!(foo.attr("id"), None);
            assert_eq!(bar.attr("id"), None);
            assert_eq!(baz.attr("id"), None);
            assert_eq!(quux.attr("id"), None);
            assert_eq!(quux.attr("class"), Some("another-thing"));
        }

        test "Node::parent()" {
            assert_eq!(html.parent(), None);
            assert_eq!(head.parent(), Some(html));
            assert_eq!(body.parent(), Some(html));
            assert_eq!(foo.parent(), Some(body));
            assert_eq!(bar.parent(), Some(body));
            assert_eq!(baz.parent(), Some(bar));
            assert_eq!(quux.parent(), Some(bar));
        }

        test "Node::prev() / Node::next()" {
            assert_eq!(html.prev(), None);
            assert_eq!(html.next(), None);
            assert_eq!(head.prev(), None);
            assert_eq!(head.next(), Some(body));
            assert_eq!(body.prev(), Some(head));
            assert_eq!(body.next(), None);
            assert_eq!(foo.prev(), None);
            assert_eq!(foo.next(), Some(bar));
            assert_eq!(bar.prev(), Some(foo));
            assert_eq!(bar.next(), None);
            assert_eq!(baz.prev(), None);
            assert_eq!(baz.next(), Some(quux));
            assert_eq!(quux.prev(), Some(baz));
            assert_eq!(quux.next(), None);
        }

        test "Node::text()" {
            assert_eq!(html.text(), "foobaz");
            assert_eq!(head.text(), "");
            assert_eq!(body.text(), "foobaz");
            assert_eq!(foo.text(), "foo");
            assert_eq!(bar.text(), "baz");
            assert_eq!(baz.text(), "baz");
            assert_eq!(quux.text(), "");
        }

        test "Node::find()" {
            {
                use select::predicate::*;
                let document = Document::from(include_str!("fixtures/struct.Vec.html"));
                let main = document.find(Attr("id", "main"));
                let main = main.iter().next().unwrap();

                assert_eq!(main.find(Name("span")).iter().count(), 1785);
                assert_eq!(main.find(Name("div")).iter().count(), 204);
            };
        }

        test "Node::is()" {
            {
                use select::predicate::*;
                let document = Document::from(include_str!("fixtures/struct.Vec.html"));
                for div in &document.find(Name("div")) {
                    assert!(div.is(Name("div")));
                }
            };
        }

        test "Node::as_text()" {
            assert_eq!(foo.as_text(), Some("foo"));
            assert_eq!(bar.as_text(), None);
            assert_eq!(baz.as_text(), Some("baz"));
        }

        test "Node::as_comment()" {
            assert_eq!(foo.as_comment(), None);
            assert_eq!(comment.as_comment(), Some("comment"));
        }

        test "Node::html()" {
            assert_eq!(html.html(), "<html><head></head><body id=\"something\">\
                                     foo<bar>baz<quux class=\"another-thing\">\
                                     <!--comment--></quux></bar></body></html>");
            assert_eq!(head.html(), "<head></head>");
            assert_eq!(foo.html(), "foo");
            assert_eq!(quux.html(), "<quux class=\"another-thing\"><!--comment--></quux>");
            assert_eq!(comment.html(), "<!--comment-->");

            let document = Document::from("<div a=b c=d e=f g=h i=j>");
            let div = document.nth(3).unwrap();
            assert_eq!(div.name(), Some("div"));
            assert_eq!(div.html(), r#"<div a="b" c="d" e="f" g="h" i="j"></div>"#);
        }

        test "Node::inner_html()" {
            assert_eq!(html.inner_html(), "<head></head><body id=\"something\">\
                                           foo<bar>baz<quux class=\"another-thing\">\
                                           <!--comment--></quux></bar></body>");
            assert_eq!(head.inner_html(), "");
            assert_eq!(foo.inner_html(), "");
            assert_eq!(quux.inner_html(), "<!--comment-->");
            assert_eq!(comment.inner_html(), "");
        }

        test "Node::children()" {
            let children = html.children();
            let mut children = children.iter();
            assert_eq!(children.next().unwrap().name(), Some("head"));
            assert_eq!(children.next().unwrap().name(), Some("body"));
            assert_eq!(children.next(), None);

            assert_eq!(body.children().iter().count(), 2);

            assert_eq!(baz.children().iter().count(), 0);

            assert_eq!(quux.children().iter().count(), 1);
        }
    }
}
