use yew::prelude::*;

use crate::components::svg::{
    flags::Germany,
    emojis::Heart,
};

use crate::AppContext;

#[function_component(Aboutme)]
pub fn aboutme() -> Html {

    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_aboutme_content(app_context: AppContext) -> Html {
	match app_context.language.current {
	    "de" => html!{
		<>
		    <p>{ "Xin chào, tên tôi là Đức." }</p>
		    <p>{ "Tôi đang học ngành Công Nghệ Thông Tin tại Việt Nam" }
			{ " University HCM " } { " và có niềm đam mê" }
			{ " với lập trình " } <Heart class="svg-text svg-heart" /> { " Chủ yếu tôi làm việc với " }
			<strong> { "JavaScript" } </strong>
			{ " (Node.js & React) und " } <strong> { "Rust" } </strong>
			{ " (Actix & WebAssembly) um coole, " }
			{ "để lập trình các ứng dụng web nhanh." }</p>
		    <p>{ "Bạn có câu hỏi nào về phát triển web hoặc về tôi không? Sau đó liên hệ với tôi." }</p>
		</>
	    },
	    "jp" => html!{
		<>
		    <p>{ "はじめまして。マウラ・マークと申します。" }</p>
		    <p>{ "フランクフルト" } <Germany class="svg-text" /> { "の大学に通って日本学と韓国学を勉強しています。" }
			{ "プログラミングに情熱を注いでいます" } <Heart class="svg-text svg-heart" /> { "普段は速くて素晴らしいウェブアプリを" }
			{ "作る為に " } <strong> { "JavaScript"} </strong> { "(Node.jsやReact)と" }
			<strong> { "Rust" } </strong> { "(ActixやWebAssembly)を使っています。" }</p>
		    <p>{ "興味があれば、連絡してください。" }</p>
		</>
	    },
	    "kr" => html!{
		<>
		    <p>{ "안녕 하세요? 저는 머이라 마크입니다." }</p>
		    <p>{ "프랑크푸르트" } <Germany class="svg-text" /> { " 대학에 다니고 일본학과 한국학을 공부합니다. " }
			{ "그리고 프로그래밍에도 푹 빠졌습니다" } <Heart class="svg-text svg-heart" /> { " 평소에 빠르고 좋은 웹앱을 " }
			{ "프로그래밍하기 위해서 " } <strong> { "JavaScript" } </strong> { " (Node.js와 React)와 " }
			<strong> { "Rust" } </strong> { "(Actix와 WebAssembly)를 사용하고 있습니다. " }</p>
		    <p>{ "저와 함께 이야기를 나누고 싶거나, 같이 일해 보고 싶다면 언제든지 연락해주세요." }</p>
		</>
	    },
	    "eng" | _ => html! {
		<>
		    <p>{ "Hi! My name is Marc" }</p>
		    <p>{ "I'm a Japanese and Korean Studies student at the Goethe" }
			{ " University in Frankfurt" } <Germany class="svg-text" />
			{ " with a passion for programming" } <Heart class="svg-text svg-heart" />
			{ " I mostly work with " } <strong> { "JavaScript" } </strong>
			{ " (Node.js & React) and " } <strong> { "Rust" } </strong>
			{ " (Actix & WebAssembly) to develop cool and fast webapps." }</p>
		    <p>{ "If you have any web development needs, feel free to contact or get to know me a little better." }</p>
		</>
	    }
	}
	
    }

    html!{
	<div class="aboutme">
            <div class="aboutme__description">
                <div class="aboutme__description__text">{ handle_aboutme_content(app_context) }</div>
            </div>
        </div>
    }
}
