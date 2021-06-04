#[macro_use]
extern crate vst;
extern crate vst_gui;

use crate::vst::host::Host;
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::HostCallback;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::util::ParameterTransfer;

use std::sync::Arc;

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

// fn inline_style(s: &str) -> String {
//     format!(r#"<style type="text/css">{}</style>"#, s)
// }

fn get_html() -> String {
    format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <link rel="stylesheet" href="https://unpkg.com/element-ui/lib/theme-chalk/index.css">
                <script src="https://unpkg.com/element-ui/lib/index.js"></script>
                <style>
                    @import url('https://fonts.googleapis.com/css2?family=Baloo+Tammudu+2:wght@500&display=swap');
                </style>
            </head>
            <body>
                <div id="app"></div>
                <!--[if lt IE 9]>
                <div class="ie-upgrade-container">
                    <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
                    <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
                </div>
                <![endif]-->
                <!--[if gte IE 9 | !IE ]> <!-->
                {scripts}
                <![endif]-->
            </body>
        </html>
        "#,
        // style = inline_style(include_str!("style.css")),
        scripts = inline_script(include_str!("bundle.js"))
    )
}

#[derive(Default)]
struct SimpleGain {
    params: Arc<SimpleGainParameter>,
    gui: SimpleGainGUI,
}

#[derive(Default)]
struct SimpleGainParameter {
    transfer: ParameterTransfer,
}

#[derive(Default)]
struct SimpleGainGUI {
    params: Arc<SimpleGainParameter>,
    #[allow(dead_code)]
    host: HostCallback,
}

impl SimpleGainGUI {
    fn new(params: Arc<SimpleGainParameter>, host: HostCallback) -> Self {
        Self {
            params: params,
            host: host,
        }
    }

    fn javascript_callback(&self) -> vst_gui::JavascriptCallback {
        let params = Arc::clone(&self.params);
        let host = self.host;
        Box::new(move |message: String| {
            let mut tokens = message.split_whitespace();

            let command = tokens.next().unwrap_or("");
            let argument = tokens.next().unwrap_or("").parse::<f32>();

            match command {
                "getGain" => {
                    return params.get_parameter_text(0);
                }
                "setGain" => {
                    params.set_parameter(0, argument.unwrap());
                    host.automate(0, params.get_parameter(0));
                }
                "mouseOverGain" => {
                    host.begin_edit(0);
                }
                "releaseGain" => {
                    host.end_edit(0);
                }
                _ => {}
            }

            String::new()
        })
    }
}

impl PluginParameters for SimpleGainParameter {
    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "[-]".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "gain".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.3}", self.transfer.get_parameter(index as usize)),
            _ => format!(""),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        self.transfer.get_parameter(index as usize)
    }

    fn set_parameter(&self, index: i32, value: f32) {
        self.transfer.set_parameter(index as usize, value);
    }

    fn can_be_automated(&self, index: i32) -> bool {
        match index {
            0 => true,
            _ => false,
        }
    }
}

impl Plugin for SimpleGain {
    fn new(host: HostCallback) -> Self {
        let params = Arc::new(SimpleGainParameter {
            transfer: ParameterTransfer::new(1),
        });
        let gui = SimpleGainGUI::new(params.clone(), host);
        Self {
            params: params,
            gui: gui,
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Simple Gain".to_string(),
            vendor: "Psykhedelic Mandala".to_string(),
            unique_id: 1337,
            category: Category::Effect,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            ..Info::default()
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let gain = self.params.get_parameter(0);

        let (inputs, outputs) = buffer.split();

        let (l, r) = inputs.split_at(1);
        let stereo_in = l[0].iter().zip(r[0].iter());

        let (mut l, mut r) = outputs.split_at_mut(1);
        let stereo_out = l[0].iter_mut().zip(r[0].iter_mut());

        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            *left_out = *left_in * gain;
            *right_out = *right_in * gain;
        }
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        let gui = vst_gui::new_plugin_gui(
            String::from(get_html()),
            self.gui.javascript_callback(),
            Some((480, 500)),
        );
        Some(Box::new(gui))
    }
}

plugin_main!(SimpleGain);
