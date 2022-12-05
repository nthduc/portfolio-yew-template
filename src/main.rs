use std::rc::Rc;
use yew::prelude::*;
use gloo_storage::{
    LocalStorage,
    Storage,
    errors::StorageError,
};
use gloo_console::{
    log as console_log,
    error as console_error,
};
use web_sys::{
    Window,
    Navigator,
    MediaQueryList,
};

mod components;
use components::{
    nav::Nav,
    landing::Landing,
    aboutme::Aboutme,
    skills::Skills,
    projects::Projects,
    contact::Contact,
    footer::Footer,
};

pub enum ThemeAction {
    Light,
    Dark,
    Rust,
}

#[derive(PartialEq, Debug)]
struct ThemeState {
    current: &'static str,
}

impl Default for ThemeState {
    fn default() -> Self {
	// Get the theme stored in local storage
	let ls_theme_option: Result<String, StorageError> = LocalStorage::get("theme");

	let ls_theme: &str = match &ls_theme_option {
	    Ok(theme) => theme,
	    _ => {
		let window: Window = web_sys::window().expect("No Window Object!");
		// let match_media_result: Result<Option<MediaQueryList>, JsValue> = window.match_media("(prefers-color-scheme: dark)");
		let match_media_result = window.match_media("(prefers-color-scheme: dark)"); // : Result<Option<MediaQueryList>, JsValue>
		match match_media_result {
		    Ok(match_media_option) => {
			let match_media: MediaQueryList = match_media_option.expect("MEDIAQUERYLIST NOT SUPPORTED!");
			if match_media.matches() {
			    "dark"
			} else {
			    "light"
			}
		    }
		    _ => "light",
		}
	    },
	};

	match ls_theme {
	    "light" => Self { current: "light" },
	    "dark" => Self { current: "dark" },
	    "rust" => Self { current: "rust" },
	    _ => Self { current: "light" },
	}
    }
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
	let next_theme = match action {
	    ThemeAction::Light => "light",
	    ThemeAction::Dark => "dark",
	    ThemeAction::Rust => "rust",
	};

	Self { current: next_theme }.into()
    }
}

pub enum LanguageAction {
    English,
    German,
    Japanese,
    Korean,
}

#[derive(PartialEq, Debug)]
struct LanguageState {
    current: &'static str,
}

impl Default for LanguageState {
    fn default() -> Self {
	// Get the lang stored in local storage
	let ls_lang_option: Result<String, StorageError> = LocalStorage::get("lang");

	let ls_lang: &str = match &ls_lang_option {
	    Ok(lang) => &lang,
	    _ => {
		let window: Window = web_sys::window().expect("No Window Object!");
		let navigator: Navigator = window.navigator();
		let browser_language: Option<String> = navigator.language();
		match browser_language {
		    Some(bl) => {
			match bl.as_str() {
			    "ja" => "jp",
			    "ko-KR" | "ko-kp" => "kr",
			    "de" | "de-de" | "de-at" | "de-ch" | "de-li" | "de-lu"  => "de",
			    "en-US" | "en" | _ => "en",
			}
		    },
		    None => "eng",
		}
	    },
	};

	match ls_lang {
	    "eng" => Self { current: "eng" },
	    "de" => Self { current: "vn" },
	    "jp" => Self { current: "jp" },
	    "kr" => Self { current: "kr" },
	    _ => {
		Self { current: "eng" }
	    }
	}
    }
}

impl Reducible for LanguageState {
    type Action = LanguageAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
	let next_lang = match action {
	    LanguageAction::English => "eng",
	    LanguageAction::German => "de",
	    LanguageAction::Japanese => "jp",
	    LanguageAction::Korean => "kr",
	};

	Self { current : next_lang }.into()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    theme: UseReducerHandle<ThemeState>,
    language: UseReducerHandle<LanguageState>,
    theme_cycle: Vec<&'static str>,
}

#[function_component(App)]
fn app() -> Html {

    let theme: UseReducerHandle<ThemeState> = use_reducer(ThemeState::default);
    let language: UseReducerHandle<LanguageState> = use_reducer(LanguageState::default);
    let theme_cycle: Vec<&str> = vec!["light", "dark", "rust"];

    fn get_classes(theme: UseReducerHandle<ThemeState>) -> &'static str {
        match theme.current {
            "dark" => "root dark",
            "rust" => "root rust",
            "light" | _ => "root",
        }
    }

    {
	/* Set the local storage to the current theme (& if it changes) */
	let theme: UseReducerHandle<ThemeState> = theme.clone();
	use_effect_with_deps(move |theme: &UseReducerHandle<ThemeState>| {
	    match LocalStorage::set("theme", &theme.current) {
		Ok(()) => console_log!(format!("Theme set to {}", &theme.current)),
		_ => console_error!("Couldn't set LocalStorage. Please turn the feature in your Browser on if possible."),
	    };
	    || ()
	}, theme.clone())
    }

    {
	/* Set the local storage to the current language (& if it changes) */
	let language: UseReducerHandle<LanguageState> = language.clone();
	use_effect_with_deps(move |language: &UseReducerHandle<LanguageState>| {
	    match LocalStorage::set("lang", &language.current) {
		Ok(()) => console_log!(format!("Language set to {}", &language.current)),
		_ => console_error!("Couldn't set LocalStorage. Please turn the feature in your Browser on if possible."),
	    };
	    || ()
	}, language.clone())
    }

    html!{
	<ContextProvider<AppContext> context={AppContext {
	    theme: theme.clone(),
	    language: language,
	    theme_cycle: theme_cycle
	}}>
	    <div class={ get_classes(theme) }>
		<Nav />
		<div class="maincontent">
		    <Landing />
		    <Aboutme />
		    <Skills />
		    <Projects />
		    <Contact />
		    <Footer />
		    // <NavMobile />
		</div>
	    </div>
	</ContextProvider<AppContext>>
    }
}


fn main() {
    yew::start_app::<App>();
}
