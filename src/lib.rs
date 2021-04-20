#![allow(non_snake_case)] 
use duplicate::duplicate  ;
use std::io::prelude::*  ;
use std::time::  {
	SystemTime, 
	UNIX_EPOCH, }
; 
//ENUMS  ;
pub enum TEXTAREAWRAP  {
	Hard, Soft, }
pub enum TYPE  {
	Button, Checkbox, Color , Date, DatetimeLocal, Email, File, Hidden, Image, Month, Number,  Password, Radio, Range, Reset, Search, Submit, Tel, Text, Time, Url, Week, }
pub enum PRELOAD  {
	Auto, Metadata, None, }
pub enum ENCTYPE  {
	Application, Multipart, Text, }
pub enum METHOD  {
	Get, Post, Patch, }
pub enum TARGET  {
	Blank, Self_, Parent, Top, }
pub enum FORMREL  {
	External, Help, License, Next, Nofollow, Noopener, Noreferrer, Opener, Prev, Search, }
pub enum AREL  {
	Alternate ,Author ,Bookmark ,External ,Help ,License ,Next ,Nofollow ,Noreferrer ,Noopener  ,Prev ,Search ,Tag , }
pub enum REFPOLICY  {
	NoReferrer ,NoReferrerWhenDowngrade ,Origin ,OriginWhenCrossOrigin ,UnsafeUrl , }
pub enum LISTTYPE  {
	N ,A ,Aa ,I ,Ii , }
pub enum MARQUEEBEHAVIOR  {
	Scroll, Slide, Alternate, }
pub enum MARQUEEDIRECTION  {
	Left, Right, Up, Down, }
struct Tag  {
	pub name: String , 
	pub atts: Vec<(String,String)> , 
	pub val: String , }
impl Tag  {
	fn new( name:String , val:String )->Tag  {
		return Tag  {
			name , 
			atts:  vec![] , 
			val , }}
	fn att( &mut self , name:String , val:String ) -> &mut Self  {
		self.atts.push( ( name , val ) )  ;
		return self  ;}}
pub struct Html  {
	pub version: VERSION, 
	pub title: String, 
	pub desc: String, 
	pub keywords: Vec<String>, 
	pub content: String, 
	pub theme: String, 
	pub scale: u32, 
	pub css: Vec<String>, 
	pub js: Vec<JAVASCRIPT>, 
	pub favicon: String, 
	pub ogimage: (String,String), }
pub enum JAVASCRIPT  {
	Module(String), 
	Script(String), }
#[derive(Debug ,Clone)] pub enum VERSION  {
	Auto ,Number(String) , }
impl Html  {
	pub fn new(x:&str) ->Html  {
		return Html  {
			version: VERSION::Number("0".to_string()), 
			title: "".to_string(), 
			desc: "".to_string(), 
			keywords: vec!(), 
			content: x.to_string(), 
			theme: "#111111".to_string(), 
			scale: 3, 
			css: vec!(), 
			js: vec!(), 
			favicon: "".to_string(), 
			ogimage: ("".to_string(),"".to_string()), }}
	fn versioning( &mut self ) -> &mut Html  {
		//[1]  ;
		let ver:String  ;
		match &self.version  {
			VERSION::Auto =>  {
				let t = SystemTime::now().duration_since( UNIX_EPOCH )    .expect( "ERR: Time is before Unix Epoch" ).as_millis()  ;
				ver = format!( "?v={}" ,t)  ;}
			VERSION::Number(n) =>  {
				ver = format!( "?v={}" ,n)  ;}}
		for link in &mut self.css  {
			link.push_str( &ver )  ;}
		for script in &mut self.js  {
			match script  {
				JAVASCRIPT::Module(s) =>  {
					JAVASCRIPT::Module(format!("{}{}" ,s ,&ver))  ;}
				JAVASCRIPT::Script(s) =>  {
					JAVASCRIPT::Script(format!("{}{}" ,s ,&ver))  ;}}}
		self.favicon.push_str( &ver )  ;
		return self  ;}
	pub fn print( &mut self ) -> String  {
		let html = self.versioning()  ;
		let mut x = format!( "<title>{}</title>\n" , html.title )  ;
		x.push_str( "<meta http-equiv='Content-Type' content='text/html; charset=utf-8'>\n" )  ;
		x.push_str( &format!( "<meta name='description' content='{}'>\n" , html.desc ) )  ;
		if html.ogimage.0 != "".to_string()  {
			x.push_str( &format!( "<meta property='og:image' content='{}'>\n" , html.ogimage.0 ) )  ;
			x.push_str( &format!( "<meta property='og:image:alt' content='{}'>\n" , html.ogimage.1 ) )  ;}
		if html.keywords.len() > 0  {
			let mut kws = "".to_string()  ;
			for k in &html.keywords  {
				kws += &format!( "{}, " , k )  ;}
			let kws = kws[..kws.len()-2].to_string()  ;
			x.push_str( &format!( "<meta name='keywords' content='{}'>\n" , kws ) )  ;}
		x.push_str( &format!( "<meta name='viewport' content='width=device-width, initial-scale=1,   maximum-scale={}'>\n" , html.scale ) )  ;
		x.push_str( &format!( "<meta name='theme-color' content='{}'>\n" , html.theme ) )  ;
		for i in &html.css  {
			x.push_str( &format!( "<link rel='stylesheet' href='{}' />\n" , i ) )  ;}
		x.push_str( &format!( "<link rel='shortcut icon' href='{}' />\n" , &html.favicon ) )  ;
		x = format!( "<!DOCTYPE html>\n<html>\n{}\n<body>\n{}\n</body>\n" , x , html.content )  ;
		for i in &html.js  {
			match i  {
				JAVASCRIPT::Script(s) =>  {
					x.push_str(&format!("<script src='{}'></script>\n" ,s))  ;}
				JAVASCRIPT::Module(s) =>  {
					x.push_str(&format!("<script type='module'      src='{}'></script>\n" ,s))  ;}}}
		x.push_str( "</html>" )  ;
		return x  ;}
	pub fn write( &mut self , file:&str )  {
		let p = std::path::Path::new( file )  ;
		let mut f = std::fs::File::create( &p )  .expect( format!( "ERR: COULD NOT CREATE FILE {}" , file ).as_str() )  ;
		let h = self.print()  ;
		f.write_all( h.as_bytes())  .expect( format!("ERR: COULD NOT WRITE TO {}",p.display()).as_str() )  ;}}
//[1]If version is Auto,the css/js version will be generated on every reload.I want this when developing.  ;
//T  ;
#[duplicate(T; [MARQUEE]; [Q]; [MAINN]; [NAV]; [FOOTER]; [HEADER]; [B]; [LABEL]; [DATALIST]; [CODE]; [ARTICLE]; [ASIDE]; [SECTION]; [P]; [OL]; [UL]; [LI]; [OPTION]; [PRE]; [CANVAS]; [BLOCKQUOTE]; [SOURCE]; [SPAN]; [A]; [FORM]; [TEMPLATE]; [VIDEO]; [TEXTAREA]; [SELECT]; [H1]; [H2]; [H3]; [H4]; [H5]; [H6]; [BUTTON]; [DIV]; [IMG]; [INPUT] )] 
pub struct T( Tag ) ; 
#[duplicate(f name T ; [marquee]["marquee"][MARQUEE]; [q]["q"][Q]; [nav]["nav"][NAV]; [footer]["footer"][FOOTER]; [header]["header"][HEADER]; [b]["b"][B]; [label]["label"][LABEL]; [datalist]["datalist"][DATALIST]; [code]["code"][CODE]; [article]["article"][ARTICLE]; [aside]["aside"][ASIDE]; [section]["section"][SECTION]; [p]["p"][P]; [ol]["ol"][OL]; [ul]["ul"][UL]; [li]["li"][LI]; [option]["option"][OPTION]; [pre]["pre"][PRE]; [canvas]["canvas"][CANVAS]; [blockquote]["blockquote"][BLOCKQUOTE]; [source]["source"][SOURCE]; [span]["span"][SPAN]; [a]["a"][A]; [form]["form"][FORM]; [template]["template"][TEMPLATE]; [video]["video"][VIDEO]; [textarea]["textarea"][TEXTAREA]; [select]["select"][SELECT]; [h1]["h1"][H1]; [h2]["h2"][H2]; [h3]["h3"][H3]; [h4]["h4"][H4]; [h5]["h5"][H5]; [h6]["h6"][H6]; [button]["button"][BUTTON]; [div]["div"][DIV]; [img]["img"][IMG]; [input]["input"][INPUT] )] 
pub fn f(val:&str) ->T  {
	let t = Tag::new( name.to_string() , val.to_string() )  ;
	return T( t )  ;}
pub    fn mainn(val:&str) -> MAINN  {
	let t = Tag::new("main".to_string() ,val.to_string())  ;
	return MAINN(t)  ;}
#[duplicate(T; [MARQUEE]; [Q]; [MAINN]; [NAV]; [FOOTER]; [HEADER]; [B]; [LABEL]; [DATALIST]; [CODE]; [ARTICLE]; [ASIDE]; [SECTION]; [P]; [OL]; [UL]; [LI]; [OPTION]; [PRE]; [CANVAS]; [BLOCKQUOTE]; [SOURCE]; [SPAN]; [A]; [FORM]; [TEMPLATE]; [VIDEO]; [TEXTAREA]; [SELECT]; [H1]; [H2]; [H3]; [H4]; [H5]; [H6]; [BUTTON]; [DIV]; [IMG]; [INPUT] )] 
impl T  {
	pub fn id( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "id".to_string() , val.to_string() )  ;
		return self  ;}
	pub fn class( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "class".to_string() , val.to_string() )  ;
		return self  ;}
	pub fn onclick( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "onclick".to_string() , val.to_string() )  ;
		return self  ;}
	pub fn onload( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "onload".to_string() , val.to_string() )  ;
		return self  ;}
	pub fn data( &mut self , key:&str , val:&str ) -> &mut Self  {
		self.0.att( format!( "data-{}",key ).to_string() , val.to_string() )  ;
		return self  ;}
	pub fn contenteditable( &mut self ) -> &mut Self  {
		self.0.att( "contenteditable".to_string() , "true".to_string() )  ;
		return self  ;}
	pub fn tabindex( &mut self ,n:i32 ) -> &mut Self  {
		self.0.att( "tabindex".to_string() , format!("{}" ,n) )  ;
		return self  ;}
	pub fn autocorrect( &mut self ,b:bool) -> &mut Self  {
		if b  {
			self.0.att( "autocorrect".to_string() , "on".to_string() )  ;}
		else  {
			self.0.att( "autocorrect".to_string() , "off".to_string() )  ;}
		return self  ;}
	pub fn autocapitalize( &mut self ,b:bool) -> &mut Self  {
		if b  {
			self.0.att( "autocapitalize".to_string() , "on".to_string() )  ;}
		else  {
			self.0.att( "autocapitalize".to_string() , "off".to_string() )  ;}
		return self  ;}
	pub fn spellcheck( &mut self ,b:bool) -> &mut Self  {
		if b  {
			self.0.att( "spellcheck".to_string() , "true".to_string() )  ;}
		else  {
			self.0.att( "spellcheck".to_string() , "false".to_string() )  ;}
		return self  ;}
	pub fn noAutos( &mut self ) -> &mut Self  {
		self  .autocorrect(false)  .autocapitalize(false)  .spellcheck(false)  ;
		return self  ;}
	pub fn n( &mut self ) -> String  {
		let mut x = format!( "\n<{} " , self.0.name )  ;
		for att in &self.0.atts  {
			if att.1 == ""  {
				x += &format!( "{} " , &att.0 )  ;}
			else  {
				x += &format!( "{}='{}' " , &att.0 , &att.1 )  ;}}
		let val = format!( "\n{}\n" , &self.0.val )  ;
		let val = unlines( val.lines().map( tab ).collect() )  ;
		x += &format!( ">{}</{}>" , &val , &self.0.name )  ;
		return format!( "{}" , x )  ;}}
impl MARQUEE  {
	pub fn behavior(&mut self ,x:MARQUEEBEHAVIOR) -> &mut Self  {
		match x  {
			MARQUEEBEHAVIOR::Scroll =>  {
				self.0.att("bahavior".to_string() ,"scroll".to_string())  ;
				return self  ;}
			MARQUEEBEHAVIOR::Slide =>  {
				self.0.att("bahavior".to_string() ,"slide".to_string())  ;
				return self  ;}
			MARQUEEBEHAVIOR::Alternate =>  {
				self.0.att("bahavior".to_string() ,"alternate".to_string())  ;
				return self  ;}}}
	pub fn direction(&mut self ,x:MARQUEEDIRECTION) -> &mut Self  {
		match x  {
			MARQUEEDIRECTION::Left =>  {
				self.0.att("direction".to_string() ,"left".to_string())  ;
				return self  ;}
			MARQUEEDIRECTION::Right =>  {
				self.0.att("direction".to_string() ,"right".to_string())  ;
				return self  ;}
			MARQUEEDIRECTION::Up =>  {
				self.0.att("direction".to_string() ,"up".to_string())  ;
				return self  ;}
			MARQUEEDIRECTION::Down =>  {
				self.0.att("direction".to_string() ,"down".to_string())  ;
				return self  ;}}}
	pub fn loop_(&mut self ,x:u32) -> &mut Self  {
		self.0.att("loop".to_string() ,format!("{}" ,&x))  ;
		return self  ;}
	pub fn scrollamount(&mut self ,x:u32) -> &mut Self  {
		//  bigger = faster  ;
		self.0.att("scrollamount".to_string() ,format!("{}" ,&x))  ;
		return self  ;}
	pub fn hspace(&mut self ,x:&str) -> &mut Self  {
		//  bigger = faster  ;
		self.0.att("hspace".to_string() ,x.to_string())  ;
		return self  ;}}
impl LI  {
	pub fn value( &mut self ,n:u32) -> &mut Self  {
		self.0.att( "value".to_string() , format!("{}" ,n))  ;
		return self  ;}}
impl BLOCKQUOTE  {
	pub fn cite(&mut self ,x:&str) -> &mut Self  {
		self.0.att( "cite".to_string() , format!("{}" ,x))  ;
		return self  ;}}
//cite accepts a url  ;
impl LABEL  {
	pub fn for_( &mut self ,x:&str) -> &mut Self  {
		self.0.att( "for".to_string() , format!("{}" ,x))  ;
		return self  ;}}
//Only for <ol> lists.Specifies the start value of a list item.The following list items will increment from that number  ;
impl PRE  {
	pub fn oninput( &mut self ,f:&str) -> &mut Self  {
		self.0.att( "oninput".to_string() , format!("{}" ,f))  ;
		return self  ;}
	pub fn onchange( &mut self ,f:&str) -> &mut Self  {
		self.0.att( "onchange".to_string() , format!("{}" ,f))  ;
		return self  ;}
	pub fn wrap( &mut self ) -> &mut Self  {
		self.0.att( "wrap".to_string() , "".to_string())  ;
		return self  ;}}
impl OL  {
	pub fn reversed( &mut self ) -> &mut Self  {
		self.0.att( "reversed".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn start( &mut self ,n:u32) -> &mut Self  {
		// Specifies the start value of an ordered list  ;
		self.0.att( "start".to_string() , format!("{}" ,n) )  ;
		return self  ;}
	pub fn type_( &mut self ,t:LISTTYPE) -> &mut Self  {
		match t  {
			LISTTYPE::N =>  {
				self.0.att( "type".to_string() ,"1".to_string() )  ;
				return self  ;}
			LISTTYPE::A =>  {
				self.0.att( "type".to_string() ,"A".to_string() )  ;
				return self  ;}
			LISTTYPE::Aa =>  {
				self.0.att( "type".to_string() ,"a".to_string() )  ;
				return self  ;}
			LISTTYPE::I =>  {
				self.0.att( "type".to_string() ,"I".to_string() )  ;
				return self  ;}
			LISTTYPE::Ii =>  {
				self.0.att( "type".to_string() ,"i".to_string() )  ;
				return self  ;}}}}
impl A  {
	pub fn download( &mut self ) -> &mut Self  {
		self.0.att( "download".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn href( &mut self , url:&str ) -> &mut Self  {
		self.0.att( "href".to_string() , url.to_string() )  ;
		return self  ;}
	pub fn type_( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "type".to_string() , x.to_string() )  ;
		return self  ;}
	pub fn hreflang( &mut self , two_digit_code:&str ) -> &mut Self  {
		self.0.att( "hreflang".to_string() , two_digit_code.to_string() )  ;
		return self  ;}
	pub fn ping( &mut self , urls:&str ) -> &mut Self  {
		// space separated urls  ;
		self.0.att( "hreflang".to_string() , urls.to_string() )  ;
		return self  ;}
	pub fn target( &mut self , x:TARGET ) -> &mut Self  {
		match x  {
			TARGET::Blank =>  {
				self.0.att( "target".to_string() , "blank".to_string() )  ;
				return self  ;}
			TARGET::Parent =>  {
				self.0.att( "target".to_string() , "parent".to_string() )  ;
				return self  ;}
			TARGET::Self_ =>  {
				self.0.att( "target".to_string() , "self".to_string() )  ;
				return self  ;}
			TARGET::Top =>  {
				self.0.att( "target".to_string() , "top".to_string() )  ;
				return self  ;}}}
	pub fn rel( &mut self , x:AREL ) -> &mut Self  {
		match x  {
			AREL::Alternate =>  {
				self.0.att( "rel".to_string() , "alternate".to_string() )  ;
				return self  ;}
			AREL::Author =>  {
				self.0.att( "rel".to_string() , "author".to_string() )  ;
				return self  ;}
			AREL::Bookmark =>  {
				self.0.att( "rel".to_string() , "bookmark".to_string() )  ;
				return self  ;}
			AREL::External =>  {
				self.0.att( "rel".to_string() , "external".to_string() )  ;
				return self  ;}
			AREL::Help =>  {
				self.0.att( "rel".to_string() , "help".to_string() )  ;
				return self  ;}
			AREL::License =>  {
				self.0.att( "rel".to_string() , "license".to_string() )  ;
				return self  ;}
			AREL::Next =>  {
				self.0.att( "rel".to_string() , "next".to_string() )  ;
				return self  ;}
			AREL::Nofollow =>  {
				self.0.att( "rel".to_string() , "nofollow".to_string() )  ;
				return self  ;}
			AREL::Noreferrer =>  {
				self.0.att( "rel".to_string() , "noreferrer".to_string() )  ;
				return self  ;}
			AREL::Noopener =>  {
				self.0.att( "rel".to_string() , "noopener".to_string() )  ;
				return self  ;}
			AREL::Prev =>  {
				self.0.att( "rel".to_string() , "prev".to_string() )  ;
				return self  ;}
			AREL::Search =>  {
				self.0.att( "rel".to_string() , "search".to_string() )  ;
				return self  ;}
			AREL::Tag =>  {
				self.0.att( "rel".to_string() , "tag".to_string() )  ;
				return self  ;}}}
	pub fn novalidate( &mut self ) -> &mut Self  {
		self.0.att( "novalidate".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn name( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "name".to_string() , x.to_string() )  ;
		return self  ;}
	pub fn action( &mut self , url:&str ) -> &mut Self  {
		self.0.att( "action".to_string() , url.to_string() )  ;
		return self  ;}
	pub fn p( &mut self , x:METHOD ) -> &mut Self  {
		match x  {
			METHOD::Get =>  {
				self.0.att( "method".to_string() , "get".to_string() )  ;
				return self  ;}
			METHOD::Post =>  {
				self.0.att( "method".to_string() , "post".to_string() )  ;
				return self  ;}
			METHOD::Patch =>  {
				self.0.att( "method".to_string() , "patch".to_string() )  ;
				return self  ;}}}
	pub fn enctype( &mut self , x:ENCTYPE ) -> &mut Self  {
		match x  {
			ENCTYPE::Application =>  {
				self.0.att( "enctype".to_string() , "application/x-www-form-urlencoded".to_string() )  ;
				return self  ;}
			ENCTYPE::Multipart =>  {
				self.0.att( "enctype".to_string() , "multipart/form-data".to_string() )  ;
				return self  ;}
			ENCTYPE::Text =>  {
				self.0.att( "enctype".to_string() , "text/plain".to_string() )  ;
				return self  ;}}}
	pub fn autocomplete( &mut self , on:bool ) -> &mut Self  {
		if on  {
			self.0.att( "autocomplete".to_string() , "on".to_string() )  ;}
		else  {
			self.0.att( "autocomplete".to_string() , "off".to_string() )  ;}
		return self  ;}}
impl FORM  {
	pub fn target( &mut self , x:TARGET ) -> &mut Self  {
		match x  {
			TARGET::Blank =>  {
				self.0.att( "rel".to_string() , "_blank".to_string() )  ;
				return self  ;}
			TARGET::Self_ =>  {
				self.0.att( "rel".to_string() , "_self".to_string() )  ;
				return self  ;}
			TARGET::Parent =>  {
				self.0.att( "rel".to_string() , "_parent".to_string() )  ;
				return self  ;}
			TARGET::Top =>  {
				self.0.att( "rel".to_string() , "_top".to_string() )  ;
				return self  ;}}}
	pub fn rel( &mut self , x:FORMREL ) -> &mut Self  {
		match x  {
			FORMREL::External =>  {
				self.0.att( "rel".to_string() , "external".to_string() )  ;
				return self  ;}
			FORMREL::Help =>  {
				self.0.att( "rel".to_string() , "help".to_string() )  ;
				return self  ;}
			FORMREL::License =>  {
				self.0.att( "rel".to_string() , "license".to_string() )  ;
				return self  ;}
			FORMREL::Next =>  {
				self.0.att( "rel".to_string() , "next".to_string() )  ;
				return self  ;}
			FORMREL::Nofollow =>  {
				self.0.att( "rel".to_string() , "nofollow".to_string() )  ;
				return self  ;}
			FORMREL::Noopener =>  {
				self.0.att( "rel".to_string() , "noopener".to_string() )  ;
				return self  ;}
			FORMREL::Noreferrer =>  {
				self.0.att( "rel".to_string() , "noreferrer".to_string() )  ;
				return self  ;}
			FORMREL::Opener =>  {
				self.0.att( "rel".to_string() , "opener".to_string() )  ;
				return self  ;}
			FORMREL::Prev =>  {
				self.0.att( "rel".to_string() , "prev".to_string() )  ;
				return self  ;}
			FORMREL::Search =>  {
				self.0.att( "rel".to_string() , "search".to_string() )  ;
				return self  ;}}}
	pub fn novalidate( &mut self ) -> &mut Self  {
		self.0.att( "novalidate".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn name( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "name".to_string() , x.to_string() )  ;
		return self  ;}
	pub fn onsubmit( &mut self ,f:&str ) -> &mut Self  {
		self.0.att( "onsubmit".to_string() , f.to_string() )  ;
		return self  ;}
	pub fn action( &mut self , url:&str ) -> &mut Self  {
		self.0.att( "action".to_string() , url.to_string() )  ;
		return self  ;}
	pub fn method( &mut self , x:METHOD ) -> &mut Self  {
		match x  {
			METHOD::Get =>  {
				self.0.att( "method".to_string() , "get".to_string() )  ;
				return self  ;}
			METHOD::Post =>  {
				self.0.att( "method".to_string() , "post".to_string() )  ;
				return self  ;}
			METHOD::Patch =>  {
				self.0.att( "method".to_string() , "patch".to_string() )  ;
				return self  ;}}}
	pub fn enctype( &mut self , x:ENCTYPE ) -> &mut Self  {
		match x  {
			ENCTYPE::Application =>  {
				self.0.att( "enctype".to_string() , "application/x-www-form-urlencoded".to_string() )  ;
				return self  ;}
			ENCTYPE::Multipart =>  {
				self.0.att( "enctype".to_string() , "multipart/form-data".to_string() )  ;
				return self  ;}
			ENCTYPE::Text =>  {
				self.0.att( "enctype".to_string() , "text/plain".to_string() )  ;
				return self  ;}}}
	pub fn autocomplete( &mut self , on:bool ) -> &mut Self  {
		if on  {
			self.0.att( "autocomplete".to_string() , "on".to_string() )  ;}
		else  {
			self.0.att( "autocomplete".to_string() , "off".to_string() )  ;}
		return self  ;}}
impl IMG  {
	pub fn src( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "src".to_string() , val.to_string() )  ;
		return self  ;}}
impl SOURCE  {
	pub fn src( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "src".to_string() , val.to_string() )  ;
		return self  ;}}
impl VIDEO  {
	pub fn preload( &mut self , x:PRELOAD) -> &mut Self  {
		match x  {
			PRELOAD::Auto =>  {
				self.0.att( "preload".to_string() , "auto".to_string() )  ;
				return self  ;}
			PRELOAD::Metadata =>  {
				self.0.att( "preload".to_string() , "metadata".to_string() )  ;
				return self  ;}
			PRELOAD::None =>  {
				self.0.att( "preload".to_string() , "none".to_string() )  ;
				return self  ;}}}
	pub fn autoplay( &mut self ) -> &mut Self  {
		self.0.att( "autoplay".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn controls( &mut self ) -> &mut Self  {
		self.0.att( "controls".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn loop_( &mut self ) -> &mut Self  {
		self.0.att( "loop".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn muted( &mut self ) -> &mut Self  {
		self.0.att( "muted".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn poster( &mut self ,url:&str ) -> &mut Self  {
		self.0.att( "poster".to_string() , url.to_string() )  ;
		return self  ;}
	pub fn src( &mut self ,url:&str ) -> &mut Self  {
		self.0.att( "src".to_string() , url.to_string() )  ;
		return self  ;}
	pub fn width( &mut self , x:i32 ) -> &mut Self  {
		self.0.att( "width".to_string() , x.to_string() )  ;
		return self  ;}
	pub fn height( &mut self , x:i32 ) -> &mut Self  {
		self.0.att( "height".to_string() , x.to_string() )  ;
		return self  ;}}
impl SELECT  {
	pub fn autofocus( &mut self ) -> &mut Self  {
		self.0.att( "autofocus".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn disabled( &mut self ) -> &mut Self  {
		self.0.att( "disabled".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn required( &mut self ) -> &mut Self  {
		self.0.att( "required".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn name( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "name".to_string() , x.to_string() )  ;
		return self  ;}
	pub fn size( &mut self , x:i32 ) -> &mut Self  {
		self.0.att( "size".to_string() , format!( "{}",x ) )  ;
		return self  ;}
	pub fn onchange( &mut self ,f:&str ) -> &mut Self  {
		self.0.att( "onchange".to_string() , f.to_string() )  ;
		return self  ;}}
impl OPTION  {
	pub fn disabled( &mut self ) -> &mut Self  {
		self.0.att( "disabled".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn selected( &mut self ) -> &mut Self  {
		self.0.att( "selected".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn hidden( &mut self ) -> &mut Self  {
		self.0.att( "hidden".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn value( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "value".to_string() , x.to_string() )  ;
		return self  ;}}
impl BUTTON  {
	pub fn formaction( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "formaction".to_string() , format!("{}" , val.to_string() ) )  ;
		return self  ;}
	pub fn autofocus( &mut self ) -> &mut Self  {
		self.0.att( "autofocus".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn disabled( &mut self ) -> &mut Self  {
		self.0.att( "disabled".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn name( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "name".to_string() , x.to_string() )  ;
		return self  ;}
	pub fn value( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "value".to_string() , x.to_string() )  ;
		return self  ;}}
impl INPUT  {
	pub fn multiple(&mut self) -> &mut Self  {
		self.0.att( "multiple".to_string() ,"".to_string() )  ;
		return self  ;}
	pub fn minlenght( &mut self , val:i32 ) -> &mut Self  {
		self.0.att( "minlenght".to_string() , format!("{}" , val.to_string() ) )  ;
		return self  ;}
	pub fn value( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "value".to_string() , format!("{}" , val.to_string() ) )  ;
		return self  ;}
	pub fn formaction( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "formaction".to_string() , format!("{}" , val.to_string() ) )  ;
		return self  ;}
	pub fn title( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "title".to_string() , format!("{}" , val.to_string() ) )  ;
		return self  ;}
	pub fn size( &mut self , val:i32 ) -> &mut Self  {
		self.0.att( "size".to_string() , format!("{}" , val.to_string() ) )  ;
		return self  ;}
	pub fn max( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "max".to_string() , val.to_string() )  ;
		return self  ;}
	pub fn list( &mut self , id:&str ) -> &mut Self  {
		// id of the datalist tag (input autocompletion)  ;
		self.0.att( "list".to_string() , id.to_string() )  ;
		return self  ;}
	pub fn name( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "name".to_string() , val.to_string() )  ;
		return self  ;}
	pub fn placeholder( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "placeholder".to_string() , val.to_string() )  ;
		return self  ;}
	pub fn readonly( &mut self ) -> &mut Self  {
		self.0.att( "readonly".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn min( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "min".to_string() , val.to_string() )  ;
		return self  ;}
	pub fn checked( &mut self ) -> &mut Self  {
		self.0.att( "checked".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn autofocus( &mut self ) -> &mut Self  {
		self.0.att( "autofocus".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn disabled( &mut self ) -> &mut Self  {
		self.0.att( "disabled".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn autocomplete( &mut self , val:bool ) -> &mut Self  {
		if val  {
			self.0.att( "autocomplete".to_string() , "on".to_string() )  ;}
		else  {
			self.0.att( "autocomplete".to_string() , "off".to_string() )  ;}
		return self  ;}
	pub fn alt( &mut self , val:&str ) -> &mut Self  {
		let mut b = false  ;
		for a in &self.0.atts  {
			if a.0 == "type" && a.1 == "image"  {
				b = true  ;}}
		self.0.att( "accept".to_string() , val.to_string() )  ;
		if !b  {
			panic!( "\n\nTHE 'alt' ATT IS ONLY VALID FOR ATT: type='image'\n\n" )  ;}
		return self  ;}
	pub fn accept( &mut self , val:&str ) -> &mut Self  {
		let mut b = false  ;
		for a in &self.0.atts  {
			if a.0 == "type" && a.1 == "text" || a.0 == "type" && a.1 == "file"  {
				b = true  ;}}
		self.0.att( "accept".to_string() , val.to_string() )  ;
		if !b  {
			panic!("\n\nTHE 'accept' ATT IS ONLY VALID FOR ATT: type='text||file'\n\n")  ;}
		return self  ;}
	pub fn type_( &mut self , val:TYPE) -> &mut Self  {
		match val  {
			TYPE::Button =>  {
				self.0.att( "type".to_string() , "button".to_string() )  ;
				return self  ;}
			TYPE::Checkbox =>  {
				self.0.att( "type".to_string() , "checkbox".to_string() )  ;
				return self  ;}
			TYPE::Color =>  {
				self.0.att( "type".to_string() , "color".to_string() )  ;
				return self  ;}
			TYPE::Date =>  {
				self.0.att( "type".to_string() , "date".to_string() )  ;
				return self  ;}
			TYPE::DatetimeLocal =>  {
				self.0.att( "type".to_string() , "datetimeLocal".to_string() )  ;
				return self  ;}
			TYPE::Email =>  {
				self.0.att( "type".to_string() , "email".to_string() )  ;
				return self  ;}
			TYPE::File =>  {
				self.0.att( "type".to_string() , "file".to_string() )  ;
				return self  ;}
			TYPE::Hidden =>  {
				self.0.att( "type".to_string() , "hidden".to_string() )  ;
				return self  ;}
			TYPE::Image =>  {
				self.0.att( "type".to_string() , "image".to_string() )  ;
				return self  ;}
			TYPE::Month =>  {
				self.0.att( "type".to_string() , "month".to_string() )  ;
				return self  ;}
			TYPE::Number =>  {
				self.0.att( "type".to_string() , "number".to_string() )  ;
				return self  ;}
			TYPE::Password =>  {
				self.0.att( "type".to_string() , "password".to_string() )  ;
				return self  ;}
			TYPE::Radio =>  {
				self.0.att( "type".to_string() , "radio".to_string() )  ;
				return self  ;}
			TYPE::Range =>  {
				self.0.att( "type".to_string() , "range".to_string() )  ;
				return self  ;}
			TYPE::Reset =>  {
				self.0.att( "type".to_string() , "reset".to_string() )  ;
				return self  ;}
			TYPE::Search =>  {
				self.0.att( "type".to_string() , "search".to_string() )  ;
				return self  ;}
			TYPE::Submit =>  {
				self.0.att( "type".to_string() , "submit".to_string() )  ;
				return self  ;}
			TYPE::Tel =>  {
				self.0.att( "type".to_string() , "tel".to_string() )  ;
				return self  ;}
			TYPE::Text =>  {
				self.0.att( "type".to_string() , "text".to_string() )  ;
				return self  ;}
			TYPE::Time =>  {
				self.0.att( "type".to_string() , "time".to_string() )  ;
				return self  ;}
			TYPE::Url =>  {
				self.0.att( "type".to_string() , "url".to_string() )  ;
				return self  ;}
			TYPE::Week =>  {
				self.0.att( "type".to_string() , "week".to_string() )  ;
				return self  ;}}}
	pub fn onchange( &mut self ,f:&str ) -> &mut Self  {
		self.0.att( "onchange".to_string() , f.to_string() )  ;
		return self  ;}
	pub fn oninput( &mut self ,f:&str ) -> &mut Self  {
		self.0.att( "onchange".to_string() , f.to_string() )  ;
		return self  ;}
	pub fn oninvalid( &mut self ,f:&str ) -> &mut Self  {
		self.0.att( "oninvalid".to_string() , f.to_string() )  ;
		return self  ;}
	pub fn required(&mut self) -> &mut Self  {
		self.0.att( "required".to_string() , "".to_string() )  ;
		return self  ;}
	// FICTIONAL ATTRIBUTES  ;
	pub fn required_with(&mut self ,validation_message:&str) -> &mut Self  {
		self.0.att( "required".to_string() , "".to_string() )  ;
		if validation_message != ""  {
			self.0.att("oninvalid".to_string()    ,format!("this.setCustomValidity(\"{}\")" ,validation_message))  ;
			self.0.att("oninput".to_string()    ,"this.setCustomValidity(\"\")".to_string())  ;}
		return self  ;}}
impl TEXTAREA  {
	pub fn autofocus( &mut self ) -> &mut Self  {
		self.0.att( "autofocus".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn disabled( &mut self ) -> &mut Self  {
		self.0.att( "disabled".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn readonly( &mut self ) -> &mut Self  {
		self.0.att( "readonly".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn required( &mut self ) -> &mut Self  {
		self.0.att( "required".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn cols( &mut self , x:i32 ) -> &mut Self  {
		self.0.att( "cols".to_string() , format!( "{}",x ).to_string() )  ;
		return self  ;}
	pub fn rows( &mut self , x:i32 ) -> &mut Self  {
		self.0.att( "rows".to_string() , format!( "{}",x ).to_string() )  ;
		return self  ;}
	pub fn maxlength( &mut self , x:i32 ) -> &mut Self  {
		self.0.att( "maxlength".to_string() , format!( "{}",x ).to_string() )  ;
		return self  ;}
	pub fn name( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "name".to_string() , x.to_string() )  ;
		return self  ;}
	pub fn placeholder( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "placeholder".to_string() , x.to_string() )  ;
		return self  ;}
	pub fn onchange( &mut self ,f:&str ) -> &mut Self  {
		self.0.att( "onchange".to_string() , f.to_string() )  ;
		return self  ;}
	pub fn oninput( &mut self , f:&str ) -> &mut Self  {
		self.0.att( "oninput".to_string() , f.to_string() )  ;
		return self  ;}}
//HELPERS  ;
fn tab( x:&str ) -> String  {
	//let mut y = "\t".to_string()  ;
	let mut y = "".to_string()  ;
	y.push_str( x )  ;
	return y  ;}
fn unlines( ls:Vec<String> ) -> String  {
	let mut x = "".to_string()  ;
	for l in ls  {
		x.push_str( &l )  ;
		x.push_str( "\n" )  ;}
	return x[..x.len()-1].to_string()  ;}
//READY CONTRUCTS  ;
// PAGED  ;
pub fn paged( pages:Vec<String> ) -> String  {
	let mut pagernav = input( "" ) .class( "term" ) .n()  ;
	pagernav += &form( &pagernav ).class( "hidden" ).n()  ;
	let pager = div( "" ).class( "pager" ).n()  ;
	pagernav += &pager  ;
	pagernav += &div( &pagernav ).class( "nav" ).n()  ;
	for page in pages  {
		pagernav += &div( &page ).class( "page" ).n()  ;}
	let paged = div( &pagernav ).class( "paged" ).id( "PAGED" ).n()  ;
	return paged  ;}
// CALENDAR  ;
// needed by js function: calendar  ;
pub fn calendar(id:&str) -> String  {
	let mut y = "".to_string()  ;
	let mut x = "".to_string()  ;
	x += &div("&lt;") .class("CALENDAR_PREV") .n()  ;
	y += &x  ;
	x = span("month") .class("CALENDAR_MONTH") .n()  ;
	x += &span("@") .class("CALENDAR_YEAR") .n()  ;
	x = div(&x) .n()  ;
	y += &x  ;
	y += &div("&gt;") .class("CALENDAR_NEXT") .n()  ;
	y = div(&y) .class("calendar_header") .n()  ;
	//  ;
	x = div("Δε") .class("calendar_day_header") .n()  ;
	x += &div("Τρ") .class("calendar_day_header") .n()  ;
	x += &div("Τε") .class("calendar_day_header") .n()  ;
	x += &div("Πε") .class("calendar_day_header") .n()  ;
	x += &div("Πα") .class("calendar_day_header") .n()  ;
	x += &div("Σα") .class("calendar_day_header") .n()  ;
	x += &div("Κυ") .class("calendar_day_header") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x += &div("00") .class("calendar_day") .data("date","") .n()  ;
	x = div(&x) .class("calendar_grid") .n()  ;
	y += &x  ;
	x = div("") .class("CALENDAR_PRINT_TODAY") .n()  ;
	x += &div("") .class("CALENDAR_PRINT_SELECTED") .n()  ;
	x = div(&x) .class("CALENDAR_PRINT") .n()  ;
	y += &x  ;
	y = div(&y) .id(id) .data("date" ,"") .data("selected" ,"") .n()  ;
	return y  ;}
//REFERENCE  ;
//https://www.w3schools.com/TAGS/default.ASP  ;
