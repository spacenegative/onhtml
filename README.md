# html dsl

## DISCLAIMER
this is an html dsl library for rust, not an html template. not complete but easily extensible.

## rational
the case for an html dsl instead of an html template is best described in this [here](https://codeburst.io/80-of-my-coding-is-doing-this-or-why-templates-are-dead-b640fc149e22)

## example usage:

```
use	html::*
use	html::JAVASCRIPT::*

fn test() ->String  {
	let mut x = input("") .placeholder("something") .name("key") .n()  ;
	x += &input("") .placeholder("something") .name("pass") .n()  ;
	x += &button("ok") .value("ok") .n()  ;
	x = form(&x) .method(Method::Post) .action("auth/") .n()  ;
	x = div(&x) .class("something") .id("something") .data("something" ,"something") .n()  ;

	let mut h = Html::new(&x);
	h.version = VERSION::Number("v1.0.0");
	h.title = "my title" .to_string();
	h.desc = "my description" .to_string();
	h.css = vec!("/index.css" .to_string());
	h.js = vec!(Module("/index.js" .to_string()));
	h.favicon = "/img/favicon.svg" .to_string();
	h .print()
}
```
