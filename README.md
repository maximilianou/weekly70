# weekly70 - rust - 2024

-----------

## leptos.dev

-------------

https://book.leptos.dev/getting_started/index.html

```sh
apt install pkg-config
apt install libssl-dev

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

cargo info leptos


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
Async
```

```rust
use gloo_timers::future::TimeoutFuture;
use leptos::*;

async fn load_data(value: i32) -> i32 {
  TimeoutFuture::new(1_000).await;
  value * 10
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let async_data = create_resource(
        count,
        |value| async move { load_data(value).await },
    );
    let stable = create_resource(|| (), |_| async move {load_data(1).await});
    let async_result = move || {
        async_data
          .get()
          .map( | value | format!("Server returned {value:?}"))
          .unwrap_or_else( ||  "Loading..".into())
    };
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading.." } else { "Idle" };
    view! {
      <button
        on:click= move |_| {
            set_count.update(|n| *n += 1 );
        }
      >"Click me"</button>
      <p><code>"stable"</code>": " {move || stable.get()}</p>
      <p><code>"count"</code>": " {count}</p>
      <p><code>"async_value"</code>": " 
        {async_result}
      <br/>
        {is_loading}
      </p>
      
    }
}

fn main() {
  leptos::mount_to_body(App)
}

```

```sh
<Suspense>
```

```rust
use leptos::*;
use gloo_timers::future::TimeoutFuture;
async fn important_api_call(name: String) -> String {
  TimeoutFuture::new(1_000).await;
  name.to_ascii_uppercase()
}
#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal("Rene".to_string());
    let async_data = create_resource(
        name, |name| async move { important_api_call(name).await },
    ); 
    view! {
      <input
        on:input=move |ev| {
            set_name(event_target_value(&ev))
        }
        prop:value=name />
        <p><code>"name: "</code>{name}</p>
        <Suspense fallback=move || view! { <p>"Loading.."</p> } >
          <p>"Your shouting name is " 
            { move || async_data.get()}
          </p>
        </Suspense>
    }
}
fn main() {
  leptos::mount_to_body(App);
}

```



```rust
use gloo_timers::future::TimeoutFuture;
use leptos::*;
async fn important_api_call(id: usize) -> String {
  TimeoutFuture::new(1_000).await;
  match id {
    0 => "Luca",
    1 => "Paolo",
    2 => "Giovanni",
    _ => "User not found",
  }.to_string()
}
#[component]
fn App() -> impl IntoView {
    let (tab, set_tab) = create_signal(0);
    let user_data = create_resource(tab, |tab| async move { important_api_call(tab).await });
    view! {
      <div class="buttons">
      <button on:click=move |_| set_tab(0)
      class:selected=move || tab() == 0
    >"Secondo Tab A"</button>
    <button on:click=move |_| set_tab(1)
      class:selected=move || tab() == 1
    >"Terzo Tab A"</button>
    <button on:click=move |_| set_tab(2)
      class:selected=move || tab() == 2
    >"Primo Tab A"</button>
    { move || if user_data.loading().get(){
        "Loading.."
    }else{
        ""
    }}        
      </div>
      <Transition
        fallback=move || view! { <p>"Loading.."</p>}
      >
      <p>{move || user_data.get() } </p>
      </Transition>
    }
}
fn main() {
  leptos::mount_to_body(App);
}
```

```sh
https://book.leptos.dev/async/13_actions.html
```


```rust
use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, *};
use uuid::Uuid;
async fn add_todo(text: &str) -> Uuid{
    _ = text;
    TimeoutFuture::new(1_000).await;
    Uuid::new_v4()
}
#[component]
fn App() -> impl IntoView {
  let add_todo = create_action( |input: &String| {
    let input = input.to_owned();
    async move { add_todo(&input).await }
  });
  let submitted = add_todo.input();
  let pending   = add_todo.pending();
  let todo_id   = add_todo.value();
  let input_ref = create_node_ref::<Input>();
  view! {
    <form
      on:submit= move |ev| {
        ev.prevent_default();
        let input = input_ref.get().expect("input to exist");
        add_todo.dispatch( input.value() );
      }
    >
      <label>"What do you need to do ?"
        <input type="text" node_ref=input_ref />
      </label>
      <button type="submit" >"Add Todo"</button>
    </form>
    <p>{move || pending().then(|| "Loading..") }</p>
    <p>"Submitted"
      <code>{move || format!("{:#?}", submitted())}</code>
    </p>
    <p>"Pending"
      <code>{move || format!("{:#?}", pending())}</code>
    </p>
    <p>"Todo Id"
      <code>{move || format!("{:#?}", todo_id())}</code>
    </p>
  }
}
fn main() {
  leptos::mount_to_body(App)
}
```

```sh
Projecting Children
```

```sh
Global State Management
```

```rust
use leptos::*;

#[component]
fn Option2() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    provide_context(count);
    view! {
      <h1>"Option 2 : Passing Signals"</h1>
      <SetterButton set_count />
      <div style="display:flex">
        <FancyMath/>
        <ListItems/>
      </div>
    }
}
#[component]
fn SetterButton(set_count: WriteSignal<u32>) -> impl IntoView {
  view! {
    <div class="provider red" style="background-color:red;">
    <button on:click=move |_| set_count.update(|count| *count += 1) >
      "Increment Global Count"
    </button>
    </div>
  }
}
#[component]
fn FancyMath() -> impl IntoView {
    let count = use_context::<ReadSignal<u32>>()
    .expect("there to be a count signal provided");
    let is_even = move || count() & 1 == 0;
    view! {
        <div class="consumer blue">
          "the number "
          <strong>{count}</strong>
          {move || if is_even() {
            " is "
          }else{
            " is not "
          }}
          " even."
        </div>
    }
}
#[component]
fn ListItems() -> impl IntoView {
    let count = use_context::<ReadSignal<u32>>().expect("there to be a count signal provided");
    let squares = move || {
        (0..count())
          .map(|n| view! {<li>{n}<sup>"2"</sup>" is " {n*n}</li>})
          .collect::<Vec<_>>()
    };
    view! {
        <div class="consumer green">
          <ul>{squares}</ul>
        </div>
    }
}
#[derive(Default, Clone, Debug)]
pub struct GlobalState {
    count: u32,
    name: String,
}
#[component]
fn Option3() -> impl IntoView {
  let state = create_rw_signal(GlobalState::default());
  provide_context(state);
  view! {
    <h1>"Option 3: Passing signals"</h1>
    <div class="red consumer" style="with: 10%;background-color:red;">
      <h2>"Current Glocal State"</h2>
      <pre>
        {move || {
            format!("{:#?}", state.get())
        }}
      </pre>
    </div>
    <div style="display: flex">
      <GlobalStateCounter />
      <GlobalStateInput />
    </div>
  }  
}
#[component]
fn GlobalStateCounter() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");
    let (count, set_count) = create_slice(
        state,
        |state| state.count,
        |state, n| state.count = n,
    );
    view! {
      <div class="consume blue" style="background-color: blue;">
        <button
          on:click=move |_| {
            set_count( count() + 1 );
          }
        >"Incremental Global Count"</button>
        <br/>
        <span>"Count is: "{count}</span>
      </div>
    }
}
#[component]
fn GlobalStateInput() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");
    let (name, set_name) = create_slice(
        state,
        |state| state.name.clone(),
        |state, n| state.name = n,
    );
    view! {
      <div class="consumer green" style="background-color:green;">
        <input 
          type="text"
          prop:value=name
          on:input=move |ev| {
            set_name(event_target_value(&ev));
          }
        />
        <br/>
        <span>"Name is: " {name}</span>
      </div>
    }
}
fn main() {
    leptos::mount_to_body( || view! { <Option2/><Option3/> } )
}
```


```sh
Nested Routing

https://book.leptos.dev/router/17_nested_routing.html

reference: Practical Way of Conceptual Review the Remix Project with React explanation:
https://remix.run/

```

```rust
use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
  view! {
    <Router>
      <h1>"Contact App"</h1>
      <nav>
        <h2>"Navigation"</h2>
        <a href="/">"Home"</a>
        <a href="/contacts">"Contacts"</a>
      </nav>
      <main>
        <Routes>
          <Route path="/contacts" view=ContactList >
            <Route path=":id" view=ContactInfo >
              <Route path="" view=|| view! {
                <div class="tab">
                "(Contact Info)"
                </div>
              }/>
              <Route path="conversatios" view=|| view! {
                <div class="tab">
                "(Conversations)"
                </div>
              }/>
            </Route>
            <Route path="" view=|| view! {
              <div class="select-user">
                "Select a user to view contacts info."
              </div>
            } />            
          </Route>
        </Routes>
      </main>
    </Router>    
  }
}

#[component]
fn ContactList() -> impl IntoView {
  view! {
    <div class="contact-list">
      <div class="contact-list-contacts">
        <h3>"Contacts"</h3>
        <A href="juan">"Juan"</A>
        <A href="paolo">"Paolo"</A>
        <A href="luca">"Luca"</A>
      </div>
      <Outlet />
    </div>
  }
}
#[component]
fn ContactInfo() -> impl IntoView {
  let params = use_params_map();
  let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
  let name = move || match id().as_str() {
    "juan" => "Juan",
    "paolo" => "Paolo",
    "luca" => "Luca",
    _ => "User not found",
  };
  view! {
    <div class="contact-info">
      <h4>{name}</h4>
      <div class="tabs">
        <A href="" exact=true >"Contact Info"</A>
        <A href="conversations"  >"Conversations"</A>
      </div>
      <Outlet />
    </div>
  }
}
fn main() {
  leptos::mount_to_body(App);
}
```

```sh
cargo add leptos --features=csr,nightly
cargo add leptos_router --features=csr,nightly
trunk serve
```

```sh
Params and Queries
https://book.leptos.dev/router/18_params_and_queries.html
```

```rust
use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
    view! {
      <Router>
        <h1>"Contact App"</h1>
        <nav>
          <h2>"Navigation"</h2>
          <a href="/">"Home"</a>
          <a href="/contacts">"Contacts"</a>
        </nav>        
        <main>
          <Routes>
            <Route path="/" view=|| view! {
              <h3>"Home"</h3>
            } />
            <Route path="/contacts" view=ContactList >
              <Route path=":id" view=ContactInfo>
                <Route path="" view=|| view! {
                  <div class="tab">
                    "(Contact Info)"
                  </div>
                }/>
                <Route path="conversations" view=|| view! {
                  <div class="tab">
                    "(Conversations)"
                  </div>
                }/>        
              </Route>
              <Route path="" view=|| view!{
                <div class="select-user" >
                  "Select a user to view contact info"
                </div>
              }/>
            </Route>
          </Routes>
        </main>
      </Router>
    }
}

#[component]
fn ContactList() -> impl IntoView {
    view! {
        <div class="contact-list">
          <div class="contact-list-contacts">
            <h3>"Contacts"</h3>
            <A href="giovanni">"Giovanni"</A>
            <A href="pietro">"Pietro"</A>
            <A href="jonas">"Jonas"</A>
          </div>
          <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let name = move || match id().as_str() {
        "giovanni" => "Giovanni",
        "pietro" => "Pietro",
        "jonas" => "Jonas",
        _ => "User not found.",
    };
    view! {
        <div class="contact-info">
          <h4>{name}</h4>
          <div class="tabs">
            <A href="" exact=true>"Contact Info"</A>
            <A href="conversations">"Conversations"</A>
          </div>
          <Outlet />
        </div>
    }
}

fn main() {
  leptos::mount_to_body(App)
}
```



```sh
https://book.leptos.dev/router/19_a.html
```

```rust
use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView  {
    view! {
        <Router>
            <h1>"Contact App"</h1>
            <nav>
                <h2>"Navigation"</h2>
                <a href="/">"Home"</a>
                <a href="/contacts">"Contacts"</a>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=|| view! {
                        <h3>"Home"</h3>
                    }/>
                    <Route
                        path="/contacts"
                        view=ContactList
                      >
                        <Route path=":id" view=ContactInfo>
                            <Route path="" view=|| view! {
                                <div class="tab">
                                    "(Contact Info)"
                                </div>
                            }/>
                            <Route path="conversations" view=|| view! {
                                <div class="tab">
                                    "(Conversations)"
                                </div>
                            }/>
                        </Route>
                        <Route path="" view=|| view! {
                            <div class="select-user">
                                "Select a user to view contact info."
                            </div>
                        }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
#[component]
fn ContactList() -> impl IntoView {
    view! {
        <div class="contact-list">
            <div class="contact-list-contacts">
                <h3>"Contacts"</h3>
                <A href="mateo">"Mateo"</A>
                <A href="juan">"Juan"</A>
                <A href="judaz">"Judaz"</A>
            </div>
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let name = move || match id().as_str() {
        "juan" => "Juan",
        "mateo" => "Mateo",
        "judaz" => "Judaz",
        _ => "User not found.",
    };
    view! {
        <div class="contact-info">
            <h4>{name}</h4>
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>
           <Outlet/>
        </div>
    }
}
fn main() {
    leptos::mount_to_body(App)
}
```


```sh
Form
https://book.leptos.dev/router/20_form.html
```


```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Leptos Rust Web</title>
        <link data-trunk rel="rust" data-wasm-opt="0" data-keep-debug=true />
        <link rel="stylesheet" href="https://unpkg.com/open-props"/>

        <!-- optional imports that use the props -->
        <link rel="stylesheet" href="https://unpkg.com/open-props/normalize.min.css"/>
        <link rel="stylesheet" href="https://unpkg.com/open-props/buttons.min.css"/>
    
        <style>
            h1, h2, h3, h4, h5, h6 {
                margin-top: var(--size-4);
                text-align: center;
            }

            p {
                margin: var(--size-2);
            }

            body > picture, button, p, progress, label, input, div {
                display: block;
                margin-left: auto;
                margin-right: auto;
                text-align: center;
            }

            nav, .contact-list-contacts, .tabs {
                display: flex;
                justify-content: center;
            }

            nav a, .contact-list-contacts a, .tabs a {
                margin: var(--size-2);
            }

            img {
                height: 100px;
                width: auto;
                margin: auto;
            }
        </style>
    </head>
    <body>
    </body>
</html>
```

```rust
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {

    }
}
fn main() {
    leptos::move_to_body(App);
}

```

```rust
use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <h1><code>" <Form /> "</code></h1>
            <main>
              <Routes>
               <Route path="" view=FormExample />
              </Routes>
            </main>
        </Router>
    }
}
#[component]
fn FormExample() -> impl IntoView {
    let query  = use_query_map();
    let name   = move || query().get("name").cloned().unwrap_or_default();
    let number = move || query().get("number").cloned().unwrap_or_default();
    let select = move || query().get("select").cloned().unwrap_or_default();
    view! {
        <table>
          <tr>
            <td><code>"name"</code></td>
            <td>{name}</td>
          </tr>
          <tr>
            <td><code>"number"</code></td>
            <td>{number}</td>
          </tr>
          <tr>
            <td><code>"select"</code></td>
            <td>{select}</td>
          </tr>
        </table>
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="" >
          <input type="text" name="name" value=name />
          <input type="number" name="number" value=number />
          <select name="select">
            <option selected=move || select() == "A">
              "A"
            </option>
            <option selected=move || select() == "B">
              "B"
            </option>
            <option selected=move || select() == "C">
              "C"
            </option>
          </select>
          <input type="submit" />
        </Form>
        <h2>"Automatic submission"</h2>
        <Form method="GET" action="" >
        <input type="text" name="name" value=name 
        oninput="this.form.requestSubmit()" />
        <input type="number" name="number" value=number 
        oninput="this.form.requestSubmit()" />
        <select name="select" onchange="this.form.requestSubmit()">
        <option selected=move || select() == "A">"A"</option>
        <option selected=move || select() == "B">"B"</option>
        <option selected=move || select() == "C">"C"</option>
        </select>
        </Form>
    }
}
fn main() {
    leptos::mount_to_body(App);
}

```


```sh
https://book.leptos.dev/interlude_styling.html

https://github.com/abishekatp/stylers


```

```rust
use stylers::style;

#[component]
pub fn App() -> impl IntoView {
    let styler_class = style! { "App",
        #two{
            color: blue;
        }
        div.one{
            color: red;
            content: raw_str(r#"\hello"#);
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
        }
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
        h2 {
            color: purple;
        }
        @media only screen and (max-width: 1000px) {
            h3 {
                background-color: lightblue;
                color: blue
            }
        }
    };

    view! { class = styler_class,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
            <h2>"and"</h2>
            <h3>"friends!"</h3>
        </div>
    }
}
```

```sh
https://book.leptos.dev/web_sys.html

Server Side Rendering SSR
https://book.leptos.dev/ssr/index.html
https://book.leptos.dev/ssr/21_cargo_leptos.html

cargo install cargo-leptos

cargo leptos new --git leptos-rs/start
cargo leptos new --git leptos-rs/start-axum

rustup target add wasm32-unknown-unknown

cargo leptos watch

https://github.com/leptos-rs/cargo-leptos/blob/main/README.md 

```



```rust

//// src/app.rs
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web31-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

//// src/main.rs
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use web31_ssr::app::*;
    use web31_ssr::fileserv::file_and_error_handler;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}


//// src/lib.rs
pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}


//// src/fileserv.rs
use crate::app::App;
use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use leptos::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let (parts, body) = req.into_parts();

    let mut static_parts = parts.clone();
    static_parts.headers.clear();
    if let Some(encodings) = parts.headers.get("accept-encoding") {
        static_parts
            .headers
            .insert("accept-encoding", encodings.clone());
    }

    let res = get_static_file(Request::from_parts(static_parts, Body::empty()), &root)
        .await
        .unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream(options.to_owned(), App);
        handler(Request::from_parts(parts, body))
            .await
            .into_response()
    }
}

async fn get_static_file(
    request: Request<Body>,
    root: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root)
        .precompressed_gzip()
        .precompressed_br()
        .oneshot(request)
        .await
    {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error serving files: {err}"),
        )),
    }
}



//// src/error_template.rs
use http::status::StatusCode;
use leptos::*;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => create_rw_signal(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }

    view! {
        <h1>{if errors.len() > 1 {"Errors"} else {"Error"}}</h1>
        <For
            // a function that returns the items we're iterating over; a signal is fine
            each= move || {errors.clone().into_iter().enumerate()}
            // a unique key for each item as a reference
            key=|(index, _error)| *index
            // renders each item to a view
            children=move |error| {
                let error_string = error.1.to_string();
                let error_code= error.1.status_code();
                view! {
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }
}


/// end2end/tests/example.spec.ts
import { test, expect } from "@playwright/test";

test("homepage has title and heading text", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Welcome to Leptos");

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
});


```





```rust


/// src/app.rs
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web32-ssr-activx.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

/// src/lib.rs
pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}


//// src/main.rs
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use web32_ssr_activx::app::*;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use web32_ssr_activx::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}


/// end2end/tests/example.spec.ts
import { test, expect } from "@playwright/test";

test("homepage has title and links to intro page", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Welcome to Leptos");

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
});


```




```sh
Debian
```
```dockerfile
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# If you’re using stable, use this instead
# FROM rust:1.74-bullseye as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/leptos_start /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/leptos_start"]
```

```sh
Alpine
```
```dockerfile
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/leptos_start /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/leptos_start"]
```




```sh
https://book.leptos.dev/server/26_extractors.html
```


```sh
rustup update

rustup update
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2024-11-05, rust version 1.84.0-nightly (fbab78289 2024-11-04)
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
 19.1 MiB /  19.1 MiB (100 %)   4.8 MiB/s in  5s ETA:  0s
info: downloading component 'cargo'
  8.7 MiB /   8.7 MiB (100 %)   1.3 MiB/s in  2s ETA:  0s
info: downloading component 'clippy'
  2.8 MiB /   2.8 MiB (100 %)   1.3 MiB/s in  1s ETA:  0s
info: downloading component 'rust-docs'
 16.7 MiB /  16.7 MiB (100 %)   3.6 MiB/s in  4s ETA:  0s
info: downloading component 'rust-std'
...
```

```sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-generate
cargo leptos build --release
  target/server/release
  target/site

Copy these files to your remote server. The directory structure should be:
  leptos_start
  site/

Set the following environment variables (updating for your project as needed):
  export LEPTOS_OUTPUT_NAME="leptos_start"
  export LEPTOS_SITE_ROOT="site"
  export LEPTOS_SITE_PKG_DIR="pkg"
  export LEPTOS_SITE_ADDR="127.0.0.1:3000"
  export LEPTOS_RELOAD_PORT="3001"


```


```sh
cargo init web36-minimal
cargo add leptos --features=ssr,csr,hydrate,nightly
touch index.html
trunk serve

   Compiling leptos_dom v0.6.15
   Compiling leptos_server v0.6.15
error[E0609]: no field `children` on type `html::HtmlElement<El>`
   --> /home/debian/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos_dom-0.6.15/src/html.rs:628:30
    |
628 |             if matches!(self.children, ElementChildren::Chunks(_)) {
    |                              ^^^^^^^^ unknown field
```

```sh
rustup self uninstall
cargo install trunk

```

```sh
## TODO: here

[TODO] docker - local development docker compose
rustup self uninstall
cargo install trunk

rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-generate


apt -y install apt-transport-https software-properties-common ca-certificates curl gnupg lsb-release; echo  'deb [arch=amd64] https://download.docker.com/linux/debian  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable' | tee /etc/apt/sources.list.d/docker.list > /dev/null ;  install -m 0755 -d /etc/apt/keyrings ; curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg ; chmod a+r /etc/apt/keyrings/docker.gpg ; echo "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null ; apt -y update; apt -y remove docker docker-engine docker.io containerd runc; apt -y install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin ; usermod -aG docker dev01


touch Dockerfile
cargo init web38
cd web38
cargo add leptos --features=csr,nightly
cargo add leptos-router --features=csr,nightly
rustup target add wasm32-unknown-unknown
rustup toolchain install nightly
rustup install cargo-generate
docker build .

-------------


-------------
web39-docker
-------------


cargo leptos new -g leptos-rs/start-axum -n app
cargo leptos watch
curl http://localhost:3000/
cargo leptos build
./target/debug/app
curl http://localhost:3000/
cargo leptos test
cargo leptos build --release
./target/release/app
curl http://localhost:3000/
```

```sh
touch Dockerfile
```

```dockerfile
# ./Dockerfile
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bookworm AS builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/app .

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site ./site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml ./

# Set any required env variables and
ENV RUST_LOG="debug"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/app"]
```

```sh
docker build -t rustapp01 .
docker run -p 8080:8080 rustapp01
```




```sh
We can develop, run the docker compose to connect services over the internal network, and continue to change the code while developing, without shuting down the services.

Local Development Environmanent - 
Watch mode into the container over 
cargo leptos watch, 
dockerfile
docker compose up
```

```sh
/web40-docker/

docker compose up
```

```yml
# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Docker compose reference guide at
# https://docs.docker.com/reference/compose-file/

# Here the instructions define your application as a service called "server".
# This service is built from the Dockerfile in the current directory.
# You can add other services your application may depend on here, such as a
# database or a cache. For examples, see the Awesome Compose repository:
# https://github.com/docker/awesome-compose
services:
  server:
    build:
      context: .
      target: development
    ports:
      - 8080:8080
    environment:
      - PG_DBNAME=example
      - PG_HOST=db
      - PG_USER=postgres
      - PG_PASSWORD=mysecretpassword
      - ADDRESS=0.0.0.0:8000
      - RUST_LOG=debug
    volumes:
      - ./app:/app      
    # The commented out section below is an example of how to define a PostgreSQL
    # database that your application can use. `depends_on` tells Docker Compose to
    # start the database before your application. The `db-data` volume persists the
    # database data between container restarts. The `db-password` secret is used
    # to set the database password. You must create `db/password.txt` and add
    # a password of your choosing to it before running `docker compose up`.
#    depends_on:
#      db:
#        condition: service_healthy
#  db:
#    image: postgres
#    restart: always
#    user: postgres
#    secrets:
#      - db-password
#    volumes:
#      - db-data:/var/lib/postgresql/data
#    environment:
#      - POSTGRES_DB=example
#      - POSTGRES_PASSWORD_FILE=/run/secrets/db-password
#    expose:
#      - 5432
#    healthcheck:
#      test: ["CMD", "pg_isready"]
#      interval: 10s
#      timeout: 5s
#      retries: 5
#volumes:
#  db-data:
#secrets:
#  db-password:
#    file: db/password.txt
```

```dockerfile
# ./Dockerfile
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bookworm AS development

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY ./app .

# Set any required env variables and
ENV RUST_LOG="debug"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
CMD [ "cargo", "leptos", "watch" ]
```


```sh
[DONE] : docker-compose.yml development

https://github.com/maximilianou/weekly50/blob/main/docker/docker-compose.yml

[DONE] : Dockerfile.dev

https://github.com/maximilianou/weekly50/blob/main/docker/Dockerfile.dev.draft

[DONE] : Dockerfile.prod

https://github.com/maximilianou/weekly50/blob/main/docker/Dockerfile.prod.draft

```



```makefile
# ./Makefile
new-app:
	cargo leptos new -g leptos-rs/start-axum -n app 
	curl -o Dockerfile https://raw.githubusercontent.com/maximilianou/weekly70/refs/heads/main/web44-docker/Dockerfile
	curl -o Dockerfile.prod https://raw.githubusercontent.com/maximilianou/weekly70/refs/heads/main/web44-docker/Dockerfile.prod
	curl -o compose.yml https://raw.githubusercontent.com/maximilianou/weekly70/refs/heads/main/web44-docker/compose.yml

dev-app:
	docker compose up --remove-orphans

run-app:
	docker build -f Dockerfile.prod -t rust-app-prod .

del-app:
	rm -r app
	rm Dockerfile Dockerfile.prod compose.yml

clean:
	cd app && cargo clean
	docker container prune
	docker-compose up -d --remove-orphans
	# docker image ls
	# docker image rm rustapp01
	# docker image rm web40-docker-server
	# docker image rm web40-docker-app-dev
	# docker image rm rust-app-watch	
```


```yml
# ./compose.yml
services:
  server:
    build:
      context: .
      target: development
    user: appuser
    ports:
      - 8080:8080
#    environment:
#      - PG_DBNAME=${POSTGRES_DB}
#      - PG_HOST=postgres
#      - PG_USER=${POSTGRES_USER}
#      - PG_PASSWORD=${POSTGRES_PW}
#      - ADDRESS=0.0.0.0:8000
#      - RUST_LOG=debug
    volumes:
      - ./app:/app      
    depends_on:
      db:
        condition: service_healthy


  db:
    image: postgres
    environment:
      POSTGRES_USER: demo
      POSTGRES_PASSWORD: demo
      POSTGRES_DB: demo
    ports:
      - 5432:5432
    #user: postgres
    healthcheck:
      test: ["CMD", "pg_isready -U postgres -d demo"]
      interval: 10s
      timeout: 5s
      retries: 5

  adminer:
    image: adminer
    ports:
      - 3333:8080
```

```Dockerfile
# ./Dockerfile
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bookworm AS development

RUN groupadd -g 1000 appgroup && \
    useradd -m -u 1000 -g appgroup appuser 
RUN mkdir /app && chown appuser:appgroup /app
USER appuser
WORKDIR /app

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY ./app .

# Set any required env variables and
ENV RUST_LOG="debug"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
CMD [ "cargo", "leptos", "watch" ]
```

```dockerfile
# ./Dockerfile.prod
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bookworm AS builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY ./app .

# Build the app
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder ./app/target/release/app /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder ./app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder ./app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/app"]
```

```sh
make new-app
docker compose up
docker build -f Dockerfile.prod -t rust-app-prod .
```




```sh
cargo new bookstore	
cd bookstore
cargo add tokio --features=full
cargo add sqlx --features=postgres,runtime-tokio-rustls
cargo check
```

```sh
[package]
name = "bookstore"
version = "0.1.0"
edition = "2021"

[dependencies]
sqlx = { version = "0.8.2", features = ["postgres", "runtime-tokio-rustls"] }
tokio = { version = "1.41.1", features = ["full"] }
```

```rust
use std::error::Error;
use sqlx::Connection;
use sqlx::Row;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://demo:demo@localhost:5432/demo";
    let mut conn = sqlx::postgres::PgConnection::connect(url).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&mut conn)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);
    Ok(())
}
```

```sh
Database Connection Pool
```

```rust
use std::error::Error;
use sqlx::Row;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://demo:demo@localhost:5432/demo";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);
    Ok(())
}
```

```sh
cargo run
 1 + 1 = 2 
```

```sh
Database Create Table with migrattions
```


```sql
-- ./migrations/0001_bookstore_table.sql
CREATE TABLE book (
    title  varchar not null,
    author varchar not null,
    isbn   varchar not null
);

create unique index book_isbn_idx on book (isbn);
```

```rust
use std::error::Error;
use sqlx::Row;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://demo:demo@localhost:5432/demo";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);
    Ok(())
}
```

```sh
cargo install sqlx-cli
```

```sh
~/.cargo/bin/sqlx migrate build-script
```

```rust
// build.rs
// generated by `sqlx migrate build-script`
fn main() {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");
}
```




```rust
use std::error::Error;
use sqlx::Row;

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn updating(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = %1, author  = %2 WHERE isbn = $3";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://demo:demo@localhost:5432/demo";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);


    let book = Book{
        title:  "Men searching for meaning".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
    };
    create(&book, &pool).await?;

    let bookUpdate = Book{
        title:  "Il Dio Inconscio".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
    };


    updating(&bookUpdate, &bookUpdate.isbn, &pool).await?;

    Ok(())
}
```


```sh
create
update
select

```

```rust
use std::error::Error;
use sqlx::Row;

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn updating(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author  = $2 WHERE isbn = $3";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);
    let row = query.fetch_one(conn).await?;
//    let row = query.fetch_optional(conn).await?;
//    let row = query.fetch_all(conn).await?;
    let book = Book{
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    };
    Ok(book)
}

async fn read2(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);
    let maybe_row = query.fetch_optional(conn).await?;
//    let row = query.fetch_all(conn).await?;
    let book = maybe_row.map( |row| {Book{
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    }});
    Ok(book.unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://demo:demo@localhost:5432/demo";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);

/*
    let book = Book{
        title:  "Men searching for meaning".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
    };
    create(&book, &pool).await?;
*/
    let book_update = Book{
        title:  "Il Dio Inconscio".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
    };

    updating(&book_update, &book_update.isbn, &pool).await?;
    let book_read = read(&pool).await?;
    println!("book 1: {}", &book_read.title);
    let book_read2 = read2(&pool).await?;
    println!("book 2: {}", &book_read2.title);
    Ok(())
}
```


```sh
query return result of table, and print into pretty print.
```


```rust
use std::error::Error;
use sqlx::Row;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn updating(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author  = $2 WHERE isbn = $3";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";

    let query = sqlx::query_as::<_, Book>(q);
    
    let books = query.fetch_all(conn).await?;
    
    Ok(books)
}

async fn read2(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);
    let maybe_row = query.fetch_optional(conn).await?;
//    let row = query.fetch_all(conn).await?;
    let book = maybe_row.map( |row| {Book{
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    }});
    Ok(book.unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://demo:demo@localhost:5432/demo";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);

    let book_update = Book{
        title:  "Il Dio Inconscio".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
    };

    updating(&book_update, &book_update.isbn, &pool).await?;

    let book_read = read(&pool).await?;

    println!("{:?}", &book_read); // pretty print

    let book_read2 = read2(&pool).await?;

    println!("book 2: {}", &book_read2.title);

    Ok(())
}

```


```sh
create-bookstore-app db-01:	
	cargo new bookstore	

db-02:	
	cd bookstore && cargo add tokio --features=full
	cd bookstore && cargo add sqlx  --features=postgres,runtime-tokio-rustls
	cd bookstore && cargo check

db-03:	
	cd bookstore && cargo run

db-04:
	cd bookstore && mkdir migrations

db-05
	cargo install sqlx-cli

db-06
	~/.cargo/bin/sqlx migrate build-script
```

```rust
use std::error::Error;
use sqlx::Row;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn updating(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author  = $2 WHERE isbn = $3";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&isbn)
        .execute(pool)
        .await?;
    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query_as::<_, Book>(q);
    let books = query.fetch_all(conn).await?;    
    Ok(books)
}

async fn read2(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);
    let maybe_row = query.fetch_optional(conn).await?;
//    let row = query.fetch_all(conn).await?;
    let book = maybe_row.map( |row| {Book{
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    }});
    Ok(book.unwrap())
}

async fn first_steps() -> Result<(), Box<dyn Error>>{
    let url = "postgres://demo:demo@localhost:5432/demo";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    let sum: i32 = res.get("sum");
    println!(" 1 + 1 = {} ", sum);
    let book_update = Book{
        title:  "Il Dio Inconscio".to_string(),
        author: "Vicktor Frankl".to_string(),
        isbn:   "234-3-234-12345-2".to_string(),
    };
    updating(&book_update, &book_update.isbn, &pool).await?;
    let book_read = read(&pool).await?;
//    println!("book 1: {}", &book_read.title);
    println!("{:?}", &book_read);
    let book_read2 = read2(&pool).await?;
    println!("book 2: {}", &book_read2.title);
    Ok(())
}

async fn insert_book(
  book: Book, conn: &sqlx::PgPool
) -> Result<(), Box<dyn Error>>{
    let mut txn = conn.begin().await?
    let author_q = r"
        INSERT INTO author (name) VALUES ($1) RETURNING id
    ";
    let book_q = r"
        INSERT INTO book (title, author_id, isbn) VALUES ($1, $2, $3)
    ";
    let author_id: (i64,) = sqlx::query_as(author_q)
        .bind(&book.author)
        .fetch_one(&mut txn)
        .await?;
    sqlx::query(book_q)
        .bind(&book.title)
        .bind(&author_id.0)
        .bind(&book.isbn)
        .execute(&mut txn)
        .await?;
    txn.commit().await?;
    Ok(())
}
async fn second_steps() -> Result<(), Box<dyn Error>>{
    
  Ok( () )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {    
//    Ok( first_steps().await? )
    Ok( second_steps().await? )
}
```




```sh
[TODO] db
```


```sh
cargo leptos new --git leptos-rs/start
  [TODO] server side
  [TODO] client side

cargo leptos new --git leptos-rs/start-axum
  [TODO] server side
  [TODO] client side

[TODO] docker - TDD development docker compose
[TODO] docker - prod dockerfile
[TODO] docker - kubernetes deployment
[TODO] docker - CI -
[TODO] docker - CD -

```


![GitOps Continuous Delivery on Kubernetes with Flux LFS269 - Linux Fundation](https://github.com/maximilianou/weekly50/blob/main/share/maximiliano-usich-gitops-continuous-delivery-on-kubernetes-with-flux-lfs269-certificate.png "Maximiliano Usich - GitOps Continuous Delivery on Kubernetes with Flux LFS269 - Linux Fundation")

----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------




























































----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
----------------------------------------------
