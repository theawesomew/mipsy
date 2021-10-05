use yew::prelude::*;
use yew::Html;
use log::info;

pub struct NavBar {
    link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Debug, Clone)]
pub struct Props {
    #[prop_or_default]
    pub load_onchange: Callback<ChangeData>,
    pub run_onclick: Callback<MouseEvent>,
    pub reset_onclick: Callback<MouseEvent>,
    pub exit_status: Option<Option<i32>>,
    pub step_forward_onclick: Callback<MouseEvent>,
    pub step_back_onclick: Callback<MouseEvent>,
    pub kill_onclick: Callback<MouseEvent>,
    pub open_modal_onclick: Callback<MouseEvent>,
}

struct Icon {
    label: String,
    callback: Option<yew::Callback<yew::MouseEvent>>,
    html: Html,
}

impl NavBar {
    fn icons(props: <NavBar as Component>::Properties) -> Vec<Icon> {
        let icons = vec![
            Icon {
                label: String::from("Run"),
                html: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
                    </svg>
                },
                callback: Some(props.run_onclick),
            }, 
            Icon {
                label: String::from("Reset"),
                html: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                         <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
                    </svg>
                },
                callback: Some(props.reset_onclick),
            },
            Icon {
                label: String::from("Kill"),
                html: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                    </svg>
                },
                callback: Some(props.kill_onclick),
            },
            Icon {
                label: String::from("Step Back"),
                html: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                },
                callback: Some(props.step_back_onclick),
            },
            Icon {
                label: String::from("Step Next"),
                html: html! {
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                    </svg>
                },
                callback: Some(props.step_forward_onclick),
            },
        ];

        icons
    }
}

impl Component for NavBar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavBar { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let icons = Self::icons(self.props.clone());
        let noop = self.link.callback(|_| {});
        html! {
        <nav class="flex items-center justify-between flex-wrap bg-th-primary p-4">
          <div class="flex items-center flex-shrink-0 text-black mr-6">
            <span class="font-semibold text-xl tracking-tight">{"Mipsy"}</span>
          </div>
          <div class="w-full block flex-grow flex items-center w-auto">
            <div class="flex-grow flex flex-row">
              <label for="load_file" class="mr-2 text-sm flex place-items-center flex-row inline-block cursor-pointer px-3 py-3 leading-none border rounded text-black border-black hover:border-transparent hover:text-teal-500 hover:bg-white">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M2 6a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1H8a3 3 0 00-3 3v1.5a1.5 1.5 0 01-3 0V6z" clip-rule="evenodd" />
                  <path d="M6 12a2 2 0 012-2h8a2 2 0 012 2v2a2 2 0 01-2 2H2h2a2 2 0 002-2v-2z" />
                </svg>
                {"Load"}
              </label>
              <input id="load_file" onchange=&self.props.load_onchange type="file" accept=".s" class="hidden" />
              {
                  for icons.iter().map(|item| {
                      // special behaviour for the Run button
                      if item.label == "Run" || item.label == "Step Next"{
                        let is_disabled = match &self.props.exit_status {
                            Some(mips_exit_status) => match mips_exit_status {
                                Some(_exit_status) => true,
                                None => false,
                            }
                            None => false,
                        };

                        let mut button_classes = String::from("mr-2 flex place-items-center flex-row inline-block cursor-pointer text-sm px-2 py-2 border rounded text-black border-black hover:border-transparent hover:text-teal-500 hover:bg-white");
                        
                        if is_disabled {
                            button_classes.push_str(" opacity-50 cursor-not-allowed"); 
                        }
                        
                        html! {
                            <button disabled={is_disabled} onclick={item.callback.clone().unwrap()} class={button_classes}>
                                { item.html.clone() }
                                { item.label.clone() }
                            </button>
                        }
                      }
                      else {
                          let onclick = if item.callback.is_some() {
                              item.callback.clone().unwrap()
                          } else {
                              noop.clone()
                          };

                          html! {
                              <button onclick={onclick} class="mr-2 flex place-items-center flex-row inline-block cursor-pointer text-sm px-2 py-2 border rounded text-black border-black hover:border-transparent hover:text-teal-500 hover:bg-white">
                                { item.html.clone() }
                                { item.label.clone() }
                              </button>
                          }
                      }
                  })
              }
            </div>
            <button onclick={self.props.open_modal_onclick.clone()} class="mr-2 flex place-items-center flex-row inline-block cursor-pointer text-sm px-2 py-2 border rounded text-black border-black hover:border-transparent hover:text-teal-500 hover:bg-white">
            {"About Mipsy Web"}
            </button>
          </div>
        </nav>
              }
    }
}
