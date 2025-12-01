### Tasks

## TODO

-   if we could add hot reload to the css then it would be a game changer
-   bug is that if we update the rust code then it correctly shows that that we have an error and the css is not found, but when we update the css file, the rust code is uneffected and we have to restart the rust analyzer to make it work, even when i change other sections it doesn't change again, cause it assumes it to be correct
-   even changing the very exact rust code then changing it back doesn't show that it's wrong
-   even if can't validate the css, it still does make sense to have ourside css files cause it leads to better code organization
-   what if instead of this string checkined we just passed variables as classes,so

```
<div class="scoped-demo">
    <h2 class="demo-title">"Component A"</h2>
    <p class="primary-text">"This uses scoped CSS."</p>
</div>
```

then we can just pass variables as classes

```
<div class={scoped-demo}>
    <h2 class={demo-title}>"Component A"</h2>
    <p class={primary-text}>"This uses scoped CSS."</p>
</div>
```

if we do this then we can just have to figure out what is the best way to generate the css classes

rules for style tag 
- values must be double quoted
- we also check the validity of names
- can we check the validity of all values?
- so in the css i can write and this is valid, only breaking the name is not valid 

```
border: 1p x solid rgba(148, 163, 184, 0.1);
```

but also most importantly our variables wouldn't be usable as 

```
.scoped-demo {
    background: linear-gradient(
        135deg,
        rgba(30, 41, 59, 0.6) 0%,
        rgba(51, 65, 85, 0.6) 100%
    );
    border: 1px solid rgba(148, 163, 184, 0.1);
    border-radius: 1rem;
    padding: 2rem;
    margin-bottom: 2rem;
    backdrop-filter: blur(10px);
    position: relative;
    overflow: hidden;
    transition: all 0.3s ease;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}
```
scoped-demo alternate just like here
```
#[azumi::component]
pub fn component_b() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson5.css" />
        <div class="scoped-demo alternate">
            <h2 class="demo-title">"Component B"</h2>
            <p class="primary-text">"Same CSS, no conflicts!"</p>
        </div>
    }
}
```
but 
```
#[azumi::component]
pub fn component_b() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson5.css" />
        <div class={scoped-demo alternate}>
            <h2 class={demo-title}>
                "Component B"
            </h2>
            <p class={primary-text}>
                "Same CSS, no conflicts!"
            </p>
        </div>
    }
}
```

- this would mean we ban external css files, inline css, and classes with spaces, but also 


## Later

-   Style all lessons in a modern dark theme
    -   [x] style lesson 0
    -   [x] style lesson 1
    -   [x] style lesson 2
    -   [x] style lesson 3
    -   [x] style lesson 4
    -   [x] style lesson 5
    -   [x] style lesson 6
    -   [x] style lesson 7
    -   [x] style lesson 8
    -   [x] style lesson 9
    -   [x] style lesson 10
    -   [x] style lesson 11
    -   [x] style lesson 12
    -   [x] style lesson 13
    -   [x] style lesson 14
    -   [x] style lesson 15
    -   [x] style lesson 16
    -   [x] style lesson 17
    -   [x] style lesson 18
    -   [x] style lesson 19
    -   [x] style lesson 20
    -   [x] style lesson 21
    -   [x] style lesson 22
    -   [x] style lesson 23
    -   [x] style lesson 24
    -   [x] style lesson 25
    -   [x] style lesson 26
    -   [x] style lesson 27
    -   [x] style lesson 28
    -   [x] style lesson 29
    -   [x] style lesson 30
    -   [x] style lesson 31
    -   [x] style lesson 32
    -   [x] style lesson 33
    -   [x] style lesson 34

## Done
