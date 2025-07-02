use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Request, RequestInit, Response};
use console_error_panic_hook;

// 定义返回的 IP 信息数据结构
#[derive(Serialize, Deserialize)]
struct IpInfo {
    origin: String,  // 客户端 IP 地址
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello from Rust WebAssembly!".to_string()
}

#[wasm_bindgen]
pub async fn fetch_ip() -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    
    let request = Request::new_with_str_and_init("https://httpbin.org/ip", &opts)?;
    
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    
    // 将 JSON 转换为 Rust 结构体
    let ip_info: IpInfo = serde_wasm_bindgen::from_value(json)?;
    
    // 将结构体转换回 JsValue
    Ok(serde_wasm_bindgen::to_value(&ip_info)?)
}

#[wasm_bindgen]
pub fn init() {
    // 设置 panic 钩子，使 Rust panic 在浏览器控制台中可见
    console_error_panic_hook::set_once();
}

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub fn setup_ui() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // 获取 IP 信息容器元素
    if let Some(ip_container) = document.get_element_by_id("ip-info") {
        // 创建一个按钮来获取 IP 信息
        let button = document.create_element("button")?;
        button.set_inner_html("获取我的 IP");
        
        // 创建一个 div 来显示结果
        let result_div = document.create_element("div")?;
        result_div.set_id("result");
        result_div.set_class_name("result-container");
        
        // 将按钮和结果 div 添加到容器中
        ip_container.append_child(&button)?;
        ip_container.append_child(&result_div)?;
        
        // 为按钮添加点击事件处理程序
        let document_clone = document.clone();
        let closure = Closure::wrap(Box::new(move || {
            let document = document_clone.clone();
            if let Some(result_div) = document.get_element_by_id("result") {
                result_div.set_inner_html("加载中...");
                
                let document_for_promise = document.clone();
                let _ = future_to_promise(async move {
                    match fetch_ip().await {
                        Ok(ip_data) => {
                            let ip_info_js: js_sys::Object = ip_data.dyn_into()?;
                            let origin = js_sys::Reflect::get(&ip_info_js, &"origin".into())?
                                .as_string()
                                .unwrap_or_else(|| "未知".to_string());
                            
                            if let Some(result_div) = document_for_promise.get_element_by_id("result") {
                                result_div.set_inner_html(&format!("您的 IP 地址是: {}", origin));
                            }
                            Ok(JsValue::NULL)
                        },
                        Err(err) => {
                            if let Some(result_div) = document_for_promise.get_element_by_id("result") {
                                result_div.set_inner_html("获取 IP 信息失败");
                            }
                            log(&format!("Error: {:?}", err));
                            Err(err)
                        }
                    }
                });
            }
        }) as Box<dyn FnMut()>);
        
        button.dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .set_onclick(Some(closure.as_ref().unchecked_ref()));
        
        // 防止闭包被垃圾回收
        closure.forget();
    }
    
    Ok(())
}