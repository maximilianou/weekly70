# weekly70 - rust - 2024

-----------

## leptos.dev

-------------

https://book.leptos.dev/getting_started/index.html

```sh
cargo install trunk

cargo init web07_leptos

cd web07_letpos

# install global 
rustup toolchain install nightly
rustup default nightly

# install in the project only
rustup toolchain install nightly
cd web07_leptos
rustup override set nightly

# install wasm32 to compile and run webassembly in the browser
rustup target add wasm32-unknown-unknown


cargo add leptos --features=csr,nightly

```
```html
<!DOCTYPE html>
<html>
    <head><title></title></head>
    <body></body>
</html>
```
```rust
use leptos::*;
fn main() {
    mount_to_body( || view! { <><p>"Hi, here we are!"</p><ul><li>"Rust"</li><li>"Web"</li></ul></> } )
}
```
```sh


cargo add console_error_panic_hook

console_error_panic_hook::set_once();


https://book.leptos.dev/view/index.html

cargo install trunk
cargo install cargo-generate

##
## on debian - to have SSL libraries install librust-openssl-dev
##
root@srv04:~# apt install librust-openssl-dev

cargo install cargo-generate



cargo generate --git https://github.com/leptos-community/start-csr

trunk serve --port 3000 --open



```

### Rust leptos.dev web components 


```sh

https://book.leptos.dev/view/01_basic_component.html


```
```rust
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count(3);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
    }
}

```

```sh
cargo init web09_components
cd web09_components
cargo add leptos --features=csr,nightly

touch index.html
```

```html
<!DOCTYPE html>
<html>
    <head><title></title></head>
    <body></body>
</html>
```

```rust
use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count(3);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
    }
}

```

```sh

https://book.leptos.dev/view/02_dynamic_attributes.html

```

```html
<!DOCTYPE html>
<html>
    <head><title></title>
        <style>
            .red {
                color: red;
            }
        </style>
    </head>
    <body></body>
</html>

```

```rust
use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
use leptos::SignalUpdate;
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body( || view! { <App />}    );
}
#[component]
fn App() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  view! {
    <button on:click=move |_| { set_count.update( |n| *n += 1); } 
    class:red=move || count() % 2 == 1
    >
      "Click me:" {move || count()}
    </button>
  }  
}
```



```rust
use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
use leptos::SignalUpdate;
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body( || view! { <App />}    );
}
#[component]
fn App() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  let (x, _set_x) = create_signal(0);
  view! {
    <button on:click=move |_| { set_count.update( |n| *n += 1); } 
    class:red=move || count() % 2 == 1
    >
      "Click me:" {move || count()}
    </button>
    <button on:click=move |_| { _set_x.update( |n| *n += 10); } 
    style="position: absolute"
    style:left=move || format!("{}px", x() + 100)
    style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
    style:max-width="400px"
    style=("--columns", x)

    >
      "Move this:" 
    </button>
  }  
}
```
```sh

https://book.leptos.dev/view/03_components.html


```
```rust
use leptos::*;
use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn ProgressBar(
  #[prop(into)]
  progressValue: ReadSignal<i32>
) -> impl IntoView {
    view! {
        <progress max="100" value=progressValue />
    }
}


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count( count() + 1);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
        <progress max="100" value=double_count />
        <p>"Double Count: " {double_count}</p>
        <ProgressBar progressValue=count />
    }
}

```



```sh
https://book.leptos.dev/view/04_iteration.html

```

```sh
Static List
```

```rust
use leptos::*;
#[component]
fn App() -> impl IntoView {
    view!{
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if your list is static"</p>
        <StaticList length=5 />
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the list in your list will change"</p>
        <DynamicList initial_length=7 />
    }
}
#[component]
fn StaticList( length: usize) -> impl IntoView {
    let counters = (1..=length).map( |idx| create_signal(idx) );
    let counter_buttons = counters
          .map( |(count, set_count)| {
            view!{
              <li>
                <button 
                on:click=move |_| set_count.update( |n| *n += 1 )
                >
                  {count}
                </button>
              </li>
            }
          }).collect::<Vec<_>>();
  view! {
    <ul>
      {counter_buttons}
    </ul>
  }
}
#[component]
fn DynamicList( initial_length: usize ) -> impl IntoView {
  view! {
    <ul>
    <li>{initial_length}</li>
    </ul>
  }
}

fn main() {
  leptos::mount_to_body(App)
}
```


```sh
Dynamic List with For
```

```rust
use leptos::*;
#[component]
fn App() -> impl IntoView {
    view!{
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if your list is static"</p>
        <StaticList length=5 />
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the list in your list will change"</p>
        <DynamicList initial_length=7 />
    }
}
#[component]
fn StaticList( length: usize) -> impl IntoView {
    let counters = (1..=length).map( |idx| create_signal(idx) );
    let counter_buttons = counters
          .map( |(count, set_count)| {
            view!{
              <li>
                <button 
                on:click=move |_| set_count.update( |n| *n += 1 )
                >
                  {count}
                </button>
              </li>
            }
          }).collect::<Vec<_>>();
  view! {
    <ul>
      {counter_buttons}
    </ul>
  }
}
#[component]
fn DynamicList( initial_length: usize ) -> impl IntoView {
  let mut next_counter_id = initial_length;
  let initial_counters = (0..initial_length)
    .map(|id| (id, create_signal(id+1)))
    .collect::<Vec<_>>();
  let (counters, set_counters) = create_signal(initial_counters);
  let add_counter = move |_| {
    let sig = create_signal(next_counter_id + 1);
    set_counters.update( move |counters| {
      counters.push((next_counter_id, sig))
    });
    next_counter_id +=1;
  };
  view! {
    <div>
      <button on:click=add_counter>
        "Add Counter"
      </button>
      <ul>
        <For 
          each=counters
          key=|counter| counter.0
          children=move |(id, (count, set_count))|{
            view! {
                <li>
                  <button
                    on:click=move |_| set_count.update(|n| *n += 1)
                  >
                    {count}
                  </button>
                  <button
                    on:click=move |_| {
                      set_counters.update(|counters| {
                        counters.retain(|(counter_id, _)| counter_id != &id)
                      });
                    }
                  >
                    "Remove"
                  </button>

                </li>
            }
          }
        />
      </ul>
    </div>
  }
}

fn main() {
  leptos::mount_to_body(App)
}
```




```sh
Reactiity - Iteration and data structure in the frontend
```

```rust
use leptos::*;
/*
#[derive(Debug, Clone)]
struct DatabaseEntry{
    key: String,
    value: i32,
}
*/
/*
#[derive(Debug, Clone)]
struct DatabaseEntry{
    key: String,
    value: RwSignal<i32>,
}
*/
#[derive(Debug, Clone)]
struct DatabaseEntry{
    key: String,
    value: i32,
}

#[component]
fn App() -> impl IntoView {

  let (data, set_data) = create_signal(vec![
    DatabaseEntry {
        key: "Une".to_string(),
//        value: create_rw_signal(10),
        value: 100,
    },
    DatabaseEntry {
        key: "Due".to_string(),
//        value: create_rw_signal(20),
        value: 200,
    },
    DatabaseEntry {
        key: "Tre".to_string(),
//        value: create_rw_signal(30),
        value: 300,
    },
  ]);

  view! {
    <button on:click=move |_| {
        /*
      set_data.update(|data| {
        for row in data {
            row.value += 2;
        }
      });
            */
            /*
       data.with(|data|{
         for row in data {
            row.value.update(|value| *value *= 2);
         }
       });
       */
       set_data.update(|data| {
        for row in data {
            row.value += 2;
        }
      });

      logging::log!("{:?}", data.get());
    }>
      "Update Values"
    </button>

/*    <For each=data
//       key=|state| ( state.key.clone(), state.value ) 
         key=|state| state.key.clone()
         let:child
         >
         <p>{child.value}</p>
    </For>
*/
      <For
        each=move || data().into_iter().enumerate()
        key=|(_, state)| state.key.clone()
        children=move |(index, _)|{
          let value=create_memo( move |_| {
            data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
          });
          view! {
            <p>{value}</p>
          }
        }
      />
  }
}

fn main() {
    leptos::mount_to_body(App);
}

```

```sh
https://book.leptos.dev/view/05_forms.html


# web16_forms - Controlled / Uncontrolled Component

```


```rust
use leptos::{ev::SubmitEvent, *};

#[component]
fn App() -> impl IntoView {
  view! {
    <h2>"Controlled Component"</h2>
    <ControlledComponent />
    <h2>"Uncontrolled Component"</h2>
    <UncontrolledComponent />
  }
}
#[component]
fn ControlledComponent() -> impl IntoView {
    let (name, set_name) = create_signal( "Controlled".to_string() );
    view! {
      <input type="text"
        on:input=move |ev| {
          set_name( event_target_value(&ev) );
        }
        prop:value=name
      />
      <p>"Name is : " {name}</p>      
    }
}
#[component]
fn UncontrolledComponent() -> impl IntoView {
      use leptos::html::Input;
      let (name, set_name) = create_signal( "Uncontrolled".to_string() );
      let input_element: NodeRef<Input> = create_node_ref();
      let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element()
                    .expect("<input> to exist")
                    .value();
        set_name( value );
      };
    view! {
      <form on:submit=on_submit>
        <input type="text"
          value=name
          node_ref=input_element
        />
        <input type="submit" value="Submit this"/>
      </form>
      <p>"Name is : " {name}</p>
    }
}
fn main() {
    leptos::mount_to_body(App);
}
```


```sh
https://book.leptos.dev/view/06_control_flow.html

```

```rust
use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None }; 
    view! {
      <h1>"Control Flow"</h1>
      <button on:click=move |_| set_value.update(|n| *n += 1 ) >
      " +1 "
      </button>
      <p>"Value is : " { value }</p>
      <hr />
      <h2><code>"Option<T>"</code></h2>

      <p>{odd_text}</p>
      <p>{ move || odd_text().map(|text| text.len()) }</p>
      <p>"Conditional Logic"</p>
      <p>{ move || if is_odd() {
            "Odd!"
          }else{
            "Even!"
          }}</p>
      <p class:hidden=is_odd>"Appears if Even!"</p>
      <Show when=is_odd
            fallback=|| view! { <p>"Even Steven"</p>}>
        <p>"Oddment"</p>
      </Show>
      { move || is_odd().then( || view! { <p>"Oddity!"</p>} ) }
      <p>"Converting Between Types"</p>
      {
        move || match is_odd() {
            true if value() == 1 => {
                view! { <pre>"One"</pre> }.into_any()
            },
            false if value() == 2 => {
              view! { <p>"Two"</p> }.into_any()
            }
            _ => view! { <textarea>{value()}</textarea> }.into_any()
        }
      }
    }
}
fn main() {
  leptos::mount_to_body(App);
}
```

```sh
https://book.leptos.dev/view/07_errors.html

Error Handling
```

```rust
use leptos::*;
#[component]
fn App() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>()) ;
    view! {
        <h1>"Error Handling"</h1>
      <label>"Type a number (or something that's not a number!)"
        <input type="number" on:input=on_input />
        <ErrorBoundary
          fallback=|errors| view! {
            <div class="error">
              <p>"Not e number! Errors:"</p>
              <ul>
                { move || errors.get()
                    .into_iter()
                    .map(| ( _, e ) | view!{ <li> { e.to_string() } </li> } )
                    .collect::<Vec<_>>()

                }
              </ul>
            </div>
          }
        >
          <p>
            "You entered: "
            <strong> { value } </strong>
          </p>
        </ErrorBoundary>

      </label>

    }
}
fn main() {
  leptos::mount_to_body(App);
}

```

```sh
https://book.leptos.dev/view/09_component_children.html
```


```rust
use leptos::{ev::MouseEvent, *};
#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);
#[component]
fn App() -> impl IntoView {
  let (red, set_red) = create_signal(false);
  let (right, set_right) = create_signal(false);
  let (italics, set_italics) = create_signal(false);
  let (smallcaps, set_smallcaps) = create_signal(false);
  provide_context(SmallcapsContext(set_smallcaps));
  view! {
    <main>
      <p
        class:red=red
        class:right=right
        class:italics=italics
        class:smallcaps=smallcaps
      >
      "Writing down text into the paragraph"
      </p>
      <ButtonA setter=set_red />
      <ButtonB on_click=move |_| set_right.update( |value| *value = !*value ) />
      <ButtonC on:click=move |_| set_italics.update( |value| *value = !*value )  />
      <ButtonD />
    </main>
  }
}
#[component]
pub fn ButtonA(
    setter: WriteSignal<bool>,
) -> impl IntoView {
  view! {
    <button 
      on:click= move |_| setter.update( |value| *value = !*value ) 
    >
    "Toggle Red"
    </button>
  }
}
#[component]
pub fn ButtonB<F>(
    on_click: F,
) -> impl IntoView 
     where F: Fn(MouseEvent) + 'static,
{
  view! {
    <button on:click=on_click > "Toggle Right" </button>
  }  
}
#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>"Toggle Italics"</button>
    }
}
#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<SmallcapsContext>().unwrap().0;
    view! {
        <button
          on:click=move |_| setter.update(|value| *value = !*value ) 
        >"Toggle Small Caps"</button>
    }
}
fn main() {
  leptos::mount_to_body(App);
}
```


```sh
https://book.leptos.dev/view/09_component_children.html
```

```rust
use leptos::*;
#[component]
pub fn App() -> impl IntoView {
    let (items, set_items) = create_signal(vec![0, 1, 2]);
    let render_props = move || {
      let len = move || items.with(Vec::len);
      view! {
        <p>"Length: " {len}</p>
      }
    };
    view! {
        <TakesChildren
          render_prop=render_props
        >
      <p>"Here is a child!"</p>
      <p>"here is another child."</p>
      </TakesChildren>
      <WrapsChildren>
        <p>"Here is a child"</p>
        <p>"Here is another child."</p>
      </WrapsChildren>
    }
}

#[component]
pub fn TakesChildren<F, IV>(
    render_prop: F,
    children: Children,
) -> impl IntoView 
where 
  F: Fn() -> IV,
  IV: IntoView,   
{
    view! {
        <h1><code>"<TakesChildren>"</code></h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr/>
        <h2>"Children"</h2>
        {children()}
    }
}
#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    let children  = children()
      .nodes
      .into_iter()
      .map( |child| view! { <li>{child}</li> } )
      .collect::<Vec<_>>();
    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        <ul>{children}</ul>
    }
}
fn main() {
  leptos::mount_to_body(App);
}

```

```sh
https://book.leptos.dev/reactivity/index.html

https://book.leptos.dev/reactivity/14_create_effect.html

 - Signals - Observable - reactivity values
 - Effects - Observer   - pieces of code
```

```rust
use leptos::html::Input;
use leptos::*;

#[derive(Copy, Clone)]
struct LogContext( RwSignal<Vec<String>> );

#[component]
fn App() -> impl IntoView {
  let log = create_rw_signal::<Vec<String>>(vec![]);
  let logged = move || log().join("\n");
  provide_context(LogContext(log));
  view! {
    <CreateAnEffect/>
    <pre>{logged}</pre>
  }
}

#[component]
fn CreateAnEffect() -> impl IntoView {
  let (first, set_first) = create_signal(String::new());
  let (last, set_last)   = create_signal(String::new());
  let (use_last, set_use_last) = create_signal(true);
  create_effect( move |_| {
    log(if use_last() {
      with!(|first, last| format!("{first} {last}"))
    } else {
      first()
    })
  });
  view! {
    <h1>
      <code>"create effect"</code>
      "Version"
    </h1>
    <form>
      <label>
        "First Name"
        <input type="text" name="first" prop:value=first
          on:change=move |ev| set_first(event_target_value(&ev))  />
      </label>
      <label>
        "Last Name"
        <input type="text" name="last" prop:value=last 
          on:change=move |ev| set_last(event_target_value(&ev))  />
      </label>
      <label>
        "Show Last Name"
        <input type="checkbox" name="use_last" prop:checked=use_last
          on:change=move |ev| set_use_last(event_target_checked(&ev)) />
      </label>
    </form>
  }
}

#[component]
fn ManualVersion() -> impl IntoView {
    let first    = create_node_ref::<Input>();
    let last     = create_node_ref::<Input>();
    let use_last = create_node_ref::<Input>();
    let mut prev_name = String::new();
    let on_change = move |_| {
      log("      listener");
      let first    = first.get().unwrap();
      let last     = last.get().unwrap();
      let use_last = use_last.get().unwrap();
      let this_one = if use_last.checked() {
        format!("{} {}", first.value(), last.value())
      } else {
        first.value()
      };
      if this_one != prev_name {
        log(&this_one);
        prev_name = this_one;
      }
    };
    view! {
      <h1>
        "Manual Version"
      </h1>
      <form on:change=on_change>
        <label>"First Name" 
        <input type="text" name="first" node_ref=first />
        </label>
        <label>"Last Name" 
        <input type="text" name="last" node_ref=last />
        </label>
        <label>"Show Last Name" 
        <input type="checkbox" name="use_last" checked node_ref=use_last />
        </label>

      </form>
    }
}


#[component]
fn EffectVsDerivedSignal() -> impl IntoView {
    let (my_value, set_my_value) = create_signal(String::new());
    let my_optional_value = move || (!my_value.with(String::is_empty)).then(|| Some(my_value.get()));
    view! {
      <input prop:value=my_value on:input=move |ev| set_my_value(event_target_value(&ev)) />
      <p>
        <code>"my_optional_value"</code>
        " is "
        <code>
          <Show when=move || my_optional_value().is_some() fallback=|| view! { "None"} >
            "Some(\"" 
            { my_optional_value().unwrap() }
            "\")"
          </Show>
        </code>
      </p>
    }
}

#[component]
pub fn Show<F, W, IV>(
    children: Box<dyn Fn() -> Fragment>,
    when: W,
    fallback: F,
) -> impl IntoView 
     where W: Fn() -> bool + 'static,
     F: Fn() -> IV + 'static,
     IV: IntoView,
{
    let memorized_when = create_memo( move |_| when() );
    move || match memorized_when.get() {
        true  => children().into_view(),
        false => fallback().into_view(), 
    }
}
fn log(msg: impl std::fmt::Display){
    let log = use_context::<LogContext>().unwrap().0;
    log.update( |log| log.push(msg.to_string()) );  
}
fn main() {
  leptos::mount_to_body(App)
}

```

```sh
The key phrase here is “runs some kind of code.” The natural way to “run some kind of code” at an arbitrary point in time—in Rust or in any other programming language—is to call a function. And in fact every UI framework is based on rerunning some kind of function over and over:

1 - virtual DOM (VDOM) frameworks like React, Yew, or Dioxus rerun a component or render function over and over, to generate a virtual DOM tree that can be reconciled with the previous result to patch the DOM

2 - compiled frameworks like Angular and Svelte divide your component templates into “create” and “update” functions, rerunning the update function when they detect a change to the component’s state

3 - in fine-grained reactive frameworks like SolidJS, Sycamore, or Leptos, you define the functions that rerun

```

```sh
https://book.leptos.dev/testing.html
```


```sh
## TODO: here

```































































----------------------------------------------
----------------------------------------------
----------------------------------------------
## Yew
```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        <p>{ "Homelab Day 1: " }</p>
        <p>{ "Homelab Day 2: " }</p>
        <p>{ "Homelab Day 3: GitOps way of kubernetes " }</p>
      </div>
      <div>
        <h3>{ "Homelab Day 3: GitOps way of kubernetes " }</h3>
        <img src="https://via.placeholder.com/640x360.png?text=Homelab Day 3: GitOps way of kubernetes" alt="video thumbnail" />
      </div>
    </>
  }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

-----------


```rust
use yew::{prelude::*, virtual_dom::VNode};
#[function_component(App)]
fn app() -> Html {

  struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,  
  }  
  let videos: Vec<Video> = vec![
    Video {
      id: 1,
      title: "Homelab Day 1: proxmox".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 1".to_string(),
    },
    Video {
      id: 2,
      title: "Homelab Day 2: ".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 2".to_string(),
    },
    Video {
      id: 3,
      title: "Homelab Day 3: GitOps way of kubernetes".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 3".to_string(),
    },
  ];
  let videos: VNode = videos.iter().map(|video: &Video| html!{ 
    <>
    <p key={ video.id }>{format!(" {}: {} ", video.speaker, video.title)}</p>
    <img url={video.url.clone()} />    
    </>
  }).collect::<Html>();

  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        {videos}
      </div>
     </>
  }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
```

-----------


```rust
use yew::{prelude::*};
#[derive(Clone, PartialEq)]
struct Video {
  id: usize,
  title: String,
  speaker: String,
  url: String,  
}
#[derive(Properties, PartialEq)]
struct VideosListProps {
  videos: Vec<Video>,
}
#[function_component(VideosList)]
fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
  videos.iter().map(|video: &Video| html!{ 
    <>
    <p key={ video.id }>{format!(" {}: {} ", video.speaker, video.title)}</p>
    <img url={video.url.clone()} />    
    </>
  }).collect()
}
#[function_component(App)]
fn app() -> Html {
  let videos: Vec<Video> = vec![
    Video {
      id: 1,
      title: "Homelab Day 1: proxmox".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 1".to_string(),
    },
    Video {
      id: 2,
      title: "Homelab Day 2: ".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 2".to_string(),
    },
    Video {
      id: 3,
      title: "Homelab Day 3: GitOps way of kubernetes".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 3".to_string(),
    },
  ];  
  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        <VideosList videos={ videos } />
      </div>
     </>
  }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
```

-----------

#### Modules

```rust
use yew::{prelude::*};
use crate::video::*;
mod video {
  use yew::{prelude::*};
  #[derive(Clone, PartialEq)]
  pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,  
  }  
  #[derive(Properties, PartialEq)]
  pub struct VideosListProps {
    pub videos: Vec<Video>,
  }
  #[function_component(VideosList)]
  pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos.iter().map(|video: &Video| html!{ 
      <>
      <p key={ video.id }>{format!(" {}: {} ", video.speaker, video.title)}</p>
      <img url={video.url.clone()} />    
      </>
    }).collect()
  }
}
#[function_component(App)]
fn app() -> Html {  
  let videos: Vec<Video> = vec![
    Video {
      id: 1,
      title: "Homelab Day 1: proxmox".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v01homelab01proxmox.jpg".to_string(),
    },
    Video {
      id: 2,
      title: "Homelab Day 2: ".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v02homelab02.webp".to_string(),
    },
    Video {
      id: 3,
      title: "Homelab Day 3: GitOps way of kubernetes".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v03homelab03gitopskubernetes.webp".to_string(),
    },
  ];  
  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        <VideosList videos={ videos } />
      </div>
     </>
  }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
```

-----------

#### Modules in different files

- src/main.rs
```rust
use yew::{prelude::*};
mod video;
use video::video::*;
#[function_component(App)]
fn app() -> Html {  
  let videos: Vec<Video> = vec![
    Video {
      id: 1,
      title: "Homelab Day 1: proxmox".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v01homelab01proxmox.jpg".to_string(),
    },
    Video {
      id: 2,
      title: "Homelab Day 2: ".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v02homelab02.webp".to_string(),
    },
    Video {
      id: 3,
      title: "Homelab Day 3: GitOps way of kubernetes".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v03homelab03gitopskubernetes.webp".to_string(),
    },
  ];
  
  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        <VideosList videos={ videos } />
      </div>
     </>
  }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

- src/video/mod.rs
```rust
pub mod video {
    use yew::{prelude::*};  
    #[derive(Clone, PartialEq)]
    pub struct Video {
      pub id: usize,
      pub title: String,
      pub speaker: String,
      pub url: String,  
    }
    #[derive(Properties, PartialEq)]
    pub struct VideosListProps {
      pub videos: Vec<Video>,
    }    
    #[function_component(VideosList)]
    pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
      videos.iter().map(|video: &Video| html!{ 
        <>
        <p key={ video.id }>{format!(" {}: {} ", video.speaker, video.title)}</p>
        <img url={video.url.clone()} />    
        </>
      }).collect()
    }
  }
```

```sh
trunk serve
```

-----------

#### Modules 

