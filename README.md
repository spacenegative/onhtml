# html dsl

## DISCLAIMER
This is an html dsl library for rust, not an html temlate. Not complete but easily extensible.

## Rational
The case for an html dsl instead of an html template is best described in this <a href="https://codeburst.io/80-of-my-coding-is-doing-this-or-why-templates-are-dead-b640fc149e22">post</a>.

## Examle usage:

```
use	html::*
#[get("/ads/jobs/login/")] 
pub async fn get_ads_jobs_login() ->impl Responder  {
	let mut x = input("") .placeholder("ΝΟΥΜΕΡΟ ΑΓΓΕΛΙΑΣ") .name("key") .n()  ;
	x += &input("") .placeholder("ΚΩΔΙΚΟΣ ΑΓΓΕΛΙΑΣ") .name("pass") .n()  ;
	x += &button("ok") .value("ok") .n()  ;
	x = form(&x) .method(Method::Post) .action("auth/") .n()  ;
	x = div(&x) .n()  ;
	let mut h =  Html  {
		title: "Nailartist".to_string() , 
		desc: "".to_string() , 
		kws: vec![] , 
		theme: "#111111".to_string() , 
		css: vec!["/res/login.css".to_string()] , 
		//js: vec![(JAVASCRIPT::Module,"/res/submit.js".to_string())] , 
		js: vec![] , 
		favicon: "/res/img/favicon.svg".to_string() , 
		scale: 3 , 
		content: x.to_string(), }
	; 
	return HttpResponse::Ok() .body(h .print())  ;}
```

