use yew::prelude::*;

use crate::AppContext;

#[function_component(Footer)]
pub fn footer() -> Html {

    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_footer_content(app_context: AppContext) -> Html {
	let yew_link: Html = html!{ <a href="https://yew.rs/" target="_blank" rel="noopener noreferrer">{ "Yew Framework" }</a> };
	let wasm_link: Html = html!{ <a href="https://webassembly.org/" target="_blank" rel="noopener noreferrer">{ "WebAssembly" }</a> };

	match app_context.language.current {
	    "de" => html!{
                <>
		    {  "Trang web này đã được tạo ra trong " } { yew_link } { " được lập trình và để " } { wasm_link } { " biên soạn." }
                </>
	    },
	    "jp" => html!{
		<>
		    {  "このサイトは" } { yew_link } { "で作られて" } { wasm_link } { "にコンパイルされました。" }
                </>
	    },
	    "kr" => html!{
		<>
		    {  "이 사이트는 " } { yew_link } { "에서 제작되어 " } { wasm_link } { "에 편집되었습니다." }
                </>
	    },
	    "eng" | _ => html!{
		<>
		    {  "This website was created with the " } { yew_link } { " and is compiled to " } { wasm_link } { "." }
                </>
	    },
	}
    }

    html!{
	<footer>
	    { handle_footer_content(app_context) }
	</footer>
    }

}
