#![allow(non_snake_case)] 
use duplicate::duplicate  ;
use std::io::prelude::*  ;
use std::time::  {
	SystemTime, 
	UNIX_EPOCH, }
; 
//ENUMS  ;
pub enum TextareaWrap  {
	Hard, Soft, }
pub enum Type  {
	Button, Checkbox, Color , Date, DatetimeLocal, Email, File, Hidden, Image, Month, Number,  Password, Radio, Range, Reset, Search, Submit, Tel, Text, Time, Url, Week, }
pub enum Preload  {
	Auto, Metadata, None, }
pub enum Enctype  {
	Application, Multipart, Text, }
pub enum Method  {
	Get, Post, }
pub enum Target  {
	Blank, Self_, Parent, Top, }
pub enum FormRel  {
	External, Help, License, Next, Nofollow, Noopener, Noreferrer, Opener, Prev, Search, }
pub enum ARel  {
	Alternate ,Author ,Bookmark ,External ,Help ,License ,Next ,Nofollow ,Noreferrer ,Noopener  ,Prev ,Search ,Tag , }
pub enum Refpolicy  {
	NoReferrer ,NoReferrerWhenDowngrade ,Origin ,OriginWhenCrossOrigin ,UnsafeUrl , }
pub enum ListType  {
	N ,A ,Aa ,I ,Ii , }
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
	pub title: String, 
	pub desc: String, 
	pub kws: Vec<String>, 
	pub content: String, 
	pub theme: String, 
	pub scale: u32, 
	pub css: Vec<String>, 
	pub js: Vec<(JAVASCRIPT,String)>, 
	pub favicon: String, }
pub enum JAVASCRIPT  {
	Module ,Script , }
impl Html  {
	pub fn version( &mut self ) -> &mut Html  {
		let t = SystemTime::now().duration_since( UNIX_EPOCH )   .expect( "ERR: Time is before Unix Epoch" ).as_millis()  ;
		let ver = format!( "?v={}" , t )  ;
		for link in &mut self.css  {
			link.push_str( &ver )  ;}
		for script in &mut self.js  {
			script .1 .push_str( &ver )  ;}
		self.favicon.push_str( &ver )  ;
		return self  ;}
	pub fn print( &mut self ) -> String  {
		if self.desc.len() > 120  {
			panic!( format!( "\n\n\tHtml.desc IS {} CHARS BUT MUST BE UP TO 120\n\n"    , self.desc.len() ) )  ;}
		if self.title.len() > 60  {
			panic!( format!( "\n\n\tHtml.title IS {} CHARS BUT MUST BE UP TO 60\n\n"    , self.title.len() ) )  ;}
		if self.kws.len() > 10  {
			panic!( format!( "\n\n\tHtml.kws HAS {} KEYWORDS, BUT MUST BE UP TO 10\n\n"    , self.title.len() ) )  ;}
		let html = self.version()  ;
		let mut x = format!( "<title>{}</title>\n" , html.title )  ;
		x.push_str( "<meta http-equiv='Content-Type' content='text/html; charset=utf-8'>\n" )  ;
		x.push_str( &format!( "<meta name='description' content='{}'>\n" , html.desc ) )  ;
		if html.kws.len() > 0  {
			let mut kws = "".to_string()  ;
			for k in &html.kws  {
				kws += &format!( "{}, " , k )  ;}
			let kws = kws[..kws.len()-2].to_string()  ;
			x.push_str( &format!( "<meta name='keywords' content='{}'>\n" , kws ) )  ;}
		x.push_str( &format!( "<meta name='viewport' content='width=device-width, initial-scale=1,    maximum-scale={}'>\n" , html.scale ) )  ;
		x.push_str( &format!( "<meta name='theme-color' content='{}'>\n" , html.theme ) )  ;
		for i in &html.css  {
			x.push_str( &format!( "<link rel='stylesheet' href='{}' />\n" , i ) )  ;}
		x.push_str( &format!( "<link rel='shortcut icon' href='{}' />\n" , &html.favicon ) )  ;
		x = format!( "<!DOCTYPE html>\n<html>\n{}\n<body>\n{}\n</body>\n" , x , html.content )  ;
		for i in &html.js  {
			match i .0  {
				JAVASCRIPT::Script =>  {
					x.push_str(&format!("<script src='{}'></script>\n" ,i .1))  ;}
				JAVASCRIPT::Module =>  {
					x.push_str(&format!("<script type='module'       src='{}'></script>\n" ,i .1))  ;}}}
		x.push_str( "</html>" )  ;
		return x  ;}
	pub fn write( &mut self , file:&str )  {
		let p = std::path::Path::new( file )  ;
		let mut f = std::fs::File::create( &p )   .expect( format!( "ERR: COULD NOT CREATE FILE {}" , file ).as_str() )  ;
		let h = self.print()  ;
		f.write_all( h.as_bytes())   .expect( format!("ERR: COULD NOT WRITE TO {}",p.display()).as_str() )  ;}}
//T  ;
#[duplicate(T; [B]; [Label]; [Datalist]; [Code]; [Article]; [Aside]; [Section]; [P]; [Ol]; [Ul]; [Li]; [Option]; [Pre]; [Canvas]; [Blockquote]; [Source]; [Span]; [A]; [Form]; [Template]; [Video]; [Textarea]; [Select]; [H1]; [H2]; [H3]; [H4]; [H5]; [H6]; [Button]; [Div]; [Img]; [Input] )] 
pub struct T( Tag ) ; 
#[duplicate(f name T ; [b]["b"][B]; [label]["label"][Label]; [datalist]["datalist"][Datalist]; [code]["code"][Code]; [article]["article"][Article]; [aside]["aside"][Aside]; [section]["section"][Section]; [p]["p"][P]; [ol]["ol"][Ol]; [ul]["ul"][Ul]; [li]["li"][Li]; [option]["option"][Option]; [pre]["pre"][Pre]; [canvas]["canvas"][Canvas]; [blockquote]["blockquote"][Blockquote]; [source]["source"][Source]; [span]["span"][Span]; [a]["a"][A]; [form]["form"][Form]; [template]["template"][Template]; [video]["video"][Video]; [textarea]["textarea"][Textarea]; [select]["select"][Select]; [h1]["h1"][H1]; [h2]["h2"][H2]; [h3]["h3"][H3]; [h4]["h4"][H4]; [h5]["h5"][H5]; [h6]["h6"][H6]; [button]["button"][Button]; [div]["div"][Div]; [img]["img"][Img]; [input]["input"][Input] )] 
pub    fn     f( val:&str ) -> T  {
	let t = Tag::new( name.to_string() , val.to_string() )  ;
	return T( t )  ;}
#[duplicate(T; [B]; [Label]; [Datalist]; [Code]; [Article]; [Aside]; [Section]; [P]; [Ol]; [Ul]; [Li]; [Option]; [Pre]; [Canvas]; [Blockquote]; [Source]; [Span]; [A]; [Form]; [Template]; [Video]; [Textarea]; [Select]; [H1]; [H2]; [H3]; [H4]; [H5]; [H6]; [Button]; [Div]; [Img]; [Input] )] 
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
		self   .autocorrect(false)   .autocapitalize(false)   .spellcheck(false)  ;
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
impl Li  {
	pub fn value( &mut self ,n:u32) -> &mut Self  {
		self.0.att( "value".to_string() , format!("{}" ,n))  ;
		return self  ;}}
impl Label  {
	pub fn for_( &mut self ,x:&str) -> &mut Self  {
		self.0.att( "for".to_string() , format!("{}" ,x))  ;
		return self  ;}}
//Only for <ol> lists.Specifies the start value of a list item.The following list items will increment from that number  ;
impl Pre  {
	pub fn oninput( &mut self ,f:&str) -> &mut Self  {
		self.0.att( "oninput".to_string() , format!("{}" ,f))  ;
		return self  ;}
	pub fn onchange( &mut self ,f:&str) -> &mut Self  {
		self.0.att( "onchange".to_string() , format!("{}" ,f))  ;
		return self  ;}
	pub fn wrap( &mut self ) -> &mut Self  {
		self.0.att( "wrap".to_string() , "".to_string())  ;
		return self  ;}}
impl Ol  {
	pub fn reversed( &mut self ) -> &mut Self  {
		self.0.att( "reversed".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn start( &mut self ,n:u32) -> &mut Self  {
		// Specifies the start value of an ordered list  ;
		self.0.att( "start".to_string() , format!("{}" ,n) )  ;
		return self  ;}
	pub fn type_( &mut self ,t:ListType) -> &mut Self  {
		match t  {
			ListType::N =>  {
				self.0.att( "type".to_string() ,"1".to_string() )  ;
				return self  ;}
			ListType::A =>  {
				self.0.att( "type".to_string() ,"A".to_string() )  ;
				return self  ;}
			ListType::Aa =>  {
				self.0.att( "type".to_string() ,"a".to_string() )  ;
				return self  ;}
			ListType::I =>  {
				self.0.att( "type".to_string() ,"I".to_string() )  ;
				return self  ;}
			ListType::Ii =>  {
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
	pub fn target( &mut self , x:Target ) -> &mut Self  {
		match x  {
			Target::Blank =>  {
				self.0.att( "target".to_string() , "blank".to_string() )  ;
				return self  ;}
			Target::Parent =>  {
				self.0.att( "target".to_string() , "parent".to_string() )  ;
				return self  ;}
			Target::Self_ =>  {
				self.0.att( "target".to_string() , "self".to_string() )  ;
				return self  ;}
			Target::Top =>  {
				self.0.att( "target".to_string() , "top".to_string() )  ;
				return self  ;}}}
	pub fn rel( &mut self , x:ARel ) -> &mut Self  {
		match x  {
			ARel::Alternate =>  {
				self.0.att( "rel".to_string() , "alternate".to_string() )  ;
				return self  ;}
			ARel::Author =>  {
				self.0.att( "rel".to_string() , "author".to_string() )  ;
				return self  ;}
			ARel::Bookmark =>  {
				self.0.att( "rel".to_string() , "bookmark".to_string() )  ;
				return self  ;}
			ARel::External =>  {
				self.0.att( "rel".to_string() , "external".to_string() )  ;
				return self  ;}
			ARel::Help =>  {
				self.0.att( "rel".to_string() , "help".to_string() )  ;
				return self  ;}
			ARel::License =>  {
				self.0.att( "rel".to_string() , "license".to_string() )  ;
				return self  ;}
			ARel::Next =>  {
				self.0.att( "rel".to_string() , "next".to_string() )  ;
				return self  ;}
			ARel::Nofollow =>  {
				self.0.att( "rel".to_string() , "nofollow".to_string() )  ;
				return self  ;}
			ARel::Noreferrer =>  {
				self.0.att( "rel".to_string() , "noreferrer".to_string() )  ;
				return self  ;}
			ARel::Noopener =>  {
				self.0.att( "rel".to_string() , "noopener".to_string() )  ;
				return self  ;}
			ARel::Prev =>  {
				self.0.att( "rel".to_string() , "prev".to_string() )  ;
				return self  ;}
			ARel::Search =>  {
				self.0.att( "rel".to_string() , "search".to_string() )  ;
				return self  ;}
			ARel::Tag =>  {
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
	pub fn method( &mut self , x:Method ) -> &mut Self  {
		match x  {
			Method::Get =>  {
				self.0.att( "method".to_string() , "get".to_string() )  ;
				return self  ;}
			Method::Post =>  {
				self.0.att( "method".to_string() , "post".to_string() )  ;
				return self  ;}}}
	pub fn enctype( &mut self , x:Enctype ) -> &mut Self  {
		match x  {
			Enctype::Application =>  {
				self.0.att( "enctype".to_string() , "application/x-www-form-urlencoded".to_string() )  ;
				return self  ;}
			Enctype::Multipart =>  {
				self.0.att( "enctype".to_string() , "multipart/form-data".to_string() )  ;
				return self  ;}
			Enctype::Text =>  {
				self.0.att( "enctype".to_string() , "text/plain".to_string() )  ;
				return self  ;}}}
	pub fn autocomplete( &mut self , on:bool ) -> &mut Self  {
		if on  {
			self.0.att( "autocomplete".to_string() , "on".to_string() )  ;}
		else  {
			self.0.att( "autocomplete".to_string() , "off".to_string() )  ;}
		return self  ;}}
impl Form  {
	pub fn target( &mut self , x:Target ) -> &mut Self  {
		match x  {
			Target::Blank =>  {
				self.0.att( "rel".to_string() , "_blank".to_string() )  ;
				return self  ;}
			Target::Self_ =>  {
				self.0.att( "rel".to_string() , "_self".to_string() )  ;
				return self  ;}
			Target::Parent =>  {
				self.0.att( "rel".to_string() , "_parent".to_string() )  ;
				return self  ;}
			Target::Top =>  {
				self.0.att( "rel".to_string() , "_top".to_string() )  ;
				return self  ;}}}
	pub fn rel( &mut self , x:FormRel ) -> &mut Self  {
		match x  {
			FormRel::External =>  {
				self.0.att( "rel".to_string() , "external".to_string() )  ;
				return self  ;}
			FormRel::Help =>  {
				self.0.att( "rel".to_string() , "help".to_string() )  ;
				return self  ;}
			FormRel::License =>  {
				self.0.att( "rel".to_string() , "license".to_string() )  ;
				return self  ;}
			FormRel::Next =>  {
				self.0.att( "rel".to_string() , "next".to_string() )  ;
				return self  ;}
			FormRel::Nofollow =>  {
				self.0.att( "rel".to_string() , "nofollow".to_string() )  ;
				return self  ;}
			FormRel::Noopener =>  {
				self.0.att( "rel".to_string() , "noopener".to_string() )  ;
				return self  ;}
			FormRel::Noreferrer =>  {
				self.0.att( "rel".to_string() , "noreferrer".to_string() )  ;
				return self  ;}
			FormRel::Opener =>  {
				self.0.att( "rel".to_string() , "opener".to_string() )  ;
				return self  ;}
			FormRel::Prev =>  {
				self.0.att( "rel".to_string() , "prev".to_string() )  ;
				return self  ;}
			FormRel::Search =>  {
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
	pub fn method( &mut self , x:Method ) -> &mut Self  {
		match x  {
			Method::Get =>  {
				self.0.att( "method".to_string() , "get".to_string() )  ;
				return self  ;}
			Method::Post =>  {
				self.0.att( "method".to_string() , "post".to_string() )  ;
				return self  ;}}}
	pub fn enctype( &mut self , x:Enctype ) -> &mut Self  {
		match x  {
			Enctype::Application =>  {
				self.0.att( "enctype".to_string() , "application/x-www-form-urlencoded".to_string() )  ;
				return self  ;}
			Enctype::Multipart =>  {
				self.0.att( "enctype".to_string() , "multipart/form-data".to_string() )  ;
				return self  ;}
			Enctype::Text =>  {
				self.0.att( "enctype".to_string() , "text/plain".to_string() )  ;
				return self  ;}}}
	pub fn autocomplete( &mut self , on:bool ) -> &mut Self  {
		if on  {
			self.0.att( "autocomplete".to_string() , "on".to_string() )  ;}
		else  {
			self.0.att( "autocomplete".to_string() , "off".to_string() )  ;}
		return self  ;}}
impl Img  {
	pub fn src( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "src".to_string() , val.to_string() )  ;
		return self  ;}}
impl Source  {
	pub fn src( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "src".to_string() , val.to_string() )  ;
		return self  ;}}
impl Video  {
	pub fn preload( &mut self , x:Preload ) -> &mut Self  {
		match x  {
			Preload::Auto =>  {
				self.0.att( "preload".to_string() , "auto".to_string() )  ;
				return self  ;}
			Preload::Metadata =>  {
				self.0.att( "preload".to_string() , "metadata".to_string() )  ;
				return self  ;}
			Preload::None =>  {
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
impl Select  {
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
impl Option  {
	pub fn disabled( &mut self ) -> &mut Self  {
		self.0.att( "disabled".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn selected( &mut self ) -> &mut Self  {
		self.0.att( "selected".to_string() , "".to_string() )  ;
		return self  ;}
	pub fn value( &mut self , x:&str ) -> &mut Self  {
		self.0.att( "value".to_string() , x.to_string() )  ;
		return self  ;}}
impl Button  {
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
impl Input  {
	pub fn multiple(&mut self) -> &mut Self  {
		self.0.att( "multiple".to_string() ,"".to_string() )  ;
		return self  ;}
	pub fn minlenght( &mut self , val:i32 ) -> &mut Self  {
		self.0.att( "minlenght".to_string() , format!("{}" , val.to_string() ) )  ;
		return self  ;}
	pub fn value( &mut self , val:&str ) -> &mut Self  {
		self.0.att( "value".to_string() , format!("{}" , val.to_string() ) )  ;
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
	pub fn type_( &mut self , val:Type ) -> &mut Self  {
		match val  {
			Type::Button =>  {
				self.0.att( "type".to_string() , "button".to_string() )  ;
				return self  ;}
			Type::Checkbox =>  {
				self.0.att( "type".to_string() , "checkbox".to_string() )  ;
				return self  ;}
			Type::Color =>  {
				self.0.att( "type".to_string() , "color".to_string() )  ;
				return self  ;}
			Type::Date =>  {
				self.0.att( "type".to_string() , "date".to_string() )  ;
				return self  ;}
			Type::DatetimeLocal =>  {
				self.0.att( "type".to_string() , "datetimeLocal".to_string() )  ;
				return self  ;}
			Type::Email =>  {
				self.0.att( "type".to_string() , "email".to_string() )  ;
				return self  ;}
			Type::File =>  {
				self.0.att( "type".to_string() , "file".to_string() )  ;
				return self  ;}
			Type::Hidden =>  {
				self.0.att( "type".to_string() , "hidden".to_string() )  ;
				return self  ;}
			Type::Image =>  {
				self.0.att( "type".to_string() , "image".to_string() )  ;
				return self  ;}
			Type::Month =>  {
				self.0.att( "type".to_string() , "month".to_string() )  ;
				return self  ;}
			Type::Number =>  {
				self.0.att( "type".to_string() , "number".to_string() )  ;
				return self  ;}
			Type::Password =>  {
				self.0.att( "type".to_string() , "password".to_string() )  ;
				return self  ;}
			Type::Radio =>  {
				self.0.att( "type".to_string() , "radio".to_string() )  ;
				return self  ;}
			Type::Range =>  {
				self.0.att( "type".to_string() , "range".to_string() )  ;
				return self  ;}
			Type::Reset =>  {
				self.0.att( "type".to_string() , "reset".to_string() )  ;
				return self  ;}
			Type::Search =>  {
				self.0.att( "type".to_string() , "search".to_string() )  ;
				return self  ;}
			Type::Submit =>  {
				self.0.att( "type".to_string() , "submit".to_string() )  ;
				return self  ;}
			Type::Tel =>  {
				self.0.att( "type".to_string() , "tel".to_string() )  ;
				return self  ;}
			Type::Text =>  {
				self.0.att( "type".to_string() , "text".to_string() )  ;
				return self  ;}
			Type::Time =>  {
				self.0.att( "type".to_string() , "time".to_string() )  ;
				return self  ;}
			Type::Url =>  {
				self.0.att( "type".to_string() , "url".to_string() )  ;
				return self  ;}
			Type::Week =>  {
				self.0.att( "type".to_string() , "week".to_string() )  ;
				return self  ;}}}
	pub fn onchange( &mut self ,f:&str ) -> &mut Self  {
		self.0.att( "onchange".to_string() , f.to_string() )  ;
		return self  ;}
	pub fn oninput( &mut self ,f:&str ) -> &mut Self  {
		self.0.att( "onchange".to_string() , f.to_string() )  ;
		return self  ;}}
impl Textarea  {
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
