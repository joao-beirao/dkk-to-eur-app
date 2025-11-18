use wasm_bindgen::prelude::*;
use yew::prelude::*;

// constant for conversion
const DKK_TO_EUR: f64 = 0.134;

#[function_component(App)]
fn app() -> Html {
    let dkk: UseStateHandle<String> = use_state(|| String::new());
    let result: UseStateHandle<String> = {
        let _dkk: UseStateHandle<String> = dkk.clone();
        use_state(move || String::from("Enter value and click Convert."))
    };

    let oninput: Callback<InputEvent> = {
        let dkk: UseStateHandle<String> = dkk.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            dkk.set(input.value());
        })
    };

    let onconvert: Callback<MouseEvent> = {
        let dkk: UseStateHandle<String> = dkk.clone();
        let result: UseStateHandle<String> = result.clone();
        Callback::from(move |_| {
            let k: &str = dkk.trim();
            match k.parse::<f64>() {
                Ok(v) if v.is_finite() => {
                    let euros: f64 = v * DKK_TO_EUR;
                    let euros_str: String = format!("{:.2}", euros);
                    let euros_str: &str = euros_str.trim_end_matches('0').trim_end_matches('.');
                    result.set(format!("{}€", euros_str));
                }
                _invalid_number => {
                    result.set(String::from("Please enter a valid number."));
                }
            }
        })
    };

    html! {
        <div style="font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif; display:flex; min-height:100vh; align-items:center; justify-content:center; background:#222222;">
        <div style="background:white; padding:24px; border-radius:12px; box-shadow:0 6px 18px rgba(0,0,0,0.08); width:320px;">
          <h2>{"DKK → EUR"}</h2>
          <label for="dkk">{"Danish Krone (DKK)"}</label>
          <input id="dkk" type="number" step="any" placeholder="e.g. 75"
                 value={(*dkk).clone()}
                 {oninput}
                 style="width:100%; padding:10px; margin-top:6px; margin-bottom:12px; font-size:16px; border-radius:8px; border:1px solid #ddd;" />
          <button onclick={onconvert} style="width:100%; padding:10px; font-size:16px; border-radius:8px; border:none; cursor:pointer;">{"Convert"}</button>
          <div style="margin-top:14px; font-weight:600;">{(*result).clone()}</div>
        </div>
        </div>
    }
}

// expose start function for trunk to mount
#[wasm_bindgen(start)]
pub fn start() {
    yew::Renderer::<App>::new().render();
}