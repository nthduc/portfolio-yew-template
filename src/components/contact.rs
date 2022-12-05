use yew::prelude::*;

use crate::components::svg::{
    flags::{ Germany, England, Japan, Korea, China, Egypt },
    emojis::{ Blushing, Mail },
    undraw::QuickChat,
};
use crate::AppContext;

#[function_component(Contact)]
pub fn contact() -> Html {

    let app_context = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_title(app_context: AppContext) -> &'static str {
	match app_context.language.current {
	    "kr" => "편하게 연락해주세요!",
	    "eng" | "de" | "jp" | _ => "Let's chat",
	}
    }

    fn handle_contact_description(app_context: AppContext) -> Html {
	match app_context.language.current {
	    "de" => html!{
		<>
		    <p>{ "Không thành vấn đề nếu bạn liên hệ với tôi về kinh doanh " }
			{ "hoặc chỉ muốn đi chơi với tôi, tôi mong được trò chuyện với bạn." }</p>
		    <p>{ "Tiếng mẹ đẻ của tôi là tiếng Việt" } <Germany class="svg-text" />
			{ ", nhưng tôi cũng có thể nói Tiếng Kinh" } <England class="svg-text" />
			{ ", Miền Nam" } <Japan class="svg-text" />
			{ ", Miền Trung" } <Korea class="svg-text" />
			{ " và hiện đang học tiếng Trung" } <China class="svg-text" /> { " " } <Egypt class="svg-text" />
			{ ". Bạn được hoan nghênh viết thư cho tôi bằng một trong những ngôn ngữ này." }</p>
		    <p>{ "tôi thích nhất " } <a href="mailto:nguyenthaiduc0212@gmail.com">{ "email" }<Mail class="svg-text" /></a></p>
		</>
	    },
	    "jp" => html!{
		<>
		    <p>{ "母国語はドイツ語" } <Germany class="svg-text" />
			{ "ですが、英語" } <England class="svg-text" />
			{ "と日本語" } <Japan class="svg-text" />
			{ "と韓国語" } <Korea class="svg-text" />
			{ "も喋れます。そして、今中国語" } <China class="svg-text" /> { "とアラビア語" } <Egypt class="svg-text" />
			{ "を勉強しています。" }
			{ "私と仕事をしてみたい、または楽しく会話してみたい、と思った方はどんな言語でも気軽に連絡して下さい。" }</p>
		    <p><a href="mailto:marc.maeurer@pm.me">{ "お問い合わせは" } <Mail class="svg-text" /></a></p>
		</>

	    },
	    "kr" => html!{
		<>
		    <p>{ "저와 함께 이야기를 나누고 싶거나, 같이 일해 보고 싶다면 언제든지 연락해주세요. " }
			{ "제 모국어는 독일어" } <Germany class="svg-text" />
			{ "이지만 저는 영어" } <England class="svg-text" />
			{ "와 일본어" } <Japan class="svg-text" />
			{ ",  한국어" } <Korea class="svg-text" />
			{ "도 가능합니다. 그뿐만 아니라 현재 중국어" } <China class="svg-text" /> { "와 아랍어" } <Egypt class="svg-text" />
			{ "도 배우고 있습니다. 어떤 언어로든 저에게 부담없이 연락하주시면 됩니다." }</p>
		    <p><a href="mailto:marc.maeurer.@pm.me">{ "문의처" } <Mail class="svg-text" /></a></p>
		</>

	    },
	    "eng" | _ => html!{
		<>
		    <p>{ "Whether you are interested to do business with me, " }
			{ " want to chat about some of my content, or just want to hang out, " }
			{ " I am happy to talk to you. " }</p>
		    <p>{ "My mother tongue is German" } <Germany class="svg-text" />
			{ ", but I can speak English" } <England class="svg-text" />
			{ ", Japanese" } <Japan class="svg-text" />
			{ ", Korean" } <Korea class="svg-text" />
			{ " as well and I'm currently learning Mandarin" } <China class="svg-text" /> { " and Arabic" } <Egypt class="svg-text" />
			{ " Feel free to contact me in any of those languages." }</p>
		    <p>{ "My preferred way of contact is via " }
			<a href="mailto:marc.maeurer@pm.me">{ "email" }<Mail class="svg-text" /></a>
		    </p>
		</>

	    },
	}
    } 


    html!{
	<>
	    <h3 id="contact"> { handle_title(app_context.clone()) } { " " } <Blushing /> </h3>

	    <div class="contact margin-top">
		<QuickChat class="contact__illustration"/>
		<div class="contact__paragraph"> { handle_contact_description(app_context) } </div>
	    </div>
        </>
    }
}
